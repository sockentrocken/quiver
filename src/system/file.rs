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
    let path = CString::new(ScriptData::get_path(lua, &path))
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
    let path = CString::new(ScriptData::get_path(lua, &path))
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
    let path = CString::new(ScriptData::get_path(lua, &path))
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
    let path = CString::new(ScriptData::get_path(lua, &path))
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
    let path = CString::new(ScriptData::get_path(lua, &path))
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
    let path = CString::new(ScriptData::get_path(lua, &path))
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
    let path = CString::new(ScriptData::get_path(lua, &path))
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
