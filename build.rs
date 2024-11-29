use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

//================================================================

const PATH_SYSTEM: &str = "src/system/";

/// This function is responsible for parsing the src/system/ folder and finding every special comment
/// in the source code to then output it to the GitHub documentation and the Lua LSP definition file.
fn main() {
    // Create parser object.
    let mut parser = Parser::new();

    // Read every file in the API directory.
    for file in std::fs::read_dir(PATH_SYSTEM).unwrap() {
        // Convert to string.
        let file = file.expect("build.rs: Could not unwrap file.");

        // Get file path.
        let path = file.path();
        let path = path
            .to_str()
            .expect("build.rs: Could not convert file path to string.");

        // Get file name.
        let name = file.file_name();
        let name = name
            .to_str()
            .expect("build.rs: Could not convert file name to string.");

        // Don't parse a mod.rs file.
        if name == "mod.rs" {
            continue;
        }

        // Open file.
        let file = File::open(path)
            .unwrap_or_else(|_| panic!("build.rs: Could not open file \"{path}\"."));
        let file = BufReader::new(file).lines();

        // Create GitHub wiki documentation file.
        parser.new_wiki(name);

        // For each line in the file...
        for (i, line) in file.map_while(Result::ok).enumerate() {
            parser.parse(path, name, &line, i);
            //wiki.parse(path, name, &line, i);
        }
    }
}

//================================================================

struct Parser {
    /// True if currently writing a class comment.
    class: bool,
    /// True if currently writing a entry comment.
    entry: bool,
    /// True if currently in example code.
    example: bool,
    /// Working buffer for the current comment.
    comment_line: String,
    /// Working buffer for the current example.
    example_line: String,
    /// The wiki file.
    wiki_file: Option<BufWriter<File>>,
    /// The meta file.
    meta_file: BufWriter<File>,
}

impl Parser {
    const META_FILE: &'static str = "meta.lua";

    #[rustfmt::skip]
    const META_FILE_HEADER: &'static str =
r#"---@meta

---The Quiver API.
---@class quiver
quiver = {}

"#;

    #[rustfmt::skip]
    const META_CLASS_HEADER: &'static str =
r#"---{info}
"#;

    #[rustfmt::skip]
    const META_CLASS_FOOTER: &'static str =
r#"---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/{path}#L{line})
---@class {name}
{name} = {}

"#;

    #[rustfmt::skip]
    const META_FIELD: &'static str =
r#"---@field {name} {kind} # {info}
"#;

    #[rustfmt::skip]
    const META_ENTRY_HEADER: &'static str =
r#"---{info}
"#;

        #[rustfmt::skip]
    const META_ENTRY_FOOTER: &'static str =
r#"---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/{path}#L{line})
function {name}({member}) end

"#;

        #[rustfmt::skip]
    const META_PARAMETER: &'static str =
r#"---@param {name} {kind} # {info}
"#;
        #[rustfmt::skip]
    const META_RETURN: &'static str =
r#"---@return {kind} {name} # {info}
"#;

    //================================================================

    #[rustfmt::skip]
    const WIKI_CLASS_HEADER: &'static str =
r#"## {name}

```lua
{code}
```

{info}

"#;

    #[rustfmt::skip]
    const WIKI_CLASS_FOOTER: &'static str =
r#"[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/{path}#L{line})

"#;

    #[rustfmt::skip]
    const WIKI_FIELD: &'static str =
r#"* Field: `{name}` – {info}

"#;

    #[rustfmt::skip]
    const WIKI_ENTRY_HEADER: &'static str =
r#"## {name}

```lua
{code}
```

{info}

"#;

        #[rustfmt::skip]
    const WIKI_ENTRY_FOOTER: &'static str =
r#"[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/{path}#L{line})

"#;

        #[rustfmt::skip]
    const WIKI_PARAMETER: &'static str =
r#"* Parameter: `{name}` – {info}

"#;

        #[rustfmt::skip]
    const WIKI_RETURN: &'static str =
r#"* Return: `{name}` – {info}

"#;

    /// Create a new instance.
    pub fn new() -> Self {
        let meta_file = File::create(format!("src/asset/{}", Self::META_FILE))
            .unwrap_or_else(|_| panic!("build.rs: Could not create \"{}\" file.", Self::META_FILE));
        let mut meta_file = BufWriter::new(meta_file);

        meta_file
            .write_all(Self::META_FILE_HEADER.as_bytes())
            .expect("Meta::new(): Could not write to file.");

        Self {
            class: false,
            entry: false,
            example: false,
            comment_line: String::new(),
            example_line: String::new(),
            wiki_file: None,
            meta_file,
        }
    }

    /// Create a new instance.
    pub fn new_wiki(&mut self, name: &str) {
        let name = &name[0..name.len() - 3];

        let file = File::create(format!("../quiver.wiki/{name}.md"))
            .unwrap_or_else(|_| panic!("build.rs: Could not create \"{name}\" file."));
        let file = BufWriter::new(file);

        self.wiki_file = Some(file);
    }

    /// Parse a line from a file.
    pub fn parse(&mut self, path: &str, name: &str, text: &str, line: usize) {
        let text = text.trim();

        if text == "*/" {
            // We were in class mode; write a class out.
            if self.class {
                self.write_meta_class(path, name, line);
                self.write_wiki_class(path, name, line);
            }

            // We were in entry mode; write a entry out.
            if self.entry {
                self.write_meta_entry(path, name, line);
                self.write_wiki_entry(path, name, line);
            }

            // Reset mode.
            self.class = false;
            self.entry = false;
            self.example = false;
            self.comment_line.clear();
            self.example_line.clear();
        }

        // We are currently writing an example comment. Push a new line.
        if self.example {
            self.example_line.push_str(text);
            self.example_line.push_str("\n");
        }

        // Example comment. Enable example mode.
        if text == "example" {
            self.example = true;
        }

        // We are currently writing either a class or an entry comment, and we are not currently writing an example comment. Push a new line.
        if (self.class || self.entry) && !self.example {
            self.comment_line.push_str(text);
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

    fn write_meta_class(&mut self, path: &str, _name: &str, line: usize) {
        let class: Class = serde_json::from_str(&self.comment_line).unwrap_or_else(|_| {
            panic!("Meta::write_meta_class(): Could not deserialize class. Path: {path}, Line: {line}, Text: {}", self.comment_line)
        });

        let mut data = Self::META_CLASS_HEADER.to_string();
        data = data.replace("{info}", &class.info);

        if self.example {
            data.push_str("---```lua\n");

            let split: Vec<&str> = self.example_line.split("\n").collect();

            for text in split {
                data.push_str(&format!("---{text}\n"));
            }

            data.push_str("---```\n");
        }

        if let Some(class_member) = class.member {
            for member in class_member {
                let field = Self::META_FIELD;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{kind}", &member.kind);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        data.push_str(Self::META_CLASS_FOOTER);
        data = data.replace("{name}", &class.name);
        data = data.replace("{path}", path);
        data = data.replace("{line}", &format!("{}", line + 2));

        self.meta_file
            .write_all(data.as_bytes())
            .expect("Meta::write_meta_class(): Could not write to file.");
    }

    fn write_meta_entry(&mut self, path: &str, _name: &str, line: usize) {
        let entry: Entry = serde_json::from_str(&self.comment_line).unwrap_or_else(|_| {
            panic!("Meta::write_meta_entry(): Could not deserialize class. Path: {path}, Line: {line}, Text: {}", self.comment_line)
        });

        let mut data = Self::META_ENTRY_HEADER.to_string();
        data = data.replace("{info}", &entry.info);

        if self.example {
            data.push_str("---```lua\n");

            let split: Vec<&str> = self.example_line.split("\n").collect();

            for text in split {
                data.push_str(&format!("---{text}\n"));
            }

            data.push_str("---```\n");
        }

        if let Some(entry_member) = &entry.member {
            for member in entry_member {
                let field = Self::META_PARAMETER;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{kind}", &member.kind);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        if let Some(entry_result) = &entry.result {
            for member in entry_result {
                let field = Self::META_RETURN;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{kind}", &member.kind);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        data.push_str(Self::META_ENTRY_FOOTER);
        data = data.replace("{name}", &entry.name);
        data = data.replace("{path}", path);
        data = data.replace("{line}", &format!("{}", line + 2));

        let mut data_member = String::new();

        if let Some(entry_member) = &entry.member {
            for (i, member) in entry_member.iter().enumerate() {
                data_member.push_str(&member.name);

                if i != entry_member.len() - 1 {
                    data_member.push(',');
                }
            }
        }

        data = data.replace("{member}", &data_member);

        self.meta_file
            .write_all(data.as_bytes())
            .expect("Meta::write_meta_entry(): Could not write to file.");
    }

    //================================================================

    fn write_wiki_class(&mut self, path: &str, _name: &str, line: usize) {
        let class: Class = serde_json::from_str(&self.comment_line).unwrap_or_else(|_| {
            panic!("Wiki::write_wiki_class(): Could not deserialize class. Path: {path}, Line: {line}, Text: {}", self.comment_line)
        });

        let mut data = Self::WIKI_CLASS_HEADER.to_string();
        data = data.replace("{name}", &class.name);
        data = data.replace("{code}", &format!("{} = {{}}", class.name));
        data = data.replace("{info}", &class.info);

        if self.example {
            data.push_str("```lua\n");
            data.push_str(&self.example_line);
            data.push_str("```\n\n");
        }

        if let Some(class_member) = class.member {
            for member in class_member {
                let field = Self::WIKI_FIELD;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        data.push_str(Self::WIKI_CLASS_FOOTER);
        data = data.replace("{path}", path);
        data = data.replace("{line}", &format!("{}", line + 2));

        self.wiki_file
            .as_mut()
            .unwrap()
            .write_all(data.as_bytes())
            .expect("Wiki::write_wiki_class(): Could not write to file.");
    }

    fn write_wiki_entry(&mut self, path: &str, _name: &str, line: usize) {
        let entry: Entry = serde_json::from_str(&self.comment_line).unwrap_or_else(|_| {
            panic!("Wiki::write_wiki_entry(): Could not deserialize class. Path: {path}, Line: {line}, Text: {}", self.comment_line)
        });

        let mut data = Self::WIKI_ENTRY_HEADER.to_string();
        data = data.replace("{info}", &entry.info);

        let mut data_member = String::new();

        if let Some(entry_member) = &entry.member {
            for (i, member) in entry_member.iter().enumerate() {
                data_member.push_str(&format!("{} : {}", member.name, member.kind));

                if i != entry_member.len() - 1 {
                    data_member.push_str(", ");
                }
            }
        }

        let mut data_result = String::new();

        if let Some(entry_result) = &entry.result {
            for (i, result) in entry_result.iter().enumerate() {
                let i = if i == 0 { "->" } else { &format!("{i}.") };

                data_result.push_str(&format!("\n  {} {} : {}", i, result.name, result.kind));
            }
        }

        data = data.replace(
            "{code}",
            &format!("function {}({}){}", entry.name, data_member, data_result),
        );

        if self.example {
            data.push_str("```lua\n");
            data.push_str(&self.example_line);
            data.push_str("```\n\n");
        }

        if let Some(entry_member) = &entry.member {
            for member in entry_member {
                let field = Self::WIKI_PARAMETER;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        if let Some(entry_result) = &entry.result {
            for member in entry_result {
                let field = Self::WIKI_RETURN;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        data.push_str(Self::WIKI_ENTRY_FOOTER);
        data = data.replace("{name}", &entry.name);
        data = data.replace("{path}", path);
        data = data.replace("{line}", &format!("{}", line + 2));

        self.wiki_file
            .as_mut()
            .unwrap()
            .write_all(data.as_bytes())
            .expect("Wiki::write_wiki_entry(): Could not write to file.");
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
