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

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//================================================================

use crate::script::*;
use crate::status::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.video", "info": "The video API.", "head": true }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, info: &Info, table: &mlua::Table) -> mlua::Result<()> {
    if !info.head {
        return Ok(());
    }
    
    let video = lua.create_table()?;

    video.set("new", lua.create_function(self::Video::new)?)?;

    table.set("video", video)?;

    Ok(())
}

/* class
{ "version": "1.0.0", "name": "video", "info": "An unique handle to a video in memory." }
*/
struct Video(MediaStream);

unsafe impl Send for Video {}

impl mlua::UserData for Video {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("ID", |_: &Lua, this| Ok(this.0.videoTexture.id));
        field.add_field_method_get("shape_x", |_: &Lua, this| Ok(this.0.videoTexture.width));
        field.add_field_method_get("shape_y", |_: &Lua, this| Ok(this.0.videoTexture.height));

        // GetMediaProperties
        field.add_field_method_get("duration", |_: &Lua, this| unsafe {
            let property = GetMediaProperties(this.0);
            Ok(property.durationSec)
        });
        field.add_field_method_get("frame_rate", |_: &Lua, this| unsafe {
            let property = GetMediaProperties(this.0);
            Ok(property.avgFPS)
        });
        field.add_field_method_get("video", |_: &Lua, this| unsafe {
            let property = GetMediaProperties(this.0);
            Ok(property.hasVideo)
        });
        field.add_field_method_get("audio", |_: &Lua, this| unsafe {
            let property = GetMediaProperties(this.0);
            Ok(property.hasAudio)
        });

        // GetMediaState
        field.add_field_method_get("state", |_: &Lua, this| unsafe {
            Ok(GetMediaState(this.0))
        });

        // SetMediaState
        field.add_field_method_set("state", |_: &Lua, this, value| unsafe {
            SetMediaState(this.0, value);
            Ok(())
        });

        // GetMediaPosition
        field.add_field_method_get("position", |_: &Lua, this| unsafe {
            Ok(GetMediaPosition(this.0))
        });

        // SetMediaPosition
        field.add_field_method_set("position", |_: &Lua, this, value| unsafe {
            SetMediaPosition(this.0, value);
            Ok(())
        });

        // SetMediaLooping
        field.add_field_method_set("loop", |_: &Lua, this, value| unsafe {
            SetMediaLooping(this.0, value);
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "video:update",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("update", |_: &Lua, this, _: ()| {
            unsafe {
                UpdateMedia(&mut this.0);
            }

            Ok(())
        });

        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "video:draw",
            "info": "Draw a texture.",
            "member": [
                { "name": "point", "info": "TO-DO", "kind": "vector_2" },
                { "name": "angle", "info": "TO-DO", "kind": "number"   },
                { "name": "scale", "info": "TO-DO", "kind": "number"   },
                { "name": "color", "info": "TO-DO", "kind": "color"    }
            ]
        }
        */
        method.add_method(
            "draw",
            |lua: &Lua, this, (point, angle, scale, color): (LuaValue, f32, f32, LuaValue)| unsafe {
                let texture: ffi::Texture = std::mem::transmute(this.0.videoTexture);
                Ok(crate::system::texture::texture_draw(
                    lua,
                    (&texture, point, angle, scale, color),
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "video:draw_pro",
            "info": "Draw a texture (pro).",
            "member": [
                { "name": "box_a", "info": "TO-DO", "kind": "box_2"    },
                { "name": "box_b", "info": "TO-DO", "kind": "box_2"    },
                { "name": "point", "info": "TO-DO", "kind": "vector_2" },
                { "name": "angle", "info": "TO-DO", "kind": "number"   },
                { "name": "color", "info": "TO-DO", "kind": "color"    }
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
                )| unsafe {
                    let texture : ffi::Texture = std::mem::transmute(this.0.videoTexture);
                    Ok(crate::system::texture::texture_pro_draw(
                        lua,
                        (&texture, box_a, box_b, point, angle, color),
                    ))
                },
            );
    }
}

impl Video {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.video.new",
        "info": "Create a new video resource.",
        "member": [
            { "name": "path", "info": "Path to video file.", "kind": "string" }
        ],
        "result": [
            { "name": "video", "info": "Video resource.", "kind": "video" }
        ]
    }
    */
    fn new(lua: &Lua, path: String) -> mlua::Result<Self> {
        let name = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

        unsafe {
            // Adjust the capacity of the circular buffer (A):
            SetMediaFlag(3, 64 * 1024); // Default: 16 * 1024 bytes

            // Set the size of the chunk (B) uploaded from A to the AudioStream each time it processes data:
            // (Note: Since a callback isn't available, this size is fixed.)
            SetMediaFlag(9, 16 * 1024); // Default: 4 * 1024 bytes

            // Configure the size of the AudioStream buffer (C):
            SetMediaFlag(4, 4 * 1024); // Default: 1 * 1024 bytes

            let data = LoadMedia(name.as_ptr());

            if IsMediaValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Video::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

impl Drop for Video {
    fn drop(&mut self) {
        unsafe {
            UnloadMedia(&mut self.0);
        }
    }
}
