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

use crate::script::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

type RLTexture = ffi::Texture2D;
type RLRenderTexture = ffi::RenderTexture2D;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.texture", "info": "The texture API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let texture = lua.create_table()?;

    texture.set("new", lua.create_function(self::Texture::new)?)?;

    table.set("texture", texture)?;

    let render_texture = lua.create_table()?;

    render_texture.set("new", lua.create_function(self::RenderTexture::new)?)?;

    table.set("render_texture", render_texture)?;

    Ok(())
}

fn texture_draw(
    lua: &Lua,
    (texture, point, angle, scale, color): (&ffi::Texture, LuaValue, f32, f32, LuaValue),
) -> mlua::Result<()> {
    let point: Vector2 = lua.from_value(point)?;
    let color: Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawTextureEx(*texture, point.into(), angle, scale, color.into());
        Ok(())
    }
}

fn texture_pro_draw(
    lua: &Lua,
    (texture, rec_a, rec_b, point, angle, color): (
        &ffi::Texture,
        LuaValue,
        LuaValue,
        LuaValue,
        f32,
        LuaValue,
    ),
) -> mlua::Result<()> {
    let rec_a: Rectangle = lua.from_value(rec_a)?;
    let rec_b: Rectangle = lua.from_value(rec_b)?;
    let point: Vector2 = lua.from_value(point)?;
    let color: Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawTexturePro(
            *texture,
            rec_a.into(),
            rec_b.into(),
            point.into(),
            angle,
            color.into(),
        );
        Ok(())
    }
}

/* class
{
    "version": "1.0.0",
    "name": "texture",
    "info": "An unique handle for a texture in memory.",
    "member": [
        { "name": "shape_x", "info": "Shape of the texture (X).", "kind": "number" },
        { "name": "shape_y", "info": "Shape of the texture (Y).", "kind": "number" }
    ]
}
*/
pub struct Texture(pub RLTexture);

impl mlua::UserData for Texture {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("shape_x", |_: &Lua, this| Ok(this.0.width));
        field.add_field_method_get("shape_y", |_: &Lua, this| Ok(this.0.height));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "texture:set_mipmap",
            "info": "Set the mipmap for a texture."
        }
        */
        method.add_method_mut("set_mipmap", |_: &Lua, this, _: ()| {
            unsafe {
                ffi::GenTextureMipmaps(&mut this.0);
            }
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:set_filter",
            "info": "Set the filter for a texture.",
            "member": [
                { "name": "filter", "info": "Texture filter.", "kind": "texture_filter" }
            ]
        }
        */
        method.add_method_mut("set_filter", |_: &Lua, this, filter: i32| {
            unsafe {
                ffi::SetTextureFilter(this.0, filter);
            }
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:set_wrap",
            "info": "Set the wrap for a texture.",
            "member": [
                { "name": "wrap", "info": "Texture wrap.", "kind": "texture_wrap" }
            ]
        }
        */
        method.add_method_mut("set_wrap", |_: &Lua, this, wrap: i32| {
            unsafe {
                ffi::SetTextureWrap(this.0, wrap);
            }
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:draw",
            "info": "Draw a texture.",
            "member": [
                { "name": "point", "info": "", "kind": "vector_2" },
                { "name": "angle", "info": "", "kind": "number"   },
                { "name": "scale", "info": "", "kind": "number"   },
                { "name": "color", "info": "", "kind": "color"    }
            ]
        }
        */
        method.add_method(
            "draw",
            |lua: &Lua, this, (point, angle, scale, color): (LuaValue, f32, f32, LuaValue)| {
                Ok(texture_draw(lua, (&this.0, point, angle, scale, color)))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:draw_pro",
            "info": "Draw a texture (pro).",
            "member": [
                { "name": "box_a", "info": "", "kind": "box_2"    },
                { "name": "box_b", "info": "", "kind": "box_2"    },
                { "name": "point", "info": "", "kind": "vector_2" },
                { "name": "angle", "info": "", "kind": "number"   },
                { "name": "color", "info": "", "kind": "color"    }
            ]
        }
        */
        method.add_method(
                "draw_pro",
                |lua: &Lua,
                 this,
                 (box_a, box_b, point, angle, color): (
                    LuaValue,
                    LuaValue,
                    LuaValue,
                    f32,
                    LuaValue,
                )| {
                    Ok(texture_pro_draw(
                        lua,
                        (&this.0, box_a, box_b, point, angle, color),
                    ))
                },
            );
    }
}

impl Texture {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.texture.new",
        "info": "Create a new texture resource.",
        "member": [
            { "name": "path", "info": "Path to texture file.", "kind": "string" }
        ],
        "result": [
            { "name": "texture", "info": "Texture resource.", "kind": "texture" }
        ]
    }
    */
    fn new(lua: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(ScriptData::get_path(lua, &path)?)
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadTexture(name.as_ptr());

            if ffi::IsTextureValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Texture::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadTexture(self.0);
        }
    }
}

/* class
{
    "version": "1.0.0",
    "name": "render_texture",
    "info": "An unique handle for a render texture in memory.",
    "member": [
        { "name": "shape_x", "info": "Shape of the texture (X).", "kind": "number" },
        { "name": "shape_y", "info": "Shape of the texture (Y).", "kind": "number" }
    ]
}
*/
pub struct RenderTexture(pub RLRenderTexture);

impl mlua::UserData for RenderTexture {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("shape_x", |_: &Lua, this| Ok(this.0.texture.width));
        field.add_field_method_get("shape_y", |_: &Lua, this| Ok(this.0.texture.height));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "render_texture:begin",
            "info": "Initialize drawing to the render texture.",
            "member": [
                { "name": "call", "info": "The draw code.", "kind": "function" }
            ]
        }
        */
        method.add_method("begin", |_: &Lua, this, call: mlua::Function| {
            unsafe {
                ffi::BeginTextureMode(this.0);

                call.call::<()>(())?;

                ffi::EndTextureMode();
            }

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "render_texture:draw",
            "info": "Draw a texture.",
            "member": [
                { "name": "point", "info": "", "kind": "vector_2" },
                { "name": "angle", "info": "", "kind": "number"   },
                { "name": "scale", "info": "", "kind": "number"   },
                { "name": "color", "info": "", "kind": "color"    }
            ]
        }
        */
        method.add_method(
            "draw",
            |lua: &Lua, this, (point, angle, scale, color): (LuaValue, f32, f32, LuaValue)| {
                Ok(texture_draw(
                    lua,
                    (&this.0.texture, point, angle, scale, color),
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "render_texture:draw_pro",
            "info": "Draw a texture (pro).",
            "member": [
                { "name": "box_a", "info": "", "kind": "box_2"    },
                { "name": "box_b", "info": "", "kind": "box_2"    },
                { "name": "point", "info": "", "kind": "vector_2" },
                { "name": "angle", "info": "", "kind": "number"   },
                { "name": "color", "info": "", "kind": "color"    }
            ]
        }
        */
        method.add_method(
                "draw_pro",
                |lua: &Lua,
                 this,
                 (box_a, box_b, point, angle, color): (
                    LuaValue,
                    LuaValue,
                    LuaValue,
                    f32,
                    LuaValue,
                )| {
                    Ok(texture_pro_draw(
                        lua,
                        (&this.0.texture, box_a, box_b, point, angle, color),
                    ))
                },
            );
    }
}

impl RenderTexture {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.render_texture.new",
        "info": "Create a new render texture resource.",
        "member": [
            { "name": "shape", "info": "", "kind": "vector_2" }
        ],
        "result": [
            { "name": "render_texture", "info": "Render texture resource.", "kind": "render_texture" }
        ]
    }
    */
    fn new(lua: &Lua, shape: LuaValue) -> mlua::Result<Self> {
        let shape: Vector2 = lua.from_value(shape)?;

        unsafe {
            let data = ffi::LoadRenderTexture(shape.x as i32, shape.y as i32);

            if ffi::IsRenderTextureValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "RenderTexture::new(): Could not load render texture.".to_string(),
                ))
            }
        }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadRenderTexture(self.0);
        }
    }
}
