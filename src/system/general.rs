use crate::script::*;
use crate::status::*;
use crate::window::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::sync::Mutex;

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

/* meta
---@class file
local file = {}
*/
use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    sync::Arc,
};

pub struct File {
    pub reader: BufReader<std::fs::File>,
    pub writer: BufWriter<std::fs::File>,
}

impl mlua::UserData for File {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method_mut("get_all_string", |_: &Lua, this, _: ()| {
            let mut data = String::new();
            this.reader.read_to_string(&mut data)?;

            Ok(data)
        });

        method.add_method_mut("get_string", |_: &Lua, this, _: ()| {
            let mut data = String::new();
            this.reader.read_line(&mut data)?;

            Ok(data)
        });

        method.add_method_mut("set_string", |_: &Lua, this, data: String| {
            this.writer.write_all(data.as_bytes())?;
            this.writer.flush()?;

            Ok(())
        });
    }
}

impl File {
    /* meta
    ---An unique file handle for a file in memory.
    ---@param path string Path to file.
    ---@return file # The user-data object.
    function File(path) end
    */
    pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        if !std::path::Path::new(&path).exists() {
            std::fs::File::create(&path)?;
        }

        let file = std::fs::File::options().read(true).write(true).open(path)?;

        Ok(Self {
            reader: BufReader::new(file.try_clone().unwrap()),
            writer: BufWriter::new(file.try_clone().unwrap()),
        })
    }
}

//================================================================

use notify::{Config, Event, PollWatcher, RecursiveMode, Watcher};
use std::path::Path;

/* meta
---@class watcher_info
local watcher_info = {}

---@class file_watcher
local file_watcher = {}
*/
pub struct FileWatcher(PollWatcher, Arc<Mutex<Option<Event>>>);

impl mlua::UserData for FileWatcher {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* meta
        ---Poll for a notification in the file watcher's directory.
        ---@return watcher_info | nil # Will return a non-nil value on event.
        function file_watcher:update() end
        */
        method.add_method("update", |lua: &Lua, this, _: ()| {
            this.0
                .poll()
                .map_err(|e| mlua::Error::runtime(e.to_string()))?;

            let mut lock = this
                .1
                .lock()
                .map_err(|e| mlua::Error::runtime(e.to_string()))?;

            if let Some(event) = lock.as_mut() {
                let result = event.clone();

                *lock = None;

                Ok(lua.to_value(&result)?)
            } else {
                Ok(mlua::Nil)
            }
        });
    }
}

impl FileWatcher {
    /* meta
    ---An unique handle for a file watcher in memory.
    ---@param path string Path to file/directory.
    ---@return file_watcher # The user-data object.
    function FileWatcher(path) end
    */
    pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = PollWatcher::new(tx, Config::default().with_manual_polling())
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;
        watcher
            .watch(Path::new(&path), RecursiveMode::Recursive)
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;

        let value = Arc::new(Mutex::new(None));
        let clone = value.clone();

        std::thread::spawn(move || {
            for res in rx {
                match res {
                    Ok(event) => {
                        let mut lock = clone
                            .lock()
                            .expect("FileWatcher::new(): Error awaiting lock.");

                        *lock = Some(event);
                    }
                    Err(event) => println!("{:?}", event),
                }
            }
        });

        Ok(Self(watcher, value.clone()))
    }
}

//================================================================

#[rustfmt::skip]
    pub fn set_global(lua: &Lua, global: &mlua::Table, status: StatusPointer, window : WindowPointer) -> mlua::Result<()> {
        /* meta
        ---Load the engine.
        function engine_load() end
        */
        let clone = status.clone();
        global.set("engine_load", lua.create_function(move |_, _ : ()| {
            *clone.borrow_mut() = Status::Restart;
            Ok(())
        })?)?;

        /* meta
        ---Exit the engine.
        function engine_exit() end
        */
        let clone = status.clone();
        global.set("engine_exit", lua.create_function(move |_, _ : ()| {
            *clone.borrow_mut() = Status::Closure;
            Ok(())
        })?)?;

        /* meta
       ---Get the current state of the debug window.
        function get_debug_state() end
        */
        let clone = window.clone();
        global.set("get_debug_state", lua.create_function(move |_, _ : ()| {
            Ok(clone.borrow().active)
        })?)?;

        /* meta
        ---Set the current state of the debug window.
        ---@param value boolean New state.
        function set_debug_state(value) end
        */
        let clone = window.clone();
        global.set("set_debug_state", lua.create_function(move |_, value : bool| {
            let mut clone = clone.borrow_mut();
            clone.active = value;
            clone.parser.dirty = value;

            Ok(())
        })?)?;

        /* meta
        ---Get the current state of the debug logger.
        function get_logger_state() end
        */
        let clone = window.clone();
        global.set("get_logger_state", lua.create_function(move |_, _ : ()| {
            Ok(clone.borrow_mut().logger.active)
        })?)?;

        /* meta
        ---Set the current state of the debug logger.
        ---@param value boolean New state.
        function set_logger_state(value) end
        */
        let clone = window.clone();
        global.set("set_logger_state", lua.create_function(move |_, value : bool| {
            clone.borrow_mut().logger.active = value;
            Ok(())
        })?)?;

        /* meta
        ---Wipe the debug logger text.
        function wipe_logger() end
        */
        let clone = window.clone();
        global.set("wipe_logger", lua.create_function(move |_, _ : ()| {
            Logger::wipe(&mut clone.borrow_mut().logger);
            Ok(())
        })?)?;

        /* meta
        ---Show the debug logger text.
        ---@param value boolean New state.
        function show_logger(value) end
        */
        let clone = window.clone();
        global.set("show_logger", lua.create_function(move |_, value: bool| {
            Logger::show(&mut clone.borrow_mut().logger, value);
            Ok(())
        })?)?;

        /* meta
        ---Push a new string to the debug logger.
        ---@param label  string Label for line to print.
        ---@param color? color  Color for line to print.
        function push_logger(label, color) end
        */
        let clone = window.clone();
        global.set("push_logger", lua.create_function(move |lua: &Lua, (label, color) : (String, Option<LuaValue>)| {
            println!("{label}");

            let color : Color = {
                if let Some(color) = color {
                    lua.from_value(color)?
                } else {
                    Color::new(1.0, 1.0, 1.0, 1.0)
                }
            };
            clone.borrow_mut().logger.push(LogLine::new(label, [color.r, color.g, color.b, color.a]));
            Ok(())
        })?)?;

        /* meta
        ---Push a new method to the debug parser.
        ---@param name string Name for method to push.
        ---@param info string Info for method to push.
        ---@param call function Function call-back for method to push.
        function push_parser(name, info, call) end
        */
        let clone = window.clone();
        global.set("push_parser", lua.create_function(move |_, (name, info, call) : (String, String, mlua::Function)| {
            clone.borrow_mut().parser.method.insert(name, ParserMethod { call, info });
            Ok(())
        })?)?;

        global.set("File",           lua.create_function(self::File::new)?)?;
        global.set("FileWatcher",    lua.create_function(self::FileWatcher::new)?)?;
        //global.set("Automation",    lua.create_function(self::Automation::new)?)?;

        global.set("get_file_exist",   lua.create_function(self::get_file_exist)?)?;
        global.set("set_exit_key",   lua.create_function(self::set_exit_key)?)?;
        global.set("get_time",       lua.create_function(self::get_time)?)?;
        global.set("get_frame_time", lua.create_function(self::get_frame_time)?)?;
        global.set("get_frame_rate", lua.create_function(self::get_frame_rate)?)?;
        global.set("set_frame_rate", lua.create_function(self::set_frame_rate)?)?;
        global.set("json_to_table",  lua.create_function(self::json_to_table)?)?;
        global.set("table_to_json",  lua.create_function(self::table_to_json)?)?;
        //global.set("open_link",  lua.create_function(self::open_link)?)?;
        //global.set("compress_data",  lua.create_function(self::compress_data)?)?;
        //global.set("decompress_data",  lua.create_function(self::decompress_data)?)?;
        //global.set("encode_data_base64",  lua.create_function(self::encode_data_base64)?)?;
        //global.set("decode_data_base64",  lua.create_function(self::decode_data_base64)?)?;

        Ok(())
    }

//================================================================

/* meta
---Set a key to exit Quiver.
---@param key input_board Key to exit Quiver with.
function set_exit_key(key) end
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

/* meta
---Get the current time. Will count up since the initialization of the window.
---@return number # Current time.
function get_time() end
*/
fn get_time(_: &Lua, _: ()) -> mlua::Result<f64> {
    unsafe { Ok(ffi::GetTime()) }
}

/* meta
---Get the current frame time.
---@return number # Current frame time.
function get_frame_time() end
*/
fn get_frame_time(_: &Lua, _: ()) -> mlua::Result<f32> {
    unsafe { Ok(ffi::GetFrameTime()) }
}

/* meta
---Get the current frame rate.
---@return number # Current frame rate.
function get_frame_rate() end
*/
fn get_frame_rate(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetFPS()) }
}

/* meta
---Set the current frame rate.
---@param value number Value to set the frame rate to.
function set_frame_rate(value) end
*/
fn set_frame_rate(_: &Lua, rate: i32) -> mlua::Result<()> {
    unsafe {
        ffi::SetTargetFPS(rate);
        Ok(())
    }
}

/* meta
---Convert a table to a JSON string.
---@param value table Table to convert to a JSON string.
---@return string # JSON conversion of table.
function table_to_json(value) end
*/
fn json_to_table(lua: &Lua, data: String) -> mlua::Result<LuaValue> {
    let json: serde_json::Value =
        serde_json::from_str(&data).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    lua.to_value(&json)
}

/* meta
---Convert a JSON string to a table.
---@param value string JSON string to convert to a table.
---@return table # Table conversion of a JSON string.
function json_to_table(value) end
*/
fn table_to_json(_: &Lua, data: LuaValue) -> mlua::Result<String> {
    let json = serde_json::to_string(&data).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    Ok(json)
}

/* meta
---Check if a file does exist in disk.
---@param path string Path for file to check.
---@return boolean # True if file does exist, false otherwise.
function get_file_exist(path) end
*/
fn get_file_exist(_: &Lua, path: String) -> mlua::Result<bool> {
    Ok(std::path::Path::new(&path).exists())
}
