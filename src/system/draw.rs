use crate::script::*;

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

pub fn set_global(lua: &Lua, table: &mlua::Table, system: &ModuleSystem) -> mlua::Result<()> {
    draw_2d::set_global(lua, table, system)?;
    draw_3d::set_global(lua, table, system)?;

    Ok(())
}

mod draw_2d {
    use super::*;

    /* class
    { "name": "quiver.draw_2d", "info": "The 2D drawing API." }
    */
    #[rustfmt::skip]
    pub fn set_global(lua: &Lua, table: &mlua::Table, _system : &ModuleSystem) -> mlua::Result<()> {
        let draw_2d = lua.create_table()?;

        draw_2d.set("begin", lua.create_function(self::begin)?)?;
        draw_2d.set("close", lua.create_function(self::close)?)?;
        draw_2d.set("draw_box_2", lua.create_function(self::draw_box_2)?)?;
        draw_2d.set("draw_text", lua.create_function(self::draw_text)?)?;

        table.set("draw_2d", draw_2d)?;

        Ok(())
    }

    /* function
    {
        "name": "quiver.draw_2d.begin",
        "info": "Initialize the 2D draw mode. **MUST** call *quiver.draw_2d.close* after 2D drawing is done.",
        "parameter": [
            { "optional": false, "name": "camera", "info": "The 2D camera to use for drawing.", "type": "camera_2d" }
        ]
    }
    */
    fn begin(lua: &Lua, camera: LuaValue) -> mlua::Result<()> {
        let value: crate::system::general::Camera2D = lua.from_value(camera)?;

        unsafe {
            ffi::BeginMode2D(value.into());
            Ok(())
        }
    }

    /* function
    { "name": "quiver.draw_2d.close", "info": "Finalize the 2D draw mode." }
    */
    fn close(_: &Lua, _: ()) -> mlua::Result<()> {
        unsafe {
            ffi::EndMode2D();
            Ok(())
        }
    }

    /* function
    {
        "name": "quiver.draw_2d.draw_box_2",
        "info": "Draw 2D box.",
        "parameter": [
            { "optional": false, "name": "shape", "info": "The shape of the box.", "type": "box_2"    },
            { "optional": false, "name": "point", "info": "The point of the box.", "type": "vector_2" },
            { "optional": false, "name": "angle", "info": "The angle of the box.", "type": "number"   },
            { "optional": false, "name": "color", "info": "The color of the box.", "type": "color"    }
        ]
    }
    */
    fn draw_box_2(
        lua: &Lua,
        (shape, point, angle, color): (LuaValue, LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let shape: crate::system::general::Box2 = lua.from_value(shape)?;
        let point: crate::system::general::Vector2 = lua.from_value(point)?;
        let color: crate::system::general::Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawRectanglePro(shape.into(), point.into(), angle, color.into());
            Ok(())
        }
    }

    /* function
    {
        "name": "quiver.draw_2d.draw_text",
        "info": "Draw text.",
        "parameter": [
            { "optional": false, "name": "label", "info": "The label of the text.", "type": "string"   },
            { "optional": false, "name": "point", "info": "The point of the text.", "type": "vector_2" },
            { "optional": false, "name": "scale", "info": "The angle of the text.", "type": "number"   },
            { "optional": false, "name": "color", "info": "The color of the text.", "type": "color"    }
        ]
    }
    */
    fn draw_text(
        lua: &Lua,
        (text, point, scale, color): (String, LuaValue, i32, LuaValue),
    ) -> mlua::Result<()> {
        let text = CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))?;
        let point: crate::system::general::Vector2 = lua.from_value(point)?;
        let color: crate::system::general::Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawText(
                text.as_ptr(),
                point.x as i32,
                point.y as i32,
                scale,
                color.into(),
            );
            Ok(())
        }
    }
}

mod draw_3d {
    use super::*;

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
    fn draw_cube(
        lua: &Lua,
        (point, shape, color): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
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
}
