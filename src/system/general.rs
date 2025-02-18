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

use mlua::prelude::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use sysinfo::System;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.general", "info": "The general API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let general = lua.create_table()?;

    general.set("set_log_level",   lua.create_function(self::set_log_level)?)?;
    general.set("open_link",       lua.create_function(self::open_link)?)?;
    general.set("set_exit_key",    lua.create_function(self::set_exit_key)?)?;
    general.set("get_time",        lua.create_function(self::get_time)?)?;
    general.set("get_frame_time",  lua.create_function(self::get_frame_time)?)?;
    general.set("get_frame_rate",  lua.create_function(self::get_frame_rate)?)?;
    general.set("set_frame_rate",  lua.create_function(self::set_frame_rate)?)?;
    general.set("get_argument",    lua.create_function(self::get_argument)?)?;
    general.set("get_system",      lua.create_function(self::get_system)?)?;
    general.set("get_memory",      lua.create_function(self::get_memory)?)?;

    table.set("general", general)?;

    Ok(())
}

//================================================================

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
    "name": "quiver.general.get_system",
    "info": "TO-DO"
}
*/
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

//================================================================

#[derive(Deserialize, Serialize)]
pub struct Camera2D {
    pub shift: Vector2,
    pub focus: Vector2,
    pub angle: f32,
    pub zoom: f32,
}

impl Into<ffi::Camera2D> for Camera2D {
    fn into(self) -> ffi::Camera2D {
        ffi::Camera2D {
            offset: self.shift.into(),
            target: self.focus.into(),
            rotation: self.angle,
            zoom: self.zoom,
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

impl Into<ffi::Camera3D> for Camera3D {
    fn into(self) -> ffi::Camera3D {
        ffi::Camera3D {
            position: self.point.into(),
            target: self.focus.into(),
            up: self.angle.into(),
            fovy: self.zoom,
            projection: self.kind,
        }
    }
}
