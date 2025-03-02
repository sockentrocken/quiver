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
use crate::status::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.music", "info": "The music API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, info: &Info, table: &mlua::Table) -> mlua::Result<()> {
    if !info.head {
        return Ok(());
    }
    
    let music = lua.create_table()?;

    music.set("new",             lua.create_async_function(self::Music::new)?)?;
    music.set("new_from_memory", lua.create_async_function(self::Music::new_from_memory)?)?;

    table.set("music", music)?;

    Ok(())
}

type RLMusic = ffi::Music;

/* class
{ "version": "1.0.0", "name": "music", "info": "An unique handle for music in memory." }
*/
struct Music(RLMusic, Option<Vec<u8>>);

unsafe impl Send for Music {}

impl Music {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.music.new",
        "info": "Create a new music resource.",
        "member": [
            { "name": "path", "info": "Path to music file.", "kind": "string" }
        ],
        "result": [
            { "name": "music", "info": "Music resource.", "kind": "music" }
        ]
    }
    */
    async fn new(lua: Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(ScriptData::get_path(&lua, &path)?)
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;

        tokio::task::spawn_blocking(move || unsafe {
            let data = ffi::LoadMusicStream(name.as_ptr());

            if ffi::IsMusicValid(data) {
                Ok(Self(data, None))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Music::new(): Could not load file \"{path}\"."
                )))
            }
        })
        .await
        .unwrap()
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.music.new_from_memory",
        "info": "TO-DO"
    }
    */
    async fn new_from_memory(_: Lua, (data, kind): (LuaValue, String)) -> mlua::Result<Self> {
        let data = crate::system::data::Data::get_buffer(data)?;

        tokio::task::spawn_blocking(move || unsafe {
            let buffer = data.0.clone();
            let data = ffi::LoadMusicStreamFromMemory(
                Script::rust_to_c_string(&kind)?.as_ptr(),
                buffer.as_ptr(),
                buffer.len() as i32,
            );

            if ffi::IsMusicValid(data) {
                Ok(Self(data, Some(buffer)))
            } else {
                Err(mlua::Error::RuntimeError(
                    "Music::new_from_memory(): Could not load file.".to_string(),
                ))
            }
        })
        .await
        .unwrap()
    }
}

impl Drop for Music {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadMusicStream(self.0);
        }
    }
}

impl mlua::UserData for Music {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        { "version": "1.0.0", "name": "music:play", "info": "Play the music." }
        */
        method.add_method("play", |_, this, ()| unsafe {
            ffi::PlayMusicStream(this.0);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "music:get_playing",
            "info": "Check if music is currently playing.",
            "result": [
                { "name": "state", "info": "State of the music.", "kind": "boolean" }
            ]
        }
        */
        method.add_method("get_playing", |_, this, ()| unsafe {
            Ok(ffi::IsMusicStreamPlaying(this.0))
        });

        /* entry
        { "version": "1.0.0", "name": "music:stop", "info": "Stop the music." }
        */
        method.add_method("stop", |_, this, ()| unsafe {
            ffi::StopMusicStream(this.0);
            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "music:pause", "info": "Pause the music." }
        */
        method.add_method("pause", |_, this, ()| unsafe {
            ffi::PauseMusicStream(this.0);
            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "music:resume", "info": "Resume the music." }
        */
        method.add_method("resume", |_, this, ()| unsafe {
            ffi::ResumeMusicStream(this.0);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "music:set_volume",
            "info": "Set volume for the music. (range: 0.0 - 1.0)",
            "member": [
                { "name": "volume", "info": "Current volume.", "kind" : "number" }
            ]
        }
        */
        method.add_method("set_volume", |_, this, value: f32| unsafe {
            ffi::SetMusicVolume(this.0, value);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "music:set_pitch",
            "info": "Set pitch for the music.",
            "member": [
                { "name": "pitch", "info": "Current pitch.", "kind" : "number" }
            ]
        }
        */
        method.add_method("set_pitch", |_, this, value: f32| unsafe {
            ffi::SetMusicPitch(this.0, value);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "music:set_pan",
            "info": "Set pan for the music. (range: 0.0 - 1.0; 0.5 is center)",
            "member": [
                { "name": "pan", "info": "Current pan.", "kind" : "number" }
            ]
        }
        */
        method.add_method("set_pan", |_, this, value: f32| unsafe {
            ffi::SetMusicPan(this.0, value);
            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "music:update", "info": "Update the music." }
        */
        method.add_method("update", |_, this, ()| unsafe {
            ffi::UpdateMusicStream(this.0);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "music:set_position",
            "info": "Set position for the music.",
            "member": [
                { "name": "position", "info": "Current position.", "kind" : "number" }
            ]
        }
        */
        method.add_method("set_position", |_, this, value: f32| unsafe {
            ffi::SeekMusicStream(this.0, value);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "music:get_length",
            "info": "Get time length for the music.",
            "result": [
                { "name": "length", "info": "Time length.", "kind" : "number" }
            ]
        }
        */
        method.add_method("get_length", |_, this, _: ()| unsafe {
            Ok(ffi::GetMusicTimeLength(this.0))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "music:get_played",
            "info": "Get time played for the music.",
            "result": [
                { "name": "played", "info": "Time played.", "kind" : "number" }
            ]
        }
        */
        method.add_method("get_played", |_, this, _: ()| unsafe {
            Ok(ffi::GetMusicTimePlayed(this.0))
        });
    }
}
