use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use serde::{Deserialize, Serialize};

//================================================================

#[derive(Deserialize, Serialize)]
pub struct Field {
    pub name: String,
    pub info: String,
    pub r#type: String,
}

#[derive(Deserialize, Serialize)]
pub struct Class {
    pub name: String,
    pub info: String,
    pub field: Option<Vec<Field>>,
}

impl Class {
    pub const CLASS_HEADER: &'static str = r#"## {name}

*{info}*
"#;

    pub const FIELD_HEADER: &'static str = r#"
**Field list:**
"#;

    pub const FIELD: &'static str = r#"
* {name} **{type}**
  - *{info}*
"#;

    pub const QUIVER: &'static str = r#"{ "name": "quiver", "info": "The main Quiver API." }"#;

    pub fn to_wiki(data: &str, file: &str) -> String {
        let mut result = String::new();

        let data: Self = serde_json::from_str(data)
            .map_err(|e| {
                panic!(
                    "\nbuild.rs: Error deserializing class in file \"{file}\": \n\n{data} \n{e}\n"
                )
            })
            .unwrap();

        let mut wiki_header = Self::CLASS_HEADER.to_string();
        wiki_header = wiki_header.replace("{name}", &data.name);
        wiki_header = wiki_header.replace("{info}", &data.info);

        result.push_str(&wiki_header);

        if let Some(parameter) = &data.field {
            result.push_str(Self::FIELD_HEADER);

            for i in parameter {
                let mut j = Self::FIELD.to_string();
                j = j.replace("{name}", &i.name);
                j = j.replace("{type}", &i.r#type);
                j = j.replace("{info}", &i.info);

                result.push_str(&j);
            }
        }

        result.push('\n');

        result
    }

    pub fn to_meta(data: &str, file: &str) -> String {
        let mut result = String::new();

        let data: Self = serde_json::from_str(data)
            .map_err(|e| {
                panic!(
                    "\nbuild.rs: Error deserializing function in file \"{file}\": \n\n{data} \n{e}\n"
                )
            })
            .unwrap();

        result.push_str(&format!("---{}\n", data.info));
        result.push_str(&format!("---@class {}\n", data.name));

        if let Some(field) = &data.field {
            for i in field {
                result.push_str(&format!("---@field {} {} # {}\n", i.name, i.r#type, i.info));
            }
        }

        result.push_str(&format!("{} = {{}}\n\n", data.name));

        result
    }
}

#[derive(Deserialize, Serialize)]
pub struct Parameter {
    pub optional: bool,
    pub name: String,
    pub info: String,
    pub r#type: String,
}

#[derive(Deserialize, Serialize)]
pub struct Function {
    pub name: String,
    pub info: String,
    pub parameter: Option<Vec<Parameter>>,
    pub r#return: Option<Vec<Parameter>>,
}

impl Function {
    pub const FUNCTION_HEADER: &'static str = r#"## {name}

*{info}*

[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/{file}.rs#L{line})

```lua
{code}
```
"#;

    pub const PARAMETER_HEADER: &'static str = r#"
**Parameter list:**
"#;

    pub const RETURN_HEADER: &'static str = r#"
**Return list:**
"#;

    pub const PARAMETER: &'static str = r#"
* {name} **{type}** {optional}
  - *{info}*
"#;

    pub fn to_wiki(data: &str, file: &str, line: usize) -> String {
        let mut result = String::new();

        let data: Self = serde_json::from_str(data)
            .map_err(|e| {
                panic!(
                    "\nbuild.rs: Error deserializing function in file \"{file}\": \n\n{data} \n{e}\n"
                )
            })
            .unwrap();

        let mut wiki_header = Self::FUNCTION_HEADER.to_string();
        wiki_header = wiki_header.replace("{name}", &data.name);
        wiki_header = wiki_header.replace("{info}", &data.info);
        wiki_header = wiki_header.replace("{file}", &file.to_lowercase());
        wiki_header = wiki_header.replace("{line}", &format!("{line}"));
        let mut code = data.name.clone();

        code.push('(');

        if let Some(parameter) = &data.parameter {
            for (i, j) in parameter.iter().enumerate() {
                code.push_str(&j.name);

                if i != parameter.len() - 1 {
                    code.push_str(", ");
                }
            }
        }

        code.push(')');

        if let Some(r#return) = &data.r#return {
            code.push_str(" ->\n");

            for (i, j) in r#return.iter().enumerate() {
                code.push_str(&format!("   {} : {}", j.name, j.r#type));

                if i != r#return.len() - 1 {
                    code.push_str("\n");
                }
            }
        }

        wiki_header = wiki_header.replace("{code}", &code);

        result.push_str(&wiki_header);

        if let Some(parameter) = &data.parameter {
            result.push_str(Self::PARAMETER_HEADER);

            for i in parameter {
                let mut j = Self::PARAMETER.to_string();
                j = j.replace("{optional}", if i.optional { "*Optional*" } else { "" });
                j = j.replace("{name}", &i.name);
                j = j.replace("{type}", &i.r#type);
                j = j.replace("{info}", &i.info);

                result.push_str(&j);
            }
        }

        if let Some(r#return) = &data.r#return {
            result.push_str(Self::RETURN_HEADER);

            for i in r#return {
                let mut j = Self::PARAMETER.to_string();
                j = j.replace("{optional}", if i.optional { "*Optional*" } else { "" });
                j = j.replace("{name}", &i.name);
                j = j.replace("{type}", &i.r#type);
                j = j.replace("{info}", &i.info);

                result.push_str(&j);
            }
        }

        result.push('\n');

        result
    }

    pub fn to_meta(data: &str, file: &str, line: usize) -> String {
        let mut result = String::new();

        let data: Self = serde_json::from_str(data)
            .map_err(|e| {
                panic!(
                    "\nbuild.rs: Error deserializing function in file \"{file}\": \n\n{data} \n{e}\n"
                )
            })
            .unwrap();

        result.push_str(&format!("---{}\n", data.info));

        if let Some(parameter) = &data.parameter {
            for i in parameter {
                result.push_str(&format!(
                    "---@param {}{} {} {}\n",
                    i.name,
                    if i.optional { "?" } else { "" },
                    i.r#type,
                    i.info
                ));
            }
        }

        if let Some(r#return) = &data.r#return {
            for i in r#return {
                result.push_str(&format!(
                    "---@return {} {} # {}\n",
                    i.r#type, i.name, i.info
                ));
            }
        }

        result.push_str(&format!("--- ---\n"));
        result.push_str(&format!("---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/{}.rs#L{line})\n", file.to_lowercase()));

        result.push_str(&format!("function {}", data.name));

        result.push('(');

        if let Some(parameter) = &data.parameter {
            for (i, j) in parameter.iter().enumerate() {
                result.push_str(&j.name);

                if i != parameter.len() - 1 {
                    result.push_str(", ");
                }
            }
        }

        result.push_str(") end\n\n");

        result
    }
}

/// responsible for writing:
/// * base.lua: Quiver's standard Lua library (Vector2, Vector3, Box2, Box3, etc.)
/// * meta.lua: Lua LSP documentation
/// * {system}.md: GitHub wiki documentation for each system
fn main() {
    // this SHOULD match the engine's internal meta/base file file-name.
    let meta_name = "meta.lua";
    let base_name = "base.lua";

    // this SHOULD match the engine's path for the Rust-Lua API.
    let system_path = "src/system/";

    //================================================================

    // create the meta.lua/base.lua file in the asset folder.
    let meta = File::create(format!("src/asset/{meta_name}"))
        .unwrap_or_else(|_| panic!("build.rs: Could not create \"{meta_name}\" file."));
    let base = File::create(format!("src/asset/{base_name}"))
        .unwrap_or_else(|_| panic!("build.rs: Could not create \"{base_name}\" file."));
    let mut meta = BufWriter::new(meta);
    let mut base = BufWriter::new(base);
    let mut function_write = false;
    let mut class_write = false;
    let mut base_write = false;

    //================================================================

    // write the ---@meta header for meta.lua.
    meta.write_all(b"---@meta\n\n").unwrap();

    meta.write_all(Class::to_meta(Class::QUIVER, "").as_bytes())
        .unwrap();

    // read every file in the API directory.
    for file in std::fs::read_dir(system_path).unwrap() {
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

        let name = &name[0..name.len() - 3];

        // don't document mod.rs
        if name == "mod" {
            continue;
        }

        let name = &format!("{}{}", &name[0..1].to_uppercase(), &name[1..name.len()]);

        // create the wiki documentation file in the quiver.wiki repository.
        let wiki = File::create(format!("../quiver.wiki/{name}.md")).unwrap_or_else(|_| {
            panic!(
                "build.rs: Could not create wiki documentation file \"../quiver.wiki/{name}.md\" file."
            )
        });
        let mut wiki = BufWriter::new(wiki);

        // current working meta + wiki string.
        let mut work = String::new();

        // current line.
        let mut line_index = 0;

        // open file, use bufReader.
        let source = File::open(path)
            .unwrap_or_else(|_| panic!("build.rs: Could not open source file \"{path}\"."));
        let source = BufReader::new(source).lines();

        // for every line in the file...
        for (i, line) in source.map_while(Result::ok).enumerate() {
            // keep the original version of the line, with trailing white-space and all, for base.lua.
            let copy = line.clone();

            // trim the line for comparison with /* marker.
            let line = line.trim();

            // END marker. write line.
            if line.starts_with("*/") {
                // write function line.
                if function_write {
                    let wiki_string = Function::to_wiki(&work, name, line_index);
                    let meta_string = Function::to_meta(&work, name, line_index);

                    wiki.write_all(wiki_string.as_bytes()).unwrap();
                    meta.write_all(meta_string.as_bytes()).unwrap();

                    work.clear();
                }

                // write class line.
                if class_write {
                    let wiki_string = Class::to_wiki(&work, name);
                    let meta_string = Class::to_meta(&work, name);

                    wiki.write_all(wiki_string.as_bytes()).unwrap();
                    meta.write_all(meta_string.as_bytes()).unwrap();

                    work.clear();
                }

                // write base.lua line.
                if base_write {
                    base.write_all(b"\n").unwrap();
                }

                // do not write any other line until reaching another START marker for either file.
                function_write = false;
                class_write = false;
                base_write = false;
            }

            // can write to meta.lua + documentation, write!
            if function_write {
                work.push_str(line);
                work.push_str("\n");
            }

            // can write to meta.lua, write!
            if class_write {
                work.push_str(line);
                work.push_str("\n");
            }

            // can write to base.lua, write!
            if base_write {
                base.write_all(copy.as_bytes()).unwrap();
                base.write_all(b"\n").unwrap();
            }

            // START function marker
            if line.starts_with("/* function") {
                function_write = true;
                line_index = i;
            }

            // START class marker
            if line.starts_with("/* class") {
                class_write = true;
                line_index = i;
            }

            // START base marker, begin writing to base.lua.
            if line.starts_with("/* base") {
                base_write = true;
            }
        }
    }
}
