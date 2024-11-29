use mlua::prelude::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

//================================================================

/* class
{ "name": "quiver.general", "info": "The general API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let general = lua.create_table()?;

    general.set("set_exit_key",   lua.create_function(self::set_exit_key)?)?;
    general.set("get_time",       lua.create_function(self::get_time)?)?;
    general.set("get_frame_time", lua.create_function(self::get_frame_time)?)?;
    general.set("get_frame_rate", lua.create_function(self::get_frame_rate)?)?;
    general.set("set_frame_rate", lua.create_function(self::set_frame_rate)?)?;

    table.set("general", general)?;

    Ok(())
}

//================================================================

/* entry
{ "name": "quiver.general.load", "info": "Load the engine.", "skip_definition": "true" }
*/

/* entry
{ "name": "quiver.general.exit", "info": "Exit the engine.", "skip_definition": "true" }
*/

/* entry
{
    "name": "quiver.general.set_exit_key",
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
    "name": "quiver.general.get_time",
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
    "name": "quiver.general.get_frame_time",
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
    "name": "quiver.general.get_frame_rate",
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
    "name": "quiver.general.set_frame_rate",
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
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Into<ffi::Vector2> for Vector2 {
    fn into(self) -> ffi::Vector2 {
        ffi::Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Into<ffi::Vector3> for Vector3 {
    fn into(self) -> ffi::Vector3 {
        ffi::Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

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

#[derive(Deserialize, Serialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Into<ffi::Color> for Color {
    fn into(self) -> ffi::Color {
        ffi::Color {
            r: (self.r * 255.0) as u8,
            g: (self.g * 255.0) as u8,
            b: (self.b * 255.0) as u8,
            a: (self.a * 255.0) as u8,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Box2 {
    pub min: Vector2,
    pub max: Vector2,
}

impl Into<ffi::Rectangle> for Box2 {
    fn into(self) -> ffi::Rectangle {
        ffi::Rectangle {
            x: self.min.x,
            y: self.min.y,
            width: self.max.x,
            height: self.max.y,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Box3 {
    pub min: Vector3,
    pub max: Vector3,
}

impl Into<ffi::BoundingBox> for Box3 {
    fn into(self) -> ffi::BoundingBox {
        ffi::BoundingBox {
            min: self.min.into(),
            max: self.max.into(),
        }
    }
}
