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
use std::ffi::{CStr, CString};

//================================================================

pub const BOARD_RANGE_LOWER: i32 = 0;
pub const BOARD_RANGE_UPPER: i32 = 384;
pub const MOUSE_RANGE_LOWER: i32 = 0;
pub const MOUSE_RANGE_UPPER: i32 = 6;
pub const CURSOR_RANGE_LOWER: i32 = 0;
pub const CURSOR_RANGE_UPPER: i32 = 10;
pub const PAD_RANGE_LOWER: i32 = 0;
pub const PAD_RANGE_UPPER: i32 = 17;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.input", "info": "The input API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let input = lua.create_table()?;

    input.set("set_clipboard_text", lua.create_function(self::set_clipboard_text)?)?;
    input.set("get_clipboard_text", lua.create_function(self::get_clipboard_text)?)?;

    //================================================================

    /* class
    { "version": "1.0.0", "name": "quiver.input.board", "info": "The board input API." }
    */
    let board = lua.create_table()?;

    board.set("get_key_code_queue", lua.create_function(self::get_board_key_code_queue)?)?;
    board.set("get_uni_code_queue", lua.create_function(self::get_board_uni_code_queue)?)?;
    board.set("get_name",           lua.create_function(self::get_board_name)?)?;
    board.set("get_up",             lua.create_function(self::get_board_up)?)?;
    board.set("get_down",           lua.create_function(self::get_board_down)?)?;
    board.set("get_press",          lua.create_function(self::get_board_press)?)?;
    board.set("get_release",        lua.create_function(self::get_board_release)?)?;

    input.set("board", board)?;

    //================================================================

    /* class
    { "version": "1.0.0", "name": "quiver.input.mouse", "info": "The mouse input API." }
    */
    let mouse = lua.create_table()?;

    mouse.set("set_active",  lua.create_function(self::set_mouse_active)?)?;
    mouse.set("set_hidden",  lua.create_function(self::set_mouse_hidden)?)?;
    mouse.set("get_hidden",  lua.create_function(self::get_mouse_hidden)?)?;
    mouse.set("get_screen",  lua.create_function(self::get_mouse_screen)?)?;
    mouse.set("get_point",   lua.create_function(self::get_mouse_point)?)?;
    mouse.set("get_delta",   lua.create_function(self::get_mouse_delta)?)?;
    mouse.set("set_point",   lua.create_function(self::set_mouse_point)?)?;
    mouse.set("set_shift",   lua.create_function(self::set_mouse_shift)?)?;
    mouse.set("set_scale",   lua.create_function(self::set_mouse_scale)?)?;
    mouse.set("set_cursor",  lua.create_function(self::set_mouse_cursor)?)?;
    mouse.set("get_wheel",   lua.create_function(self::get_mouse_wheel)?)?;
    mouse.set("get_up",      lua.create_function(self::get_mouse_up)?)?;
    mouse.set("get_down",    lua.create_function(self::get_mouse_down)?)?;
    mouse.set("get_press",   lua.create_function(self::get_mouse_press)?)?;
    mouse.set("get_release", lua.create_function(self::get_mouse_release)?)?;

    input.set("mouse", mouse)?;

    //================================================================

    /* class
    { "version": "1.0.0", "name": "quiver.input.pad", "info": "The pad input API." }
    */
    let pad = lua.create_table()?;

    pad.set("get_state",      lua.create_function(self::get_pad_state)?)?;
    pad.set("get_name",       lua.create_function(self::get_pad_name)?)?;
    pad.set("get_queue",      lua.create_function(self::get_pad_queue)?)?;
    pad.set("get_axis_count", lua.create_function(self::get_pad_axis_count)?)?;
    pad.set("get_axis_state", lua.create_function(self::get_pad_axis_state)?)?;
    pad.set("get_up",         lua.create_function(self::get_pad_up)?)?;
    pad.set("get_down",       lua.create_function(self::get_pad_down)?)?;
    pad.set("get_press",      lua.create_function(self::get_pad_press)?)?;
    pad.set("get_release",    lua.create_function(self::get_pad_release)?)?;

    input.set("pad", pad)?;

    //================================================================

    table.set("input", input)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.board.set_clipboard_text",
    "info": "Set the clipboard text.",
    "member": [
        { "name": "text", "info": "Clipboard text.", "kind": "string" }
    ]
}
*/
fn set_clipboard_text(_: &Lua, text: String) -> mlua::Result<()> {
    let text = CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe {
        ffi::SetClipboardText(text.as_ptr());
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.board.get_clipboard_text",
    "info": "Get the clipboard text.",
    "result": [
        { "name": "text", "info": "Clipboard text.", "kind": "string" }
    ]
}
*/
fn get_clipboard_text(_: &Lua, _: ()) -> mlua::Result<String> {
    unsafe {
        let name = ffi::GetClipboardText();
        Ok(CStr::from_ptr(name).to_str().unwrap().to_string())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.board.get_key_code_queue",
    "info": "Get the last unicode glyph in the queue.",
    "result": [
        { "name": "key_code", "info": "Key-code. If 0, queue is empty.", "kind": "number" }
    ]
}
*/
fn get_board_key_code_queue(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetKeyPressed()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.board.get_uni_code_queue",
    "info": "Get the last unicode glyph in the queue.",
    "result": [
        { "name": "uni_code", "info": "Uni-code. If 0, queue is empty.", "kind": "number" }
    ]
}
*/
fn get_board_uni_code_queue(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetCharPressed()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.board.get_name",
    "info": "Get the name of a given key.",
    "member": [
        { "name": "board", "info": "The board button to get a name for.", "kind": "input_board" }
    ],
    "result": [
        { "name": "name", "info": "The name.", "kind": "string" }
    ]
}
*/
fn get_board_name(_: &Lua, value: i32) -> mlua::Result<String> {
    if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
        unsafe {
            let name = ffi::GetKeyName(value);
            Ok(CStr::from_ptr(name).to_str().unwrap().to_string())
        }
    } else {
        Err(mlua::Error::runtime("get_board_up(): Unknown value."))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.board.get_up",
    "info": "Get the state of an input (up).",
    "member": [
        { "name": "board", "info": "The board button to check for.", "kind": "input_board" }
    ]
}
*/
fn get_board_up(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsKeyUp(value)) }
    } else {
        Err(mlua::Error::runtime("get_board_up(): Unknown value."))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.board.get_down",
    "info": "Get the state of an input (down).",
    "member": [
        { "name": "board", "info": "The board button to check for.", "kind": "input_board" }
    ]
}
*/
fn get_board_down(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsKeyDown(value)) }
    } else {
        Err(mlua::Error::runtime("get_board_down(): Unknown value."))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.board.get_press",
    "info": "Get the state of an input (press).",
    "member": [
        { "name": "board", "info": "The board button to check for.", "kind": "input_board" }
    ]
}
*/
fn get_board_press(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsKeyPressed(value)) }
    } else {
        Err(mlua::Error::runtime("get_board_press(): Unknown value."))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.board.get_release",
    "info": "Get the state of an input (release).",
    "member": [
        { "name": "board", "info": "The board button to check for.", "kind": "input_board" }
    ]
}
*/
fn get_board_release(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsKeyReleased(value)) }
    } else {
        Err(mlua::Error::runtime("get_board_release(): Unknown value."))
    }
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.set_active",
    "info": "Set the active state of the mouse.",
    "member": [
        { "name": "state", "info": "Current state.", "kind": "boolean" }
    ]
}
*/
fn set_mouse_active(_: &Lua, value: bool) -> mlua::Result<()> {
    unsafe {
        if value {
            ffi::EnableCursor();
            Ok(())
        } else {
            ffi::DisableCursor();
            Ok(())
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.set_hidden",
    "info": "Set the hidden state of the mouse.",
    "member": [
        { "name": "state", "info": "Current state.", "kind": "boolean" }
    ]
}
*/
fn set_mouse_hidden(_: &Lua, value: bool) -> mlua::Result<()> {
    unsafe {
        if value {
            ffi::HideCursor();
            Ok(())
        } else {
            ffi::ShowCursor();
            Ok(())
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.get_hidden",
    "info": "Get the hidden state of the mouse.",
    "result": [
        { "name": "state", "info": "Current state.", "kind": "boolean" }
    ]
}
*/
fn get_mouse_hidden(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsCursorHidden()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.get_screen",
    "info": "Check if the mouse is currently over the screen.",
    "result": [
        { "name": "state", "info": "Current state.", "kind": "boolean" }
    ]
}
*/
fn get_mouse_screen(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsCursorOnScreen()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.get_point",
    "info": "Get the current point of the mouse.",
    "result": [
        { "name": "point_x", "info": "The point of the mouse (X).", "kind": "number" },
        { "name": "point_y", "info": "The point of the mouse (Y).", "kind": "number" }
    ]
}
*/
fn get_mouse_point(_: &Lua, _: ()) -> mlua::Result<(f32, f32)> {
    unsafe {
        let value = ffi::GetMousePosition();
        Ok((value.x, value.y))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.set_point",
    "info": "Set the current point of the mouse.",
    "member": [
        { "name": "point", "info": "The point of the mouse.", "kind": "vector_2" }
    ]
}
*/
fn set_mouse_point(lua: &Lua, point: LuaValue) -> mlua::Result<()> {
    unsafe {
        let point: Vector2 = lua.from_value(point)?;
        ffi::SetMousePosition(point.x as i32, point.y as i32);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.get_delta",
    "info": "Get the current delta (i.e. mouse movement) of the mouse.",
    "result": [
        { "name": "delta_x", "info": "The delta of the mouse (X).", "kind": "number" },
        { "name": "delta_y", "info": "The delta of the mouse (Y).", "kind": "number" }
    ]
}
*/
fn get_mouse_delta(_: &Lua, _: ()) -> mlua::Result<(f32, f32)> {
    unsafe {
        let value = ffi::GetMouseDelta();
        Ok((value.x, value.y))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.set_shift",
    "info": "Set the current shift of the mouse.",
    "member": [
        { "name": "shift", "info": "The shift of the mouse.", "kind": "vector_2" }
    ]
}
*/
fn set_mouse_shift(lua: &Lua, point: LuaValue) -> mlua::Result<()> {
    unsafe {
        let point: Vector2 = lua.from_value(point)?;
        ffi::SetMouseOffset(point.x as i32, point.y as i32);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.set_scale",
    "info": "Set the current scale of the mouse.",
    "member": [
        { "name": "scale", "info": "The scale of the mouse.", "kind": "vector_2" }
    ]
}
*/
fn set_mouse_scale(lua: &Lua, point: LuaValue) -> mlua::Result<()> {
    unsafe {
        let point: Vector2 = lua.from_value(point)?;
        ffi::SetMouseScale(point.x, point.y);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.set_cursor",
    "info": "Set the current cursor of the mouse.",
    "member": [
        { "name": "cursor", "info": "The cursor of the mouse.", "kind": "cursor_mouse" }
    ]
}
*/
fn set_mouse_cursor(_: &Lua, value: i32) -> mlua::Result<()> {
    if (self::CURSOR_RANGE_LOWER..=self::CURSOR_RANGE_UPPER).contains(&value) {
        unsafe {
            ffi::SetMouseCursor(value);
            Ok(())
        }
    } else {
        Err(mlua::Error::runtime(
            "set_mouse_cursor(): Unknown cursor value.",
        ))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.get_wheel",
    "info": "Get the current delta (i.e. mouse wheel movement) of the mouse wheel.",
    "result": [
        { "name": "delta_x", "info": "The delta of the mouse wheel (X).", "kind": "number" },
        { "name": "delta_y", "info": "The delta of the mouse wheel (Y).", "kind": "number" }
    ]
}
*/
fn get_mouse_wheel(_: &Lua, _: ()) -> mlua::Result<(f32, f32)> {
    unsafe {
        let value = ffi::GetMouseWheelMoveV();
        Ok((value.x, value.y))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.get_up",
    "info": "Get the state of an input (up).",
    "member": [
        { "name": "mouse", "info": "The mouse button to check for.", "kind": "input_mouse" }
    ]
}
*/
fn get_mouse_up(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsMouseButtonUp(value)) }
    } else {
        Err(mlua::Error::runtime("get_mouse_up(): Unknown value."))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.get_down",
    "info": "Get the state of an input (down).",
    "member": [
        { "name": "mouse", "info": "The mouse button to check for.", "kind": "input_mouse" }
    ]
}
*/
fn get_mouse_down(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsMouseButtonDown(value)) }
    } else {
        Err(mlua::Error::runtime("get_mouse_down(): Unknown value."))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.get_press",
    "info": "Get the state of an input (press).",
    "member": [
        { "name": "mouse", "info": "The mouse button to check for.", "kind": "input_mouse" }
    ]
}
*/
fn get_mouse_press(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsMouseButtonPressed(value)) }
    } else {
        Err(mlua::Error::runtime("get_mouse_press(): Unknown value."))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.mouse.get_release",
    "info": "Get the state of an input (release).",
    "member": [
        { "name": "mouse", "info": "The mouse button to check for.", "kind": "input_mouse" }
    ]
}
*/
fn get_mouse_release(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsMouseButtonReleased(value)) }
    } else {
        Err(mlua::Error::runtime("get_mouse_release(): Unknown value."))
    }
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.pad.get_state",
    "info": "Get the state of a pad.",
    "member": [
        { "name": "index", "info": "The index of the pad to check for.", "kind": "number" }
    ],
    "result": [
        { "name": "state", "info": "The state of the pad.", "kind": "boolean" }
    ]
}
*/
fn get_pad_state(_: &Lua, index: i32) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsGamepadAvailable(index)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.pad.get_name",
    "info": "Get the name of a pad.",
    "member": [
        { "name": "index", "info": "The index of the pad to check for.", "kind": "number" }
    ],
    "result": [
        { "name": "name", "info": "The name of the pad.", "kind": "string" }
    ]
}
*/
fn get_pad_name(_: &Lua, index: i32) -> mlua::Result<String> {
    unsafe {
        let name = ffi::GetGamepadName(index);
        Ok(CStr::from_ptr(name).to_str().unwrap().to_string())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.pad.get_queue",
    "info": "Get the last pad button press.",
    "result": [
        { "name": "input", "info": "The last pad button press.", "kind": "input_pad" }
    ]
}
*/
fn get_pad_queue(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetGamepadButtonPressed()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.pad.get_axis_count",
    "info": "Get the axis count of a pad.",
    "member": [
        { "name": "index", "info": "The index of the pad to check for.", "kind": "number" }
    ],
    "result": [
        { "name": "axis_count", "info": "The axis count of the pad.", "kind": "number" }
    ]
}
*/
fn get_pad_axis_count(_: &Lua, index: i32) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetGamepadAxisCount(index)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.pad.get_axis_state",
    "info": "Get the axis state of a pad.",
    "member": [
        { "name": "index", "info": "The index of the pad to check for.", "kind": "number" },
        { "name": "axis",  "info": "The axis of the pad to check for.",  "kind": "number" }
    ],
    "result": [
        { "name": "axis_state", "info": "The axis state of the pad.", "kind": "number" }
    ]
}
*/
fn get_pad_axis_state(_: &Lua, (index, axis): (i32, i32)) -> mlua::Result<f32> {
    unsafe { Ok(ffi::GetGamepadAxisMovement(index, axis)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.pad.get_up",
    "info": "Get the state of an input (up).",
    "member": [
        { "name": "pad", "info": "The pad button to check for.", "kind": "input_pad" }
    ]
}
*/
fn get_pad_up(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
    if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsGamepadButtonUp(index, value)) }
    } else {
        Err(mlua::Error::runtime("get_pad_up(): Unknown value."))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.pad.get_down",
    "info": "Get the state of an input (down).",
    "member": [
        { "name": "pad", "info": "The pad button to check for.", "kind": "input_pad" }
    ]
}
*/
fn get_pad_down(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
    if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsGamepadButtonDown(index, value)) }
    } else {
        Err(mlua::Error::runtime("get_pad_down(): Unknown value."))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.pad.get_press",
    "info": "Get the state of an input (press).",
    "member": [
        { "name": "pad", "info": "The pad button to check for.", "kind": "input_pad" }
    ]
}
*/
fn get_pad_press(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
    if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsGamepadButtonPressed(index, value)) }
    } else {
        Err(mlua::Error::runtime("get_pad_press(): Unknown value."))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.pad.get_release",
    "info": "Get the state of an input (release).",
    "member": [
        { "name": "pad", "info": "The pad button to check for.", "kind": "input_pad" }
    ]
}
*/
fn get_pad_release(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
    if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsGamepadButtonReleased(index, value)) }
    } else {
        Err(mlua::Error::runtime("get_pad_release(): Unknown value."))
    }
}
