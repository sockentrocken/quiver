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

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::{CStr, CString};

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.window", "info": "The window API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let window = lua.create_table()?;

    // WindowShouldClose
    window.set("get_close", lua.create_function(self::get_close)?)?;
    // IsWindowFullscreen
    window.set("get_fullscreen", lua.create_function(self::get_fullscreen)?)?;
    // IsWindowHidden
    window.set("get_hidden", lua.create_function(self::get_hidden)?)?;
    // IsWindowMinimized
    window.set("get_minimize", lua.create_function(self::get_minimize)?)?;
    // IsWindowMaximized
    window.set("get_maximize", lua.create_function(self::get_maximize)?)?;
    // IsWindowFocused
    window.set("get_focus", lua.create_function(self::get_focus)?)?;
    // IsWindowResized
    window.set("get_resize", lua.create_function(self::get_resize)?)?;
    // IsWindowState
    window.set("get_state", lua.create_function(self::get_state)?)?;
    // SetWindowState
    window.set("set_state", lua.create_function(self::set_state)?)?;
    // ToggleFullscreen
    window.set("set_fullscreen", lua.create_function(self::set_fullscreen)?)?;
    // ToggleBorderlessWindowed
    window.set("set_borderless", lua.create_function(self::set_borderless)?)?;
    // MaximizeWindow
    window.set("set_maximize", lua.create_function(self::set_maximize)?)?;
    // MinimizeWindow
    window.set("set_minimize", lua.create_function(self::set_minimize)?)?;
    // RestoreWindow
    window.set("set_restore", lua.create_function(self::set_restore)?)?;
    // SetWindowIcon/SetWindowIcons
    window.set("set_icon", lua.create_function(self::set_icon)?)?;
    // SetWindowTitle
    window.set("set_name", lua.create_function(self::set_name)?)?;
    // SetWindowPosition
    window.set("set_point", lua.create_function(self::set_point)?)?;
    // SetWindowMonitor
    window.set("set_screen", lua.create_function(self::set_screen)?)?;
    // SetWindowMinSize
    window.set("set_shape_min", lua.create_function(self::set_shape_min)?)?;
    // SetWindowMaxSize
    window.set("set_shape_max", lua.create_function(self::set_shape_max)?)?;
    // SetWindowSize
    window.set("set_shape", lua.create_function(self::set_shape)?)?;
    // SetWindowOpacity
    window.set("set_alpha", lua.create_function(self::set_alpha)?)?;
    // SetWindowFocused
    window.set("set_focus", lua.create_function(self::set_focus)?)?;
    // GetScreenWidth/GetScreenHeight
    window.set("get_shape", lua.create_function(self::get_shape)?)?;
    // GetRenderWidth/GetRenderHeight
    window.set("get_render_shape", lua.create_function(self::get_render_shape)?)?;
    // GetMonitorCount
    window.set("get_screen_count", lua.create_function(self::get_screen_count)?)?;
    // GetCurrentMonitor
    window.set("get_screen_focus", lua.create_function(self::get_screen_focus)?)?;
    // GetMonitorPosition
    window.set("get_screen_point", lua.create_function(self::get_screen_point)?)?;
    // GetMonitorWidth/GetMonitorHeight
    window.set("get_screen_shape", lua.create_function(self::get_screen_shape)?)?;
    // GetMonitorPhysicalWidth/GetMonitorPhysicalHeight
    window.set("get_screen_shape_physical", lua.create_function(self::get_screen_shape_physical)?)?;
    // GetMonitorRefreshRate
    window.set("get_screen_rate", lua.create_function(self::get_screen_rate)?)?;
    // GetWindowPosition
    window.set("get_point", lua.create_function(self::get_point)?)?;
    // GetWindowScaleDPI
    window.set("get_scale", lua.create_function(self::get_scale)?)?;
    // GetMonitorName
    window.set("get_screen_name", lua.create_function(self::get_screen_name)?)?;

    table.set("window", window)?;

    Ok(())
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
{ "version": "1.0.0", "name": "quiver.window.set_restore", "info": "Restore the window." }
*/
fn set_restore(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::RestoreWindow();
        Ok(())
    }
}

/* entry
{ "version": "1.0.0", "name": "quiver.window.set_icon", "info": "Set the window icon." }
*/
fn set_icon(_: &Lua, icon: LuaAnyUserData) -> mlua::Result<()> {
    if icon.is::<crate::system::image::Image>() {
        let icon = icon.borrow::<crate::system::image::Image>().unwrap();
        let icon = &*icon;

        unsafe {
            ffi::SetWindowIcon(icon.0);
            return Ok(());
        }
    }

    Err(mlua::Error::runtime(
        "set_icon(): Icon is not a valid image.",
    ))
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

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.set_screen",
    "info": "Set the window monitor.",
    "member": [
        { "name": "index", "info": "Index of monitor to move window to.", "kind": "number" }
    ]
}
*/
fn set_screen(_: &Lua, index: i32) -> mlua::Result<()> {
    unsafe {
        ffi::SetWindowMonitor(index);
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
{ "version": "1.0.0", "name": "quiver.window.set_focus", "info": "Focus the window." }
*/
fn set_focus(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::SetWindowFocused();
        Ok(())
    }
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
    "name": "quiver.window.get_render_shape",
    "info": "Get the shape of the current render view.",
    "result": [
        { "name": "shape_x", "info": "Shape of the render view (X).", "kind": "number" },
        { "name": "shape_y", "info": "Shape of the render view (Y).", "kind": "number" }
    ]
}
*/
fn get_render_shape(_: &Lua, _: ()) -> mlua::Result<(i32, i32)> {
    unsafe { Ok((ffi::GetRenderWidth(), ffi::GetRenderHeight())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_screen_count",
    "info": "Get the available monitor amount.",
    "result": [
        { "name": "count", "info": "Monitor count.", "kind": "number" }
    ]
}
*/
fn get_screen_count(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetMonitorCount()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_screen_focus",
    "info": "Get the current active monitor, where the window is.",
    "result": [
        { "name": "index", "info": "Current active monitor index.", "kind": "number" }
    ]
}
*/
fn get_screen_focus(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetCurrentMonitor()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_screen_point",
    "info": "Get the point of the given monitor.",
    "member": [
        { "name": "index", "info": "Index of the monitor.", "kind": "number" }
    ],
    "result": [
        { "name": "point_x", "info": "Point of the monitor (X).", "kind": "number" },
        { "name": "point_y", "info": "Point of the monitor (Y).", "kind": "number" }
    ]
}
*/
fn get_screen_point(_: &Lua, index: i32) -> mlua::Result<(f32, f32)> {
    unsafe {
        let value = ffi::GetMonitorPosition(index);
        Ok((value.x, value.y))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_screen_shape",
    "info": "Get the shape of the given monitor.",
    "member": [
        { "name": "index", "info": "Index of the monitor.", "kind": "number" }
    ],
    "result": [
        { "name": "shape_x", "info": "Shape of the window (X).", "kind": "number" },
        { "name": "shape_y", "info": "Shape of the window (Y).", "kind": "number" }
    ]
}
*/
fn get_screen_shape(_: &Lua, index: i32) -> mlua::Result<(i32, i32)> {
    unsafe { Ok((ffi::GetMonitorWidth(index), ffi::GetMonitorHeight(index))) }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_screen_shape_physical",
    "info": "Get the physical shape of the given monitor.",
    "member": [
        { "name": "index", "info": "Index of the monitor.", "kind": "number" }
    ],
    "result": [
        { "name": "shape_x", "info": "Physical shape of the window (X).", "kind": "number" },
        { "name": "shape_y", "info": "Physical shape of the window (Y).", "kind": "number" }
    ]
}
*/
fn get_screen_shape_physical(_: &Lua, index: i32) -> mlua::Result<(i32, i32)> {
    unsafe {
        Ok((
            ffi::GetMonitorPhysicalWidth(index),
            ffi::GetMonitorPhysicalHeight(index),
        ))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_screen_rate",
    "info": "Get the refresh rate of the given monitor.",
    "member": [
        { "name": "index", "info": "Index of the monitor.", "kind": "number" }
    ],
    "result": [
        { "name": "rate", "info": "Refresh rate of the monitor.", "kind": "number" }
    ]
}
*/
fn get_screen_rate(_: &Lua, index: i32) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetMonitorRefreshRate(index)) }
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
    "name": "quiver.window.get_screen_name",
    "info": "Get the name of the given monitor.",
    "member": [
        { "name": "index", "info": "Index of the monitor.", "kind": "number" }
    ],
    "result": [
        { "name": "name", "info": "Name of the monitor.", "kind": "string" }
    ]
}
*/
fn get_screen_name(_: &Lua, index: i32) -> mlua::Result<String> {
    unsafe {
        let name = ffi::GetMonitorName(index);
        Ok(CStr::from_ptr(name)
            .to_str()
            .map_err(|e| mlua::Error::runtime(e.to_string()))?
            .to_string())
    }
}
