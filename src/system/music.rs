use crate::script::*;

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

type RLMusic = ffi::Music;

//================================================================

/* class
{ "name": "quiver.music", "info": "The music API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, system: &ModuleSystem) -> mlua::Result<()> {
    let music = lua.create_table()?;

    if system.music { music.set("new", lua.create_function(self::Music::new)?)?; }

    table.set("music", music)?;

    Ok(())
}

/* class
{ "name": "music", "info": "An unique handle for music in memory." }
*/
pub struct Music(RLMusic);

impl Music {
    /* function
    {
        "name": "quiver.music.new",
        "info": "Create a new music resource.",
        "parameter": [
            { "optional": false, "name": "path", "info": "Path to music file.", "type": "string" }
        ],
        "return": [
            { "optional": false, "name": "music", "info": "Music resource.", "type": "music" }
        ]
    }
    */
    pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadMusicStream(name.as_ptr());

            if ffi::IsMusicReady(data) {
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
        /* function
        { "name": "music:play", "info": "Play the music." }
        */
        method.add_method("play", |_, this, ()| unsafe {
            ffi::PlayMusicStream(this.0);
            Ok(())
        });

        /* function
        {
            "name": "music:get_playing",
            "info": "Check if music is currently playing.",
            "return": [
                { "optional": false, "name": "state", "info": "State of the music.", "type": "boolean" }
            ]
        }
        */
        method.add_method("get_playing", |_, this, ()| unsafe {
            Ok(ffi::IsMusicStreamPlaying(this.0))
        });

        /* function
        { "name": "music:stop", "info": "Stop the music." }
        */
        method.add_method("stop", |_, this, ()| unsafe {
            ffi::StopMusicStream(this.0);
            Ok(())
        });

        /* function
        { "name": "music:pause", "info": "Pause the music." }
        */
        method.add_method("pause", |_, this, ()| unsafe {
            ffi::PauseMusicStream(this.0);
            Ok(())
        });

        /* function
        { "name": "music:resume", "info": "Resume the music." }
        */
        method.add_method("resume", |_, this, ()| unsafe {
            ffi::ResumeMusicStream(this.0);
            Ok(())
        });

        /* function
        {
            "name": "music:set_volume",
            "info": "Set volume for the music. (range: 0.0 - 1.0)",
            "parameter": [
                { "optional": false, "name": "volume", "info": "Current volume.", "type" : "number" }
            ]
        }
        */
        method.add_method("set_volume", |_, this, value: f32| unsafe {
            ffi::SetMusicVolume(this.0, value);
            Ok(())
        });

        /* function
        {
            "name": "music:set_pitch",
            "info": "Set pitch for the music.",
            "parameter": [
                { "optional": false, "name": "pitch", "info": "Current pitch.", "type" : "number" }
            ]
        }
        */
        method.add_method("set_pitch", |_, this, value: f32| unsafe {
            ffi::SetMusicPitch(this.0, value);
            Ok(())
        });

        /* function
        {
            "name": "music:set_pan",
            "info": "Set pan for the music. (range: 0.0 - 1.0; 0.5 is center)",
            "parameter": [
                { "optional": false, "name": "pan", "info": "Current pan.", "type" : "number" }
            ]
        }
        */
        method.add_method("set_pan", |_, this, value: f32| unsafe {
            ffi::SetMusicPan(this.0, value);
            Ok(())
        });

        /* function
        { "name": "music:update", "info": "Update the music." }
        */
        method.add_method("update", |_, this, ()| unsafe {
            ffi::UpdateMusicStream(this.0);
            Ok(())
        });

        /* function
        {
            "name": "music:set_position",
            "info": "Set position for the music.",
            "parameter": [
                { "optional": false, "name": "position", "info": "Current position.", "type" : "number" }
            ]
        }
        */
        method.add_method("set_position", |_, this, value: f32| unsafe {
            ffi::SeekMusicStream(this.0, value);
            Ok(())
        });

        /* function
        {
            "name": "music:get_length",
            "info": "Get time length for the music.",
            "return": [
                { "optional": false, "name": "length", "info": "Time length.", "type" : "number" }
            ]
        }
        */
        method.add_method("get_length", |_, this, _: ()| unsafe {
            Ok(ffi::GetMusicTimeLength(this.0))
        });

        /* function
        {
            "name": "music:get_played",
            "info": "Get time played for the music.",
            "return": [
                { "optional": false, "name": "played", "info": "Time played.", "type" : "number" }
            ]
        }
        */
        method.add_method("get_played", |_, this, _: ()| unsafe {
            Ok(ffi::GetMusicTimePlayed(this.0))
        });
    }
}
