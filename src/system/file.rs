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

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::{CStr, CString};

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.file", "info": "The file API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let file = lua.create_table()?;

    file.set("get",                       lua.create_function(self::get)?)?;
    file.set("set",                       lua.create_function(self::set)?)?;
    file.set("get_file_exist",            lua.create_function(self::get_file_exist)?)?;
    file.set("get_path_exist",            lua.create_function(self::get_path_exist)?)?;
    file.set("get_file_extension_check",  lua.create_function(self::get_file_extension_check)?)?;
    file.set("get_file_size",             lua.create_function(self::get_file_size)?)?;
    file.set("get_file_extension",        lua.create_function(self::get_file_extension)?)?;
    file.set("get_file_name",             lua.create_function(self::get_file_name)?)?;
    file.set("get_work_directory",        lua.create_function(self::get_work_directory)?)?;
    file.set("get_application_directory", lua.create_function(self::get_application_directory)?)?;
    file.set("scan_path",                 lua.create_function(self::scan_path)?)?;
    file.set("get_path_escape",           lua.create_function(self::get_path_escape)?)?;
    file.set("set_path_escape",           lua.create_function(self::set_path_escape)?)?;

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
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string" }
    ],
    "result": [
        { "name": "exist", "info": "True if file does exist, false otherwise.", "kind": "boolean" }
    ]
}
*/
fn get_file_exist(lua: &Lua, path: String) -> mlua::Result<bool> {
    let path = CString::new(ScriptData::get_path(lua, &path)?)
        .map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe { Ok(ffi::FileExists(path.as_ptr())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get_path_exist",
    "info": "Check if a path does exist.",
    "member": [
        { "name": "path", "info": "Path.", "kind": "string" }
    ],
    "result": [
        { "name": "exist", "info": "True if path does exist, false otherwise.", "kind": "boolean" }
    ]
}
*/
fn get_path_exist(lua: &Lua, path: String) -> mlua::Result<bool> {
    let path = CString::new(ScriptData::get_path(lua, &path)?)
        .map_err(|e| mlua::Error::runtime(e.to_string()))?;

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
    let path = CString::new(ScriptData::get_path(lua, &path)?)
        .map_err(|e| mlua::Error::runtime(e.to_string()))?;
    let extension = CString::new(extension).map_err(|e| mlua::Error::runtime(e.to_string()))?;

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
    let path = CString::new(ScriptData::get_path(lua, &path)?)
        .map_err(|e| mlua::Error::runtime(e.to_string()))?;

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
    let path = CString::new(ScriptData::get_path(lua, &path)?)
        .map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe {
        let result = ffi::GetFileExtension(path.as_ptr());
        Ok(CStr::from_ptr(result)
            .to_str()
            .map_err(|e| mlua::Error::runtime(e.to_string()))?
            .to_string())
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
    let path = CString::new(ScriptData::get_path(lua, &path)?)
        .map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe {
        if extension {
            let result = ffi::GetFileName(path.as_ptr());
            Ok(CStr::from_ptr(result)
                .to_str()
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .to_string())
        } else {
            let result = ffi::GetFileNameWithoutExt(path.as_ptr());
            Ok(CStr::from_ptr(result)
                .to_str()
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .to_string())
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
        Ok(CStr::from_ptr(result)
            .to_str()
            .map_err(|e| mlua::Error::runtime(e.to_string()))?
            .to_string())
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
        Ok(CStr::from_ptr(result)
            .to_str()
            .map_err(|e| mlua::Error::runtime(e.to_string()))?
            .to_string())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.scan_path",
    "info": "Scan a path.",
    "member": [
        { "name": "path",      "info": "Path to scan.",                                                                              "kind": "string"  },
        { "name": "filter",    "info": "OPTIONAL: Extension filter. If filter is 'DIR', will includ every directory in the result.", "kind": "string"  },
        { "name": "recursive", "info": "Recursive toggle. If true, recursively scan the directory.",                                 "kind": "boolean" }
    ],
    "result": [
        { "name": "list", "info": "File list.", "kind": "table" }
    ]
}
*/
fn scan_path(
    lua: &Lua,
    (path, filter, recursive): (String, Option<String>, bool),
) -> mlua::Result<LuaValue> {
    let path = CString::new(ScriptData::get_path(lua, &path)?)
        .map_err(|e| mlua::Error::runtime(e.to_string()))?;
    let mut data: Vec<String> = Vec::new();

    unsafe {
        let result = {
            if let Some(filter) = filter {
                let filter =
                    CString::new(filter).map_err(|e| mlua::Error::runtime(e.to_string()))?;

                ffi::LoadDirectoryFilesEx(path.as_ptr(), filter.as_ptr(), recursive)
            } else {
                ffi::LoadDirectoryFilesEx(path.as_ptr(), std::ptr::null(), recursive)
            }
        };

        for x in 0..result.count {
            let path = *result.paths.wrapping_add(x.try_into().unwrap());

            let path = CStr::from_ptr(path)
                .to_str()
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .to_string();

            data.push(path);
        }

        ffi::UnloadDirectoryFiles(result);

        lua.to_value(&data)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.get_path_escape",
    "info": "TO-DO"
}
*/
fn get_path_escape(lua: &Lua, _: ()) -> mlua::Result<bool> {
    ScriptData::get_path_escape(lua)
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.file.set_path_escape",
    "info": "TO-DO"
}
*/
fn set_path_escape(lua: &Lua, state: bool) -> mlua::Result<()> {
    ScriptData::set_path_escape(lua, state)
}
