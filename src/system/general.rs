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

use crate::script::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::CString;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.general", "info": "The general API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let general = lua.create_table()?;

    general.set("open_link", lua.create_function(self::open_link)?)?;
    general.set("serialize",         lua.create_function(self::serialize)?)?;
    general.set("deserialize",       lua.create_function(self::deserialize)?)?;
    general.set("set_exit_key",      lua.create_function(self::set_exit_key)?)?;
    general.set("get_time",          lua.create_function(self::get_time)?)?;
    general.set("get_frame_time",    lua.create_function(self::get_frame_time)?)?;
    general.set("get_frame_rate",    lua.create_function(self::get_frame_rate)?)?;
    general.set("set_frame_rate",    lua.create_function(self::set_frame_rate)?)?;
    general.set("get_memory",        lua.create_function(self::get_memory)?)?;
    general.set("get_info",          lua.create_function(self::get_info)?)?;
    general.set("collision_box_box", lua.create_function(self::collision_box_box)?)?;
    //general.set("get_clipboard_text",  lua.create_function(self::get_clipboard_text)?)?;
    //general.set("set_clipboard_text",  lua.create_function(self::set_clipboard_text)?)?;
    //general.set("get_clipboard_image", lua.create_function(self::get_clipboard_image)?)?;

    table.set("general", general)?;

    Ok(())
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.open_link",
    "info": ""
}
*/
fn open_link(_: &Lua, link: String) -> mlua::Result<()> {
    unsafe {
        ffi::OpenURL(link.as_ptr() as *const i8);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.collision_box_box",
    "info": "",
    "member": [
        { "name": "box_a", "info": "", "kind": "box_3" },
        { "name": "box_b", "info": "", "kind": "box_3" }
    ],
    "result": [
        { "name": "result", "info": "", "kind": "boolean" }
    ]
}
*/
fn collision_box_box(lua: &Lua, (box_a, box_b): (LuaValue, LuaValue)) -> mlua::Result<bool> {
    let box_a: BoundingBox = lua.from_value(box_a)?;
    let box_b: BoundingBox = lua.from_value(box_b)?;

    unsafe { Ok(ffi::CheckCollisionBoxes(box_a.into(), box_b.into())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.serialize",
    "info": "Serialize a given Lua value as a JSON string.",
    "member": [
        { "name": "value", "info": "Lua value to serialize.", "kind": "any" }
    ],
    "result": [
        { "name": "value", "info": "The value, in string form.", "kind": "string" }
    ]
}
*/
fn serialize(lua: &Lua, value: LuaValue) -> mlua::Result<String> {
    let value: serde_json::Value = lua.from_value(value)?;
    serde_json::to_string_pretty(&value).map_err(|e| mlua::Error::runtime(e.to_string()))
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.deserialize",
    "info": "Deserialize a given JSON string as a Lua value.",
    "member": [
        { "name": "value", "info": "String to deserialize.", "kind": "string" }
    ],
    "result": [
        { "name": "value", "info": "The value, in Lua value form.", "kind": "any" }
    ]
}
*/
fn deserialize(lua: &Lua, value: String) -> mlua::Result<LuaValue> {
    let value: serde_json::Value =
        serde_json::from_str(&value).map_err(|e| mlua::Error::runtime(e.to_string()))?;
    lua.to_value(&value)
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
    "info": "set the current frame rate.",
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
    "name": "quiver.general.get_memory",
    "info": ""
}
*/
fn get_memory(lua: &Lua, _: ()) -> mlua::Result<usize> {
    Ok(lua.used_memory())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.general.get_info",
    "info": "Get the current info manifest data.",
    "result": [
        { "name": "safe", "info": "Safe mode.", "kind": "boolean" },
        { "name": "path", "info": "Main path.", "kind": "string"  }
    ]
}
*/
fn get_info(lua: &Lua, _: ()) -> mlua::Result<(bool, String)> {
    let script_data = lua.app_data_ref::<ScriptData>().unwrap();

    Ok((script_data.info.safe, script_data.info.path.clone()))
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
