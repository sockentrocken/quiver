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

use mlua::prelude::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.general", "info": "The general API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let general = lua.create_table()?;

    general.set("set_exit_key",   lua.create_function(self::set_exit_key)?)?;
    general.set("get_time",       lua.create_function(self::get_time)?)?;
    general.set("get_frame_time", lua.create_function(self::get_frame_time)?)?;
    general.set("get_frame_rate", lua.create_function(self::get_frame_rate)?)?;
    general.set("set_frame_rate", lua.create_function(self::set_frame_rate)?)?;
    //general.set("get_clipboard_text",  lua.create_function(self::get_clipboard_text)?)?;
    //general.set("set_clipboard_text",  lua.create_function(self::set_clipboard_text)?)?;
    //general.set("get_clipboard_image", lua.create_function(self::get_clipboard_image)?)?;

    table.set("general", general)?;

    Ok(())
}

//================================================================

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
    if (crate::system::input::BOARD_RANGE_LOWER..=crate::system::input::BOARD_RANGE_UPPER)
        .contains(&value)
    {
        unsafe {
            ffi::SetExitKey(value);
            Ok(())
        }
    } else {
        Err(mlua::Error::runtime("set_exit_key(): Unknown key value."))
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
}

impl Into<ffi::Camera3D> for Camera3D {
    fn into(self) -> ffi::Camera3D {
        ffi::Camera3D {
            position: self.point.into(),
            target: self.focus.into(),
            up: self.angle.into(),
            fovy: self.zoom,
            projection: 0,
        }
    }
}
