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
use crate::status::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
};

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.window", "info": "The window API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, info: &Info, table: &mlua::Table) -> mlua::Result<()> {
    if !info.head {
        return Ok(());
    }
    
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
    // TakeScreenshot
    window.set("get_screen_shot", lua.create_function(self::get_screen_shot)?)?;

    /* RFD specific */
    window.set("file_dialog", lua.create_async_function(self::file_dialog)?)?;
    window.set("text_dialog", lua.create_async_function(self::text_dialog)?)?;

    table.set("window", window)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.file_dialog",
    "info": "TO-DO"
}
*/
async fn file_dialog(
    lua: Lua,
    (kind, filter, path, name, title): (
        i32,
        Option<LuaValue>,
        Option<String>,
        Option<String>,
        Option<String>,
    ),
) -> mlua::Result<LuaValue> {
    let mut file = rfd::AsyncFileDialog::new();

    if let Some(filter) = filter {
        let filter: HashMap<String, Vec<String>> = lua.from_value(filter)?;

        for (k, v) in filter {
            println!("adding filter {k} : {v:?}");
            file = file.add_filter(k, &v);
        }
    }

    if let Some(path) = path {
        file = file.set_directory(path);
    }

    if let Some(name) = name {
        file = file.set_file_name(name);
    }

    if let Some(title) = title {
        file = file.set_title(title);
    }

    match kind {
        0..2 => {
            let file = {
                match kind {
                    0 => file.pick_file().await,
                    _ => file.pick_folder().await,
                }
            };

            if let Some(file) = file {
                lua.to_value(&file.path().display().to_string())
            } else {
                Ok(mlua::Nil)
            }
        }
        2..4 => {
            let file = {
                match kind {
                    2 => file.pick_files().await,
                    _ => file.pick_folders().await,
                }
            };

            if let Some(file) = file {
                let mut result = Vec::new();

                for entry in file {
                    result.push(entry.path().display().to_string());
                }

                lua.to_value(&result)
            } else {
                Ok(mlua::Nil)
            }
        }
        _ => {
            let file = file.save_file().await;

            if let Some(file) = file {
                lua.to_value(&file.path().display().to_string())
            } else {
                Ok(mlua::Nil)
            }
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.text_dialog",
    "info": "TO-DO"
}
*/
async fn text_dialog(
    lua: Lua,
    (level, title, description, button): (Option<i32>, Option<String>, Option<String>, Option<i32>),
) -> mlua::Result<LuaValue> {
    let mut text = rfd::AsyncMessageDialog::new();

    if let Some(level) = level {
        match level {
            0 => text = text.set_level(rfd::MessageLevel::Info),
            1 => text = text.set_level(rfd::MessageLevel::Warning),
            _ => text = text.set_level(rfd::MessageLevel::Error),
        }
    }

    if let Some(title) = title {
        text = text.set_title(title)
    }

    if let Some(description) = description {
        text = text.set_description(description)
    }

    if let Some(button) = button {
        match button {
            0 => text = text.set_buttons(rfd::MessageButtons::Ok),
            1 => text = text.set_buttons(rfd::MessageButtons::OkCancel),
            2 => text = text.set_buttons(rfd::MessageButtons::YesNo),
            _ => text = text.set_buttons(rfd::MessageButtons::YesNoCancel),
        }
    }

    match text.show().await {
        rfd::MessageDialogResult::Yes => lua.to_value(&0),
        rfd::MessageDialogResult::No => lua.to_value(&1),
        rfd::MessageDialogResult::Ok => lua.to_value(&2),
        rfd::MessageDialogResult::Cancel => lua.to_value(&3),
        rfd::MessageDialogResult::Custom(result) => lua.to_value(&result),
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

/* entry
{
    "version": "1.0.0",
    "name": "quiver.window.get_screen_shot",
    "info": "TO-DO"
}
*/
fn get_screen_shot(lua: &Lua, path: String) -> mlua::Result<()> {
    unsafe {
        let path = ScriptData::get_path(lua, &path)?;
        let path = Script::rust_to_c_string(&path)?;
        ffi::TakeScreenshot(path.as_ptr());
        Ok(())
    }
}
