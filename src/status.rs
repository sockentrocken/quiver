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
    Wizard(Window),
    Closure,
}

impl Status {
    pub const FONT: &'static [u8] = include_bytes!("asset/font.ttf");
    pub const LOGO: &'static [u8] = include_bytes!("asset/logo.png");
    pub const ICON: &'static [u8] = include_bytes!("asset/icon.png");

    pub fn panic(text: &str) {
        rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("Fatal Error")
            .set_description(text)
            .set_buttons(rfd::MessageButtons::Ok)
            .show();
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
                InfoResult::Missing => Self::Wizard(Window::new(handle, thread)),
            },
        }
    }

    pub fn initialize() -> (RaylibHandle, RaylibThread, RaylibAudio) {
        let (handle, thread) = raylib::init()
            .msaa_4x()
            .vsync()
            .size(1024, 768)
            .title("Quiver")
            .build();

        let audio = RaylibAudio::init_audio_device()
            .map_err(|e| Self::panic(&e.to_string()))
            .unwrap();

        (handle, thread, audio)
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
            let mut draw = handle.begin_drawing(thread);
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

        Some(Status::Closure)
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
    ) -> Option<Status> {
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);

        window.draw(&mut draw)
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
