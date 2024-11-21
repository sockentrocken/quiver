use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

#[rustfmt::skip]
pub fn set_global(lua: &Lua, global: &mlua::Table) -> mlua::Result<()> {
    global.set("set_interface_active",  lua.create_function(self::set_interface_active)?)?;
    global.set("set_interface_lock",  lua.create_function(self::set_interface_lock)?)?;
    global.set("get_interface_lock",  lua.create_function(self::get_interface_lock)?)?;
    global.set("set_interface_alpha",  lua.create_function(self::set_interface_alpha)?)?;
    global.set("set_interface_state",  lua.create_function(self::set_interface_state)?)?;
    global.set("get_interface_state",  lua.create_function(self::get_interface_state)?)?;
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
---Set the interface active state.
---@param value boolean The active state of the interface.
function set_interface_active(value) end
*/
fn set_interface_active(_: &Lua, value: bool) -> mlua::Result<()> {
    unsafe {
        if value {
            ffi::GuiEnable();
        } else {
            ffi::GuiDisable();
        }
        Ok(())
    }
}

/* meta
---Set the interface lock state.
---@param value boolean The lock state of the interface.
function set_interface_lock(value) end
*/
fn set_interface_lock(_: &Lua, value: bool) -> mlua::Result<()> {
    unsafe {
        if value {
            ffi::GuiLock();
        } else {
            ffi::GuiUnlock();
        }
        Ok(())
    }
}

/* meta
---Get the interface lock state.
---@return boolean # The lock state of the interface.
function set_interface_lock(value) end
*/
fn get_interface_lock(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::GuiIsLocked()) }
}

/* meta
---Set the interface state.
---@param state number The state of the interface.
function set_interface_lock(value) end
*/
fn set_interface_state(_: &Lua, state: i32) -> mlua::Result<()> {
    unsafe {
        ffi::GuiSetState(state);
        Ok(())
    }
}

/* meta
---Get the interface state.
---@return number # The state of the interface.
function set_interface_lock(value) end
*/
fn get_interface_state(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GuiGetState()) }
}

/* meta
---Set the interface alpha.
---@param value number The alpha of the interface.
function set_interface_alpha(value) end
*/
fn set_interface_alpha(_: &Lua, value: f32) -> mlua::Result<()> {
    unsafe {
        ffi::GuiSetAlpha(value);
        Ok(())
    }
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
