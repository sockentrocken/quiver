use crate::script::*;

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

type RLTexture = raylib::core::texture::Texture2D;

//================================================================

/* class
{ "name": "quiver.texture", "info": "The texture API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, system: &ModuleSystem) -> mlua::Result<()> {
    let texture = lua.create_table()?;

    if system.texture { texture.set("new", lua.create_function(self::Texture::new)?)?; }

    table.set("texture", texture)?;

    Ok(())
}

fn texture_draw(
    lua: &Lua,
    (texture, point, angle, scale, color): (&ffi::Texture, LuaValue, f32, f32, LuaValue),
) -> mlua::Result<()> {
    let point: crate::system::general::Vector2 = lua.from_value(point)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

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
    let rec_a: crate::system::general::Box2 = lua.from_value(rec_a)?;
    let rec_b: crate::system::general::Box2 = lua.from_value(rec_b)?;
    let point: crate::system::general::Vector2 = lua.from_value(point)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

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
    "name": "texture",
    "info": "An unique handle for a texture in memory.",
    "field": [
        { "name": "shape", "info": "Shape of the texture.", "kind": "vector_2" }
    ]
}
*/
pub struct Texture(RLTexture);

impl mlua::UserData for Texture {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("size", |lua: &Lua, this| {
            lua.to_value(&crate::system::general::Vector2::new(
                this.0.width as f32,
                this.0.height as f32,
            ))
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /*
        ---Draw the texture.
        ---@param point vector_2 The point of the texture.
        ---@param angle number   The angle of the texture.
        ---@param scale number   The scale of the texture.
        ---@param color color    The color of the texture.
        function texture:draw(point, angle, scale, color) end
        */
        method.add_method(
            "draw",
            |lua: &Lua, this, (point, angle, scale, color): (LuaValue, f32, f32, LuaValue)| {
                Ok(texture_draw(lua, (&this.0, point, angle, scale, color)))
            },
        );

        /*
        ---Draw the texture (pro variant).
        ---@param box_a box_2    The source rectangle of the texture.
        ---@param box_b box_2    The target rectangle of the texture.
        ---@param point vector_2 The point of the texture.
        ---@param angle number   The angle of the texture.
        ---@param color color    The color of the texture.
        function texture:draw_pro(box_a, box_b, point, angle, color) end
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
    pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadTexture(name.as_ptr());

            if ffi::IsTextureReady(data) {
                Ok(Self(RLTexture::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Texture::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}
