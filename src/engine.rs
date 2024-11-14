/*
* ================================================================
* engine.rs
* ================================================================
*/

use crate::script::*;
use crate::status::*;
use crate::support::*;
use crate::window::*;

//================================================================

use raylib::prelude::*;
use serde::{Deserialize, Serialize};

//================================================================

#[derive(Default)]
pub struct Engine {
    pub info: InfoEngine,
    pub status: StatusPointer,
    pub window: WindowPointer,
    pub script: Script,
}

impl Engine {
    pub const BUILD: i32 = 1;
    pub const LOGO: [&'static [u8]; 3] = [
        include_bytes!("asset/logo_512.png"),
        include_bytes!("asset/logo_256.png"),
        include_bytes!("asset/logo_128.png"),
    ];

    pub fn new() -> Self {
        let mut engine = Engine::default();

        match InfoEngine::new() {
            Ok(result) => engine.info = result,
            Err(result) => match result {
                InfoResult::Failure(error) => {
                    Status::set_failure(&engine, error);

                    return engine;
                }
                InfoResult::Missing => {
                    Status::set_wizard(&engine);

                    return engine;
                }
            },
        }

        match Script::new(&engine.info, engine.status.clone(), engine.window.clone()) {
            Ok(result) => {
                engine.script = result;

                engine
            }
            Err(result) => {
                Status::set_failure(&engine, result.to_string());

                engine
            }
        }
    }

    pub fn window(
        &mut self,
    ) -> Result<(RaylibHandle, RaylibThread, RaylibAudio, RaylibImguiSupport), String> {
        let window = &self.script.window;

        let (mut handle, thread) = raylib::init()
            .title(&window.name)
            .size(window.shape.0, window.shape.1)
            .build();

        handle.set_window_state(
            WindowState::default()
                .set_fullscreen_mode(window.fullscreen)
                .set_vsync_hint(window.sync)
                .set_msaa(window.msaa)
                .set_window_resizable(window.resize)
                .set_window_hidden(window.hidden)
                .set_window_minimized(window.minimize)
                .set_window_maximized(window.maximize)
                .set_window_undecorated(window.no_decor)
                .set_window_unfocused(window.no_focus)
                .set_window_topmost(window.on_front)
                .set_window_always_run(window.run_hidden)
                .set_window_transparent(window.draw_alpha)
                .set_window_highdpi(window.high_scale),
        );

        handle.set_target_fps(window.rate);

        let mut list: Vec<ffi::Image> = Vec::new();

        if let Some(icon_list) = &window.icon {
            for icon in icon_list {
                match Image::load_image(icon) {
                    Ok(icon) => {
                        list.push(unsafe { icon.unwrap() });
                    }
                    Err(error) => {
                        Status::set_failure(self, error.to_string());
                    }
                }
            }
        } else {
            for icon in Self::LOGO {
                match Image::load_image_from_mem(".png", icon) {
                    Ok(icon) => {
                        list.push(unsafe { icon.unwrap() });
                    }
                    Err(error) => {
                        Status::set_failure(self, error.to_string());
                    }
                }
            }
        }

        handle.set_window_icons(&mut list);

        if window.borderless {
            handle.toggle_borderless_windowed();
        }

        if let Some(point) = window.point {
            handle.set_window_position(point.0, point.1);
        }

        if let Some(shape) = window.shape_min {
            handle.set_window_min_size(shape.0, shape.1);
        }

        if let Some(shape) = window.shape_max {
            handle.set_window_max_size(shape.0, shape.1);
        }

        handle.set_window_opacity(window.alpha);

        let audio = RaylibAudio::init_audio_device().map_err(|e| e.to_string())?;

        let interface = RaylibImguiSupport::setup(&mut handle, &thread);

        Ok((handle, thread, audio, interface))
    }
}

//================================================================

pub enum InfoResult {
    Failure(String),
    Missing,
}

#[derive(Default, Deserialize, Serialize)]
pub struct InfoEngine {
    pub safe: bool,
    pub path: Vec<String>,
}

impl InfoEngine {
    pub const FILE_NAME: &'static str = "info.json";

    pub fn new() -> Result<Self, InfoResult> {
        let data = std::path::Path::new(Self::FILE_NAME);

        if data.is_file() {
            if let Ok(file) = std::fs::read_to_string(data) {
                if let Ok(info) = serde_json::from_str(&file) {
                    Ok(info)
                } else {
                    Err(InfoResult::Failure(
                        "InfoEngine::new(): Error reading JSON file.".to_string(),
                    ))
                }
            } else {
                Err(InfoResult::Failure(
                    "InfoEngine::new(): Error reading file.".to_string(),
                ))
            }
        } else {
            Err(InfoResult::Missing)
        }
    }
}
