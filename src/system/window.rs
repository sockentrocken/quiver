use crate::script::*;
use crate::system::*;

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

/* class
{ "name": "quiver.window", "info": "The window API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _system : &ModuleSystem) -> mlua::Result<()> {
    let window = lua.create_table()?;

    window.set("set_fullscreen", lua.create_function(self::set_window_fullscreen)?)?;
    window.set("set_borderless", lua.create_function(self::set_window_borderless)?)?;
    window.set("set_minimize", lua.create_function(self::set_window_minimize)?)?;
    window.set("set_maximize", lua.create_function(self::set_window_maximize)?)?;
    window.set("set_focus", lua.create_function(self::set_window_focus)?)?;
    window.set("set_restore", lua.create_function(self::set_window_restore)?)?;
    //window.set("set_icon", lua.create_function(self::set_window_icon)?)?;
    window.set("set_name", lua.create_function(self::set_window_name)?)?;
    window.set("set_monitor", lua.create_function(self::set_window_monitor)?)?;
    window.set("set_shape", lua.create_function(self::set_window_shape)?)?;
    window.set("set_shape_min", lua.create_function(self::set_window_shape_min)?)?;
    window.set("set_shape_max", lua.create_function(self::set_window_shape_max)?)?;
    window.set("set_alpha", lua.create_function(self::set_window_alpha)?)?;
    window.set("set_point", lua.create_function(self::set_window_point)?)?;
    window.set("get_fullscreen", lua.create_function(self::get_window_fullscreen)?)?;
    window.set("get_minimize", lua.create_function(self::get_window_minimize)?)?;
    window.set("get_maximize", lua.create_function(self::get_window_maximize)?)?;
    window.set("get_focus", lua.create_function(self::get_window_focus)?)?;
    window.set("get_resize", lua.create_function(self::get_window_resize)?)?;
    window.set("get_hidden", lua.create_function(self::get_window_hidden)?)?;
    window.set("get_shape", lua.create_function(self::get_window_shape)?)?;
    window.set("get_point", lua.create_function(self::get_window_point)?)?;
    window.set("get_scale", lua.create_function(self::get_window_scale)?)?;

    table.set("window", window)?;

    Ok(())
}

/* entry
{ "name": "quiver.window.set_fullscreen", "info": "Set the window to full-screen mode." }
*/
fn set_window_fullscreen(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::ToggleFullscreen();
        Ok(())
    }
}

/* entry
{ "name": "quiver.window.set_borderless", "info": "Set the window to border-less mode." }
*/
fn set_window_borderless(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::ToggleBorderlessWindowed();
        Ok(())
    }
}

/* entry
{ "name": "quiver.window.set_minimize", "info": "Minimize the window." }
*/
fn set_window_minimize(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::MinimizeWindow();
        Ok(())
    }
}

/* entry
{ "name": "quiver.window.set_maximize", "info": "Maximize the window." }
*/
fn set_window_maximize(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::MaximizeWindow();
        Ok(())
    }
}

/* entry
{ "name": "quiver.window.set_focus", "info": "Focus the window." }
*/
fn set_window_focus(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::SetWindowFocused();
        Ok(())
    }
}

/* entry
{ "name": "quiver.window.set_restore", "info": "Restore the window." }
*/
fn set_window_restore(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::RestoreWindow();
        Ok(())
    }
}

/* entry
{ "name": "quiver.window.set_name", "info": "Set the window name." }
*/
fn set_window_name(_: &Lua, text: String) -> mlua::Result<()> {
    let text = CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe {
        ffi::SetWindowTitle(text.as_ptr());
        Ok(())
    }
}

/* entry
{
    "name": "quiver.window.set_monitor",
    "info": "Set the window monitor.",
    "member": [
        { "name": "index", "info": "Index of monitor to move window to.", "kind": "number" }
    ]
}
*/
fn set_window_monitor(_: &Lua, index: i32) -> mlua::Result<()> {
    unsafe {
        ffi::SetWindowMonitor(index);
        Ok(())
    }
}

/* entry
{
    "name": "quiver.window.set_shape",
    "info": "Set the current window shape.",
    "member": [
        { "name": "shape", "info": "Shape of the window.", "kind": "vector_2" }
    ]
}
*/
fn set_window_shape(lua: &Lua, shape: LuaValue) -> mlua::Result<()> {
    let shape: general::Vector2 = lua.from_value(shape)?;

    unsafe {
        ffi::SetWindowSize(shape.x as i32, shape.y as i32);
        Ok(())
    }
}

/* entry
{
    "name": "quiver.window.set_shape_min",
    "info": "Set the minimum window shape.",
    "member": [
        { "name": "shape", "info": "Minimum shape of the window.", "kind": "vector_2" }
    ]
}
*/
fn set_window_shape_min(lua: &Lua, shape: LuaValue) -> mlua::Result<()> {
    let shape: general::Vector2 = lua.from_value(shape)?;

    unsafe {
        ffi::SetWindowMinSize(shape.x as i32, shape.y as i32);
        Ok(())
    }
}

/* entry
{
    "name": "quiver.window.set_shape_max",
    "info": "Set the maximum window shape.",
    "member": [
        { "name": "shape", "info": "Maximum shape of the window.", "kind": "vector_2" }
    ]
}
*/
fn set_window_shape_max(lua: &Lua, shape: LuaValue) -> mlua::Result<()> {
    let shape: general::Vector2 = lua.from_value(shape)?;

    unsafe {
        ffi::SetWindowMaxSize(shape.x as i32, shape.y as i32);
        Ok(())
    }
}

/* entry
{
    "name": "quiver.window.set_alpha",
    "info": "Set the window alpha.",
    "member": [
        { "name": "alpha", "info": "Alpha of the window.", "kind": "number" }
    ]
}
*/
fn set_window_alpha(_: &Lua, alpha: f32) -> mlua::Result<()> {
    unsafe {
        ffi::SetWindowOpacity(alpha);
        Ok(())
    }
}

/* entry
{
    "name": "quiver.window.set_point",
    "info": "Set the window point.",
    "member": [
        { "name": "point", "info": "Point of the window.", "kind": "vector_2" }
    ]
}
*/
fn set_window_point(lua: &Lua, shape: LuaValue) -> mlua::Result<()> {
    let shape: general::Vector2 = lua.from_value(shape)?;

    unsafe {
        ffi::SetWindowPosition(shape.x as i32, shape.y as i32);
        Ok(())
    }
}

//================================================================

/* entry
{
    "name": "quiver.window.get_fullscreen",
    "info": "Get the state of the window (full-screen).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_window_fullscreen(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowFullscreen()) }
}

/* entry
{
    "name": "quiver.window.get_minimize",
    "info": "Get the state of the window (minimize).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_window_minimize(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowMinimized()) }
}

/* entry
{
    "name": "quiver.window.get_maximize",
    "info": "Get the state of the window (maximize).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_window_maximize(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowMaximized()) }
}

/* entry
{
    "name": "quiver.window.get_focus",
    "info": "Get the state of the window (focus).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_window_focus(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowFocused()) }
}

/* entry
{
    "name": "quiver.window.get_resize",
    "info": "Get the state of the window (resize).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_window_resize(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowResized()) }
}

/* entry
{
    "name": "quiver.window.get_hidden",
    "info": "Get the state of the window (hidden).",
    "result": [
        { "name": "state", "info": "State of the window.", "kind": "boolean" }
    ]
}
*/
fn get_window_hidden(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowHidden()) }
}

/* entry
{
    "name": "quiver.window.get_shape",
    "info": "Get the shape of the window.",
    "result": [
        { "name": "shape", "info": "Shape of the window.", "kind": "vector_2" }
    ]
}
*/
fn get_window_shape(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    unsafe {
        lua.to_value(&crate::system::general::Vector2::new(
            ffi::GetScreenWidth() as f32,
            ffi::GetScreenHeight() as f32,
        ))
    }
}

/* entry
{
    "name": "quiver.window.get_point",
    "info": "Get the point of the window.",
    "result": [
        { "name": "point", "info": "Point of the window.", "kind": "vector_2" }
    ]
}
*/
fn get_window_point(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    unsafe {
        let value = ffi::GetWindowPosition();

        lua.to_value(&crate::system::general::Vector2::new(value.x, value.y))
    }
}

/* entry
{
    "name": "quiver.window.get_scale",
    "info": "Get the DPI scale of the window.",
    "result": [
        { "name": "scale", "info": "Scale of the window.", "kind": "number" }
    ]
}
*/
fn get_window_scale(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    unsafe {
        let value = ffi::GetWindowScaleDPI();

        lua.to_value(&crate::system::general::Vector2::new(value.x, value.y))
    }
}
