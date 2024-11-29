mod script;
mod status;
mod system;
mod window;

//================================================================

use crate::status::*;

//================================================================

fn main() {
    let (mut handle, thread, _audio) = Status::initialize();
    let mut status = Status::new(&mut handle, &thread);

    while !handle.window_should_close() {
        match status {
            Status::Success(ref mut script) => {
                if let Some(state) = Status::success(&mut handle, &thread, script) {
                    status = state;
                }
            }
            Status::Failure(ref mut window, ref error) => {
                if let Some(state) = Status::failure(&mut handle, &thread, window, error) {
                    status = state;
                }
            }
            Status::Missing(ref mut window) => {
                if let Some(state) = Status::missing(&mut handle, &thread, window) {
                    status = state;
                }
            }
            Status::Closure => break,
        }
    }
}
