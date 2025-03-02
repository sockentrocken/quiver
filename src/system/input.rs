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

use crate::status::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::{CStr, CString};

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.input", "info": "The input API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, info: &Info, table: &mlua::Table) -> mlua::Result<()> {
    if !info.head {
        return Ok(());
    }
    
    let input = lua.create_table()?;

    // SetClipboardText
    input.set("set_clipboard_text",  lua.create_function(self::set_clipboard_text)?)?;
    // GetClipboardText
    input.set("get_clipboard_text",  lua.create_function(self::get_clipboard_text)?)?;

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
    board.set("get_press_repeat",   lua.create_function(self::get_board_press_repeat)?)?;
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

    // IsGamepadAvailable
    pad.set("get_state",      lua.create_function(self::get_pad_state)?)?;
    // GetGamepadName
    pad.set("get_name",       lua.create_function(self::get_pad_name)?)?;
    // IsGamepadButtonPressed
    pad.set("get_press",      lua.create_function(self::get_pad_press)?)?;
    // IsGamepadButtonDown
    pad.set("get_down",       lua.create_function(self::get_pad_down)?)?;
    // IsGamepadButtonReleased
    pad.set("get_release",    lua.create_function(self::get_pad_release)?)?;
    // IsGamepadButtonUp
    pad.set("get_up",         lua.create_function(self::get_pad_up)?)?;
    // GetGamepadButtonPressed
    pad.set("get_queue",      lua.create_function(self::get_pad_queue)?)?;
    // GetGamepadAxisCount
    pad.set("get_axis_count", lua.create_function(self::get_pad_axis_count)?)?;
    // GetGamepadAxisMovement
    pad.set("get_axis_state", lua.create_function(self::get_pad_axis_state)?)?;
    // SetGamepadrumble
    pad.set("set_rumble", lua.create_function(self::set_pad_rumble)?)?;
    
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
    "test": "input/clipboard_text.lua",
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
    "test": "input/clipboard_text.lua",
    "result": [
        { "name": "text", "info": "Clipboard text.", "kind": "string" }
    ]
}
*/
fn get_clipboard_text(_: &Lua, _: ()) -> mlua::Result<String> {
    unsafe {
        let name = ffi::GetClipboardText();
        Ok(CStr::from_ptr(name)
            .to_str()
            .map_err(|e| mlua::Error::runtime(e.to_string()))?
            .to_string())
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
    "test": "input/get_name.lua",
    "member": [
        { "name": "board", "info": "The board button to get a name for.", "kind": "input_board" }
    ],
    "result": [
        { "name": "name", "info": "The name.", "kind": "string" }
    ]
}
*/
fn get_board_name(_: &Lua, value: i32) -> mlua::Result<String> {
    unsafe {
        let name = ffi::GetKeyName(value);

        if name.is_null() {
            Err(mlua::Error::runtime("get_board_name(): Unknown key code."))
        } else {
            Ok(CStr::from_ptr(name)
                .to_str()
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .to_string())
        }
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
    unsafe { Ok(ffi::IsKeyUp(value)) }
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
    unsafe { Ok(ffi::IsKeyDown(value)) }
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
    unsafe { Ok(ffi::IsKeyPressed(value)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.input.board.get_press_repeat",
    "info": "Get the state of an input (repeat-press).",
    "member": [
        { "name": "board", "info": "The board button to check for.", "kind": "input_board" }
    ]
}
*/
fn get_board_press_repeat(_: &Lua, value: i32) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsKeyPressedRepeat(value)) }
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
    unsafe { Ok(ffi::IsKeyReleased(value)) }
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
    unsafe {
        ffi::SetMouseCursor(value);
        Ok(())
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
    unsafe { Ok(ffi::IsMouseButtonUp(value)) }
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
    unsafe { Ok(ffi::IsMouseButtonDown(value)) }
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
    unsafe { Ok(ffi::IsMouseButtonPressed(value)) }
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
    unsafe { Ok(ffi::IsMouseButtonReleased(value)) }
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
        Ok(CStr::from_ptr(name)
            .to_str()
            .map_err(|e| mlua::Error::runtime(e.to_string()))?
            .to_string())
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
    unsafe { Ok(ffi::IsGamepadButtonPressed(index, value)) }
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
    unsafe { Ok(ffi::IsGamepadButtonDown(index, value)) }
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
    unsafe { Ok(ffi::IsGamepadButtonReleased(index, value)) }
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
    unsafe { Ok(ffi::IsGamepadButtonUp(index, value)) }
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
    "name": "quiver.input.pad.set_rumble",
    "info": "Set the rumble of a pad.",
    "member": [
        { "name": "index",    "info": "The index of the pad to rumble.",       "kind": "number" },
        { "name": "motor_a",  "info": "The intensity of the L. rumble motor.", "kind": "number" },
        { "name": "motor_b",  "info": "The intensity of the R. rumble motor.", "kind": "number" },
        { "name": "duration", "info": "The duration of the rumble.",           "kind": "number" }
    ]
}
*/
fn set_pad_rumble(
    _: &Lua,
    (index, motor_a, motor_b, duration): (i32, f32, f32, f32),
) -> mlua::Result<()> {
    unsafe {
        ffi::SetGamepadVibration(index, motor_a, motor_b, duration);
        Ok(())
    }
}
