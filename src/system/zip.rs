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

use crate::script::*;
use crate::status::*;

//================================================================

use mlua::prelude::*;
use std::io::Read;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.zip", "info": "The ZIP API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, script_info: Option<&ScriptInfo>) -> mlua::Result<()> {
    // part of head-less API. do not run code when doing head API.
    if script_info.is_some() {
        return Ok(())
    }

    let zip = lua.create_table()?;

    zip.set("new", lua.create_function(self::Zip::new)?)?;

    table.set("zip", zip)?;

    Ok(())
}

/* class
{ "version": "1.0.0", "name": "zip", "info": "An unique handle to a ZIP in memory." }
*/
struct Zip(zip::ZipArchive<std::fs::File>);

impl mlua::UserData for Zip {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "zip:get_file",
            "info": "Get a file from the ZIP file.",
            "member": [
                { "name": "path",   "info": "Path to the file.", "kind": "string"  },
                { "name": "binary", "info": "Read as binary.",   "kind": "boolean" }
            ],
            "result": [
                { "name": "file", "info": "The file.", "kind": "string | data" }
            ],
            "routine": true
        }
        */
        method.add_async_method_mut(
            "get_file",
            |lua: Lua, mut this, (path, binary): (String, bool)| async move {
                tokio::task::spawn_blocking(move || match this.0.by_name(&path) {
                    Ok(mut value) => {
                        if binary {
                            let mut data = Vec::new();
                            value.read_to_end(&mut data)?;

                            let data = crate::system::data::Data::new(&lua, data)?;
                            let data = lua.create_userdata(data)?;

                            Ok(mlua::Value::UserData(data))
                        } else {
                            let mut data = String::new();
                            value.read_to_string(&mut data)?;

                            lua.to_value(&data)
                        }
                    }
                    Err(value) => Err(mlua::Error::runtime(value.to_string())),
                })
                .await
                .unwrap()
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "zip:get_list",
            "info": "Get a list of every file in the ZIP file.",
            "result": [
                { "name": "list", "info": "The list of every file.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("get_list", |_: &Lua, this, _: ()| {
            let name: Vec<String> = this.0.file_names().map(|x| x.to_string()).collect();

            Ok(name)
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "zip:is_file",
            "info": "Check if the given path is a file.",
            "member": [
                { "name": "path", "info": "Path to the file.", "kind": "string" }
            ],
            "result": [
                { "name": "value", "info": "True if the path is a file, false otherwise.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut("is_file", |_: &Lua, this, path: String| {
            match this.0.by_name(&path) {
                Ok(value) => Ok(value.is_file()),
                Err(value) => Err(mlua::Error::runtime(value.to_string())),
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "zip:is_path",
            "info": "Check if the given path is a folder.",
            "member": [
                { "name": "path", "info": "Path to the folder.", "kind": "string" }
            ],
            "result": [
                { "name": "value", "info": "True if the path is a folder, false otherwise.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut("is_path", |_: &Lua, this, path: String| {
            match this.0.by_name(&path) {
                Ok(value) => Ok(value.is_dir()),
                Err(value) => Err(mlua::Error::runtime(value.to_string())),
            }
        });
    }
}

impl Zip {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.zip.new",
        "info": "Create a new ZIP resource.",
        "member": [
            { "name": "path", "info": "Path to ZIP file.", "kind": "string" }
        ],
        "result": [
            { "name": "zip", "info": "ZIP resource.", "kind": "zip" }
        ]
    }
    */
    fn new(lua: &Lua, path: String) -> mlua::Result<Self> {
        let file = std::fs::File::open(ScriptData::get_path(lua, &path)?)?;
        let file = zip::ZipArchive::new(file).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        Ok(Self(file))
    }
}
