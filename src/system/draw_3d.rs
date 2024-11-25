use crate::module::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;

//================================================================

/* class
{ "name": "quiver.draw_3d", "info": "The 3D drawing API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _system : &ModuleSystem) -> mlua::Result<()> {
    let draw_3d = lua.create_table()?;

    draw_3d.set("begin", lua.create_function(self::begin)?)?;
    draw_3d.set("close", lua.create_function(self::close)?)?;
    draw_3d.set("draw_grid", lua.create_function(self::draw_grid)?)?;
    draw_3d.set("draw_cube", lua.create_function(self::draw_cube)?)?;
    draw_3d.set("draw_ball", lua.create_function(self::draw_ball)?)?;
    draw_3d.set("draw_box_3", lua.create_function(self::draw_box_3)?)?;

    table.set("draw_3d", draw_3d)?;

    Ok(())
}

/* function
{
    "name": "quiver.draw_3d.begin",
    "info": "Initialize the 3D draw mode. **MUST** call *quiver.draw_3d.close* after 3D drawing is done.",
    "parameter": [
        { "optional": false, "name": "camera", "info": "The 3D camera to use for drawing.", "type": "camera_3d" }
    ]
}
*/
fn begin(lua: &Lua, camera: LuaValue) -> mlua::Result<()> {
    let value: crate::system::general::Camera3D = lua.from_value(camera)?;

    unsafe {
        ffi::BeginMode3D(value.into());
        Ok(())
    }
}

/* function
{ "name": "quiver.draw_3d.close", "info": "Finalize the 3D draw mode." }
*/
fn close(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::EndMode3D();
        Ok(())
    }
}

/* function
{
    "name": "quiver.draw_3d.draw_grid",
    "info": "Draw a grid.",
    "parameter": [
        { "optional": false, "name": "slice", "info": "The slice count of the grid.", "type": "number" },
        { "optional": false, "name": "space", "info": "The space shift of the grid.", "type": "number" }
    ]
}
*/
fn draw_grid(_: &Lua, (slice, space): (i32, f32)) -> mlua::Result<()> {
    unsafe {
        ffi::DrawGrid(slice, space);
        Ok(())
    }
}

/* function
{
    "name": "quiver.draw_3d.draw_cube",
    "info": "Draw a cube.",
    "parameter": [
        { "optional": false, "name": "point", "info": "The point of the cube.", "type": "vector_3" },
        { "optional": false, "name": "shape", "info": "The shape of the cube.", "type": "vector_3" },
        { "optional": false, "name": "color", "info": "The color of the cube.", "type": "color"    }
    ]
}
*/
fn draw_cube(lua: &Lua, (point, shape, color): (LuaValue, LuaValue, LuaValue)) -> mlua::Result<()> {
    let point: crate::system::general::Vector3 = lua.from_value(point)?;
    let shape: crate::system::general::Vector3 = lua.from_value(shape)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawCubeV(point.into(), shape.into(), color.into());
        Ok(())
    }
}

/* function
{
    "name": "quiver.draw_3d.draw_cube",
    "info": "Draw a ball.",
    "parameter": [
        { "optional": false, "name": "point", "info": "The point of the ball.", "type": "vector_3" },
        { "optional": false, "name": "shape", "info": "The shape of the ball.", "type": "number"   },
        { "optional": false, "name": "color", "info": "The color of the ball.", "type": "color"    }
    ]
}
*/
fn draw_ball(lua: &Lua, (point, shape, color): (LuaValue, f32, LuaValue)) -> mlua::Result<()> {
    let point: crate::system::general::Vector3 = lua.from_value(point)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawSphere(point.into(), shape, color.into());
        Ok(())
    }
}

/* function
{
    "name": "quiver.draw_3d.draw_box_3",
    "info": "Draw a 3D box.",
    "parameter": [
        { "optional": false, "name": "shape", "info": "The shape of the ball.", "type": "box_3" },
        { "optional": false, "name": "color", "info": "The color of the ball.", "type": "color" }
    ]
}
*/
fn draw_box_3(lua: &Lua, (box_3, color): (LuaValue, LuaValue)) -> mlua::Result<()> {
    let box_3: crate::system::general::Box3 = lua.from_value(box_3)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawBoundingBox(box_3.into(), color.into());
        Ok(())
    }
}
