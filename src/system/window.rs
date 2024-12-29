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
use std::ffi::CString;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.window", "info": "The window API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let window = lua.create_table()?;

    window.set("set_state", lua.create_function(self::set_state)?)?;
    window.set("set_fullscreen", lua.create_function(self::set_fullscreen)?)?;
    window.set("set_borderless", lua.create_function(self::set_borderless)?)?;
    window.set("set_minimize", lua.create_function(self::set_minimize)?)?;
    window.set("set_maximize", lua.create_function(self::set_maximize)?)?;
    window.set("set_focus", lua.create_function(self::set_focus)?)?;
    window.set("set_restore", lua.create_function(self::set_restore)?)?;
    window.set("set_name", lua.create_function(self::set_name)?)?;
    window.set("set_monitor", lua.create_function(self::set_monitor)?)?;
    window.set("set_shape", lua.create_function(self::set_shape)?)?;
    window.set("set_shape_min", lua.create_function(self::set_shape_min)?)?;
    window.set("set_shape_max", lua.create_function(self::set_shape_max)?)?;
    window.set("set_alpha", lua.create_function(self::set_alpha)?)?;
    window.set("set_point", lua.create_function(self::set_point)?)?;

    window.set("get_state", lua.create_function(self::get_state)?)?;
    window.set("get_fullscreen", lua.create_function(self::get_fullscreen)?)?;
    window.set("get_minimize", lua.create_function(self::get_minimize)?)?;
    window.set("get_maximize", lua.create_function(self::get_maximize)?)?;
    window.set("get_focus", lua.create_function(self::get_focus)?)?;
    window.set("get_resize", lua.create_function(self::get_resize)?)?;
    window.set("get_hidden", lua.create_function(self::get_hidden)?)?;
    window.set("get_shape", lua.create_function(self::get_shape)?)?;
    window.set("get_point", lua.create_function(self::get_point)?)?;
    window.set("get_scale", lua.create_function(self::get_scale)?)?;
    window.set("get_close", lua.create_function(self::get_close)?)?;

    table.set("window", window)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_state",
    "info": "Get the state of a window flag.",
    "member": [
        { "name": "flag", "info": "Window flag.", "kind": "window_flag" }
    ],
    "result": [
        { "name": "state", "info": "Window flag state.", "kind": "boolean" }
    ]
}
*/
fn get_state(_: &Lua, flag: u32) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowState(flag)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.set_state",
    "info": "Set the state of a window flag.",
    "member": [
        { "name": "flag",  "info": "Window flag.",       "kind": "window_flag" },
        { "name": "state", "info": "Window flag state.", "kind": "boolean"     }
    ]
}
*/
fn set_state(_: &Lua, (flag, state): (u32, bool)) -> mlua::Result<()> {
    unsafe {
        if state {
            ffi::SetWindowState(flag);
        } else {
            ffi::ClearWindowState(flag);
        }

        Ok(())
    }
}

/* entry
{ "version": "1.0.0", "name": "quiver.window.set_fullscreen", "info": "Set the window to full-screen mode." }
*/
fn set_fullscreen(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::ToggleFullscreen();
        Ok(())
    }
}

/* entry
{ "version": "1.0.0", "name": "quiver.window.set_borderless", "info": "Set the window to border-less mode." }
*/
fn set_borderless(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::ToggleBorderlessWindowed();
        Ok(())
    }
}

/* entry
{ "version": "1.0.0", "name": "quiver.window.set_minimize", "info": "Minimize the window." }
*/
fn set_minimize(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::MinimizeWindow();
        Ok(())
    }
}

/* entry
{ "version": "1.0.0", "name": "quiver.window.set_maximize", "info": "Maximize the window." }
*/
fn set_maximize(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::MaximizeWindow();
        Ok(())
    }
}

/* entry
{ "version": "1.0.0", "name": "quiver.window.set_focus", "info": "Focus the window." }
*/
fn set_focus(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::SetWindowFocused();
        Ok(())
    }
}

/* entry
{ "version": "1.0.0", "name": "quiver.window.set_restore", "info": "Restore the window." }
*/
fn set_restore(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::RestoreWindow();
        Ok(())
    }
}

/* entry
{ "version": "1.0.0", "name": "quiver.window.set_name", "info": "Set the window name." }
*/
fn set_name(_: &Lua, text: String) -> mlua::Result<()> {
    let text = CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe {
        ffi::SetWindowTitle(text.as_ptr());
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.set_monitor",
    "info": "Set the window monitor.",
    "member": [
        { "name": "index", "info": "Index of monitor to move window to.", "kind": "number" }
    ]
}
*/
fn set_monitor(_: &Lua, index: i32) -> mlua::Result<()> {
    unsafe {
        ffi::SetWindowMonitor(index);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.set_shape",
    "info": "Set the current window shape.",
    "member": [
        { "name": "shape", "info": "Shape of the window.", "kind": "vector_2" }
    ]
}
*/
fn set_shape(lua: &Lua, shape: LuaValue) -> mlua::Result<()> {
    let shape: Vector2 = lua.from_value(shape)?;

    unsafe {
        ffi::SetWindowSize(shape.x as i32, shape.y as i32);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.set_shape_min",
    "info": "Set the minimum window shape.",
    "member": [
        { "name": "shape", "info": "Minimum shape of the window.", "kind": "vector_2" }
    ]
}
*/
fn set_shape_min(lua: &Lua, shape: LuaValue) -> mlua::Result<()> {
    let shape: Vector2 = lua.from_value(shape)?;

    unsafe {
        ffi::SetWindowMinSize(shape.x as i32, shape.y as i32);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.set_shape_max",
    "info": "Set the maximum window shape.",
    "member": [
        { "name": "shape", "info": "Maximum shape of the window.", "kind": "vector_2" }
    ]
}
*/
fn set_shape_max(lua: &Lua, shape: LuaValue) -> mlua::Result<()> {
    let shape: Vector2 = lua.from_value(shape)?;

    unsafe {
        ffi::SetWindowMaxSize(shape.x as i32, shape.y as i32);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.set_alpha",
    "info": "Set the window alpha.",
    "member": [
        { "name": "alpha", "info": "Alpha of the window.", "kind": "number" }
    ]
}
*/
fn set_alpha(_: &Lua, alpha: f32) -> mlua::Result<()> {
    unsafe {
        ffi::SetWindowOpacity(alpha);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.set_point",
    "info": "Set the window point.",
    "member": [
        { "name": "point", "info": "Point of the window.", "kind": "vector_2" }
    ]
}
*/
fn set_point(lua: &Lua, shape: LuaValue) -> mlua::Result<()> {
    let shape: Vector2 = lua.from_value(shape)?;

    unsafe {
        ffi::SetWindowPosition(shape.x as i32, shape.y as i32);
        Ok(())
    }
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_fullscreen",
    "info": "Get the state of the window (full-screen).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_fullscreen(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowFullscreen()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_minimize",
    "info": "Get the state of the window (minimize).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_minimize(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowMinimized()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_maximize",
    "info": "Get the state of the window (maximize).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_maximize(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowMaximized()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_focus",
    "info": "Get the state of the window (focus).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_focus(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowFocused()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_resize",
    "info": "Get the state of the window (resize).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_resize(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowResized()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_hidden",
    "info": "Get the state of the window (hidden).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_hidden(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowHidden()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_shape",
    "info": "Get the shape of the window.",
    "result": [
        { "name": "shape_x", "info": "Shape of the window (X).", "kind": "number" },
        { "name": "shape_y", "info": "Shape of the window (Y).", "kind": "number" }
    ]
}
*/
fn get_shape(_: &Lua, _: ()) -> mlua::Result<(i32, i32)> {
    unsafe { Ok((ffi::GetScreenWidth(), ffi::GetScreenHeight())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_point",
    "info": "Get the point of the window.",
    "result": [
        { "name": "point_x", "info": "Point of the window (X).", "kind": "number" },
        { "name": "point_y", "info": "Point of the window (Y).", "kind": "number" }
    ]
}
*/
fn get_point(_: &Lua, _: ()) -> mlua::Result<(f32, f32)> {
    unsafe {
        let value = ffi::GetWindowPosition();

        Ok((value.x, value.y))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_scale",
    "info": "Get the DPI scale of the window.",
    "result": [
        { "name": "scale_x", "info": "Scale of the window (X).", "kind": "number" },
        { "name": "scale_y", "info": "Scale of the window (Y).", "kind": "number" }
    ]
}
*/
fn get_scale(_: &Lua, _: ()) -> mlua::Result<(f32, f32)> {
    unsafe {
        let value = ffi::GetWindowScaleDPI();

        Ok((value.x, value.y))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_close",
    "info": "Get if the window should close.",
    "result": [
        { "name": "close", "info": "True if the window should close.", "kind": "boolean" }
    ]
}
*/
fn get_close(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::WindowShouldClose()) }
}
