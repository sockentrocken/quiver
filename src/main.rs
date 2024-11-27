mod engine;
mod interface;
mod module;
mod script;
mod status;
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
    let (mut handle, thread, _audio) = engine.window().map_err(|e| { crate::utility::panic_window(&e); e })?;
    let mut interface = Interface::new(&mut handle, &thread);

    if let Some(script) = &mut engine.script {
        if let Err(error) = &mut script.main() {
            Status::set_failure(&engine, error.to_string());
        }
    }

    while !handle.window_should_close() {
        let status = engine.status.borrow().clone();

        match status {
            Status::Success =>
                Status::success(&mut engine, &mut handle, &thread),
            Status::Failure(ref text) =>
                Status::failure(&mut engine, &mut handle, &thread, text),
            Status::Wizard =>
                Status::wizard(&mut engine, &mut handle, &thread, &mut interface),
            Status::Restart => {
                Status::restart(&mut engine);
            }
            Status::Closure =>
                break,
        }
    }

    if let Some(script) = &mut engine.script {
        if let Err(error) = &mut script.main() {
            Status::set_failure(&engine, error.to_string());
        }
    }

    drop(engine);

    Ok(())
}
