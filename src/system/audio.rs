use crate::script::*;
use crate::status::*;
use crate::window::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::sync::Mutex;

type RLSound = ffi::Sound;
type RLMusic = ffi::Music;

//================================================================

/* meta
---@class sound
local sound = {}
*/
pub struct Sound(RLSound);

impl mlua::UserData for Sound {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* meta
        ---Play sound.
        function sound:play() end
        */
        method.add_method("play", |_, this, ()| unsafe {
            ffi::PlaySound(this.0);
            Ok(())
        });

        /* meta
        ---Get if sound is playing.
        function sound:get_playing() end
        */
        method.add_method("get_playing", |_, this, ()| unsafe {
            ffi::IsSoundPlaying(this.0);
            Ok(())
        });

        /* meta
        ---Stop sound.
        function sound:stop() end
        */
        method.add_method("stop", |_, this, ()| unsafe {
            ffi::StopSound(this.0);
            Ok(())
        });

        /* meta
        ---Pause sound.
        function sound:pause() end
        */
        method.add_method("pause", |_, this, ()| unsafe {
            ffi::PauseSound(this.0);
            Ok(())
        });

        /* meta
        ---Resume sound.
        function sound:resume() end
        */
        method.add_method("resume", |_, this, ()| unsafe {
            ffi::ResumeSound(this.0);
            Ok(())
        });

        /* meta
        ---Set volume for sound. (range: 0.0 - 1.0)
        ---@param value number The volume for the sound.
        function sound:volume(value) end
        */
        method.add_method("volume", |_, this, value: f32| unsafe {
            ffi::SetSoundVolume(this.0, value);
            Ok(())
        });

        /* meta
        ---Set pitch for sound.
        ---@param value number The pitch for the sound.
        function sound:pitch(value) end
        */
        method.add_method("pitch", |_, this, value: f32| unsafe {
            ffi::SetSoundPitch(this.0, value);
            Ok(())
        });

        /* meta
        ---Set pan for sound. (range: 0.0 - 1.0; 0.5 is center)
        ---@param value number The pan for the sound.
        function sound:pan(value) end
        */
        method.add_method("pan", |_, this, value: f32| unsafe {
            ffi::SetSoundPan(this.0, value);
            Ok(())
        });
    }
}

impl Sound {
    /* meta
    ---An unique handle for sound in memory.
    ---@param path string Path to file.
    ---@return sound # The user-data object.
    function Sound(path) end
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

//================================================================

/* meta
---@class music
local music = {}
*/
pub struct Music(RLMusic);

impl mlua::UserData for Music {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* meta
        ---Play music.
        function music:play() end
        */
        method.add_method("play", |_, this, ()| unsafe {
            ffi::PlayMusicStream(this.0);
            Ok(())
        });

        /* meta
        ---Get if music is playing.
        function music:get_playing() end
        */
        method.add_method("get_playing", |_, this, ()| unsafe {
            ffi::IsMusicStreamPlaying(this.0);
            Ok(())
        });

        /* meta
        ---Stop music.
        function music:stop() end
        */
        method.add_method("stop", |_, this, ()| unsafe {
            ffi::StopMusicStream(this.0);
            Ok(())
        });

        /* meta
        ---Pause music.
        function music:pause() end
        */
        method.add_method("pause", |_, this, ()| unsafe {
            ffi::PauseMusicStream(this.0);
            Ok(())
        });

        /* meta
        ---Resume music.
        function music:resume() end
        */
        method.add_method("resume", |_, this, ()| unsafe {
            ffi::ResumeMusicStream(this.0);
            Ok(())
        });

        /* meta
        ---Set volume for music. (range: 0.0 - 1.0)
        ---@param value number The volume for the music.
        function music:volume(value) end
        */
        method.add_method("volume", |_, this, value: f32| unsafe {
            ffi::SetMusicVolume(this.0, value);
            Ok(())
        });

        /* meta
        ---Set pitch for music.
        ---@param value number The pitch for the music.
        function music:pitch(value) end
        */
        method.add_method("pitch", |_, this, value: f32| unsafe {
            ffi::SetMusicPitch(this.0, value);
            Ok(())
        });

        /* meta
        ---Set pan for music. (range: 0.0 - 1.0; 0.5 is center)
        ---@param value number The pan for the music.
        function music:pan(value) end
        */
        method.add_method("pan", |_, this, value: f32| unsafe {
            ffi::SetMusicPan(this.0, value);
            Ok(())
        });

        /* meta
        ---Update music stream.
        function music:update() end
        */
        method.add_method("update", |_, this, ()| unsafe {
            ffi::UpdateMusicStream(this.0);
            Ok(())
        });

        /* meta
        ---Set position for music.
        ---@param value number The position for the music.
        function music:seek() end
        */
        method.add_method("seek", |_, this, value: f32| unsafe {
            ffi::SeekMusicStream(this.0, value);
            Ok(())
        });

        /* meta
        ---Get time length of music.
        ---@return number # The length of the music.
        function music:length() end
        */
        method.add_method("length", |_, this, _: ()| unsafe {
            ffi::GetMusicTimeLength(this.0);
            Ok(())
        });

        /* meta
        ---Get time played of music.
        ---@return number # The time played of the music.
        function music:played() end
        */
        method.add_method("played", |_, this, _: ()| unsafe {
            ffi::GetMusicTimePlayed(this.0);
            Ok(())
        });
    }
}

impl Music {
    /* meta
    ---An unique handle for music in memory.
    ---@param path string Path to file.
    ---@return music # The user-data object.
    function Music(path) end
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

//================================================================

#[rustfmt::skip]
    pub fn set_global(lua: &Lua, global: &mlua::Table, system: &ModuleSystem) -> mlua::Result<()> {
        if system.sound { global.set("Sound", lua.create_function(self::Sound::new)?)?; }
        if system.music { global.set("Music", lua.create_function(self::Music::new)?)?; }

        Ok(())
    }
