mod script;
mod status;
mod system;
mod window;

//================================================================

use crate::status::*;

//================================================================

fn main() {
    let (mut handle, thread, _audio) = Status::window();
    let mut status = Status::new(&mut handle, &thread);

    loop {
        match status {
            Status::Missing(ref mut window) => {
                if let Some(state) = Status::missing(&mut handle, &thread, window) {
                    status = state;
                }
            }
            Status::Success(ref mut script) => {
                if let Some(state) = Status::success(&mut handle, &thread, script) {
                    status = state;
                }
            }
            Status::Failure(ref mut window, ref mut script, ref error) => {
                if let Some(state) = Status::failure(&mut handle, &thread, window, script, error) {
                    status = state;
                }
            }
            Status::Closure => break,
        }
    }
}
