mod engine;
mod interface;
mod module;
mod script;
mod status;
mod support;
mod system;
mod utility;
mod window;

//================================================================

use crate::engine::*;
use crate::interface::*;
use crate::status::*;

//================================================================

#[rustfmt::skip]
fn main() -> Result<(), String> {
    let mut engine = Engine::new();
    let (mut handle, thread, _audio, mut window) = engine.window().map_err(|e| { crate::utility::panic_window(&e); e })?;
    let mut interface = Interface::new(&mut handle, &thread);

    if let Err(error) = engine.script.main() {
        Status::set_failure(&engine, error);
    }

    while !handle.window_should_close() {
        let status = engine.status.borrow().clone();

        match status {
            Status::Success =>
                Status::success(&mut engine, &mut handle, &thread, &mut window),
            Status::Failure(ref text) =>
                Status::failure(&mut engine, &mut handle, &thread, &mut window, text),
            Status::Wizard =>
                Status::wizard(&mut engine, &mut handle, &thread, &mut interface),
            Status::Restart => {
                drop(window);
                window = Status::restart(&mut engine, &mut handle, &thread);
            }
            Status::Closure =>
                break,
        }
    }

    if let Err(error) = engine.script.exit() {
        Status::set_failure(&engine, error);
    }

    drop(engine);

    Ok(())
}
