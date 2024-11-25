use crate::module::*;

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CStr;

/* base
---@enum input_board
INPUT_BOARD = {
    KEY_NULL = 0,
    KEY_APOSTROPHE = 39,
    KEY_COMMA = 44,
    KEY_MINUS = 45,
    KEY_PERIOD = 46,
    KEY_SLASH = 47,
    KEY_ZERO = 48,
    KEY_ONE = 49,
    KEY_TWO = 50,
    KEY_THREE = 51,
    KEY_FOUR = 52,
    KEY_FIVE = 53,
    KEY_SIX = 54,
    KEY_SEVEN = 55,
    KEY_EIGHT = 56,
    KEY_NINE = 57,
    KEY_SEMICOLON = 59,
    KEY_EQUAL = 61,
    KEY_A = 65,
    KEY_B = 66,
    KEY_C = 67,
    KEY_D = 68,
    KEY_E = 69,
    KEY_F = 70,
    KEY_G = 71,
    KEY_H = 72,
    KEY_I = 73,
    KEY_J = 74,
    KEY_K = 75,
    KEY_L = 76,
    KEY_M = 77,
    KEY_N = 78,
    KEY_O = 79,
    KEY_P = 80,
    KEY_Q = 81,
    KEY_R = 82,
    KEY_S = 83,
    KEY_T = 84,
    KEY_U = 85,
    KEY_V = 86,
    KEY_W = 87,
    KEY_X = 88,
    KEY_Y = 89,
    KEY_Z = 90,
    KEY_LEFT_BRACKET = 91,
    KEY_BACKSLASH = 92,
    KEY_RIGHT_BRACKET = 93,
    KEY_GRAVE = 96,
    KEY_SPACE = 32,
    KEY_ESCAPE = 256,
    KEY_ENTER = 257,
    KEY_TAB = 258,
    KEY_BACKSPACE = 259,
    KEY_INSERT = 260,
    KEY_DELETE = 261,
    KEY_RIGHT = 262,
    KEY_LEFT = 263,
    KEY_DOWN = 264,
    KEY_UP = 265,
    KEY_PAGE_UP = 266,
    KEY_PAGE_DOWN = 267,
    KEY_HOME = 268,
    KEY_END = 269,
    KEY_CAPS_LOCK = 280,
    KEY_SCROLL_LOCK = 281,
    KEY_NUM_LOCK = 282,
    KEY_PRINT_SCREEN = 283,
    KEY_PAUSE = 284,
    KEY_F1 = 290,
    KEY_F2 = 291,
    KEY_F3 = 292,
    KEY_F4 = 293,
    KEY_F5 = 294,
    KEY_F6 = 295,
    KEY_F7 = 296,
    KEY_F8 = 297,
    KEY_F9 = 298,
    KEY_F10 = 299,
    KEY_F11 = 300,
    KEY_F12 = 301,
    KEY_LEFT_SHIFT = 340,
    KEY_LEFT_CONTROL = 341,
    KEY_LEFT_ALT = 342,
    KEY_LEFT_SUPER = 343,
    KEY_RIGHT_SHIFT = 344,
    KEY_RIGHT_CONTROL = 345,
    KEY_RIGHT_ALT = 346,
    KEY_RIGHT_SUPER = 347,
    KEY_KB_MENU = 348,
    KEY_KP_0 = 320,
    KEY_KP_1 = 321,
    KEY_KP_2 = 322,
    KEY_KP_3 = 323,
    KEY_KP_4 = 324,
    KEY_KP_5 = 325,
    KEY_KP_6 = 326,
    KEY_KP_7 = 327,
    KEY_KP_8 = 328,
    KEY_KP_9 = 329,
    KEY_KP_DECIMAL = 330,
    KEY_KP_DIVIDE = 331,
    KEY_KP_MULTIPLY = 332,
    KEY_KP_SUBTRACT = 333,
    KEY_KP_ADD = 334,
    KEY_KP_ENTER = 335,
    KEY_KP_EQUAL = 336,
    KEY_BACK = 4,
    KEY_VOLUME_UP = 24,
    KEY_VOLUME_DOWN = 25,
}
*/
pub const BOARD_RANGE_LOWER: i32 = 0;
pub const BOARD_RANGE_UPPER: i32 = 384;

/* base
---@enum input_mouse
INPUT_MOUSE = {
    MOUSE_BUTTON_LEFT = 0,
    MOUSE_BUTTON_RIGHT = 1,
    MOUSE_BUTTON_MIDDLE = 2,
    MOUSE_BUTTON_SIDE = 3,
    MOUSE_BUTTON_EXTRA = 4,
    MOUSE_BUTTON_FORWARD = 5,
    MOUSE_BUTTON_BACK = 6,
}
*/
pub const MOUSE_RANGE_LOWER: i32 = 0;
pub const MOUSE_RANGE_UPPER: i32 = 6;

/* base
---@enum cursor_mouse
CURSOR_MOUSE = {
    MOUSE_CURSOR_DEFAULT       = 0,
    MOUSE_CURSOR_ARROW         = 1,
    MOUSE_CURSOR_IBEAM         = 2,
    MOUSE_CURSOR_CROSSHAIR     = 3,
    MOUSE_CURSOR_POINTING_HAND = 4,
    MOUSE_CURSOR_RESIZE_EW     = 5,
    MOUSE_CURSOR_RESIZE_NS     = 6,
    MOUSE_CURSOR_RESIZE_NWSE   = 7,
    MOUSE_CURSOR_RESIZE_NESW   = 8,
    MOUSE_CURSOR_RESIZE_ALL    = 9,
    MOUSE_CURSOR_NOT_ALLOWED   = 10
}
*/
pub const CURSOR_RANGE_LOWER: i32 = 0;
pub const CURSOR_RANGE_UPPER: i32 = 10;

/* base
---@enum input_pad
INPUT_PAD = {
    GAMEPAD_BUTTON_UNKNOWN = 0,
    GAMEPAD_BUTTON_LEFT_FACE_UP = 1,
    GAMEPAD_BUTTON_LEFT_FACE_RIGHT = 2,
    GAMEPAD_BUTTON_LEFT_FACE_DOWN = 3,
    GAMEPAD_BUTTON_LEFT_FACE_LEFT = 4,
    GAMEPAD_BUTTON_RIGHT_FACE_UP = 5,
    GAMEPAD_BUTTON_RIGHT_FACE_RIGHT = 6,
    GAMEPAD_BUTTON_RIGHT_FACE_DOWN = 7,
    GAMEPAD_BUTTON_RIGHT_FACE_LEFT = 8,
    GAMEPAD_BUTTON_LEFT_TRIGGER_1 = 9,
    GAMEPAD_BUTTON_LEFT_TRIGGER_2 = 10,
    GAMEPAD_BUTTON_RIGHT_TRIGGER_1 = 11,
    GAMEPAD_BUTTON_RIGHT_TRIGGER_2 = 12,
    GAMEPAD_BUTTON_MIDDLE_LEFT = 13,
    GAMEPAD_BUTTON_MIDDLE = 14,
    GAMEPAD_BUTTON_MIDDLE_RIGHT = 15,
    GAMEPAD_BUTTON_LEFT_THUMB = 16,
    GAMEPAD_BUTTON_RIGHT_THUMB = 17,
}
*/
pub const PAD_RANGE_LOWER: i32 = 0;
pub const PAD_RANGE_UPPER: i32 = 17;

//================================================================

/* class
{ "name": "quiver.input", "info": "The input API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _system : &ModuleSystem) -> mlua::Result<()> {
    let input = lua.create_table()?;

    //table.set("set_clipboard_text", lua.create_function(self::set_clipboard_text)?)?;
    //table.set("get_clipboard_text", lua.create_function(self::get_clipboard_text)?)?;

    //================================================================

    /* class
    { "name": "quiver.input.board", "info": "The board input API." }
    */
    let board = lua.create_table()?;

    //table.set("get_board_key_queue",      lua.create_function(self::get_keycode_queue)?)?;
    //table.set("get_board_uni_queue",      lua.create_function(self::get_unicode_queue)?)?;
    board.set("get_up",      lua.create_function(self::get_board_up)?)?;
    board.set("get_down",    lua.create_function(self::get_board_down)?)?;
    board.set("get_press",   lua.create_function(self::get_board_press)?)?;
    board.set("get_release", lua.create_function(self::get_board_release)?)?;

    input.set("board", board)?;

    //================================================================

    /* class
    { "name": "quiver.input.mouse", "info": "The mouse input API." }
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
    { "name": "quiver.input.pad", "info": "The pad input API." }
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

/* function
{
    "name": "quiver.input.mouse.set_active",
    "info": "Set the active state of the mouse.",
    "parameter": [
        { "optional": false, "name": "state", "info": "Current state.", "type": "boolean" }
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

/* function
{
    "name": "quiver.input.mouse.set_hidden",
    "info": "Set the hidden state of the mouse.",
    "parameter": [
        { "optional": false, "name": "state", "info": "Current state.", "type": "boolean" }
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

/* function
{
    "name": "quiver.input.mouse.get_hidden",
    "info": "Get the hidden state of the mouse.",
    "return": [
        { "optional": false, "name": "state", "info": "Current state.", "type": "boolean" }
    ]
}
*/
fn get_mouse_hidden(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsCursorHidden()) }
}

/* function
{
    "name": "quiver.input.mouse.get_screen",
    "info": "Check if the mouse is currently over the screen.",
    "return": [
        { "optional": false, "name": "state", "info": "Current state.", "type": "boolean" }
    ]
}
*/
fn get_mouse_screen(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsCursorOnScreen()) }
}

/* function
{
    "name": "quiver.input.mouse.get_point",
    "info": "Get the current point of the mouse.",
    "return": [
        { "optional": false, "name": "point", "info": "The point of the mouse.", "type": "vector_2" }
    ]
}
*/
fn get_mouse_point(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    unsafe {
        let value = ffi::GetMousePosition();
        lua.to_value(&crate::system::general::Vector2::new(value.x, value.y))
    }
}

/* function
{
    "name": "quiver.input.mouse.set_point",
    "info": "Set the current point of the mouse.",
    "parameter": [
        { "optional": false, "name": "point", "info": "The point of the mouse.", "type": "vector_2" }
    ]
}
*/
fn set_mouse_point(lua: &Lua, point: LuaValue) -> mlua::Result<()> {
    unsafe {
        let point: crate::system::general::Vector2 = lua.from_value(point)?;
        ffi::SetMousePosition(point.x as i32, point.y as i32);
        Ok(())
    }
}

/* function
{
    "name": "quiver.input.mouse.get_delta",
    "info": "Get the current delta (i.e. mouse movement) of the mouse.",
    "return": [
        { "optional": false, "name": "delta", "info": "The delta of the mouse.", "type": "vector_2" }
    ]
}
*/
fn get_mouse_delta(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    unsafe {
        let value = ffi::GetMouseDelta();
        lua.to_value(&crate::system::general::Vector2::new(value.x, value.y))
    }
}

/* function
{
    "name": "quiver.input.mouse.set_shift",
    "info": "Set the current shift of the mouse.",
    "parameter": [
        { "optional": false, "name": "shift", "info": "The shift of the mouse.", "type": "vector_2" }
    ]
}
*/
fn set_mouse_shift(lua: &Lua, point: LuaValue) -> mlua::Result<()> {
    unsafe {
        let point: crate::system::general::Vector2 = lua.from_value(point)?;
        ffi::SetMouseOffset(point.x as i32, point.y as i32);
        Ok(())
    }
}

/* function
{
    "name": "quiver.input.mouse.set_scale",
    "info": "Set the current scale of the mouse.",
    "parameter": [
        { "optional": false, "name": "scale", "info": "The scale of the mouse.", "type": "vector_2" }
    ]
}
*/
fn set_mouse_scale(lua: &Lua, point: LuaValue) -> mlua::Result<()> {
    unsafe {
        let point: crate::system::general::Vector2 = lua.from_value(point)?;
        ffi::SetMouseScale(point.x, point.y);
        Ok(())
    }
}

/* function
{
    "name": "quiver.input.mouse.set_cursor",
    "info": "Set the current cursor of the mouse.",
    "parameter": [
        { "optional": false, "name": "cursor", "info": "The cursor of the mouse.", "type": "cursor_mouse" }
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

/* function
{
    "name": "quiver.input.mouse.get_wheel",
    "info": "Get the current delta (i.e. mouse wheel movement) of the mouse wheel.",
    "return": [
        { "optional": false, "name": "delta", "info": "The delta of the mouse wheel.", "type": "vector_2" }
    ]
}
*/
fn get_mouse_wheel(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    unsafe {
        let value = ffi::GetMouseWheelMoveV();
        lua.to_value(&crate::system::general::Vector2::new(value.x, value.y))
    }
}

/* function
{
    "name": "quiver.input.mouse.get_up",
    "info": "Get the state of an input (up).",
    "parameter": [
        { "optional": false, "name": "mouse", "info": "The mouse button to check for.", "type": "input_mouse" }
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

/* function
{
    "name": "quiver.input.mouse.get_down",
    "info": "Get the state of an input (down).",
    "parameter": [
        { "optional": false, "name": "mouse", "info": "The mouse button to check for.", "type": "input_mouse" }
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

/* function
{
    "name": "quiver.input.mouse.get_press",
    "info": "Get the state of an input (press).",
    "parameter": [
        { "optional": false, "name": "mouse", "info": "The mouse button to check for.", "type": "input_mouse" }
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

/* function
{
    "name": "quiver.input.mouse.get_release",
    "info": "Get the state of an input (release).",
    "parameter": [
        { "optional": false, "name": "mouse", "info": "The mouse button to check for.", "type": "input_mouse" }
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

/* function
{
    "name": "quiver.input.board.get_up",
    "info": "Get the state of an input (up).",
    "parameter": [
        { "optional": false, "name": "board", "info": "The board button to check for.", "type": "input_board" }
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

/* function
{
    "name": "quiver.input.board.get_down",
    "info": "Get the state of an input (down).",
    "parameter": [
        { "optional": false, "name": "board", "info": "The board button to check for.", "type": "input_board" }
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

/* function
{
    "name": "quiver.input.board.get_press",
    "info": "Get the state of an input (press).",
    "parameter": [
        { "optional": false, "name": "board", "info": "The board button to check for.", "type": "input_board" }
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

/* function
{
    "name": "quiver.input.board.get_release",
    "info": "Get the state of an input (release).",
    "parameter": [
        { "optional": false, "name": "board", "info": "The board button to check for.", "type": "input_board" }
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

/* function
{
    "name": "quiver.input.pad.get_state",
    "info": "Get the state of a pad.",
    "parameter": [
        { "optional": false, "name": "index", "info": "The index of the pad to check for.", "type": "number" }
    ],
    "return": [
        { "optional": false, "name": "state", "info": "The state of the pad.", "type": "boolean" }
    ]
}
*/
fn get_pad_state(_: &Lua, index: i32) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsGamepadAvailable(index)) }
}

/* function
{
    "name": "quiver.input.pad.get_name",
    "info": "Get the name of a pad.",
    "parameter": [
        { "optional": false, "name": "index", "info": "The index of the pad to check for.", "type": "number" }
    ],
    "return": [
        { "optional": false, "name": "name", "info": "The name of the pad.", "type": "string" }
    ]
}
*/
fn get_pad_name(_: &Lua, index: i32) -> mlua::Result<String> {
    unsafe {
        let name = ffi::GetGamepadName(index);
        Ok(CStr::from_ptr(name).to_str().unwrap().to_string())
    }
}

/* function
{
    "name": "quiver.input.pad.get_queue",
    "info": "Get the last pad button press.",
    "return": [
        { "optional": false, "name": "input", "info": "The last pad button press.", "type": "input_pad" }
    ]
}
*/
fn get_pad_queue(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetGamepadButtonPressed()) }
}

/* function
{
    "name": "quiver.input.pad.get_axis_count",
    "info": "Get the axis count of a pad.",
    "parameter": [
        { "optional": false, "name": "index", "info": "The index of the pad to check for.", "type": "number" }
    ],
    "return": [
        { "optional": false, "name": "axis_count", "info": "The axis count of the pad.", "type": "number" }
    ]
}
*/
fn get_pad_axis_count(_: &Lua, index: i32) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetGamepadAxisCount(index)) }
}

/* function
{
    "name": "quiver.input.pad.get_axis_state",
    "info": "Get the axis state of a pad.",
    "parameter": [
        { "optional": false, "name": "index", "info": "The index of the pad to check for.", "type": "number" },
        { "optional": false, "name": "axis",  "info": "The axis of the pad to check for.",  "type": "number" }
    ],
    "return": [
        { "optional": false, "name": "axis_state", "info": "The axis state of the pad.", "type": "number" }
    ]
}
*/
fn get_pad_axis_state(_: &Lua, (index, axis): (i32, i32)) -> mlua::Result<f32> {
    unsafe { Ok(ffi::GetGamepadAxisMovement(index, axis)) }
}

/* function
{
    "name": "quiver.input.pad.get_up",
    "info": "Get the state of an input (up).",
    "parameter": [
        { "optional": false, "name": "pad", "info": "The pad button to check for.", "type": "input_pad" }
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

/* function
{
    "name": "quiver.input.pad.get_down",
    "info": "Get the state of an input (down).",
    "parameter": [
        { "optional": false, "name": "pad", "info": "The pad button to check for.", "type": "input_pad" }
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

/* function
{
    "name": "quiver.input.pad.get_press",
    "info": "Get the state of an input (press).",
    "parameter": [
        { "optional": false, "name": "pad", "info": "The pad button to check for.", "type": "input_pad" }
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

/* function
{
    "name": "quiver.input.pad.get_release",
    "info": "Get the state of an input (release).",
    "parameter": [
        { "optional": false, "name": "pad", "info": "The pad button to check for.", "type": "input_pad" }
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
