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
{ "version": "1.0.0", "name": "quiver.music", "info": "The music API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let music = lua.create_table()?;

    music.set("new", lua.create_function(self::Music::new)?)?;

    table.set("music", music)?;

    Ok(())
}

type RLMusic = ffi::Music;

/* class
{ "version": "1.0.0", "name": "music", "info": "An unique handle for music in memory." }
*/
struct Music(RLMusic);

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
    fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadMusicStream(name.as_ptr());

            if ffi::IsMusicValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Music::new(): Could not load file \"{path}\"."
                )))
            }
        }
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
