use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

//================================================================

const PATH_SYSTEM: &str = "src/system/";

/// This function is responsible for parsing the src/system/ folder and finding every special comment
/// in the source code to then output it to the GitHub documentation and the Lua LSP definition file.
fn main() {
    let mut meta = Meta::new();

    // read every file in the API directory.
    for file in std::fs::read_dir(PATH_SYSTEM).unwrap() {
        // convert to string.
        let file = file.expect("build.rs: Could not unwrap file.");

        // get file path.
        let path = file.path();
        let path = path
            .to_str()
            .expect("build.rs: Could not convert file path to string.");

        // get file name.
        let name = file.file_name();
        let name = name
            .to_str()
            .expect("build.rs: Could not convert file name to string.");

        if name == "mod.rs" {
            continue;
        }

        // open file.
        let file = File::open(path)
            .unwrap_or_else(|_| panic!("build.rs: Could not open file \"{path}\"."));
        let file = BufReader::new(file).lines();

        let mut wiki = Wiki::new(name);

        // for every line in the file...
        for (i, line) in file.map_while(Result::ok).enumerate() {
            meta.parse(path, name, &line, i);
            wiki.parse(path, name, &line, i);
        }
    }
}

//================================================================

/// A representation of the meta.lua file. Will write all the JSON documentation in LuaLS format.
struct Meta {
    /// True if currently writing a class comment.
    class: bool,
    /// True if currently writing a entry comment.
    entry: bool,
    /// Working buffer for the current comment.
    line: String,
    /// The meta.lua file.
    file: BufWriter<File>,
}

impl Meta {
    const FILE: &'static str = "meta.lua";

    #[rustfmt::skip]
    const CLASS_HEADER: &'static str =
r#"---{info}
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/{path}#L{line})
---@class {name}
"#;

    #[rustfmt::skip]
    const CLASS_FOOTER: &'static str =
r#"{name} = {}

"#;

    #[rustfmt::skip]
    const FIELD: &'static str =
r#"---@field {name} {kind} # {info}
"#;

    #[rustfmt::skip]
    const ENTRY_HEADER: &'static str =
r#"---{info}
"#;

        #[rustfmt::skip]
    const ENTRY_FOOTER: &'static str =
r#"---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/{path}#L{line})
function {name}({member}) end

"#;

        #[rustfmt::skip]
    const PARAMETER: &'static str =
r#"---@param {name} {kind} # {info}
"#;
        #[rustfmt::skip]
    const RETURN: &'static str =
r#"---@return {name} {kind} # {info}
"#;

    /// Create a new instance.
    pub fn new() -> Self {
        let file = File::create(format!("src/asset/{}", Self::FILE))
            .unwrap_or_else(|_| panic!("build.rs: Could not create \"{}\" file.", Self::FILE));
        let file = BufWriter::new(file);

        Self {
            class: false,
            entry: false,
            line: String::new(),
            file,
        }
    }

    /// Parse a line from a file.
    pub fn parse(&mut self, path: &str, name: &str, text: &str, line: usize) {
        let text = text.trim();

        if text == "*/" {
            // We were in class mode; write a class out.
            if self.class {
                self.write_class(path, name, line);
            }

            // We were in entry mode; write a entry out.
            if self.entry {
                self.write_entry(path, name, line);
            }

            // Reset mode.
            self.class = false;
            self.entry = false;
            self.line.clear();
        }

        // We are currently writing either a class or an entry comment. Push a new line.
        if self.class || self.entry {
            self.line.push_str(text);
        }

        // Class comment. Enable class mode.
        if text == "/* class" {
            self.class = true;
        }

        // Entry comment. Enable entry mode.
        if text == "/* entry" {
            self.entry = true;
        }
    }

    fn write_class(&mut self, path: &str, _name: &str, line: usize) {
        let class: Class = serde_json::from_str(&self.line).unwrap_or_else(|_| {
            panic!("Meta::write_class(): Could not deserialize class. Path: {path}, Line: {line}, Text: {}", self.line)
        });

        let data = Self::CLASS_HEADER;
        let data = data.replace("{info}", &class.info);
        let data = data.replace("{path}", path);
        let mut data = data.replace("{line}", &format!("{line}"));

        if let Some(class_member) = class.member {
            for member in class_member {
                let field = Self::FIELD;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{kind}", &member.kind);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        data.push_str(Self::CLASS_FOOTER);
        let data = data.replace("{name}", &class.name);

        self.file
            .write_all(data.as_bytes())
            .expect("Meta::write_class(): Could not write to file.");
    }

    fn write_entry(&mut self, path: &str, _name: &str, line: usize) {
        let entry: Entry = serde_json::from_str(&self.line).unwrap_or_else(|_| {
            panic!("Meta::write_class(): Could not deserialize class. Path: {path}, Line: {line}, Text: {}", self.line)
        });

        let data = Self::ENTRY_HEADER;
        let mut data = data.replace("{info}", &entry.info);

        if let Some(entry_member) = &entry.member {
            for member in entry_member {
                let field = Self::PARAMETER;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{kind}", &member.kind);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        if let Some(entry_result) = &entry.result {
            for member in entry_result {
                let field = Self::RETURN;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{kind}", &member.kind);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        data.push_str(Self::ENTRY_FOOTER);
        let data = data.replace("{name}", &entry.name);
        let data = data.replace("{path}", path);
        let data = data.replace("{line}", &format!("{line}"));

        let mut data_member = String::new();

        if let Some(entry_member) = &entry.member {
            for (i, member) in entry_member.iter().enumerate() {
                data_member.push_str(&member.name);

                if i != entry_member.len() - 1 {
                    data_member.push_str(",");
                }
            }
        }

        let data = data.replace("{member}", &data_member);

        self.file
            .write_all(data.as_bytes())
            .expect("Meta::write_class(): Could not write to file.");
    }
}

//================================================================

/// A representation of the meta.lua file. Will write all the JSON documentation in LuaLS format.
struct Wiki {
    /// True if currently writing a class comment.
    class: bool,
    /// True if currently writing a entry comment.
    entry: bool,
    /// Working buffer for the current comment.
    line: String,
    /// The meta.lua file.
    file: BufWriter<File>,
}

impl Wiki {
    #[rustfmt::skip]
    const CLASS_HEADER: &'static str =
r#"## {name}

*Class* : **{info}**

"#;

    #[rustfmt::skip]
    const CLASS_FOOTER: &'static str =
r#"[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/{path}#L{line})

"#;

    #[rustfmt::skip]
    const FIELD: &'static str =
r#"* {name} : **{kind}** *{info}*

"#;

    #[rustfmt::skip]
    const ENTRY_HEADER: &'static str =
r#"## {name}

*Function* : **{info}**

"#;

        #[rustfmt::skip]
    const ENTRY_FOOTER: &'static str =
r#"[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/{path}#L{line})

"#;

        #[rustfmt::skip]
    const PARAMETER: &'static str =
r#"* {name} : **{kind}** *{info}*

"#;

        #[rustfmt::skip]
    const RETURN: &'static str =
r#"* {name} : **{kind}** *{info}*

"#;

    /// Create a new instance.
    pub fn new(name: &str) -> Self {
        let name = &name[0..name.len() - 3];

        let file = File::create(format!("../quiver.wiki/{name}.md"))
            .unwrap_or_else(|_| panic!("build.rs: Could not create \"{name}\" file."));
        let file = BufWriter::new(file);

        Self {
            class: false,
            entry: false,
            line: String::new(),
            file,
        }
    }

    /// Parse a line from a file.
    pub fn parse(&mut self, path: &str, name: &str, text: &str, line: usize) {
        let text = text.trim();

        if text == "*/" {
            // We were in class mode; write a class out.
            if self.class {
                self.write_class(path, name, line);
            }

            // We were in entry mode; write a entry out.
            if self.entry {
                self.write_entry(path, name, line);
            }

            // Reset mode.
            self.class = false;
            self.entry = false;
            self.line.clear();
        }

        // We are currently writing either a class or an entry comment. Push a new line.
        if self.class || self.entry {
            self.line.push_str(text);
        }

        // Class comment. Enable class mode.
        if text == "/* class" {
            self.class = true;
        }

        // Entry comment. Enable entry mode.
        if text == "/* entry" {
            self.entry = true;
        }
    }

    fn write_class(&mut self, path: &str, _name: &str, line: usize) {
        let class: Class = serde_json::from_str(&self.line).unwrap_or_else(|_| {
            panic!("Wiki::write_class(): Could not deserialize class. Path: {path}, Line: {line}, Text: {}", self.line)
        });

        let data = Self::CLASS_HEADER;
        let data = data.replace("{name}", &class.name);
        let mut data = data.replace("{info}", &class.info);

        if let Some(class_member) = class.member {
            data.push_str("*Field list:*");

            for member in class_member {
                let field = Self::FIELD;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{kind}", &member.kind);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        data.push_str(Self::CLASS_FOOTER);
        let data = data.replace("{path}", path);
        let data = data.replace("{line}", &format!("{line}"));

        self.file
            .write_all(data.as_bytes())
            .expect("Wiki::write_class(): Could not write to file.");
    }

    fn write_entry(&mut self, path: &str, _name: &str, line: usize) {
        let entry: Entry = serde_json::from_str(&self.line).unwrap_or_else(|_| {
            panic!("Wiki::write_class(): Could not deserialize class. Path: {path}, Line: {line}, Text: {}", self.line)
        });

        let data = Self::ENTRY_HEADER;
        let mut data = data.replace("{info}", &entry.info);

        if let Some(entry_member) = &entry.member {
            data.push_str("*Parameter list:*\n");

            for member in entry_member {
                let field = Self::PARAMETER;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{kind}", &member.kind);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        if let Some(entry_result) = &entry.result {
            data.push_str("*Return list:*\n");

            for member in entry_result {
                let field = Self::RETURN;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{kind}", &member.kind);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        data.push_str(Self::ENTRY_FOOTER);
        let data = data.replace("{name}", &entry.name);
        let data = data.replace("{path}", path);
        let data = data.replace("{line}", &format!("{line}"));

        let mut data_member = String::new();

        if let Some(entry_member) = &entry.member {
            for (i, member) in entry_member.iter().enumerate() {
                data_member.push_str(&member.name);

                if i != entry_member.len() - 1 {
                    data_member.push_str(",");
                }
            }
        }

        let data = data.replace("{member}", &data_member);

        self.file
            .write_all(data.as_bytes())
            .expect("Wiki::write_class(): Could not write to file.");
    }
}

//================================================================

/// A representation of a Lua class.
#[derive(Deserialize, Serialize)]
struct Class {
    pub name: String,
    pub info: String,
    pub member: Option<Vec<Variable>>,
}

/// A representation of a Lua function.
#[derive(Deserialize, Serialize)]
struct Entry {
    pub name: String,
    pub info: String,
    pub member: Option<Vec<Variable>>,
    pub result: Option<Vec<Variable>>,
}

/// A representation of a Lua variable.
#[derive(Deserialize, Serialize)]
struct Variable {
    pub name: String,
    pub info: String,
    pub kind: String,
}
