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
use crate::window::*;

//================================================================

use raylib::prelude::*;
use serde::{Deserialize, Serialize};

//================================================================

pub enum Status {
    Missing(Window),
    Success(Script),
    Failure(Window, Option<Script>, String),
    Closure,
}

impl Status {
    pub const FONT: &'static [u8] = include_bytes!("asset/font.ttf");
    pub const LOGO: &'static [u8] = include_bytes!("asset/logo.png");
    pub const ICON: &'static [u8] = include_bytes!("asset/icon.png");

    // get a new status instance.
    #[rustfmt::skip]
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let info = Info::new();

        match info {
            // info does exist and did not fail to read, create script instance.
            Ok(info) => match Script::new(&info) {
                // script is OK, run Quiver normally.
                Ok(script)  => Self::Success(script),
                // script is  not OK, go-to failure state.
                Err(script) => Self::Failure(Window::new(handle, thread), None, script.to_string()),
            },
            Err(info) => match info {
                // info does exist, but there was an error parsing.
                InfoResult::Failure(info) => Self::Failure(Window::new(handle, thread), None, info.to_string()),
                // info does not exist.
                InfoResult::Missing => Self::Missing(Window::new(handle, thread)),
            },
        }
    }

    // create a RL context.
    pub fn window() -> (RaylibHandle, RaylibThread, RaylibAudio) {
        // create RL window, thread.
        let (mut handle, thread) = raylib::init().title("Quiver").size(1024, 768).build();

        // cap frame-rate.
        handle.set_target_fps(60);

        // create RL audio context.
        let audio = RaylibAudio::init_audio_device()
            .map_err(|e| Self::panic(&e.to_string()))
            .unwrap();

        // load default Quiver icon.
        let icon = Image::load_image_from_mem(".png", Self::ICON)
            .map_err(|e| Self::panic(&e.to_string()))
            .unwrap();
        handle.set_window_icon(icon);

        (handle, thread, audio)
    }

    // missing state, info_quiver.json does not exist.
    pub fn missing(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
    ) -> Option<Status> {
        window.missing(handle, thread)
    }

    // success state.
    pub fn success(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        script: &Script,
    ) -> Option<Status> {
        match script.main() {
            Ok(result) => {
                if result {
                    // need to do this, otherwise MAY cause an infinite hang.
                    unsafe {
                        ffi::PollInputEvents();
                    }

                    // return true, reload Quiver.
                    Some(Status::new(handle, thread))
                } else {
                    // return false, close Quiver.
                    Some(Status::Closure)
                }
            }
            // error, go to failure state.
            Err(result) => {
                handle.enable_cursor();
                unsafe {
                    ffi::EndMode3D();
                    ffi::EndMode2D();
                    ffi::EndTextureMode();
                    ffi::EndShaderMode();
                    ffi::EndBlendMode();
                    ffi::EndDrawing();
                }

                Some(Status::Failure(
                    Window::new(handle, thread),
                    Some(script.clone()),
                    result.to_string(),
                ))
            }
        }
    }

    // failure state.
    pub fn failure(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
        script: &Option<Script>,
        text: &str,
    ) -> Option<Status> {
        // a script instance is available, and a crash-handler was set in Lua.
        if let Some(script) = script {
            if script.fail.is_some() {
                match script.fail(text) {
                    Ok(result) => {
                        if result {
                            // need to do this, otherwise MAY cause an infinite hang.
                            unsafe {
                                ffi::PollInputEvents();
                            }

                            // return true, reload Quiver.
                            return Some(Status::new(handle, thread));
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
        window.failure(handle, thread, text)
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

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Info {
    pub safe: bool,
    pub path: String,
}

impl Info {
    pub const FILE_INFO: &'static str = "info_quiver.json";

    pub fn new() -> Result<Self, InfoResult> {
        // get the path to the info file.
        let data = std::path::Path::new(Self::FILE_INFO);

        // file does exist, read it.
        if data.is_file() {
            // read file.
            let file = std::fs::read_to_string(data)
                .map_err(|_| InfoResult::Failure("Info::new(): Error reading file.".to_string()))?;
            // return.
            serde_json::from_str(&file)
                .map_err(|_| InfoResult::Failure("Info::new(): Error reading file.".to_string()))
        } else {
            // file does not exist, return missing.
            Err(InfoResult::Missing)
        }
    }

    pub fn dump(&self) {
        // write the info file out as a .json.
        std::fs::write(
            Self::FILE_INFO,
            serde_json::to_string_pretty(self)
                .map_err(|e| Status::panic(&e.to_string()))
                .unwrap(),
        )
        .map_err(|e| Status::panic(&e.to_string()))
        .unwrap();
    }
}
