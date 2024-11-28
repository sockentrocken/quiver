mod script;
mod status;
mod system;
mod window;
mod wizard;

//================================================================

use crate::status::*;
use raylib::prelude::*;

//================================================================

fn main() -> Result<(), String> {
    let (mut handle, thread, _audio) = Status::initialize();
    let mut status = Status::new(&mut handle, &thread);

    while !handle.window_should_close() {
        match status {
            Status::Success(ref mut script) => {
                if let Some(s) = Status::success(&mut handle, &thread, script) {
                    status = s;
                }
            }
            Status::Failure(ref mut window, ref error) => {
                if let Some(s) = Status::failure(&mut handle, &thread, window, error) {
                    status = s;
                }
            }
            Status::Wizard(ref mut window, ref mut wizard) => {
                if let Some(s) = Status::wizard(&mut handle, &thread, window, wizard) {
                    status = s;
                }
            }
            Status::Restart => {
                status = Status::Closure;
            }
            Status::Closure => break,
        }
    }

    Ok(())
}
