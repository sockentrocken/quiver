use crate::module::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

type RLSound = ffi::Sound;

//================================================================

/* class
{ "name": "quiver.sound", "info": "The sound API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, system: &ModuleSystem) -> mlua::Result<()> {
    let sound = lua.create_table()?;

    if system.sound { sound.set("new", lua.create_function(self::Sound::new)?)?; }

    table.set("sound", sound)?;

    Ok(())
}

/* class
{ "name": "sound", "info": "An unique handle for sound in memory." }
*/
pub struct Sound(RLSound);

impl Sound {
    /* function
    {
        "name": "quiver.sound.new",
        "info": "Create a new sound resource.",
        "parameter": [
            { "optional": false, "name": "path", "info": "Path to sound file.", "type": "string" }
        ],
        "return": [
            { "optional": false, "name": "sound", "info": "Sound resource.", "type": "sound" }
        ]
    }
    */
    pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadSound(name.as_ptr());

            if ffi::IsSoundReady(data) {
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
        /* function
        { "name": "sound:play", "info": "Play the sound." }
        */
        method.add_method("play", |_, this, ()| unsafe {
            ffi::PlaySound(this.0);
            Ok(())
        });

        /* function
        {
            "name": "sound:get_playing",
            "info": "Check if sound is currently playing.",
            "return": [
                { "optional": false, "name": "state", "info": "State of the sound.", "type": "boolean" }
            ]
        }
        */
        method.add_method("get_playing", |_, this, ()| unsafe {
            Ok(ffi::IsSoundPlaying(this.0))
        });

        /* function
        { "name": "sound:stop", "info": "Stop the sound." }
        */
        method.add_method("stop", |_, this, ()| unsafe {
            ffi::StopSound(this.0);
            Ok(())
        });

        /* function
        { "name": "sound:pause", "info": "Pause the sound." }
        */
        method.add_method("pause", |_, this, ()| unsafe {
            ffi::PauseSound(this.0);
            Ok(())
        });

        /* function
        { "name": "sound:resume", "info": "Resume the sound." }
        */
        method.add_method("resume", |_, this, ()| unsafe {
            ffi::ResumeSound(this.0);
            Ok(())
        });

        /* function
        {
            "name": "sound:set_volume",
            "info": "Set volume for the sound. (range: 0.0 - 1.0)",
            "parameter": [
                { "optional": false, "name": "volume", "info": "Current volume.", "type" : "number" }
            ]
        }
        */
        method.add_method("set_volume", |_, this, value: f32| unsafe {
            ffi::SetSoundVolume(this.0, value);
            Ok(())
        });

        /* function
        {
            "name": "sound:set_pitch",
            "info": "Set pitch for the sound.",
            "parameter": [
                { "optional": false, "name": "pitch", "info": "Current pitch.", "type" : "number" }
            ]
        }
        */
        method.add_method("set_pitch", |_, this, value: f32| unsafe {
            ffi::SetSoundPitch(this.0, value);
            Ok(())
        });

        /* function
        {
            "name": "sound:set_pan",
            "info": "Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)",
            "parameter": [
                { "optional": false, "name": "pan", "info": "Current pan.", "type" : "number" }
            ]
        }
        */
        method.add_method("set_pan", |_, this, value: f32| unsafe {
            ffi::SetSoundPan(this.0, value);
            Ok(())
        });
    }
}
