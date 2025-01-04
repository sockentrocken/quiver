/*
* MIT License
*
* Copyright (c) 2024 sockentrocken
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
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
        let file = std::fs::File::open(ScriptData::get_path(lua, &path))?;
        let file = zip::ZipArchive::new(file).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        Ok(Self(file))
    }
}
