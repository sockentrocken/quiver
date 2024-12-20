/*
* MIT License
*
* Copyright (c) 2024 sockentrocken
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

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

        draw.set("begin",         lua.create_function(self::begin)?)?;
        draw.set("begin_blend",   lua.create_function(self::begin_blend)?)?;
        draw.set("begin_scissor", lua.create_function(self::begin_scissor)?)?;
        draw.set("get_screen_to_world_3d", lua.create_function(self::get_screen_to_world_3d)?)?;
        draw.set("get_world_to_screen_3d", lua.create_function(self::get_world_to_screen_3d)?)?;
        draw.set("get_screen_to_world_2d", lua.create_function(self::get_screen_to_world_2d)?)?;
        draw.set("get_world_to_screen_2d", lua.create_function(self::get_world_to_screen_2d)?)?;
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
            { "name": "call", "info": "The draw code.", "kind": "function" }
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

    // to-do: blend mode enumerator. error if mode is outside of the enum range.
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.begin_blend",
        "info": "Initialize drawing (blend mode) to the screen.",
        "member": [
            { "name": "call", "info": "The draw code.", "kind": "function" },
            { "name": "mode", "info": "The draw code.", "kind": "function" }
        ]
    }
    */
    fn begin_blend(_: &Lua, (call, mode): (mlua::Function, i32)) -> mlua::Result<()> {
        unsafe {
            ffi::BeginBlendMode(mode);

            call.call::<()>(())?;

            ffi::EndBlendMode();
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.begin_scissor",
        "info": "Initialize drawing (scissor mode) to the screen.",
        "member": [
            { "name": "call", "info": "The draw code.",        "kind": "function" },
            { "name": "view", "info": "The clip test region.", "kind": "box_2"    }
        ]
    }
    */
    fn begin_scissor(lua: &Lua, (call, view): (mlua::Function, LuaValue)) -> mlua::Result<()> {
        let view: Rectangle = lua.from_value(view)?;

        unsafe {
            ffi::BeginScissorMode(
                view.x as i32,
                view.y as i32,
                view.width as i32,
                view.height as i32,
            );

            call.call::<()>(())?;

            ffi::EndScissorMode();
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.get_screen_to_world_3d",
        "info": "Get a ray for a 2D screen-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",        "kind": "camera_3d" },
            { "name": "point",  "info": "The screen-space point.",    "kind": "vector_2"  },
            { "name": "shape",  "info": "The size of the view-port.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "ray", "info": "The 3D ray, beginning at the screen-space point, in 3D space.", "kind": "ray" }
        ]
    }
    */
    fn get_screen_to_world_3d(
        lua: &Lua,
        (camera, point, shape): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<LuaValue> {
        let camera: general::Camera3D = lua.from_value(camera)?;
        let point: Vector2 = lua.from_value(point)?;
        let shape: Vector2 = lua.from_value(shape)?;

        unsafe {
            let ray = ffi::GetScreenToWorldRayEx(
                point.into(),
                camera.into(),
                shape.x as i32,
                shape.y as i32,
            );

            lua.to_value(&Ray::from(ray))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.get_world_to_screen_3d",
        "info": "Get a 2D screen-space point for a 3D world-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",        "kind": "camera_3d" },
            { "name": "point",  "info": "The world-space point.",     "kind": "vector_3"  },
            { "name": "shape",  "info": "The size of the view-port.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "point", "info": "The 2D screen-space point.", "kind": "vector_2" }
        ]
    }
    */
    fn get_world_to_screen_3d(
        lua: &Lua,
        (camera, point, shape): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<LuaValue> {
        let camera: general::Camera3D = lua.from_value(camera)?;
        let point: Vector3 = lua.from_value(point)?;
        let shape: Vector2 = lua.from_value(shape)?;

        unsafe {
            let point = ffi::GetWorldToScreenEx(
                point.into(),
                camera.into(),
                shape.x as i32,
                shape.y as i32,
            );

            lua.to_value(&Vector2::from(point))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.get_screen_to_world_2d",
        "info": "Get a world-space point for a 2D screen-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",     "kind": "camera_2d" },
            { "name": "point",  "info": "The screen-space point.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "point", "info": "The 2D world-space point.", "kind": "vector_2" }
        ]
    }
    */
    fn get_screen_to_world_2d(
        lua: &Lua,
        (camera, point): (LuaValue, LuaValue),
    ) -> mlua::Result<LuaValue> {
        let camera: general::Camera2D = lua.from_value(camera)?;
        let point: Vector2 = lua.from_value(point)?;

        unsafe {
            let point = ffi::GetScreenToWorld2D(point.into(), camera.into());

            lua.to_value(&Vector2::from(point))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.get_world_to_screen_2d",
        "info": "Get a screen-space point for a 2D world-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",    "kind": "camera_2d" },
            { "name": "point",  "info": "The world-space point.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "point", "info": "The 2D screen-space point.", "kind": "vector_2" }
        ]
    }
    */
    fn get_world_to_screen_2d(
        lua: &Lua,
        (camera, point): (LuaValue, LuaValue),
    ) -> mlua::Result<LuaValue> {
        let camera: general::Camera2D = lua.from_value(camera)?;
        let point: Vector2 = lua.from_value(point)?;

        unsafe {
            let point = ffi::GetWorldToScreen2D(point.into(), camera.into());

            lua.to_value(&Vector2::from(point))
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
        draw_2d.set("draw_circle", lua.create_function(self::draw_circle)?)?;
        draw_2d.set("draw_circle_sector", lua.create_function(self::draw_circle_sector)?)?;

        table.set("draw_2d", draw_2d)?;

        Ok(())
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.begin",
        "info": "Initialize the 2D draw mode.",
        "member": [
            { "name": "call",   "info": "The draw code.", "kind": "function"  },
            { "name": "camera", "info": "The 2D camera.", "kind": "camera_2d" }
        ]
    }
    */
    fn begin(lua: &Lua, (call, camera): (mlua::Function, LuaValue)) -> mlua::Result<()> {
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

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_circle",
        "info": "Draw a circle.",
        "member": [
            { "name": "point",  "info": "", "kind": "vector_2" },
            { "name": "radius", "info": "", "kind": "number"   },
            { "name": "color",  "info": "", "kind": "color"    }
        ]
    }
    */
    fn draw_circle(
        lua: &Lua,
        (point, radius, color): (LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawCircleV(point.into(), radius, color.into());
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_circle_sector",
        "info": "Draw the sector of a circle.",
        "member": [
            { "name": "point",         "info": "", "kind": "vector_2" },
            { "name": "radius",        "info": "", "kind": "number"   },
            { "name": "begin_angle",   "info": "", "kind": "number"   },
            { "name": "close_angle",   "info": "", "kind": "number"   },
            { "name": "segment_count", "info": "", "kind": "number"   },
            { "name": "color",         "info": "", "kind": "color"    }
        ]
    }
    */
    fn draw_circle_sector(
        lua: &Lua,
        (point, radius, begin_angle, close_angle, segment_count, color): (
            LuaValue,
            f32,
            f32,
            f32,
            i32,
            LuaValue,
        ),
    ) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawCircleSector(
                point.into(),
                radius,
                begin_angle,
                close_angle,
                segment_count,
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
        draw_3d.set("draw_ray", lua.create_function(self::draw_ray)?)?;

        table.set("draw_3d", draw_3d)?;

        Ok(())
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_3d.begin",
        "info": "Initialize the 3D draw mode.",
        "member": [
            { "name": "call",   "info": "The draw code.", "kind": "function"  },
            { "name": "camera", "info": "The 2D camera.", "kind": "camera_3d" }
        ]
    }
    */
    fn begin(lua: &Lua, (call, camera): (mlua::Function, LuaValue)) -> mlua::Result<()> {
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

    /* entry
    {
        "version": "1.0.0", "name": "quiver.draw_3d.draw_ray",
        "info": "Draw a ray.",
        "member": [
            { "name": "ray",   "info": "The ray.",              "kind": "ray"   },
            { "name": "color", "info": "The color of the ray.", "kind": "color" }
        ]
    }
    */
    fn draw_ray(lua: &Lua, (ray, color): (LuaValue, LuaValue)) -> mlua::Result<()> {
        let ray: Ray = lua.from_value(ray)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawRay(ray.into(), color.into());
            Ok(())
        }
    }
}
