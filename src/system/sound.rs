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

use crate::script::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.sound", "info": "The sound API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let sound = lua.create_table()?;

    sound.set("new", lua.create_function(self::Sound::new)?)?;

    table.set("sound", sound)?;

    Ok(())
}

type RLSound = ffi::Sound;

/* class
{ "version": "1.0.0", "name": "sound", "info": "An unique handle for sound in memory." }
*/
struct Sound(RLSound);

impl Sound {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.sound.new",
        "info": "Create a new sound resource.",
        "member": [
            { "name": "path", "info": "Path to sound file.", "kind": "string" }
        ],
        "result": [
            { "name": "sound", "info": "Sound resource.", "kind": "sound" }
        ]
    }
    */
    fn new(lua: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(ScriptData::get_path(lua, &path))
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadSound(name.as_ptr());

            if ffi::IsSoundValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Sound::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadSound(self.0);
        }
    }
}

impl mlua::UserData for Sound {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        { "version": "1.0.0", "name": "sound:play", "info": "Play the sound." }
        */
        method.add_method("play", |_, this, ()| unsafe {
            ffi::PlaySound(this.0);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "sound:get_playing",
            "info": "Check if sound is currently playing.",
            "result": [
                { "name": "state", "info": "State of the sound.", "kind": "boolean" }
            ]
        }
        */
        method.add_method("get_playing", |_, this, ()| unsafe {
            Ok(ffi::IsSoundPlaying(this.0))
        });

        /* entry
        { "version": "1.0.0", "name": "sound:stop", "info": "Stop the sound." }
        */
        method.add_method("stop", |_, this, ()| unsafe {
            ffi::StopSound(this.0);
            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:pause", "info": "Pause the sound." }
        */
        method.add_method("pause", |_, this, ()| unsafe {
            ffi::PauseSound(this.0);
            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:resume", "info": "Resume the sound." }
        */
        method.add_method("resume", |_, this, ()| unsafe {
            ffi::ResumeSound(this.0);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "sound:set_volume",
            "info": "Set volume for the sound. (range: 0.0 - 1.0)",
            "member": [
                { "name": "volume", "info": "Current volume.", "kind" : "number" }
            ]
        }
        */
        method.add_method("set_volume", |_, this, value: f32| unsafe {
            ffi::SetSoundVolume(this.0, value);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "sound:set_pitch",
            "info": "Set pitch for the sound.",
            "member": [
                { "name": "pitch", "info": "Current pitch.", "kind" : "number" }
            ]
        }
        */
        method.add_method("set_pitch", |_, this, value: f32| unsafe {
            ffi::SetSoundPitch(this.0, value);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "sound:set_pan",
            "info": "Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)",
            "member": [
                { "name": "pan", "info": "Current pan.", "kind" : "number" }
            ]
        }
        */
        method.add_method("set_pan", |_, this, value: f32| unsafe {
            ffi::SetSoundPan(this.0, value);
            Ok(())
        });
    }
}
