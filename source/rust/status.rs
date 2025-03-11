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
use crate::window::*;

//================================================================

use raylib::prelude::*;

#[cfg(feature = "embed")]
use rust_embed::Embed;

use serde::{Deserialize, Serialize};

//================================================================

#[cfg(feature = "embed")]
#[derive(Embed)]
#[folder = "embed"]
#[allow_missing = true]
pub struct Asset;

pub enum Status {
    Missing,
    Success(Script),
    Failure(Option<Script>, String),
    Closure,
}

impl Status {
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const FONT: &'static [u8] = include_bytes!("../../data/font.ttf");
    pub const LOGO: &'static [u8] = include_bytes!("../../data/logo.png");
    pub const ICON: &'static [u8] = include_bytes!("../../data/icon.png");

    // get a new status instance.
    #[rustfmt::skip]
    pub async fn new() -> Self {
        let info = StatusInfo::new();

        match info {
            // info does exist and did not fail to read, create script instance.
            Ok(info) => match Script::new(&info).await {
                // script is OK, run Quiver normally.
                Ok(script)  => {
                    println!("//================================================================");
                    println!("// Quiver ({})", Self::VERSION);
                    println!("//");
                    println!("// -> StatusInfo manifest:");
                    println!("//   * Safe: {}", info.safe);
                    println!("//   * Path: {}", info.path);
                    println!("//");
                    println!("// -> Feature list:");

                    #[cfg(feature = "serialization")]
                    println!("//   * YAML/TOML/XML/INI serialization/deserialization");

                    #[cfg(feature = "system_info")]
                    println!("//   * System info");

                    #[cfg(feature = "file_notify")]
                    println!("//   * File notify");

                    #[cfg(feature = "rapier3d")]
                    println!("//   * Rapier3D");

                    #[cfg(feature = "rapier2d")]
                    println!("//   * Rapier2D");

                    #[cfg(feature = "zip")]
                    println!("//   * ZIP");

                    #[cfg(feature = "request")]
                    println!("//   * HTTP request");

                    #[cfg(feature = "steam")]
                    println!("//   * Steam");

                    #[cfg(feature = "discord")]
                    println!("//   * Discord");

                    #[cfg(feature = "embed")]
                    println!("//   * File embed");

                    #[cfg(feature = "video")]
                    println!("//   * Video");

                    println!("//================================================================");

                    Self::Success(script)
                },
                // script is  not OK, go-to failure state.
                Err(script) => Self::Failure(None, script.to_string()),
            },
            Err(info) => match info {
                // info does exist, but there was an error parsing.
                InfoResult::Failure(info) => Self::Failure(None, info.to_string()),
                // info does not exist.
                InfoResult::Missing => Self::Missing,
            },
        }
    }

    // create a RL context.
    pub async fn window(&self) -> Option<(RaylibHandle, RaylibThread, RaylibAudio)> {
        let info = match self {
            Self::Success(script) => &script.info,
            _ => &ScriptInfo::default(),
        };

        let mut flag: u32 = 0;

        if info.sync {
            flag |= ffi::ConfigFlags::FLAG_VSYNC_HINT as u32;
        }
        if info.msaa {
            flag |= ffi::ConfigFlags::FLAG_MSAA_4X_HINT as u32;
        }
        if info.scale {
            flag |= ffi::ConfigFlags::FLAG_WINDOW_HIGHDPI as u32;
        }

        unsafe {
            ffi::SetConfigFlags(flag);
        }

        // create RL window, thread.
        let (mut handle, thread) = raylib::init()
            .title(&info.name)
            .size(info.size.0, info.size.1)
            .build();

        let state = handle
            .get_window_state()
            .set_fullscreen_mode(info.full)
            .set_window_resizable(info.resizable)
            .set_window_undecorated(info.no_decor)
            .set_window_hidden(info.hidden)
            .set_window_minimized(info.minimize)
            .set_window_maximized(info.maximize)
            .set_window_unfocused(info.no_focus)
            .set_window_topmost(info.always_top)
            .set_window_always_run(info.always_run)
            .set_window_transparent(info.alpha)
            .set_interlaced_hint(info.interlace);

        handle.set_window_state(state);

        unsafe {
            if info.no_border {
                ffi::SetWindowState(ffi::ConfigFlags::FLAG_BORDERLESS_WINDOWED_MODE as u32);
            }
            if info.mouse_pass {
                ffi::SetWindowState(ffi::ConfigFlags::FLAG_WINDOW_MOUSE_PASSTHROUGH as u32);
            }
        }

        // cap frame-rate.
        handle.set_target_fps(info.rate);

        // create RL audio context.
        let audio = RaylibAudio::init_audio_device()
            .map_err(|e| Self::panic(&e.to_string()))
            .unwrap();

        if let Some(icon) = &info.icon {
            if !icon.is_empty() {
                // load icon from info manifest.
                let icon = Image::load_image(icon)
                    .map_err(|e| Self::panic(&e.to_string()))
                    .unwrap();
                handle.set_window_icon(icon);
            }
        } else {
            // load default Quiver icon.
            let icon = Image::load_image_from_mem(".png", Self::ICON)
                .map_err(|e| Self::panic(&e.to_string()))
                .unwrap();
            handle.set_window_icon(icon);
        }

        Some((handle, thread, audio))
    }

    // missing state, info.json does not exist.
    pub async fn missing(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
    ) -> Option<Status> {
        window.missing(handle, thread).await
    }

    // success state.
    pub async fn success(
        context: &Option<(RaylibHandle, RaylibThread, RaylibAudio)>,
        script: &Script,
    ) -> Option<Status> {
        match script.main().await {
            Ok(result) => {
                if result {
                    if context.is_some() {
                        // need to do this, otherwise MAY cause an infinite hang.
                        unsafe {
                            ffi::PollInputEvents();
                        }
                    }

                    // return true, reload Quiver.
                    Some(Status::new().await)
                } else {
                    // return false, close Quiver.
                    Some(Status::Closure)
                }
            }
            // error, go to failure state.
            Err(result) => {
                if context.is_some() {
                    unsafe {
                        ffi::EnableCursor();
                        ffi::EndMode3D();
                        ffi::EndMode2D();
                        ffi::EndTextureMode();
                        ffi::EndShaderMode();
                        ffi::EndBlendMode();
                        ffi::EndScissorMode();
                        ffi::EndDrawing();
                        ffi::SetMouseOffset(0, 0);
                        ffi::SetMouseScale(1.0, 1.0);
                    }
                }

                Some(Status::Failure(Some(script.clone()), result.to_string()))
            }
        }
    }

    // failure state.
    pub async fn failure(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
        script: &Option<Script>,
        text: &str,
    ) -> Option<Status> {
        // a script instance is available, and a crash-handler was set in Lua.
        if let Some(script) = script {
            if script.fail.is_some() {
                match script.fail(text).await {
                    Ok(result) => {
                        if result {
                            // need to do this, otherwise MAY cause an infinite hang.
                            unsafe {
                                ffi::PollInputEvents();
                            }

                            // return true, reload Quiver.
                            return Some(Status::new().await);
                        } else {
                            // return false, close Quiver.
                            return Some(Status::Closure);
                        }
                    }
                    // an error in the crash-handler...just panic to avoid causing an infinite loop.
                    Err(result) => {
                        Status::panic(&result);
                        return None;
                    }
                }
            }
        }

        // no script instance is available, or a custom crash-handler has not been set.
        window.failure(handle, thread, text).await
    }

    // panic window, useful for when no RL context is available to display an error.
    pub fn panic(text: &str) {
        rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("Fatal Error")
            .set_description(text)
            .set_buttons(rfd::MessageButtons::Ok)
            .show();
        panic!("{}", text);
    }
}

//================================================================

#[derive(Debug)]
pub enum InfoResult {
    Failure(String),
    Missing,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct StatusInfo {
    pub safe: bool,
    pub path: String,
}

impl StatusInfo {
    pub const FILE_: &'static str = "info.json";
    pub const MAIN_PATH: &'static str = "main";
    pub const MAIN_FILE: &'static str = "main.lua";

    pub fn new() -> Result<Self, InfoResult> {
        let mut result: Option<StatusInfo> = None;

        //================================================================

        // get the path to the main.lua file.
        let main_file = std::path::Path::new(Self::MAIN_FILE);

        if main_file.is_file() {
            result = Some(Self {
                safe: true,
                path: ".".to_string(),
            });
        }

        //================================================================

        let main_path = std::path::Path::new(Self::MAIN_PATH);

        if main_path.is_dir() {
            result = Some(Self {
                safe: true,
                path: Self::MAIN_PATH.to_string(),
            });
        }

        //================================================================

        #[cfg(feature = "embed")]
        {
            let embed_file = Asset::get("main.lua");

            if embed_file.is_some() {
                result = Some(Self {
                    safe: true,
                    path: ".".to_string(),
                });
            }
        }

        //================================================================

        // get the path to the info file.
        let data = std::path::Path::new(Self::FILE_);

        // file does exist, read it.
        if data.is_file() {
            // read file.
            let file = std::fs::read_to_string(data).map_err(|_| {
                InfoResult::Failure("StatusInfo::new(): Error reading file.".to_string())
            })?;
            // return.
            let mut info: Self = serde_json::from_str(&file).map_err(|_| {
                InfoResult::Failure("StatusInfo::new(): Error reading file.".to_string())
            })?;

            info.path = info.path.to_string();

            result = Some(info);
        }

        //================================================================

        let mut argument_pick = false;
        let mut argument_ = StatusInfo {
            safe: true,
            path: ".".to_string(),
        };
        let mut argument_list = std::env::args();

        while let Some(argument) = argument_list.next() {
            match &*argument {
                "--no-safe" => {
                    argument_.safe = false;
                    argument_pick = true;
                }
                "--path" => {
                    if let Some(next) = argument_list.next() {
                        argument_.path = next;
                    } else {
                        eprintln!("ERROR: Was expecting argument for --path.")
                    }

                    argument_pick = true;
                }
                _ => {}
            }
        }

        if argument_pick {
            result = Some(argument_);
        }

        //================================================================

        // file does not exist, return missing.
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(InfoResult::Missing)
        }
    }

    pub fn dump(&self) {
        // write the info file out as a .json.
        std::fs::write(
            Self::FILE_,
            serde_json::to_string_pretty(self)
                .map_err(|e| Status::panic(&e.to_string()))
                .unwrap(),
        )
        .map_err(|e| Status::panic(&e.to_string()))
        .unwrap();
    }
}
