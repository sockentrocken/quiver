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

use std::ffi::{CStr, CString};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::script::*;
use crate::status::*;

//================================================================

use ffi::__va_list_tag;
use mlua::prelude::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "system_info")]
use sysinfo::System;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.general", "info": "The general API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let general = lua.create_table()?;

    general.set("load_base",       lua.create_function(self::load_base)?)?;
    general.set("set_log_level",   lua.create_function(self::set_log_level)?)?;
    general.set("open_link",       lua.create_function(self::open_link)?)?;

    general.set("standard_input",       lua.create_function(self::standard_input)?)?;

    general.set("set_exit_key",    lua.create_function(self::set_exit_key)?)?;
    general.set("get_frame_time",  lua.create_function(self::get_frame_time)?)?;
    general.set("get_frame_rate",  lua.create_function(self::get_frame_rate)?)?;
    general.set("set_frame_rate",  lua.create_function(self::set_frame_rate)?)?;
    
    general.set("get_time",      lua.create_function(self::get_time)?)?;
    general.set("get_time_unix", lua.create_function(self::get_time_unix)?)?;
    general.set("get_argument",  lua.create_function(self::get_argument)?)?;

    #[cfg(feature = "system_info")]
    general.set("get_system",      lua.create_function(self::get_system)?)?;

    general.set("get_memory",      lua.create_function(self::get_memory)?)?;
    general.set("get_info",        lua.create_function(self::get_info)?)?;
    
    general.set("set_call_back_save_file", lua.create_function(self::set_call_back_save_file)?)?;
    general.set("set_call_back_load_file", lua.create_function(self::set_call_back_load_file)?)?;
    general.set("set_call_back_save_text", lua.create_function(self::set_call_back_save_text)?)?;
    general.set("set_call_back_load_text", lua.create_function(self::set_call_back_load_text)?)?;

    table.set("general", general)?;

    Ok(())
}

//================================================================

unsafe extern "C" fn call_back_save_file(
    file_name: *const i8,
    data: *mut std::ffi::c_void,
    size: i32,
) -> bool {
    unsafe {
        let pointer = &raw const CALL_BACK_SAVE_FILE;

        if let Some(Some(call)) = pointer.as_ref() {
            let file_name = Script::c_to_rust_string(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let data = data as *mut u8;
            let data = Vec::from_raw_parts(data, size as usize, size as usize);

            let value = call
                .call::<bool>((file_name, data.clone()))
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            std::mem::forget(data);

            return value;
        }

        false
    }
}

unsafe extern "C" fn call_back_load_file(file_name: *const i8, data_size: *mut i32) -> *mut u8 {
    unsafe {
        let pointer = &raw const CALL_BACK_LOAD_FILE;

        if let Some(Some(call)) = pointer.as_ref() {
            let file_name = Script::c_to_rust_string(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let value = call
                .call::<LuaValue>(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let data = crate::system::data::Data::<u8>::get_buffer(value)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let mut data = data.0.clone();

            let pointer = data.as_mut_ptr();
            let length = data.len() as i32;

            std::mem::forget(data);

            *data_size = length;
            return pointer;
        }

        *data_size = 0;

        std::ptr::null_mut()
    }
}

unsafe extern "C" fn call_back_save_text(file_name: *const i8, data: *mut i8) -> bool {
    unsafe {
        let pointer = &raw const CALL_BACK_SAVE_TEXT;

        if let Some(Some(call)) = pointer.as_ref() {
            let file_name = Script::c_to_rust_string(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let data = Script::c_to_rust_string(data)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let value = call
                .call::<bool>((file_name, data))
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            return value;
        }

        false
    }
}

unsafe extern "C" fn call_back_load_text(file_name: *const i8) -> *mut i8 {
    unsafe {
        let pointer = &raw const CALL_BACK_LOAD_TEXT;

        if let Some(Some(call)) = pointer.as_ref() {
            let file_name = Script::c_to_rust_string(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let value = call
                .call::<String>(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let value = Script::rust_to_c_string(&value)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            return value.into_raw();
        }

        std::ptr::null_mut()
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.set_call_back_save_file",
    "info": "TO-DO"
}
*/
fn set_call_back_save_file(_: &Lua, function: mlua::Function) -> mlua::Result<()> {
    unsafe {
        ffi::SetSaveFileDataCallback(Some(call_back_save_file));

        CALL_BACK_SAVE_FILE = Some(function);

        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.set_call_back_load_file",
    "info": "TO-DO"
}
*/
fn set_call_back_load_file(_: &Lua, function: mlua::Function) -> mlua::Result<()> {
    unsafe {
        ffi::SetLoadFileDataCallback(Some(call_back_load_file));

        CALL_BACK_LOAD_FILE = Some(function);

        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.set_call_back_save_text",
    "info": "TO-DO"
}
*/
fn set_call_back_save_text(_: &Lua, function: mlua::Function) -> mlua::Result<()> {
    unsafe {
        ffi::SetSaveFileTextCallback(Some(call_back_save_text));

        CALL_BACK_SAVE_TEXT = Some(function);

        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.set_call_back_load_text",
    "info": "TO-DO"
}
*/
fn set_call_back_load_text(_: &Lua, function: mlua::Function) -> mlua::Result<()> {
    unsafe {
        ffi::SetLoadFileTextCallback(Some(call_back_load_text));

        CALL_BACK_LOAD_TEXT = Some(function);

        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.standard_input",
    "info": "TO-DO"
}
*/
fn standard_input(_: &Lua, _: ()) -> mlua::Result<String> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.load_base",
    "info": "TO-DO"
}
*/
fn load_base(lua: &Lua, _: ()) -> mlua::Result<()> {
    // TO-DO only for debug. do not re-load from disk on release.
    for base in crate::script::Script::FILE_BASE {
        let data = if cfg!(debug_assertions) {
            &std::fs::read_to_string(format!("../src/asset/{}", base.name)).unwrap()
        } else {
            base.data
        };

        //let data = base.data;

        lua.load(data).set_name(format!("@{}", base.name)).exec()?;
    }

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.set_log_level",
    "info": "TO-DO"
}
*/
fn set_log_level(_: &Lua, level: i32) -> mlua::Result<()> {
    unsafe {
        ffi::SetTraceLogLevel(level);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.open_link",
    "info": "TO-DO"
}
*/
fn open_link(_: &Lua, link: String) -> mlua::Result<()> {
    unsafe {
        ffi::OpenURL(link.as_ptr() as *const i8);
        Ok(())
    }
}

/* entry
{ "version": "1.0.0", "name": "quiver.general.load", "info": "Load the engine.", "skip_definition": "true" }
*/

/* entry
{ "version": "1.0.0", "name": "quiver.general.exit", "info": "Exit the engine.", "skip_definition": "true" }
*/

/* entry
{
    "version": "1.0.0", "name": "quiver.general.set_exit_key",
    "info": "Set a key to exit Quiver.",
    "member": [
        { "name": "key", "info": "Key to exit Quiver with.", "kind": "input_board" }
    ]
}
*/
fn set_exit_key(_: &Lua, value: i32) -> mlua::Result<()> {
    unsafe {
        ffi::SetExitKey(value);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0", "name": "quiver.general.get_time",
    "info": "Get the current time. Will count up since the initialization of the window.",
    "result": [
        { "name": "time", "info": "Current time.", "kind": "number" }
    ]
}
*/
fn get_time(_: &Lua, _: ()) -> mlua::Result<f64> {
    unsafe { Ok(ffi::GetTime()) }
}

/* entry
{
    "version": "1.0.0", "name": "quiver.general.get_time_unix",
    "info": "TO-DO"
}
*/
fn get_time_unix(_: &Lua, add: Option<i64>) -> mlua::Result<String> {
    let time = SystemTime::now();
    let time = time.duration_since(UNIX_EPOCH).unwrap();
    let time = time.as_secs() + (add.unwrap_or_default() as u64);

    Ok(time.to_string())
}

/* entry
{
    "version": "1.0.0", "name": "quiver.general.get_frame_time",
    "info": "Get the current frame time.",
    "result": [
        { "name": "frame_time", "info": "Current frame time.", "kind": "number" }
    ]
}
*/
fn get_frame_time(_: &Lua, _: ()) -> mlua::Result<f32> {
    unsafe { Ok(ffi::GetFrameTime()) }
}

/* entry
{
    "version": "1.0.0", "name": "quiver.general.get_frame_rate",
    "info": "Get the current frame rate.",
    "result": [
        { "name": "frame_rate", "info": "Current frame rate.", "kind": "number" }
    ]
}
*/
fn get_frame_rate(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetFPS()) }
}

/* entry
{
    "version": "1.0.0", "name": "quiver.general.set_frame_rate",
    "info": "Set the current frame rate.",
    "member": [
        { "name": "frame_rate", "info": "Current frame rate.", "kind": "number" }
    ]
}
*/
fn set_frame_rate(_: &Lua, rate: i32) -> mlua::Result<()> {
    unsafe {
        ffi::SetTargetFPS(rate);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.get_argument",
    "info": "TO-DO"
}
*/
fn get_argument(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    let value: Vec<String> = std::env::args().collect();

    lua.to_value(&value)
}

/* entry
{
    "version": "1.0.0",
    "feature": "system_info",
    "name": "quiver.general.get_system",
    "info": "TO-DO"
}
*/
#[cfg(feature = "system_info")]
fn get_system(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    let mut system = System::new_all();
    system.refresh_all();

    lua.to_value(&system)
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.get_memory",
    "info": "TO-DO"
}
*/
fn get_memory(lua: &Lua, _: ()) -> mlua::Result<usize> {
    Ok(lua.used_memory())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.get_info",
    "info": "TO-DO"
}
*/
fn get_info(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    let script_data = lua.app_data_ref::<crate::script::ScriptData>().unwrap();

    lua.to_value(&*script_data)
}

//================================================================

#[derive(Deserialize, Serialize)]
pub struct Camera2D {
    pub shift: Vector2,
    pub focus: Vector2,
    pub angle: f32,
    pub zoom: f32,
}

impl From<Camera2D> for ffi::Camera2D {
    fn from(val: Camera2D) -> Self {
        ffi::Camera2D {
            offset: val.shift.into(),
            target: val.focus.into(),
            rotation: val.angle,
            zoom: val.zoom,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Camera3D {
    pub point: Vector3,
    pub focus: Vector3,
    pub angle: Vector3,
    pub zoom: f32,
    pub kind: i32,
}

impl From<Camera3D> for ffi::Camera3D {
    fn from(val: Camera3D) -> Self {
        ffi::Camera3D {
            position: val.point.into(),
            target: val.focus.into(),
            up: val.angle.into(),
            fovy: val.zoom,
            projection: val.kind,
        }
    }
}
