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

    font.set("new",                 lua.create_function(self::Font::new)?)?;
    font.set("new_default",         lua.create_function(self::Font::new_default)?)?;
    font.set("set_text_line_space", lua.create_function(set_text_line_space)?)?;

    table.set("font", font)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.font.set_text_line_space",
    "info": "Set the vertical space between each line-break.",
    "member": [
        { "name": "space", "info": "Vertical space.", "kind": "number" }
    ]
}
*/
fn set_text_line_space(_: &Lua, space: i32) -> mlua::Result<()> {
    unsafe {
        ffi::SetTextLineSpacing(space);
    }

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
            "version": "1.0.0",
            "name": "font:draw",
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

        /* entry
        {
            "version": "1.0.0",
            "name": "font:measure_text",
            "info": "Measure the size of a given text on screen, with a given font.",
            "member": [
                { "name": "label", "info": "Label of font to measure.", "kind": "string" },
                { "name": "scale", "info": "Scale of font to measure.", "kind": "number" },
                { "name": "space", "info": "Space of font to measure.", "kind": "number" }
            ],
            "result": [
                { "name": "size_x", "info": "Size of text (X).", "kind": "number" },
                { "name": "size_y", "info": "Size of text (Y).", "kind": "number" }
            ]
        }
        */
        method.add_method(
            "measure_text",
            |_: &Lua, this, (text, scale, space): (String, f32, f32)| {
                let text = CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))?;

                unsafe {
                    let result = ffi::MeasureTextEx(*this.0, text.as_ptr(), scale, space);
                    Ok((result.x, result.y))
                }
            },
        );
    }
}

impl Font {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.font.new",
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

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.font.new_default",
        "info": "Create a new font resource (default font).",
        "result": [
            { "name": "font", "info": "Font resource.", "kind": "font" }
        ]
    }
    */
    fn new_default(_: &Lua, _: ()) -> mlua::Result<Self> {
        unsafe {
            let data = ffi::GetFontDefault();

            Ok(Self(RLFont::from_raw(data)))
        }
    }
}
