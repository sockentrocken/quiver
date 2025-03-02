/*
* Copyright (c) 2025 sockentrocken
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

//================================================================

const PATH_SYSTEM: &str = "src/system/";

fn write_documentation() {
    // create parser object.
    let mut parser = Parser::new();

    // read every file in the API directory.
    for file in std::fs::read_dir(PATH_SYSTEM).unwrap() {
        // convert to string.
        let file = file.expect("build.rs: Could not unwrap file.");

        let kind = file.file_type().unwrap();

        if kind.is_dir() {
            continue;
        }

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

        // don't parse a mod.rs file.
        if name == "mod.rs" {
            continue;
        }

        // open file.
        let file = File::open(path)
            .unwrap_or_else(|_| panic!("build.rs: Could not open file \"{path}\"."));
        let file = BufReader::new(file).lines();

        // create GitHub wiki documentation file.
        parser.new_wiki(name);

        // for each line in the file...
        for (i, line) in file.map_while(Result::ok).enumerate() {
            parser.parse(path, name, &line, i);
        }
    }
}

#[allow(dead_code)]
fn compile_external_dependency() {
    // Tell cargo to look for shared libraries in the specified directory
    //println!("cargo:rustc-link-search=/path/to/lib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/system/external/raymedia.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("src/system/external/rmedia.c")
        .include("src/system/external")
        .compile("rmedia");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=swresample");
    println!("cargo:rustc-link-lib=swscale");
}

// this function is responsible for parsing the src/system/ folder and finding every special comment in the source code to then output it to the GitHub documentation and the Lua LSP definition file.
fn main() {
    #[cfg(feature = "documentation")]
    write_documentation();

    #[cfg(feature = "video")]
    compile_external_dependency();

    // where to search for for library linking.
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
}

//================================================================

struct Parser {
    class: bool,
    entry: bool,
    comment_line: String,
    wiki_file: Option<BufWriter<File>>,
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

---Main entry-point. Quiver will call this on initialization.
---@alias quiver.main fun()

---Fail entry-point. Quiver will call this on a script error, with the script error message as the argument. Note that this function is OPTIONAL, and Quiver will use a default crash handler if missing.
---@alias quiver.fail fun(error : string)

"#;

    #[rustfmt::skip]
    const META_CLASS_HEADER: &'static str =
r#"---{info}
---
--- ---{feature}
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/{path}#L{line})
"#;

    #[rustfmt::skip]
    const META_CLASS_MIDDLE: &'static str =
r#"---@class {name}
"#;

    #[rustfmt::skip]
    const META_CLASS_FOOTER: &'static str =
r#"{name} = {}

"#;

    #[rustfmt::skip]
    const META_MEMBER: &'static str =
r#"---@field {name} {kind} # {info}
"#;

    #[rustfmt::skip]
    const META_ENTRY_HEADER: &'static str =
r#"---{info}
"#;

        #[rustfmt::skip]
    const META_ENTRY_FOOTER: &'static str =
r#"---
--- ---{feature}
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
*Available since version: {version}.*{feature}

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
    const WIKI_MEMBER: &'static str =
r#"* Field: `{name}` – {info}

"#;

    #[rustfmt::skip]
    const WIKI_ENTRY_HEADER: &'static str =
r#"## {name}
*Available since version: {version}.*{feature}

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

    // create a new instance.
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
            comment_line: String::new(),
            wiki_file: None,
            meta_file,
        }
    }

    // create a new instance.
    pub fn new_wiki(&mut self, name: &str) {
        let name = &name[0..name.len() - 3];

        let file = File::create(format!("../quiver.wiki/{name}.md"))
            .unwrap_or_else(|_| panic!("build.rs: Could not create \"{name}\" file."));
        let file = BufWriter::new(file);

        self.wiki_file = Some(file);
    }

    // parse a line from a file.
    pub fn parse(&mut self, path: &str, name: &str, text: &str, line: usize) {
        let text = text.trim();

        if text == "*/" {
            // we were in class mode; write a class out.
            if self.class {
                self.write_meta_class(path, name, line);
                self.write_wiki_class(path, name, line);
            }

            // we were in entry mode; write a entry out.
            if self.entry {
                self.write_meta_entry(path, name, line);
                self.write_wiki_entry(path, name, line);
            }

            // reset mode.
            self.class = false;
            self.entry = false;
            self.comment_line.clear();
        }

        // we are currently writing either a class or an entry comment, push a new line.
        if self.class || self.entry {
            self.comment_line.push_str(text);
        }

        // class comment. enable class mode.
        if text == "/* class" {
            self.class = true;
        }

        // entry comment. enable entry mode.
        if text == "/* entry" {
            self.entry = true;
        }
    }

    fn write_meta_class(&mut self, path: &str, _name: &str, line: usize) {
        let class: Class = serde_json::from_str(&self.comment_line).unwrap_or_else(|e| {
            panic!("Meta::write_meta_class(): Could not deserialize class. Error: {e}, Path: {path}, Line: {line}, Text: {}", self.comment_line)
        });

        let mut data = Self::META_CLASS_HEADER.to_string();
        data = data.replace("{info}", &class.info);
        data = data.replace("{path}", path);
        data = data.replace("{line}", &format!("{}", line + 2));

        if let Some(test) = class.test {
            let test =
                std::fs::read_to_string(format!("test/system/{test}")).unwrap_or_else(|_| {
                    panic!("Meta::write_meta_class(): Could not read file {test}.")
                });

            data.push_str("---```lua\n");

            let split: Vec<&str> = test.split("\n").collect();

            for text in split {
                data.push_str(&format!("---{text}\n"));
            }

            data.push_str("---```\n");
        }

        data.push_str(Self::META_CLASS_MIDDLE);
        data = data.replace("{name}", &class.name);

        if let Some(entry_member) = &class.member {
            for member in entry_member {
                let field = Self::META_MEMBER;
                let field = field.replace("{name}", &member.name);
                let field = field.replace("{kind}", &member.kind);
                let field = field.replace("{info}", &member.info);
                data.push_str(&field);
            }
        }

        data.push_str(Self::META_CLASS_FOOTER);
        data = data.replace("{name}", &class.name);

        let feature = {
            if let Some(feature) = class.feature {
                &format!("\n---*Available with compile feature: `{feature}`.*\n---")
            } else {
                ""
            }
        };

        data = data.replace("{feature}", feature);

        self.meta_file
            .write_all(data.as_bytes())
            .expect("Meta::write_meta_class(): Could not write to file.");
    }

    fn write_meta_entry(&mut self, path: &str, _name: &str, line: usize) {
        let entry: Entry = serde_json::from_str(&self.comment_line).unwrap_or_else(|e| {
            panic!("Meta::write_meta_entry(): Could not deserialize entry. Error: {e}, Path: {path}, Line: {line}, Text: {}", self.comment_line)
        });

        let mut data = Self::META_ENTRY_HEADER.to_string();
        data = data.replace("{info}", &entry.info);

        if let Some(test) = entry.test {
            let test =
                std::fs::read_to_string(format!("test/system/{test}")).unwrap_or_else(|_| {
                    panic!("Meta::write_meta_entry(): Could not read file {test}.")
                });

            data.push_str("---```lua\n");

            let split: Vec<&str> = test.split("\n").collect();

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

        let feature = {
            if let Some(feature) = entry.feature {
                &format!("\n---*Available with compile feature: `{feature}`.*\n---")
            } else {
                ""
            }
        };

        data = data.replace("{feature}", feature);

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
        data = data.replace("{version}", &class.version);

        let feature = {
            if let Some(feature) = class.feature {
                &format!(" *Available with compile feature: `{feature}`.*")
            } else {
                ""
            }
        };

        data = data.replace("{feature}", feature);
        data = data.replace("{code}", &format!("{} = {{}}", class.name));
        data = data.replace("{info}", &class.info);

        if let Some(test) = class.test {
            let test =
                std::fs::read_to_string(format!("test/system/{test}")).unwrap_or_else(|_| {
                    panic!("Meta::write_wiki_class(): Could not read file {test}.")
                });

            data.push_str("```lua\n");
            data.push_str(&test);
            data.push_str("```\n");
        }

        if let Some(class_member) = class.member {
            for member in class_member {
                let field = Self::WIKI_MEMBER;
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
        data = data.replace("{version}", &entry.version);

        let feature = {
            if let Some(feature) = entry.feature {
                &format!(" *Available with compile feature: `{feature}`.*")
            } else {
                ""
            }
        };

        data = data.replace("{feature}", feature);

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

        if let Some(test) = entry.test {
            let test =
                std::fs::read_to_string(format!("test/system/{test}")).unwrap_or_else(|_| {
                    panic!("Meta::write_wiki_entry(): Could not read file {test}.")
                });

            data.push_str("```lua\n");
            data.push_str(&test);
            data.push_str("```\n");
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

// a representation of a Lua class.
#[derive(Deserialize, Serialize)]
struct Class {
    pub version: String,
    pub feature: Option<String>,
    pub name: String,
    pub info: String,
    pub test: Option<String>,
    pub member: Option<Vec<Variable>>,
}

// a representation of a Lua function.
#[derive(Deserialize, Serialize)]
struct Entry {
    pub version: String,
    pub feature: Option<String>,
    pub name: String,
    pub info: String,
    pub test: Option<String>,
    pub member: Option<Vec<Variable>>,
    pub result: Option<Vec<Variable>>,
}

// a representation of a Lua variable.
#[derive(Deserialize, Serialize)]
struct Variable {
    pub name: String,
    pub info: String,
    pub kind: String,
}
