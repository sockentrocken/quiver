use crate::script::*;
use crate::window::*;
use crate::wizard::*;

//================================================================

use raylib::prelude::*;
use serde::{Deserialize, Serialize};

//================================================================

pub enum Status {
    Success(Script),
    Failure(Window, String),
    Wizard(Window, Wizard),
    Restart,
    Closure,
}

impl Status {
    pub const FONT: &'static [u8] = include_bytes!("asset/font.ttf");
    pub const LOGO: &'static [u8] = include_bytes!("asset/logo.png");
    pub const ICON: [&'static [u8]; 3] = [
        include_bytes!("asset/icon_128.png"),
        include_bytes!("asset/icon_256.png"),
        include_bytes!("asset/icon_512.png"),
    ];

    pub fn initialize() -> (RaylibHandle, RaylibThread, RaylibAudio) {
        let window = ModuleWindow::default();

        let (mut handle, thread) = raylib::init()
            .title(&window.name)
            .size(window.shape.0, window.shape.1)
            .msaa_4x()
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

        //handle.set_target_fps(window.rate);
        handle.set_target_fps(144);

        let mut list: Vec<ffi::Image> = Vec::new();

        if let Some(icon_list) = &window.icon {
            for icon in icon_list {
                match Image::load_image(icon) {
                    Ok(icon) => {
                        list.push(unsafe { icon.unwrap() });
                    }
                    Err(error) => {
                        //Status::set_failure(self, error.to_string());
                    }
                }
            }
        } else {
            for icon in Self::ICON {
                match Image::load_image_from_mem(".png", icon) {
                    Ok(icon) => {
                        list.push(unsafe { icon.unwrap() });
                    }
                    Err(error) => {
                        //Status::set_failure(self, error.to_string());
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

        let audio = RaylibAudio::init_audio_device()
            /*.map_err(|e| panic_window(&e.to_string()))*/
            .unwrap();

        (handle, thread, audio)
    }

    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let info = InfoEngine::new();

        match info {
            Ok(info) => match Script::new(&info) {
                Ok(script) => Self::Success(script),
                Err(script) => Self::Failure(Window::new(handle, thread), script.to_string()),
            },
            Err(info) => match info {
                InfoResult::Failure(info) => {
                    Self::Failure(Window::new(handle, thread), info.to_string())
                }
                InfoResult::Missing => Self::Wizard(Window::new(handle, thread), Wizard::default()),
            },
        }
    }

    pub fn success(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        script: &mut Script,
    ) -> Option<Status> {
        if let Err(error) = script.main() {
            return Some(Status::Failure(
                Window::new(handle, thread),
                error.to_string(),
            ));
        }

        let mut loop_error: Option<String> = None;

        while !handle.window_should_close() {
            let mut draw = handle.begin_drawing(&thread);
            draw.clear_background(Color::WHITE);

            if let Err(error) = script.step() {
                loop_error = Some(error.to_string());
                break;
            }
        }

        if let Some(error) = loop_error {
            return Some(Status::Failure(Window::new(handle, thread), error));
        }

        if let Err(error) = script.exit() {
            return Some(Status::Failure(
                Window::new(handle, thread),
                error.to_string(),
            ));
        }

        None
    }

    pub fn failure(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
        text: &str,
    ) -> Option<Status> {
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);

        window.begin();

        let size = draw.get_screen_width();

        window.card_sharp(
            &mut draw,
            Rectangle::new(0.0, 0.0, size as f32, 48.0),
            Window::COLOR_MAIN,
        );

        window.point(Vector2::new(20.0, 12.0));
        window.text(&mut draw, "Fatal Error", Window::COLOR_TEXT);

        window.card_round(
            &mut draw,
            Rectangle::new(20.0, 72.0, size as f32 - 36.0, 128.0),
            Window::COLOR_MAIN,
        );

        window.point(Vector2::new(36.0, 84.0));
        window.text(&mut draw, text, Window::COLOR_TEXT_MAIN);

        window.point(Vector2::new(20.0, (draw.get_screen_height() - 96) as f32));
        window.button(&mut draw, "Load Module");
        window.button(&mut draw, "Exit Quiver");

        None
    }

    #[rustfmt::skip]
    pub fn wizard(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
        wizard: &mut Wizard,
    ) -> Option<Status> {
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);

        wizard.draw(&mut draw, window)
    }
}

#[derive(Debug)]
pub enum InfoResult {
    Failure(String),
    Missing,
}

#[derive(Default, Deserialize, Serialize)]
pub struct InfoEngine {
    pub safe: bool,
    pub path: String,
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

    pub fn dump(&self, path: &str) -> Result<(), String> {
        /*
        crate::utility::file::write(
            &format!("{}{}", path, InfoEngine::FILE_NAME),
            serde_json::to_string(self).map_err(|e| e.to_string())?,
        )?;
        */

        Ok(())
    }
}
