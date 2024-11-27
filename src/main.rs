mod engine;
mod script;
mod system;
mod utility;
mod window;

//================================================================

use crate::engine::*;
use crate::window::*;

//================================================================

#[rustfmt::skip]
fn main() -> Result<(), String> {
    let mut engine = Engine::new();
    let (mut handle, thread, _audio) = engine.initialize();
    let mut window = Window::new(&mut handle, &thread);
    let mut wizard = Wizard::default();

    if let Err(error) = &engine.script.main() {
        Status::set_failure(&mut engine, error.to_string());
    }

    while !handle.window_should_close() {
        match engine.status.clone() {
            Status::Success =>
                Status::success(&mut engine, &mut handle, &thread),
            Status::Failure(ref text) =>
                Status::failure(&mut engine, &mut handle, &thread, &mut window, text),
            Status::Wizard =>
                Status::wizard(&mut engine, &mut handle, &thread, &mut window, &mut wizard),
            Status::Restart =>
                Status::restart(&mut engine),
            Status::Closure =>
                break,
        }
    }

    if let Err(error) = &engine.script.exit() {
        Status::set_failure(&mut engine, error.to_string());
    }

    drop(engine);

    Ok(())
}
