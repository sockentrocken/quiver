use crate::script::*;
use crate::window::*;

//================================================================

use raylib::prelude::*;
use serde::{Deserialize, Serialize};

//================================================================

pub enum Status {
    Success(Script),
    Failure(Window, Option<Script>, String),
    Missing(Window),
    Closure,
}

impl Status {
    pub const FONT: &'static [u8] = include_bytes!("asset/font.ttf");
    pub const LOGO: &'static [u8] = include_bytes!("asset/logo.png");
    pub const ICON: &'static [u8] = include_bytes!("asset/icon.png");

    #[rustfmt::skip]
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let info = Info::new();

        match info {
            Ok(info) => match Script::new(&info) {
                Ok(script)  => Self::Success(script),
                Err(script) => Self::Failure(Window::new(handle, thread), None, script.to_string()),
            },
            Err(info) => match info {
                InfoResult::Failure(info) => Self::Failure(Window::new(handle, thread), None, info.to_string()),
                InfoResult::Missing       => Self::Missing(Window::new(handle, thread)),
            },
        }
    }

    pub fn initialize() -> (RaylibHandle, RaylibThread, RaylibAudio) {
        let (mut handle, thread) = raylib::init()
            .msaa_4x()
            .vsync()
            .size(1024, 768)
            .title("Quiver")
            .build();

        let icon = Image::load_image_from_mem(".png", Self::ICON)
            .map_err(|e| Self::panic(&e.to_string()))
            .unwrap();

        handle.set_window_icon(icon);

        let audio = RaylibAudio::init_audio_device()
            .map_err(|e| Self::panic(&e.to_string()))
            .unwrap();

        (handle, thread, audio)
    }

    pub fn success(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        script: &Script,
    ) -> Option<Status> {
        if let Err(error) = script.main() {
            return Some(Status::Failure(
                Window::new(handle, thread),
                Some(script.clone()),
                error.to_string(),
            ));
        }

        while !handle.window_should_close() {
            let mut draw = handle.begin_drawing(thread);
            draw.clear_background(Color::WHITE);

            if let Err(error) = script.step() {
                drop(draw);
                return Some(Status::Failure(
                    Window::new(handle, thread),
                    Some(script.clone()),
                    error,
                ));
            }
        }

        if let Err(error) = script.exit() {
            return Some(Status::Failure(
                Window::new(handle, thread),
                Some(script.clone()),
                error.to_string(),
            ));
        }

        Some(Status::Closure)
    }

    pub fn failure(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
        script: &Option<Script>,
        text: &str,
    ) -> Option<Status> {
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);

        if let Some(script) = script {
            if script.error.is_some() {
                if let Err(error) = script.error(text) {
                    Status::panic(&error);
                }

                return None;
            }
        }

        window.begin();

        let draw_shape = Vector2::new(
            draw.get_screen_width() as f32,
            draw.get_screen_height() as f32,
        );
        let card_shape = Rectangle::new(0.0, 0.0, draw_shape.x, 48.0);

        window.card_sharp(&mut draw, card_shape, Window::COLOR_PRIMARY_MAIN);

        window.point(Vector2::new(20.0, 12.0));
        window.text(&mut draw, "Fatal Error", Window::COLOR_TEXT);

        window.point(Vector2::new(20.0, 72.0));
        window.text(&mut draw, text, Color::BLACK);

        window.point(Vector2::new(20.0, draw_shape.y - 136.0));
        if window.button(&mut draw, "Load Module") {
            drop(draw);
            return Some(Status::new(handle, thread));
        }
        if window.button(&mut draw, "Copy Report") {
            let text =
                std::ffi::CString::new(text).expect("Status::failure(): Could not unwrap text.");

            unsafe {
                ffi::SetClipboardText(text.as_ptr());
            }
        }
        if window.button(&mut draw, "Exit Quiver") {
            return Some(Status::Closure);
        }

        None
    }

    #[rustfmt::skip]
    pub fn missing(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
    ) -> Option<Status> {
        window.draw(handle, thread)
    }

    pub fn panic(text: &str) {
        rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("Fatal Error")
            .set_description(text)
            .set_buttons(rfd::MessageButtons::Ok)
            .show();
    }
}

#[derive(Debug)]
pub enum InfoResult {
    Failure(String),
    Missing,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Info {
    pub safe: bool,
    pub path: String,
}

impl Info {
    pub const FILE_INFO: &'static str = "info.json";

    pub fn new() -> Result<Self, InfoResult> {
        let data = std::path::Path::new(Self::FILE_INFO);

        if data.is_file() {
            let file = std::fs::read_to_string(data)
                .map_err(|_| InfoResult::Failure("Info::new(): Error reading file.".to_string()))?;
            serde_json::from_str(&file)
                .map_err(|_| InfoResult::Failure("Info::new(): Error reading file.".to_string()))
        } else {
            Err(InfoResult::Missing)
        }
    }

    pub fn dump(&self) {
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
