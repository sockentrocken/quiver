/*
* BSD Zero Clause License
*
* Copyright (c) 2025 sockentrocken
*
* Permission to use, copy, modify, and/or distribute this software for any
* purpose with or without fee is hereby granted.
*
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
* REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
* AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
* INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
* LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
* OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
* PERFORMANCE OF THIS SOFTWARE.
*/

use crate::system::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    draw_general::set_global(lua, table)?;
    draw_3d::set_global(lua, table)?;
    draw_2d::set_global(lua, table)?;

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

        draw.set("begin",          lua.create_function(self::begin)?)?;
        draw.set("begin_blend",    lua.create_function(self::begin_blend)?)?;
        draw.set("begin_scissor",  lua.create_function(self::begin_scissor)?)?;
        draw.set("clear",          lua.create_function(self::clear)?)?;

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

mod draw_3d {
    use super::*;

    /* class
    { "version": "1.0.0", "name": "quiver.draw_3d", "info": "The 3D drawing API." }
    */
    #[rustfmt::skip]
    pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
        let draw_3d = lua.create_table()?;

        draw_3d.set("begin",                 lua.create_function(self::begin)?)?;
        draw_3d.set("update_camera_pro",     lua.create_function(self::update_camera_pro)?)?;
        draw_3d.set("get_matrix_projection", lua.create_function(self::get_matrix_projection)?)?;
        draw_3d.set("get_matrix_model_view", lua.create_function(self::get_matrix_model_view)?)?;
        draw_3d.set("get_screen_to_world",   lua.create_function(self::get_screen_to_world)?)?;
        draw_3d.set("get_world_to_screen",   lua.create_function(self::get_world_to_screen)?)?;
        draw_3d.set("draw_grid",             lua.create_function(self::draw_grid)?)?;
        draw_3d.set("draw_cube",             lua.create_function(self::draw_cube)?)?;
        draw_3d.set("draw_ball",             lua.create_function(self::draw_ball)?)?;
        draw_3d.set("draw_box_3",            lua.create_function(self::draw_box_3)?)?;
        draw_3d.set("draw_ray",              lua.create_function(self::draw_ray)?)?;
        draw_3d.set("draw_line",             lua.create_function(self::draw_line)?)?;
        draw_3d.set("set_backface_cull",     lua.create_function(self::set_backface_cull)?)?;
        draw_3d.set("begin_quad",            lua.create_function(self::begin_quad)?)?;
        draw_3d.set("draw_quad_color",       lua.create_function(self::draw_quad_color)?)?;
        draw_3d.set("draw_quad_normal",      lua.create_function(self::draw_quad_normal)?)?;
        draw_3d.set("draw_quad_coordinate",  lua.create_function(self::draw_quad_coordinate)?)?;
        draw_3d.set("draw_quad_vertex",      lua.create_function(self::draw_quad_vertex)?)?;

        table.set("draw_3d", draw_3d)?;

        Ok(())
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_3d.get_matrix_projection",
        "info": ""
    }
    */
    #[rustfmt::skip]
    fn get_matrix_projection(_: &Lua, _ : ()) -> mlua::Result<(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32)> {
        unsafe {
            let value = ffi::rlGetMatrixProjection();
            Ok((
                value.m0,  value.m1,  value.m2,  value.m3,
                value.m4,  value.m5,  value.m6,  value.m7,
                value.m8,  value.m9,  value.m10, value.m11,
                value.m12, value.m13, value.m14, value.m15 
            ))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_3d.get_matrix_model_view",
        "info": ""
    }
    */
    #[rustfmt::skip]
    fn get_matrix_model_view(_: &Lua, _ : ()) -> mlua::Result<(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32)> {
        unsafe {
            let value = ffi::rlGetMatrixModelview();
            Ok((
                value.m0,  value.m1,  value.m2,  value.m3,
                value.m4,  value.m5,  value.m6,  value.m7,
                value.m8,  value.m9,  value.m10, value.m11,
                value.m12, value.m13, value.m14, value.m15 
            ))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_3d.update_camera_pro",
        "info": "Update the 3D camera (pro).",
        "member": [
            { "name": "camera",   "info": "The camera to update.", "kind": "camera_3d" },
            { "name": "position", "info": "",                      "kind": "vector_3"  },
            { "name": "rotation", "info": "",                      "kind": "vector_3"  },
            { "name": "zoom",     "info": "",                      "kind": "number"    }
        ]
    }
    */
    fn update_camera_pro(
        lua: &Lua,
        (camera, position, rotation, zoom): (LuaValue, LuaValue, LuaValue, f32),
    ) -> mlua::Result<(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, i32)> {
        let camera: general::Camera3D = lua.from_value(camera)?;
        let position: Vector3 = lua.from_value(position)?;
        let rotation: Vector3 = lua.from_value(rotation)?;

        unsafe {
            let mut camera: ffi::Camera3D = camera.into();

            ffi::UpdateCameraPro(&mut camera, position.into(), rotation.into(), zoom);

            Ok((
                camera.position.x,
                camera.position.y,
                camera.position.z,
                camera.target.x,
                camera.target.y,
                camera.target.z,
                camera.up.x,
                camera.up.y,
                camera.up.z,
                camera.fovy,
                camera.projection,
            ))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_3d.get_screen_to_world",
        "info": "Get a ray for a 2D screen-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",        "kind": "camera_3d" },
            { "name": "point",  "info": "The screen-space point.",    "kind": "vector_2"  },
            { "name": "shape",  "info": "The size of the view-port.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "position_x",  "info": "The 3D ray position. (X).",  "kind": "number" },
            { "name": "position_y",  "info": "The 3D ray position. (Y).",  "kind": "number" },
            { "name": "position_z",  "info": "The 3D ray position. (Z).",  "kind": "number" },
            { "name": "direction_x", "info": "The 3D ray direction. (X).", "kind": "number" },
            { "name": "direction_y", "info": "The 3D ray direction. (Y).", "kind": "number" },
            { "name": "direction_z", "info": "The 3D ray direction. (Z).", "kind": "number" }
        ]
    }
    */
    fn get_screen_to_world(
        lua: &Lua,
        (camera, point, shape): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<(f32, f32, f32, f32, f32, f32)> {
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

            Ok((
                ray.position.x,
                ray.position.y,
                ray.position.z,
                ray.direction.x,
                ray.direction.y,
                ray.direction.z,
            ))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_3d.get_world_to_screen",
        "info": "Get a 2D screen-space point for a 3D world-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",        "kind": "camera_3d" },
            { "name": "point",  "info": "The world-space point.",     "kind": "vector_3"  },
            { "name": "shape",  "info": "The size of the view-port.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "point_x", "info": "The 2D screen-space point (X).", "kind": "number" },
            { "name": "point_y", "info": "The 2D screen-space point (Y).", "kind": "number" }
        ]
    }
    */
    fn get_world_to_screen(
        lua: &Lua,
        (camera, point, shape): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<(f32, f32)> {
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

            Ok((point.x, point.y))
        }
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
        "version": "1.0.0", "name": "quiver.draw_3d.draw_ball",
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

    /* entry
    {
        "version": "1.0.0", "name": "quiver.draw_3d.draw_line",
        "info": "Draw a line.",
        "member": [
            { "name": "point_a", "info": "The point A of the line.",   "kind": "vector_3" },
            { "name": "point_b", "info": "The point B of the line.",   "kind": "vector_3" },
            { "name": "color",   "info": "The color of the line.",     "kind": "color"    }
        ]
    }
    */
    fn draw_line(
        lua: &Lua,
        (point_a, point_b, color): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point_a: Vector3 = lua.from_value(point_a)?;
        let point_b: Vector3 = lua.from_value(point_b)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawLine3D(point_a.into(), point_b.into(), color.into());
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0", "name": "quiver.draw_3d.set_backface_cull",
        "info": "Set the current state of backface culling.",
        "member": [
            { "name": "state", "info": "The new state.", "kind": "boolean" }
        ]
    }
    */
    fn set_backface_cull(_: &Lua, state: bool) -> mlua::Result<()> {
        unsafe {
            if state {
                ffi::rlEnableBackfaceCulling();
            } else {
                ffi::rlDisableBackfaceCulling();
            }
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.begin_quad",
        "info": "",
        "member": [
            { "name": "call", "info": "The draw code.", "kind": "function" }
        ]
    }
    */
    fn begin_quad(_: &Lua, (call, texture): (mlua::Function, Option<u32>)) -> mlua::Result<()> {
        unsafe {
            if let Some(texture) = texture {
                ffi::rlSetTexture(texture);
            }

            ffi::rlBegin(ffi::RL_QUADS as i32);

            call.call::<()>(())?;

            ffi::rlEnd();

            if texture.is_some() {
                ffi::rlSetTexture(0);
            }

            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.draw_quad_color",
        "info": ""
    }
    */
    fn draw_quad_color(lua: &Lua, color: LuaValue) -> mlua::Result<()> {
        unsafe {
            let color: Color = lua.from_value(color)?;
            ffi::rlColor4ub(color.r, color.g, color.b, color.a);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.draw_quad_normal",
        "info": ""
    }
    */
    fn draw_quad_normal(lua: &Lua, normal: LuaValue) -> mlua::Result<()> {
        unsafe {
            let normal: Vector3 = lua.from_value(normal)?;
            ffi::rlNormal3f(normal.x, normal.y, normal.z);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.draw_quad_coordinate",
        "info": ""
    }
    */
    fn draw_quad_coordinate(lua: &Lua, coordinate: LuaValue) -> mlua::Result<()> {
        unsafe {
            let coordinate: Vector2 = lua.from_value(coordinate)?;
            ffi::rlTexCoord2f(coordinate.x, coordinate.y);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw.draw_quad_vertex",
        "info": ""
    }
    */
    fn draw_quad_vertex(lua: &Lua, vertex: LuaValue) -> mlua::Result<()> {
        unsafe {
            let vertex: Vector3 = lua.from_value(vertex)?;
            ffi::rlVertex3f(vertex.x, vertex.y, vertex.z);
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

        draw_2d.set("begin",                 lua.create_function(self::begin)?)?;
        draw_2d.set("get_screen_to_world",   lua.create_function(self::get_screen_to_world)?)?;
        draw_2d.set("get_world_to_screen",   lua.create_function(self::get_world_to_screen)?)?;
        draw_2d.set("draw_pixel",            lua.create_function(self::draw_pixel)?)?;
        draw_2d.set("draw_line",             lua.create_function(self::draw_line)?)?;
        draw_2d.set("draw_text",             lua.create_function(self::draw_text)?)?;
        draw_2d.set("draw_circle",           lua.create_function(self::draw_circle)?)?;
        draw_2d.set("draw_circle_sector",    lua.create_function(self::draw_circle_sector)?)?;
        draw_2d.set("draw_box_2",            lua.create_function(self::draw_box_2)?)?;
        draw_2d.set("draw_box_2_gradient_x", lua.create_function(self::draw_box_2_gradient_x)?)?;
        draw_2d.set("draw_box_2_gradient_y", lua.create_function(self::draw_box_2_gradient_y)?)?;
        draw_2d.set("draw_box_2_gradient",   lua.create_function(self::draw_box_2_gradient)?)?;
        draw_2d.set("draw_box_2_line",       lua.create_function(self::draw_box_2_line)?)?;
        draw_2d.set("draw_box_2_round",      lua.create_function(self::draw_box_2_round)?)?;
        draw_2d.set("draw_box_2_line_round", lua.create_function(self::draw_box_2_line_round)?)?;
        draw_2d.set("draw_triangle",         lua.create_function(self::draw_triangle)?)?;
        draw_2d.set("draw_triangle_line",    lua.create_function(self::draw_triangle_line)?)?;

        table.set("draw_2d", draw_2d)?;

        Ok(())
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.get_screen_to_world",
        "info": "Get a world-space point for a 2D screen-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",     "kind": "camera_2d" },
            { "name": "point",  "info": "The screen-space point.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "point_x", "info": "The 2D world-space point (X).", "kind": "number" },
            { "name": "point_y", "info": "The 2D world-space point (Y).", "kind": "number" }
        ]
    }
    */
    fn get_screen_to_world(
        lua: &Lua,
        (camera, point): (LuaValue, LuaValue),
    ) -> mlua::Result<(f32, f32)> {
        let camera: general::Camera2D = lua.from_value(camera)?;
        let point: Vector2 = lua.from_value(point)?;

        unsafe {
            let point = ffi::GetScreenToWorld2D(point.into(), camera.into());

            Ok((point.x, point.y))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.get_world_to_screen",
        "info": "Get a screen-space point for a 2D world-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",    "kind": "camera_2d" },
            { "name": "point",  "info": "The world-space point.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "point_x", "info": "The 2D screen-space point (X).", "kind": "number" },
            { "name": "point_y", "info": "The 2D screen-space point (Y).", "kind": "number" }
        ]
    }
    */
    fn get_world_to_screen(
        lua: &Lua,
        (camera, point): (LuaValue, LuaValue),
    ) -> mlua::Result<(f32, f32)> {
        let camera: general::Camera2D = lua.from_value(camera)?;
        let point: Vector2 = lua.from_value(point)?;

        unsafe {
            let point = ffi::GetWorldToScreen2D(point.into(), camera.into());

            Ok((point.x, point.y))
        }
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
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_pixel",
        "info": "Draw pixel.",
        "member": [
            { "name": "point", "info": "The point of the pixel.", "kind": "vector_2" },
            { "name": "color", "info": "The color of the pixel.", "kind": "color"    }
        ]
    }
    */
    fn draw_pixel(lua: &Lua, (point, color): (LuaValue, LuaValue)) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawPixelV(point.into(), color.into());
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_line",
        "info": "Draw a line.",
        "member": [
            { "name": "point_a", "info": "The point A of the line.",   "kind": "vector_2" },
            { "name": "point_b", "info": "The point B of the line.",   "kind": "vector_2" },
            { "name": "thick",   "info": "The thickness of the line.", "kind": "number"   },
            { "name": "color",   "info": "The color of the line.",     "kind": "color"    }
        ]
    }
    */
    fn draw_line(
        lua: &Lua,
        (point_a, point_b, thick, color): (LuaValue, LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let point_a: Vector2 = lua.from_value(point_a)?;
        let point_b: Vector2 = lua.from_value(point_b)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawLineEx(point_a.into(), point_b.into(), thick, color.into());
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_text",
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

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_box_2",
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
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_box_2_gradient_x",
        "info": "Draw 2D box with a gradient (X-direction).",
        "member": [
            { "name": "shape",   "info": "The shape of the box.",   "kind": "box_2" },
            { "name": "color_a", "info": "The color A of the box.", "kind": "color" },
            { "name": "color_b", "info": "The color B of the box.", "kind": "color" }
        ]
    }
    */
    fn draw_box_2_gradient_x(
        lua: &Lua,
        (shape, color_a, color_b): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let color_a: Color = lua.from_value(color_a)?;
        let color_b: Color = lua.from_value(color_b)?;

        unsafe {
            ffi::DrawRectangleGradientH(
                shape.x as i32,
                shape.y as i32,
                shape.width as i32,
                shape.height as i32,
                color_a.into(),
                color_b.into(),
            );
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_box_2_gradient_y",
        "info": "Draw 2D box with a gradient (Y-direction).",
        "member": [
            { "name": "shape",   "info": "The shape of the box.",   "kind": "box_2" },
            { "name": "color_a", "info": "The color A of the box.", "kind": "color" },
            { "name": "color_b", "info": "The color B of the box.", "kind": "color" }
        ]
    }
    */
    fn draw_box_2_gradient_y(
        lua: &Lua,
        (shape, color_a, color_b): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let color_a: Color = lua.from_value(color_a)?;
        let color_b: Color = lua.from_value(color_b)?;

        unsafe {
            ffi::DrawRectangleGradientV(
                shape.x as i32,
                shape.y as i32,
                shape.width as i32,
                shape.height as i32,
                color_a.into(),
                color_b.into(),
            );
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_box_2_gradient",
        "info": "Draw 2D box with a 4-point gradient.",
        "member": [
            { "name": "shape",   "info": "The shape of the box.",   "kind": "box_2" },
            { "name": "color_a", "info": "The color A (T.L.) of the box.", "kind": "color" },
            { "name": "color_b", "info": "The color B (B.L.) of the box.", "kind": "color" },
            { "name": "color_c", "info": "The color C (T.R.) of the box.", "kind": "color" },
            { "name": "color_d", "info": "The color D (B.R.) of the box.", "kind": "color" }
        ]
    }
    */
    fn draw_box_2_gradient(
        lua: &Lua,
        (shape, color_a, color_b, color_c, color_d): (
            LuaValue,
            LuaValue,
            LuaValue,
            LuaValue,
            LuaValue,
        ),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let color_a: Color = lua.from_value(color_a)?;
        let color_b: Color = lua.from_value(color_b)?;
        let color_c: Color = lua.from_value(color_c)?;
        let color_d: Color = lua.from_value(color_d)?;

        unsafe {
            ffi::DrawRectangleGradientEx(
                shape.into(),
                color_a.into(),
                color_b.into(),
                color_c.into(),
                color_d.into(),
            );
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_box_2_line",
        "info": "Draw 2D box (out-line).",
        "member": [
            { "name": "shape", "info": "The shape of the box.",     "kind": "box_2"  },
            { "name": "thick", "info": "The thickness of the box.", "kind": "number" },
            { "name": "color", "info": "The color of the box.",     "kind": "color"  }
        ]
    }
    */
    fn draw_box_2_line(
        lua: &Lua,
        (shape, thick, color): (LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawRectangleLinesEx(shape.into(), thick, color.into());
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_box_2_round",
        "info": "Draw 2D box (round).",
        "member": [
            { "name": "shape", "info": "The shape of the box.",         "kind": "box_2"  },
            { "name": "round", "info": "The roundness of the box.",     "kind": "number" },
            { "name": "count", "info": "The segment count of the box.", "kind": "number" },
            { "name": "color", "info": "The color of the box.",         "kind": "color"  }
        ]
    }
    */
    fn draw_box_2_round(
        lua: &Lua,
        (shape, round, count, color): (LuaValue, f32, i32, LuaValue),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawRectangleRounded(shape.into(), round, count, color.into());
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_box_2_line_round",
        "info": "Draw 2D box (out-line, round).",
        "member": [
            { "name": "shape", "info": "The shape of the box.",         "kind": "box_2"  },
            { "name": "round", "info": "The roundness of the box.",     "kind": "number" },
            { "name": "count", "info": "The segment count of the box.", "kind": "number" },
            { "name": "thick", "info": "The thickness of the box.",     "kind": "number" },
            { "name": "color", "info": "The color of the box.",         "kind": "color"  }
        ]
    }
    */
    fn draw_box_2_line_round(
        lua: &Lua,
        (shape, round, count, thick, color): (LuaValue, f32, i32, f32, LuaValue),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawRectangleRoundedLinesEx(shape.into(), round, count, thick, color.into());
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_triangle",
        "info": "Draw 2D triangle.",
        "member": [
            { "name": "point_a", "info": "The point A of the triangle.", "kind": "vector_2" },
            { "name": "point_b", "info": "The point B of the triangle.", "kind": "vector_2" },
            { "name": "point_c", "info": "The point C of the triangle.", "kind": "vector_2" },
            { "name": "color",   "info": "The color of the triangle.",   "kind": "color"    }
        ]
    }
    */
    fn draw_triangle(
        lua: &Lua,
        (point_a, point_b, point_c, color): (LuaValue, LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point_a: Vector2 = lua.from_value(point_a)?;
        let point_b: Vector2 = lua.from_value(point_b)?;
        let point_c: Vector2 = lua.from_value(point_c)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawTriangle(point_a.into(), point_b.into(), point_c.into(), color.into());
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.draw_2d.draw_triangle_line",
        "info": "Draw 2D triangle (out-line).",
        "member": [
            { "name": "point_a", "info": "The point A of the triangle.", "kind": "vector_2" },
            { "name": "point_b", "info": "The point B of the triangle.", "kind": "vector_2" },
            { "name": "point_c", "info": "The point C of the triangle.", "kind": "vector_2" },
            { "name": "color",   "info": "The color of the triangle.",   "kind": "color"    }
        ]
    }
    */
    fn draw_triangle_line(
        lua: &Lua,
        (point_a, point_b, point_c, color): (LuaValue, LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point_a: Vector2 = lua.from_value(point_a)?;
        let point_b: Vector2 = lua.from_value(point_b)?;
        let point_c: Vector2 = lua.from_value(point_c)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            ffi::DrawTriangleLines(point_a.into(), point_b.into(), point_c.into(), color.into());
            Ok(())
        }
    }
}
