/*
* BSD Zero Clause License
*
* Copyright (c) 2025 sockentrocken
*
* Permission to use, copy, modify, and/or distribute this software for any
* purpose with or without fee is hereby granted.
*
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
* REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
* AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
* INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
* LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
* OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
* PERFORMANCE OF THIS SOFTWARE.
*/

use std::io::Read;

use crate::script::*;

//================================================================

use mlua::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.zip", "info": "The ZIP API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
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
            "name": "zip:get_binary",
            "info": "Get a file from a ZIP file as binary data.",
            "member": [
                { "name": "path", "info": "Path to file in ZIP file.", "kind": "string" }
            ],
            "result": [
                { "name": "data", "info": "Binary data.", "kind": "table" }
            ]
        }
        */
        method.add_method_mut("get_binary", |_: &Lua, this, path: String| {
            match this.0.by_name(&path) {
                Ok(mut value) => {
                    let mut buffer = Vec::new();
                    value.read_to_end(&mut buffer)?;
                    Ok(buffer)
                }
                Err(value) => Err(mlua::Error::runtime(value.to_string())),
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "zip:get_string",
            "info": "Get a file from a ZIP file as string data.",
            "member": [
                { "name": "path", "info": "Path to file in ZIP file.", "kind": "string" }
            ],
            "result": [
                { "name": "data", "info": "String data.", "kind": "string" }
            ]
        }
        */
        method.add_method_mut("get_string", |_: &Lua, this, path: String| {
            match this.0.by_name(&path) {
                Ok(mut value) => {
                    let mut buffer = String::new();
                    value.read_to_string(&mut buffer)?;
                    Ok(buffer)
                }
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
