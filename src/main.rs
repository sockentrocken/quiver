/*
* ================================================================
* main.rs
* ================================================================
*/

mod engine;
mod script;
mod status;
mod support;
mod system;
mod utility;
mod window;
mod module;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//================================================================

use crate::engine::*;
use crate::status::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

#[rustfmt::skip]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), String> {
    let mut engine = Engine::new();
    let (mut handle, thread, _audio, mut window) = engine.window().map_err(|e| { crate::utility::panic_window(&e); e })?;

    if let Err(error) = engine.script.main().await {
        Status::set_failure(&engine, error);
    }

    unsafe {
        let name = CString::new("cfx.mkv".to_string()).unwrap();
        let mut media : MediaStream = LoadMedia(name.as_ptr());
        let pointer = &mut media as *mut MediaStream;

        while !handle.window_should_close() {
            let x = handle.get_screen_width();
            let y = handle.get_screen_height();

            let mut draw = handle.begin_drawing(&thread);

            draw.draw_pixel(0, 0, raylib::color::Color::RED);

            UpdateMedia(pointer);

            DrawTexturePro(media.videoTexture,
                Rectangle { x: 0.0, y: 0.0, width: media.videoTexture.width as f32, height: media.videoTexture.height as f32 },
                Rectangle { x : 0.0, y : 0.0, width: x as f32, height : y as f32 }, Vector2 { x : 0.0, y: 0.0 }, 0.0, Color { r: 255, g: 255, b: 255, a: 255 });

            /*
            let status = engine.status.borrow().clone();

            match status {
                Status::Success =>
                    Status::success(&mut engine, &mut handle, &thread, &mut window),
                Status::Failure(ref text) =>
                    Status::failure(&mut engine, &mut handle, &thread, &mut window, text),
                Status::Wizard =>
                    Status::wizard(&mut engine, &mut handle, &thread, &mut window),
                Status::Restart => {
                    drop(window);
                    window = Status::restart(&mut engine, &mut handle, &thread).await;
                }
                Status::Closure =>
                    break,
            }
            */
        }
    }


    if let Err(error) = engine.script.exit() {
        Status::set_failure(&engine, error);
    }

    drop(engine);

    Ok(())
}
