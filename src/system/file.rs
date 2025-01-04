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

use crate::script::*;

//================================================================

use mlua::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.file", "info": "The file API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let file = lua.create_table()?;

    file.set("get", lua.create_function(self::get)?)?;
    file.set("set", lua.create_function(self::set)?)?;

    table.set("file", file)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get",
    "info": "Get the data of a file, in string format.",
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string" }
    ],
    "result": [
        { "name": "data", "info": "File data.", "kind": "string" }
    ]
}
*/
fn get(lua: &Lua, path: String) -> mlua::Result<String> {
    std::fs::read_to_string(ScriptData::get_path(lua, &path))
        .map_err(|e| mlua::Error::runtime(e.to_string()))
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.set",
    "info": "Set the data of a file.",
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string" },
        { "name": "data", "info": "Data to copy.", "kind": "string" }
    ]
}
*/
fn set(lua: &Lua, (path, data): (String, String)) -> mlua::Result<()> {
    std::fs::write(ScriptData::get_path(lua, &path), data)
        .map_err(|e| mlua::Error::runtime(e.to_string()))
}
