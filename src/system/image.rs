/*
* Copyright (c) 2025 sockentrocken
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
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

unsafe impl Send for Image {}

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
