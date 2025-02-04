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
use crate::system::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

type RLImage = ffi::Image;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.image", "info": "The image API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let image = lua.create_table()?;

    image.set("new", lua.create_function(self::Image::new)?)?;

    table.set("image", image)?;

    Ok(())
}

/* class
{
    "version": "1.0.0",
    "name": "image",
    "info": "An unique handle for a image in memory.",
    "member": [
        { "name": "shape_x", "info": "Shape of the image (X).", "kind": "number" },
        { "name": "shape_y", "info": "Shape of the image (Y).", "kind": "number" }
    ]
}
*/
pub struct Image(pub RLImage);

impl mlua::UserData for Image {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("shape_x", |_: &Lua, this| Ok(this.0.width));
        field.add_field_method_get("shape_y", |_: &Lua, this| Ok(this.0.height));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(_: &mut M) {}
}

impl Image {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.image.new",
        "info": "Create a new image resource.",
        "member": [
            { "name": "path", "info": "Path to image file.", "kind": "string" }
        ],
        "result": [
            { "name": "image", "info": "Image resource.", "kind": "image" }
        ]
    }
    */
    fn new(lua: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(ScriptData::get_path(lua, &path)?)
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadImage(name.as_ptr());

            if ffi::IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Image::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadImage(self.0);
        }
    }
}
