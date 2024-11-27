use crate::script::*;

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

type RLFont = raylib::core::text::Font;

//================================================================

/* class
{ "name": "quiver.font", "info": "The font API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, system : &ModuleSystem) -> mlua::Result<()> {
    let font = lua.create_table()?;

    if system.font { font.set("new", lua.create_function(self::Font::new)?)?;   }

    table.set("font", font)?;

    Ok(())
}

/* class
{ "name": "font", "info": "An unique handle to a font in memory." }
*/
pub struct Font(RLFont);

impl mlua::UserData for Font {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* function
        {
            "name": "font.draw",
            "info": "Draw a font.",
            "parameter": [
                { "optional": false, "name": "label", "info": "Label of font to draw.", "type": "string"   },
                { "optional": false, "name": "point", "info": "Point of font to draw.", "type": "vector_2" },
                { "optional": false, "name": "scale", "info": "Scale of font to draw.", "type": "number"   },
                { "optional": false, "name": "space", "info": "Space of font to draw.", "type": "number"   },
                { "optional": false, "name": "color", "info": "Color of font to draw.", "type": "color"    }
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
                    let point : crate::system::general::Vector2 = lua.from_value(point)?;
                    let color : crate::system::general::Color   = lua.from_value(color)?;
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
    /* function
    {
        "name": "quiver.font.new",
        "info": "Create a new font resource.",
        "parameter": [
            { "optional": false, "name": "path", "info": "Path to font file.", "type": "string" }
        ],
        "return": [
            { "optional": false, "name": "font", "info": "Font resource.", "type": "font" }
        ]
    }
    */
    pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadFont(name.as_ptr());

            if ffi::IsFontReady(data) {
                Ok(Self(RLFont::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Font::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}
