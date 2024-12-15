use crate::system::*;

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    draw_general::set_global(lua, table)?;
    draw_2d::set_global(lua, table)?;
    draw_3d::set_global(lua, table)?;

    Ok(())
}

mod draw_general {
    use super::*;

    /* class
    { "version": "1.0.0", "name": "quiver.draw", "info": "The drawing API." }
    */
    #[rustfmt::skip]
    pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
        let draw = lua.create_table()?;

        draw.set("begin",       lua.create_function(self::begin)?)?;
        //draw.set("begin_blend", lua.create_function(self::begin_blend)?)?;
        //draw.set("begin_shape", lua.create_function(self::begin_shape)?)?;
        //draw.set("get_screen_to_world_3d", lua.create_function(self::begin_shape)?)?;
        //draw.set("get_world_to_screen_3d", lua.create_function(self::begin_shape)?)?;
        //draw.set("get_screen_to_world_2d", lua.create_function(self::begin_shape)?)?;
        //draw.set("get_world_to_screen_2d", lua.create_function(self::begin_shape)?)?;
        draw.set("clear", lua.create_function(self::clear)?)?;

        table.set("draw", draw)?;

        Ok(())
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.begin",
        "info": "Initialize drawing to the screen.",
        "member": [
            { "name": "closure", "info": "The draw code.", "kind": "function" }
        ]
    }
    */
    fn begin(_: &Lua, call: mlua::Function) -> mlua::Result<()> {
        unsafe {
            ffi::BeginDrawing();

            call.call::<()>(())?;

            ffi::EndDrawing();
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0", "name": "quiver.draw.clear",
        "info": "Clear the screen with a color.",
        "member": [
            { "name": "color", "info": "The color to use for clearing.", "kind": "color" }
        ]
    }
    */
    fn clear(lua: &Lua, color: LuaValue) -> mlua::Result<()> {
        let value: Color = lua.from_value(color)?;

        unsafe {
            ffi::ClearBackground(value.into());
            Ok(())
        }
    }
}

mod draw_2d {
    use super::*;

    /* class
    { "version": "1.0.0", "name": "quiver.draw_2d", "info": "The 2D drawing API." }
    */
    #[rustfmt::skip]
    pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
        let draw_2d = lua.create_table()?;

        draw_2d.set("begin", lua.create_function(self::begin)?)?;
        draw_2d.set("draw_box_2", lua.create_function(self::draw_box_2)?)?;
        draw_2d.set("draw_text", lua.create_function(self::draw_text)?)?;

        table.set("draw_2d", draw_2d)?;

        Ok(())
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.begin",
        "info": "Initialize the 2D draw mode.",
        "member": [
            { "name": "camera",   "info": "The 2D camera.", "kind": "camera_2d" },
            { "name": "function", "info": "The draw code.", "kind": "function"  }
        ]
    }
    */
    fn begin(lua: &Lua, (camera, call): (LuaValue, mlua::Function)) -> mlua::Result<()> {
        let value: general::Camera2D = lua.from_value(camera)?;

        unsafe {
            ffi::BeginMode2D(value.into());

            call.call::<()>(())?;

            ffi::EndMode2D();
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0", "name": "quiver.draw_2d.draw_box_2",
        "info": "Draw 2D box.",
        "member": [
            { "name": "shape", "info": "The shape of the box.", "kind": "box_2"    },
            { "name": "point", "info": "The point of the box.", "kind": "vector_2" },
            { "name": "angle", "info": "The angle of the box.", "kind": "number"   },
            { "name": "color", "info": "The color of the box.", "kind": "color"    }
        ]
    }
    */
    fn draw_box_2(
        lua: &Lua,
        (shape, point, angle, color): (LuaValue, LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawRectanglePro(shape.into(), point.into(), angle, color.into());
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0", "name": "quiver.draw_2d.draw_text",
        "info": "Draw text.",
        "member": [
            { "name": "label", "info": "The label of the text.", "kind": "string"   },
            { "name": "point", "info": "The point of the text.", "kind": "vector_2" },
            { "name": "scale", "info": "The angle of the text.", "kind": "number"   },
            { "name": "color", "info": "The color of the text.", "kind": "color"    }
        ]
    }
    */
    fn draw_text(
        lua: &Lua,
        (text, point, scale, color): (String, LuaValue, i32, LuaValue),
    ) -> mlua::Result<()> {
        let text = CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))?;
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

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
    { "version": "1.0.0", "name": "quiver.draw_3d", "info": "The 3D drawing API." }
    */
    #[rustfmt::skip]
    pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
        let draw_3d = lua.create_table()?;

        draw_3d.set("begin", lua.create_function(self::begin)?)?;
        draw_3d.set("draw_grid", lua.create_function(self::draw_grid)?)?;
        draw_3d.set("draw_cube", lua.create_function(self::draw_cube)?)?;
        draw_3d.set("draw_ball", lua.create_function(self::draw_ball)?)?;
        draw_3d.set("draw_box_3", lua.create_function(self::draw_box_3)?)?;

        table.set("draw_3d", draw_3d)?;

        Ok(())
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_3d.begin",
        "info": "Initialize the 3D draw mode.",
        "member": [
            { "name": "camera",   "info": "The 2D camera.", "kind": "camera_3d" },
            { "name": "function", "info": "The draw code.", "kind": "function"  }
        ]
    }
    */
    fn begin(lua: &Lua, (camera, call): (LuaValue, mlua::Function)) -> mlua::Result<()> {
        let value: general::Camera3D = lua.from_value(camera)?;

        unsafe {
            ffi::BeginMode3D(value.into());

            call.call::<()>(())?;

            ffi::EndMode3D();
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0", "name": "quiver.draw_3d.draw_grid",
        "info": "Draw a grid.",
        "member": [
            { "name": "slice", "info": "The slice count of the grid.", "kind": "number" },
            { "name": "space", "info": "The space shift of the grid.", "kind": "number" }
        ]
    }
    */
    fn draw_grid(_: &Lua, (slice, space): (i32, f32)) -> mlua::Result<()> {
        unsafe {
            ffi::DrawGrid(slice, space);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0", "name": "quiver.draw_3d.draw_cube",
        "info": "Draw a cube.",
        "member": [
            { "name": "point", "info": "The point of the cube.", "kind": "vector_3" },
            { "name": "shape", "info": "The shape of the cube.", "kind": "vector_3" },
            { "name": "color", "info": "The color of the cube.", "kind": "color"    }
        ]
    }
    */
    fn draw_cube(
        lua: &Lua,
        (point, shape, color): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector3 = lua.from_value(point)?;
        let shape: Vector3 = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawCubeV(point.into(), shape.into(), color.into());
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0", "name": "quiver.draw_3d.draw_cube",
        "info": "Draw a ball.",
        "member": [
            { "name": "point", "info": "The point of the ball.", "kind": "vector_3" },
            { "name": "shape", "info": "The shape of the ball.", "kind": "number"   },
            { "name": "color", "info": "The color of the ball.", "kind": "color"    }
        ]
    }
    */
    fn draw_ball(lua: &Lua, (point, shape, color): (LuaValue, f32, LuaValue)) -> mlua::Result<()> {
        let point: Vector3 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawSphere(point.into(), shape, color.into());
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0", "name": "quiver.draw_3d.draw_box_3",
        "info": "Draw a 3D box.",
        "member": [
            { "name": "shape", "info": "The shape of the ball.", "kind": "box_3" },
            { "name": "color", "info": "The color of the ball.", "kind": "color" }
        ]
    }
    */
    fn draw_box_3(lua: &Lua, (box_3, color): (LuaValue, LuaValue)) -> mlua::Result<()> {
        let box_3: BoundingBox = lua.from_value(box_3)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawBoundingBox(box_3.into(), color.into());
            Ok(())
        }
    }
}
