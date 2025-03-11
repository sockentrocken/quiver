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
use raylib::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.file", "info": "The file API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let file = lua.create_table()?;

    file.set("get",                       lua.create_function(self::get)?)?;
    file.set("set",                       lua.create_function(self::set)?)?;

    // FileExists
    file.set("get_file_exist",            lua.create_function(self::get_file_exist)?)?;
    // DirectoryExists
    file.set("get_path_exist",            lua.create_function(self::get_path_exist)?)?;
    // IsFileExtension
    file.set("get_file_extension_check",  lua.create_function(self::get_file_extension_check)?)?;
    // GetFileLength
    file.set("get_file_size",             lua.create_function(self::get_file_size)?)?;
    // GetFileExtension
    file.set("get_file_extension",        lua.create_function(self::get_file_extension)?)?;
    // GetFileName/GetFileNameWithoutExt    
    file.set("get_file_name",             lua.create_function(self::get_file_name)?)?;
    // GetWorkingDirectory
    file.set("get_work_directory",        lua.create_function(self::get_work_directory)?)?;
    // GetApplicationDirectory
    file.set("get_application_directory", lua.create_function(self::get_application_directory)?)?;
    // LoadDirectoryFiles/LoadDirectoryFilesEx
    file.set("scan_path",                 lua.create_function(self::scan_path)?)?;
    file.set("set_path_escape",           lua.create_function(self::set_path_escape)?)?;
    file.set("move",                      lua.create_function(self::move_file)?)?;
    file.set("copy",                      lua.create_function(self::copy)?)?;
    file.set("remove_file",               lua.create_function(self::remove_file)?)?;
    file.set("remove_path",               lua.create_function(self::remove_path)?)?;

    // TO-DO add mkdir

    table.set("file", file)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get",
    "info": "Get the data of a file, in string format.",
    "test": "file/get_set.lua",
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string" }
    ],
    "result": [
        { "name": "data", "info": "File data.", "kind": "string" }
    ]
}
*/
fn get(lua: &Lua, (path, binary): (String, bool)) -> mlua::Result<LuaValue> {
    if binary {
        let data = std::fs::read(ScriptData::get_path(lua, &path)?)
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;
        let data = crate::system::data::Data::new(lua, data)?;
        let data = lua.create_userdata(data)?;

        Ok(mlua::Value::UserData(data))
    } else {
        let data = std::fs::read_to_string(ScriptData::get_path(lua, &path)?)
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;

        lua.to_value(&data)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.set",
    "info": "Set the data of a file.",
    "test": "file/get_set.lua",
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string" },
        { "name": "data", "info": "Data to copy.", "kind": "string" }
    ]
}
*/
fn set(lua: &Lua, (path, data, binary): (String, LuaValue, bool)) -> mlua::Result<()> {
    if binary {
        let data = crate::system::data::Data::get_buffer(data)?;
        let data = &data.0;

        std::fs::write(ScriptData::get_path(lua, &path)?, data)
            .map_err(|e| mlua::Error::runtime(e.to_string()))
    } else {
        let data: String = lua.from_value(data)?;

        std::fs::write(ScriptData::get_path(lua, &path)?, data)
            .map_err(|e| mlua::Error::runtime(e.to_string()))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get_file_exist",
    "info": "Check if a file does exist.",
    "test": "file/get_file_exist.lua",
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string" }
    ],
    "result": [
        { "name": "exist", "info": "True if file does exist, false otherwise.", "kind": "boolean" }
    ]
}
*/
fn get_file_exist(lua: &Lua, path: String) -> mlua::Result<bool> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

    unsafe { Ok(ffi::FileExists(path.as_ptr())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get_path_exist",
    "info": "Check if a path does exist.",
    "test": "file/get_path_exist.lua",
    "member": [
        { "name": "path", "info": "Path.", "kind": "string" }
    ],
    "result": [
        { "name": "exist", "info": "True if path does exist, false otherwise.", "kind": "boolean" }
    ]
}
*/
fn get_path_exist(lua: &Lua, path: String) -> mlua::Result<bool> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

    unsafe { Ok(ffi::DirectoryExists(path.as_ptr())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get_file_extension_check",
    "info": "Check if a file's extension is the same as a given one.",
    "member": [
        { "name": "path",      "info": "Path to file.",                                   "kind": "string" },
        { "name": "extension", "info": "Extension. MUST include dot (.png, .wav, etc.).", "kind": "string" }
    ],
    "result": [
        { "name": "check", "info": "True if file extension is the same as the given one, false otherwise.", "kind": "boolean" }
    ]
}
*/
fn get_file_extension_check(lua: &Lua, (path, extension): (String, String)) -> mlua::Result<bool> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;
    let extension =
        Script::rust_to_c_string(&extension).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe { Ok(ffi::IsFileExtension(path.as_ptr(), extension.as_ptr())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get_file_size",
    "info": "Get the size of a file.",
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string" }
    ],
    "result": [
        { "name": "size", "info": "File size.", "kind": "number" }
    ]
}
*/
fn get_file_size(lua: &Lua, path: String) -> mlua::Result<i32> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

    unsafe { Ok(ffi::GetFileLength(path.as_ptr())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get_file_extension",
    "info": "Get the extension of a file.",
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string" }
    ],
    "result": [
        { "name": "extension", "info": "File extension.", "kind": "string" }
    ]
}
*/
fn get_file_extension(lua: &Lua, path: String) -> mlua::Result<String> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

    unsafe {
        let result = ffi::GetFileExtension(path.as_ptr());
        Script::c_to_rust_string(result)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get_file_name",
    "info": "Get the name of a file.",
    "member": [
        { "name": "path",      "info": "Path to file.",                                                      "kind": "string"  },
        { "name": "extension", "info": "File extension. If true, will return file name with the extension.", "kind": "boolean" }
    ],
    "result": [
        { "name": "name", "info": "File name.", "kind": "string" }
    ]
}
*/
fn get_file_name(lua: &Lua, (path, extension): (String, bool)) -> mlua::Result<String> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

    unsafe {
        if extension {
            let result = ffi::GetFileName(path.as_ptr());
            Script::c_to_rust_string(result)
        } else {
            let result = ffi::GetFileNameWithoutExt(path.as_ptr());
            Script::c_to_rust_string(result)
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get_work_directory",
    "info": "Get the current work path.",
    "result": [
        { "name": "path", "info": "Work path.", "kind": "string" }
    ]
}
*/
fn get_work_directory(_: &Lua, _: ()) -> mlua::Result<String> {
    unsafe {
        let result = ffi::GetWorkingDirectory();
        Script::c_to_rust_string(result)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get_application_directory",
    "info": "Get the current application path.",
    "result": [
        { "name": "path", "info": "Application path.", "kind": "string" }
    ]
}
*/
fn get_application_directory(_: &Lua, _: ()) -> mlua::Result<String> {
    unsafe {
        let result = ffi::GetApplicationDirectory();
        Script::c_to_rust_string(result)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.scan_path",
    "info": "Scan a path.",
    "member": [
        { "name": "path",      "info": "Path to scan.",                                                                               "kind": "string"  },
        { "name": "filter",    "info": "OPTIONAL: Extension filter. If filter is 'DIR', will include every directory in the result.", "kind": "string?" },
        { "name": "recursive", "info": "If true, recursively scan the directory.",                                                    "kind": "boolean" },
        { "name": "absolute",  "info": "If true, return path relatively.",                                                            "kind": "boolean" }
    ],
    "result": [
        { "name": "list", "info": "File list.", "kind": "table" }
    ]
}
*/
fn scan_path(
    lua: &Lua,
    (path, filter, recursive, relative): (String, Option<String>, bool, bool),
) -> mlua::Result<LuaValue> {
    let c_path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;
    let mut data: Vec<String> = Vec::new();

    unsafe {
        let result = {
            if let Some(filter) = filter {
                let filter = Script::rust_to_c_string(&filter)
                    .map_err(|e| mlua::Error::runtime(e.to_string()))?;

                ffi::LoadDirectoryFilesEx(c_path.as_ptr(), filter.as_ptr(), recursive)
            } else {
                ffi::LoadDirectoryFilesEx(c_path.as_ptr(), std::ptr::null(), recursive)
            }
        };

        for x in 0..result.count {
            let result_path = *result.paths.wrapping_add(x.try_into().unwrap());

            let result_path = Script::c_to_rust_string(result_path)?;

            if relative {
                let path: Vec<&str> = result_path.split(&path).collect();

                if let Some(path) = path.get(1) {
                    // remove the leading back-slash.
                    let path = &path[1..path.len()];

                    println!("rust: {path}");

                    data.push(path.to_string());
                }
            } else {
                data.push(result_path);
            }
        }

        ffi::UnloadDirectoryFiles(result);

        lua.to_value(&data)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.set_path_escape",
    "info": "Set the state of the path sand-box.",
    "member": [
        { "name": "state", "info": "The state of the path sand-box.", "kind": "boolean" }
    ]
}
*/
fn set_path_escape(lua: &Lua, state: bool) -> mlua::Result<()> {
    ScriptData::set_path_escape(lua, state)
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.move",
    "info": "Move a file.",
    "member": [
        { "name": "source", "info": "The source path.", "kind": "string" },
        { "name": "target", "info": "The target path.", "kind": "string" }
    ]
}
*/
fn move_file(lua: &Lua, (source, target): (String, String)) -> mlua::Result<()> {
    let source = ScriptData::get_path(lua, &source)?;
    let target = ScriptData::get_path(lua, &target)?;

    std::fs::rename(source, target).map_err(mlua::Error::runtime)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.copy",
    "info": "Copy a file.",
    "member": [
        { "name": "source", "info": "The source path.", "kind": "string" },
        { "name": "target", "info": "The target path.", "kind": "string" }
    ]
}
*/
fn copy(lua: &Lua, (source, target): (String, String)) -> mlua::Result<()> {
    let source = ScriptData::get_path(lua, &source)?;
    let target = ScriptData::get_path(lua, &target)?;

    std::fs::copy(source, target).map_err(mlua::Error::runtime)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.remove_file",
    "info": "Remove a file.",
    "member": [
        { "name": "path", "info": "The path to the file to remove.", "kind": "string" }
    ]
}
*/
fn remove_file(lua: &Lua, path: String) -> mlua::Result<()> {
    let path = ScriptData::get_path(lua, &path)?;

    std::fs::remove_file(path).map_err(mlua::Error::runtime)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.remove_path",
    "info": "Remove a folder.",
    "member": [
        { "name": "path", "info": "The path to the folder to remove.", "kind": "string" }
    ]
}
*/
fn remove_path(lua: &Lua, path: String) -> mlua::Result<()> {
    let path = ScriptData::get_path(lua, &path)?;

    std::fs::remove_dir_all(path).map_err(mlua::Error::runtime)?;

    Ok(())
}
