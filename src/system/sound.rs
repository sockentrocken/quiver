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
struct Sound(RLSound, Vec<RLSound>);

impl Sound {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.sound.new",
        "info": "Create a new sound resource.",
        "member": [
            { "name": "path",  "info": "Path to sound file.",                                          "kind": "string" },
            { "name": "alias", "info": "OPTIONAL: The total sound alias count to load for the sound.", "kind": "number" }
        ],
        "result": [
            { "name": "sound", "info": "Sound resource.", "kind": "sound" }
        ]
    }
    */
    fn new(lua: &Lua, (path, alias): (String, Option<usize>)) -> mlua::Result<Self> {
        let name = CString::new(ScriptData::get_path(lua, &path)?)
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadSound(name.as_ptr());
            let alias = alias.unwrap_or_default();
            let mut array = Vec::with_capacity(alias);

            if ffi::IsSoundValid(data) {
                for _ in 0..alias {
                    let data = ffi::LoadSoundAlias(data);
                    println!("Pushing alias...");
                    array.push(data);
                }

                Ok(Self(data, array))
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
            for alias in &self.1 {
                ffi::UnloadSoundAlias(*alias);
            }

            ffi::UnloadSound(self.0);
        }
    }
}

impl mlua::UserData for Sound {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("alias_count", |_: &Lua, this| Ok(this.1.len()));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        { "version": "1.0.0", "name": "sound:create_alias", "info": "Create a sound alias." }
        */
        method.add_method_mut("create_alias", |_, this, _: ()| unsafe {
            let data = ffi::LoadSoundAlias(this.0);
            this.1.push(data);

            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:remove_alias", "info": "Remove a sound alias." }
        */
        method.add_method_mut("remove_alias", |_, this, _: ()| unsafe {
            if !this.1.is_empty() {
                if let Some(alias) = this.1.first() {
                    ffi::UnloadSoundAlias(*alias);
                    this.1.remove(0);
                }
            }

            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:remove_alias", "info": "Clear every sound alias." }
        */
        method.add_method_mut("clear_alias", |_, this, _: ()| unsafe {
            for alias in &this.1 {
                ffi::UnloadSoundAlias(*alias);
            }

            this.1.clear();

            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:play", "info": "Play the sound." }
        */
        method.add_method("play", |_, this, alias: Option<usize>| unsafe {
            if let Some(alias) = alias {
                if let Some(alias) = this.1.get(alias) {
                    ffi::PlaySound(*alias);
                } else {
                    return Err(mlua::Error::runtime("sound::play(): Invalid alias index."));
                }
            } else {
                ffi::PlaySound(this.0);
            }

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
        method.add_method("get_playing", |_, this, alias: Option<usize>| unsafe {
            if let Some(alias) = alias {
                if let Some(alias) = this.1.get(alias) {
                    Ok(ffi::IsSoundPlaying(*alias))
                } else {
                    Err(mlua::Error::runtime(
                        "sound::get_playing(): Invalid alias index.",
                    ))
                }
            } else {
                Ok(ffi::IsSoundPlaying(this.0))
            }
        });

        /* entry
        { "version": "1.0.0", "name": "sound:stop", "info": "Stop the sound." }
        */
        method.add_method("stop", |_, this, alias: Option<usize>| unsafe {
            if let Some(alias) = alias {
                if let Some(alias) = this.1.get(alias) {
                    ffi::StopSound(*alias);
                } else {
                    return Err(mlua::Error::runtime("sound::stop(): Invalid alias index."));
                }
            } else {
                ffi::StopSound(this.0);
            }

            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:pause", "info": "Pause the sound." }
        */
        method.add_method("pause", |_, this, alias: Option<usize>| unsafe {
            if let Some(alias) = alias {
                if let Some(alias) = this.1.get(alias) {
                    ffi::PauseSound(*alias);
                } else {
                    return Err(mlua::Error::runtime("sound::pause(): Invalid alias index."));
                }
            } else {
                ffi::PauseSound(this.0);
            }

            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:resume", "info": "Resume the sound." }
        */
        method.add_method("resume", |_, this, alias: Option<usize>| unsafe {
            if let Some(alias) = alias {
                if let Some(alias) = this.1.get(alias) {
                    ffi::ResumeSound(*alias);
                } else {
                    return Err(mlua::Error::runtime(
                        "sound::resume(): Invalid alias index.",
                    ));
                }
            } else {
                ffi::ResumeSound(this.0);
            }

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
        method.add_method(
            "set_volume",
            |_, this, (value, alias): (f32, Option<usize>)| unsafe {
                if let Some(alias) = alias {
                    if let Some(alias) = this.1.get(alias) {
                        ffi::SetSoundVolume(*alias, value);
                    } else {
                        return Err(mlua::Error::runtime(
                            "sound::set_volume(): Invalid alias index.",
                        ));
                    }
                } else {
                    ffi::SetSoundVolume(this.0, value);
                }

                Ok(())
            },
        );

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
        method.add_method(
            "set_pitch",
            |_, this, (value, alias): (f32, Option<usize>)| unsafe {
                if let Some(alias) = alias {
                    if let Some(alias) = this.1.get(alias) {
                        ffi::SetSoundPitch(*alias, value);
                    } else {
                        return Err(mlua::Error::runtime(
                            "sound::set_pitch(): Invalid alias index.",
                        ));
                    }
                } else {
                    ffi::SetSoundPitch(this.0, value);
                }

                Ok(())
            },
        );

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
        method.add_method(
            "set_pan",
            |_, this, (value, alias): (f32, Option<usize>)| unsafe {
                if let Some(alias) = alias {
                    if let Some(alias) = this.1.get(alias) {
                        ffi::SetSoundPan(*alias, value);
                    } else {
                        return Err(mlua::Error::runtime(
                            "sound::set_pan(): Invalid alias index.",
                        ));
                    }
                } else {
                    ffi::SetSoundPan(this.0, value);
                }

                Ok(())
            },
        );
    }
}
