use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.font", "info": "The font API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let font = lua.create_table()?;

    font.set("new", lua.create_function(self::Font::new)?)?;

    table.set("font", font)?;

    Ok(())
}

type RLFont = raylib::core::text::Font;

/* class
{ "version": "1.0.0", "name": "font", "info": "An unique handle to a font in memory." }
*/
struct Font(RLFont);

impl mlua::UserData for Font {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0", "name": "font.draw",
            "info": "Draw a font.",
            "member": [
                { "name": "label", "info": "Label of font to draw.", "kind": "string"   },
                { "name": "point", "info": "Point of font to draw.", "kind": "vector_2" },
                { "name": "scale", "info": "Scale of font to draw.", "kind": "number"   },
                { "name": "space", "info": "Space of font to draw.", "kind": "number"   },
                { "name": "color", "info": "Color of font to draw.", "kind": "color"    }
            ]
        }
        */
        method.add_method(
                "draw",
                |lua: &Lua,
                 this,
                 (text, point, scale, space, color): (
                    String,
                    LuaValue,
                    f32,
                    f32,
                    LuaValue,
                )| {
                    let point : Vector2 = lua.from_value(point)?;
                    let color : Color   = lua.from_value(color)?;
                    let text = CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))?;

                    unsafe {
                        ffi::DrawTextEx(*this.0, text.as_ptr(), point.into(), scale, space, color.into());
                        Ok(())
                    }
                },
            );
    }
}

impl Font {
    /* entry
    {
        "version": "1.0.0", "name": "quiver.font.new",
        "info": "Create a new font resource.",
        "member": [
            { "name": "path", "info": "Path to font file.", "kind": "string" }
        ],
        "result": [
            { "name": "font", "info": "Font resource.", "kind": "font" }
        ]
    }
    */
    fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadFont(name.as_ptr());

            if ffi::IsFontValid(data) {
                Ok(Self(RLFont::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Font::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}
