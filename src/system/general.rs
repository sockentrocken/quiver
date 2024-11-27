use crate::status::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

//================================================================

/* base
---@class vector_2
---@field x number
---@field y number
vector_2 = {
    _type = "vector_2",
    x = 0.0,
    y = 0.0,
}

function vector_2:new(x, y)
    local i = {}
    setmetatable(i, {
        __index = self,
        __add = function(a, b) return vector_2:new(a.x + b.x, a.y + b.y) end,
        __sub = function(a, b) return vector_2:new(a.x - b.x, a.y - b.y) end,
        __mul = function(a, b) return vector_2:new(a.x * b.x, a.y * b.y) end,
        __div = function(a, b) return vector_2:new(a.x / b.x, a.y / b.y) end,
        __tostring = function(a) return "{ x:"..tostring(a.x).." y:"..tostring(a.y).." }"..tostring(a.z).." }" end
    })
    i.x = x
    i.y = y
    return i
end

function vector_2:x()
    return vector_2:new(1.0, 0.0)
end

function vector_2:y()
    return vector_2:new(0.0, 1.0)
end

function vector_2:one()
    return vector_2:new(1.0, 1.0)
end

function vector_2:zero()
    return vector_2:new(0.0, 0.0)
end
*/
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

/* base
---@class vector_3
---@field x number
---@field y number
---@field z number
vector_3 = {
    _type = "vector_3",
    x = 0.0,
    y = 0.0,
    z = 0.0,
}

function vector_3:new(x, y, z)
    local i = {}
    setmetatable(i, {
        __index = self,
        __add = function(a, b) return vector_3:new(a.x + b.x, a.y + b.y, a.z + b.z) end,
        __sub = function(a, b) return vector_3:new(a.x - b.x, a.y - b.y, a.z - b.z) end,
        __mul = function(a, b) return vector_3:new(a.x * b.x, a.y * b.y, a.z * b.z) end,
        __div = function(a, b) return vector_3:new(a.x / b.x, a.y / b.y, a.z / b.z) end,
        __tostring = function(a) return "{ x:"..tostring(a.x).." y:"..tostring(a.y).." z:"..tostring(a.z).." }" end
    })
    i.x = x
    i.y = y
    i.z = z
    return i
end

function vector_3:x()
    return vector_3:new(1.0, 0.0, 0.0)
end

function vector_3:y()
    return vector_3:new(0.0, 1.0, 0.0)
end

function vector_3:z()
    return vector_3:new(0.0, 0.0, 1.0)
end

function vector_3:one()
    return vector_3:new(1.0, 1.0, 1.0)
end

function vector_3:zero()
    return vector_3:new(0.0, 0.0, 0.0)
end
*/
#[derive(Deserialize, Serialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
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

/* base
---@class camera_2d
---@field shift vector_2
---@field focus vector_2
---@field angle number
---@field zoom  number
camera_2d = {
    _type = "camera_2d",
    shift = vector_2:zero(),
    focus = vector_2:zero(),
    angle = 0.0,
    zoom  = 0.0,
}

function camera_2d:new(shift, focus, angle, zoom)
    local i = {}
    setmetatable(i, {
        __index = self
    })
    i.shift = shift
    i.focus = focus
    i.angle = angle
    i.zoom = zoom
    return i
end
*/
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

/* base
---@class camera_3d
---@field point vector_3
---@field focus vector_3
---@field angle vector_3
---@field zoom  number
camera_3d = {
    _type = "camera_3d",
    point = vector_3:zero(),
    focus = vector_3:zero(),
    angle = vector_3:zero(),
    zoom  = 0.0,
}

function camera_3d:new(point, focus, angle, zoom)
    local i = {}
    setmetatable(i, {
        __index = self
    })
    i.point = point
    i.focus = focus
    i.angle = angle
    i.zoom = zoom
    return i
end
*/
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

/* base
---@class color
---@field r number
---@field g number
---@field b number
---@field a number
color = {
    r = 0.0,
    g = 0.0,
    b = 0.0,
    a = 0.0,
}

function color:new(r, g, b, a)
    local i = {}
    setmetatable(i, {
        __index = self
    })
    i.r = r
    i.g = g
    i.b = b
    i.a = a
    return i
end

function color:white()
    return color:new(1.0, 1.0, 1.0, 1.0)
end

function color:black()
    return color:new(0.0, 0.0, 0.0, 1.0)
end
*/
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

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

/* base
---@class box_2
---@field min vector_2
---@field max vector_2
box_2 = {
    _type = "box_2",
    min = vector_2:zero(),
    max = vector_2:zero(),
}

function box_2:new(min, max)
    local i = {}
    setmetatable(i, {
        __index = self
    })
    i.min = min
    i.max = max
    return i
end
*/
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

/* base
---@class box_3
---@field min vector_3
---@field max vector_3
box_3 = {
    _type = "box_3",
    min = vector_3:zero(),
    max = vector_3:zero(),
}

function box_3:new(min, max)
    local i = {}
    setmetatable(i, {
        __index = self
    })
    i.min = min
    i.max = max
    return i
end
*/
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

//================================================================

#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, status: StatusPointer) -> mlua::Result<()> {
    /* class
    { "name": "quiver.engine", "info": "The engine API." }
    */
    let engine = lua.create_table()?;

    /* function
    { "name": "quiver.engine.load", "info": "Load the engine." }
    */
    let clone = status.clone();
    engine.set("load", lua.create_function(move |_, _ : ()| {
        *clone.borrow_mut() = Status::Restart;
        Ok(())
    })?)?;

    /* function
    { "name": "quiver.engine.exit", "info": "Exit the engine." }
    */
    let clone = status.clone();
    engine.set("exit", lua.create_function(move |_, _ : ()| {
        *clone.borrow_mut() = Status::Closure;
        Ok(())
    })?)?;

    engine.set("set_exit_key",   lua.create_function(self::set_exit_key)?)?;
    engine.set("get_time",       lua.create_function(self::get_time)?)?;
    engine.set("get_frame_time", lua.create_function(self::get_frame_time)?)?;
    engine.set("get_frame_rate", lua.create_function(self::get_frame_rate)?)?;
    engine.set("set_frame_rate", lua.create_function(self::set_frame_rate)?)?;

    table.set("engine", engine)?;

    Ok(())
}

//================================================================

/* function
{
    "name": "quiver.engine.set_exit_key",
    "info": "Set a key to exit Quiver.",
    "parameter": [
        { "optional": false, "name": "key", "info": "Key to exit Quiver with.", "type": "input_board" }
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

/* function
{
    "name": "quiver.engine.get_time",
    "info": "Get the current time. Will count up since the initialization of the window.",
    "return": [
        { "optional": false, "name": "time", "info": "Current time.", "type": "number" }
    ]
}
*/
fn get_time(_: &Lua, _: ()) -> mlua::Result<f64> {
    unsafe { Ok(ffi::GetTime()) }
}

/* function
{
    "name": "quiver.engine.get_frame_time",
    "info": "Get the current frame time.",
    "return": [
        { "optional": false, "name": "frame_time", "info": "Current frame time.", "type": "number" }
    ]
}
*/
fn get_frame_time(_: &Lua, _: ()) -> mlua::Result<f32> {
    unsafe { Ok(ffi::GetFrameTime()) }
}

/* function
{
    "name": "quiver.engine.get_frame_rate",
    "info": "Get the current frame rate.",
    "return": [
        { "optional": false, "name": "frame_rate", "info": "Current frame rate.", "type": "number" }
    ]
}
*/
fn get_frame_rate(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetFPS()) }
}

/* function
{
    "name": "quiver.engine.set_frame_rate",
    "info": "set the current frame rate.",
    "parameter": [
        { "optional": false, "name": "frame_rate", "info": "Current frame rate.", "type": "number" }
    ]
}
*/
fn set_frame_rate(_: &Lua, rate: i32) -> mlua::Result<()> {
    unsafe {
        ffi::SetTargetFPS(rate);
        Ok(())
    }
}
