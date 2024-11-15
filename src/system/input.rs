use crate::script::*;
use crate::status::*;
use crate::window::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
use std::ffi::CString;
use std::sync::Mutex;

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

#[rustfmt::skip]
pub fn set_global(lua: &Lua, global: &mlua::Table) -> mlua::Result<()> {
    //global.set("set_clipboard_text", lua.create_function(self::set_clipboard_text)?)?;
    //global.set("get_clipboard_text", lua.create_function(self::get_clipboard_text)?)?;

    //global.set("get_board_key_queue",      lua.create_function(self::get_keycode_queue)?)?;
    //global.set("get_board_uni_queue",      lua.create_function(self::get_unicode_queue)?)?;
    global.set("get_board_up",      lua.create_function(self::get_board_up)?)?;
    global.set("get_board_down",    lua.create_function(self::get_board_down)?)?;
    global.set("get_board_press",   lua.create_function(self::get_board_press)?)?;
    global.set("get_board_release", lua.create_function(self::get_board_release)?)?;

    global.set("set_mouse_active",  lua.create_function(self::set_mouse_active)?)?;
    global.set("set_mouse_hidden",  lua.create_function(self::set_mouse_hidden)?)?;
    global.set("get_mouse_hidden",  lua.create_function(self::get_mouse_hidden)?)?;
    global.set("get_mouse_screen",  lua.create_function(self::get_mouse_screen)?)?;
    global.set("get_mouse_point",   lua.create_function(self::get_mouse_point)?)?;
    global.set("get_mouse_delta",   lua.create_function(self::get_mouse_delta)?)?;
    global.set("set_mouse_point",      lua.create_function(self::set_mouse_point)?)?;
    global.set("set_mouse_shift",      lua.create_function(self::set_mouse_shift)?)?;
    global.set("set_mouse_scale",      lua.create_function(self::set_mouse_scale)?)?;
    global.set("set_mouse_cursor",     lua.create_function(self::set_mouse_cursor)?)?;
    global.set("get_mouse_wheel",      lua.create_function(self::get_mouse_wheel)?)?;
    global.set("get_mouse_up",      lua.create_function(self::get_mouse_up)?)?;
    global.set("get_mouse_down",    lua.create_function(self::get_mouse_down)?)?;
    global.set("get_mouse_press",   lua.create_function(self::get_mouse_press)?)?;
    global.set("get_mouse_release", lua.create_function(self::get_mouse_release)?)?;

    global.set("get_pad_state",   lua.create_function(self::get_pad_state)?)?;
    global.set("get_pad_name",    lua.create_function(self::get_pad_name)?)?;
    global.set("get_pad_queue",   lua.create_function(self::get_pad_queue)?)?;
    global.set("get_pad_axis_count",    lua.create_function(self::get_pad_axis_count)?)?;
    global.set("get_pad_axis_state",    lua.create_function(self::get_pad_axis_state)?)?;
    global.set("get_pad_up",      lua.create_function(self::get_pad_up)?)?;
    global.set("get_pad_down",    lua.create_function(self::get_pad_down)?)?;
    global.set("get_pad_press",   lua.create_function(self::get_pad_press)?)?;
    global.set("get_pad_release", lua.create_function(self::get_pad_release)?)?;

    global.set("set_interface_alpha",  lua.create_function(self::set_interface_alpha)?)?;
    global.set("interface_button",     lua.create_function(self::interface_button)?)?;
    global.set("interface_toggle",     lua.create_function(self::interface_toggle)?)?;
    global.set("interface_check_box",  lua.create_function(self::interface_check_box)?)?;
    global.set("interface_combo_box",  lua.create_function(self::interface_combo_box)?)?;
    global.set("interface_spinner",    lua.create_function(self::interface_spinner)?)?;
    global.set("interface_slider",     lua.create_function(self::interface_slider)?)?;
    global.set("interface_slider_bar", lua.create_function(self::interface_slider_bar)?)?;

    Ok(())
}

/* meta
---Set the interface alpha.
---@param value number The alpha of the interface.
function set_interface_alpha(value) end
*/
fn set_interface_alpha(_: &Lua, value: f32) -> mlua::Result<()> {
    unsafe { Ok(ffi::GuiSetAlpha(value)) }
}

/* meta
---Draw an interface button.
---@param shape box_2  The shape of the button.
---@param label string The label of the button.
---@return boolean # True on button click.
function interface_button(shape, label) end
*/
fn interface_button(lua: &Lua, (shape, label): (LuaValue, String)) -> mlua::Result<bool> {
    let shape: crate::system::general::Box2 = lua.from_value(shape)?;
    let label = CString::new(label).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe { Ok(ffi::GuiButton(shape.into(), label.as_ptr()) > 0) }
}

/* meta
---Draw an interface toggle.
---@param shape box_2   The shape of the slider.
---@param label string  The label of the slider.
---@param value boolean The value of the slider.
---@return boolean # The new value of *value*, if any.
function interface_toggle(shape, label, value) end
*/
fn interface_toggle(
    lua: &Lua,
    (shape, label, mut value): (LuaValue, String, bool),
) -> mlua::Result<bool> {
    let shape: crate::system::general::Box2 = lua.from_value(shape)?;
    let label = CString::new(label).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe {
        ffi::GuiToggle(shape.into(), label.as_ptr(), &mut value);
        Ok(value)
    }
}

/* meta
---Draw an interface check box.
---@param shape box_2   The shape of the check box.
---@param label string  The label of the check box.
---@param value boolean The value of the check box.
---@return boolean # The new value of *value*, if any.
function interface_check_box(shape, label, value) end
*/
fn interface_check_box(
    lua: &Lua,
    (shape, label, mut value): (LuaValue, String, bool),
) -> mlua::Result<bool> {
    let shape: crate::system::general::Box2 = lua.from_value(shape)?;
    let label = CString::new(label).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe {
        ffi::GuiCheckBox(shape.into(), label.as_ptr(), &mut value);
        Ok(value)
    }
}

/* meta
---Draw an interface combo box.
---@param shape box_2  The shape of the combo box.
---@param label string The label of the combo box.
---@param value number The value of the combo box.
---@return number # The new value of *value*, if any.
function interface_combo_box(shape, label, value) end
*/
fn interface_combo_box(
    lua: &Lua,
    (shape, label, mut value): (LuaValue, String, i32),
) -> mlua::Result<i32> {
    let shape: crate::system::general::Box2 = lua.from_value(shape)?;
    let label = CString::new(label).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe {
        ffi::GuiComboBox(shape.into(), label.as_ptr(), &mut value);
        Ok(value)
    }
}

/* meta
---Draw an interface spinner.
---@param shape box_2   The shape of the spinner.
---@param label string  The label of the spinner.
---@param value number  The value of the spinner.
---@param min   number  The minimum value of the spinner.
---@param max   number  The maximum value of the spinner.
---@param edit  boolean The edit mode value of the spinner.
---@return number # The new value of *value*, if any.
function interface_spinner(shape, label, value, min, max, edit) end
*/
fn interface_spinner(
    lua: &Lua,
    (shape, label, mut value, min, max, edit): (LuaValue, String, i32, i32, i32, bool),
) -> mlua::Result<i32> {
    let shape: crate::system::general::Box2 = lua.from_value(shape)?;
    let label = CString::new(label).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe {
        ffi::GuiSpinner(shape.into(), label.as_ptr(), &mut value, min, max, edit);
        Ok(value)
    }
}

/* meta
---Draw an interface slider.
---@param shape   box_2  The shape of the slider.
---@param label_a string The label of the slider.
---@param label_b string The label of the slider.
---@param value   number The value of the slider.
---@param min     number The minimum value of the slider.
---@param max     number The maximum value of the slider.
---@return number # The new value of *value*, if any.
function interface_slider(shape, label_a, label_b, value, min, max) end
*/
fn interface_slider(
    lua: &Lua,
    (shape, label_a, label_b, mut value, min, max): (LuaValue, String, String, f32, f32, f32),
) -> mlua::Result<f32> {
    let shape: crate::system::general::Box2 = lua.from_value(shape)?;
    let label_a = CString::new(label_a).map_err(|e| mlua::Error::runtime(e.to_string()))?;
    let label_b = CString::new(label_b).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe {
        ffi::GuiSlider(
            shape.into(),
            label_a.as_ptr(),
            label_b.as_ptr(),
            &mut value,
            min,
            max,
        );
        Ok(value)
    }
}

/* meta
---Draw an interface slider bar.
---@param shape   box_2  The shape of the slider bar.
---@param label_a string The label of the slider bar.
---@param label_b string The label of the slider bar.
---@param value   number The value of the slider bar.
---@param min     number The minimum value of the slider bar.
---@param max     number The maximum value of the slider bar.
---@return number # The new value of *value*, if any.
function interface_slider_bar(shape, label_a, label_b, value, min, max) end
*/
fn interface_slider_bar(
    lua: &Lua,
    (shape, label_a, label_b, mut value, min, max): (LuaValue, String, String, f32, f32, f32),
) -> mlua::Result<f32> {
    let shape: crate::system::general::Box2 = lua.from_value(shape)?;
    let label_a = CString::new(label_a).map_err(|e| mlua::Error::runtime(e.to_string()))?;
    let label_b = CString::new(label_b).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe {
        ffi::GuiSliderBar(
            shape.into(),
            label_a.as_ptr(),
            label_b.as_ptr(),
            &mut value,
            min,
            max,
        );
        Ok(value)
    }
}

/* meta
---Set the active state of the mouse.
---@param value boolean New state. Active if true, inactive if false.
function set_mouse_active(value) end
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

/* meta
---Set the hidden state of the mouse.
---@param value boolean New state. Hide if true, show if false.
function set_mouse_hidden(value) end
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

/* meta
---Check if the mouse is currently hidden.
---@return boolean # The hidden state of the mouse.
function get_mouse_hidden() end
*/
fn get_mouse_hidden(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsCursorHidden()) }
}

/* meta
---Check if the mouse is currently over the screen.
---@return boolean # The screen state of the mouse.
function get_mouse_screen() end
*/
fn get_mouse_screen(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsCursorOnScreen()) }
}

/* meta
---Get the current point of the mouse.
---@return vector_2 # The point of the mouse.
function get_mouse_point() end
*/
fn get_mouse_point(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    unsafe {
        let value = ffi::GetMousePosition();
        lua.to_value(&crate::system::general::Vector2::new(value.x, value.y))
    }
}

/* meta
---Get the current delta (i.e. mouse movement) of the mouse.
---@return vector_2 # The delta of the mouse.
function get_mouse_delta() end
*/
fn get_mouse_delta(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    unsafe {
        let value = ffi::GetMouseDelta();
        lua.to_value(&crate::system::general::Vector2::new(value.x, value.y))
    }
}

/* meta
---Set the current point of the mouse.
---@param value vector_2 New point.
function set_mouse_point(value) end
*/
fn set_mouse_point(lua: &Lua, point: LuaValue) -> mlua::Result<()> {
    unsafe {
        let point: crate::system::general::Vector2 = lua.from_value(point)?;
        ffi::SetMousePosition(point.x as i32, point.y as i32);
        Ok(())
    }
}

/* meta
---Set the current shift of the mouse.
---@param value vector_2 New shift.
function set_mouse_shift(value) end
*/
fn set_mouse_shift(lua: &Lua, point: LuaValue) -> mlua::Result<()> {
    unsafe {
        let point: crate::system::general::Vector2 = lua.from_value(point)?;
        ffi::SetMouseOffset(point.x as i32, point.y as i32);
        Ok(())
    }
}

/* meta
---Set the current scale of the mouse.
---@param value vector_2 New scale.
function set_mouse_scale(value) end
*/
fn set_mouse_scale(lua: &Lua, point: LuaValue) -> mlua::Result<()> {
    unsafe {
        let point: crate::system::general::Vector2 = lua.from_value(point)?;
        ffi::SetMouseScale(point.x, point.y);
        Ok(())
    }
}

/* meta
---Set the current cursor of the mouse.
---@param value cursor_mouse New cursor.
function set_mouse_cursor(value) end
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

/* meta
---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return vector_2 # The delta of the mouse wheel.
function get_mouse_wheel() end
*/
fn get_mouse_wheel(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    unsafe {
        let value = ffi::GetMouseWheelMoveV();
        lua.to_value(&crate::system::general::Vector2::new(value.x, value.y))
    }
}

/* meta
---Get the state of an input (up).
---@param value input_mouse The input to check for.
function get_mouse_up(value) end
*/
fn get_mouse_up(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsMouseButtonUp(value)) }
    } else {
        Err(mlua::Error::runtime("get_mouse_up(): Unknown key value."))
    }
}

/* meta
---Get the state of an input (down).
---@param value input_mouse The input to check for.
function get_mouse_down(value) end
*/
fn get_mouse_down(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsMouseButtonDown(value)) }
    } else {
        Err(mlua::Error::runtime("get_mouse_up(): Unknown key value."))
    }
}

/* meta
---Get the state of an input (press).
---@param value input_mouse The input to check for.
function get_mouse_press(value) end
*/
fn get_mouse_press(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsMouseButtonPressed(value)) }
    } else {
        Err(mlua::Error::runtime("get_mouse_up(): Unknown key value."))
    }
}

/* meta
---Get the state of an input (release).
---@param value input_mouse The input to check for.
function get_mouse_release(value) end
*/
fn get_mouse_release(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsMouseButtonReleased(value)) }
    } else {
        Err(mlua::Error::runtime("get_mouse_up(): Unknown key value."))
    }
}

//================================================================

/* meta
---Get the state of an input (up).
---@param value input_board The input to check for.
function get_board_up(value) end
*/
fn get_board_up(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsKeyUp(value)) }
    } else {
        Err(mlua::Error::runtime("get_board_up(): Unknown key value."))
    }
}

/* meta
---Get the state of an input (down).
---@param value input_board The input to check for.
function get_board_down(value) end
*/
fn get_board_down(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsKeyDown(value)) }
    } else {
        Err(mlua::Error::runtime("get_board_up(): Unknown key value."))
    }
}

/* meta
---Get the state of an input (press).
---@param value input_board The input to check for.
function get_board_press(value) end
*/
fn get_board_press(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsKeyPressed(value)) }
    } else {
        Err(mlua::Error::runtime("get_board_up(): Unknown key value."))
    }
}

/* meta
---Get the state of an input (release).
---@param value input_board The input to check for.
function get_board_release(value) end
*/
fn get_board_release(_: &Lua, value: i32) -> mlua::Result<bool> {
    if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsKeyReleased(value)) }
    } else {
        Err(mlua::Error::runtime("get_board_up(): Unknown key value."))
    }
}

//================================================================

/* meta
---Get the state of a pad.
---@param index integer The index of the pad to check for.
---@return boolean # True if pad is available, false otherwise.
function get_pad_state(index) end
*/
fn get_pad_state(_: &Lua, index: i32) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsGamepadAvailable(index)) }
}

/* meta
---Get the name of a pad.
---@param index integer The index of the pad to check for.
---@return string # The name of the pad.
function get_pad_name(index) end
*/
fn get_pad_name(_: &Lua, index: i32) -> mlua::Result<String> {
    unsafe {
        let name = ffi::GetGamepadName(index);
        Ok(CStr::from_ptr(name).to_str().unwrap().to_string())
    }
}

/* meta
---Get the last pad button pressed.
---@return input_pad # The last pad button pressed.
function get_pad_queue() end
*/
fn get_pad_queue(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetGamepadButtonPressed()) }
}

/* meta
---Get the axis count of a pad.
---@param index integer The index of the pad to check for.
---@return number # The axis count of the pad.
function get_pad_axis_count(index) end
*/
fn get_pad_axis_count(_: &Lua, index: i32) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetGamepadAxisCount(index)) }
}

/* meta
---Get the axis state of a pad.
---@param index integer The index of the pad to check for.
---@param axis  integer The axis of the pad to check for.
---@return number # The axis state of the pad.
function get_pad_axis_state(index, axis) end
*/
fn get_pad_axis_state(_: &Lua, (index, axis): (i32, i32)) -> mlua::Result<f32> {
    unsafe { Ok(ffi::GetGamepadAxisMovement(index, axis)) }
}

/* meta
---Get the state of an input (up).
---@param index integer The index of the pad to check for.
---@param value input_pad The input to check for.
function get_pad_up(index, value) end
*/
fn get_pad_up(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
    if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsGamepadButtonUp(index, value)) }
    } else {
        Err(mlua::Error::runtime("get_pad_up(): Unknown key value."))
    }
}

/* meta
---Get the state of an input (down).
---@param index integer The index of the pad to check for.
---@param value input_pad The input to check for.
function get_pad_down(index, value) end
*/
fn get_pad_down(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
    if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsGamepadButtonDown(index, value)) }
    } else {
        Err(mlua::Error::runtime("get_pad_up(): Unknown key value."))
    }
}

/* meta
---Get the state of an input (press).
---@param index integer The index of the pad to check for.
---@param value input_pad The input to check for.
function get_pad_press(index, value) end
*/
fn get_pad_press(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
    if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsGamepadButtonPressed(index, value)) }
    } else {
        Err(mlua::Error::runtime("get_pad_up(): Unknown key value."))
    }
}

/* meta
---Get the state of an input (release).
---@param index integer The index of the pad to check for.
---@param value input_pad The input to check for.
function get_pad_release(index, value) end
*/
fn get_pad_release(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
    if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
        unsafe { Ok(ffi::IsGamepadButtonReleased(index, value)) }
    } else {
        Err(mlua::Error::runtime("get_pad_up(): Unknown key value."))
    }
}
