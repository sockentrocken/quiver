mod script;
mod status;
mod system;
mod window;

//================================================================

use crate::status::*;

//================================================================

// the main entry-point.
fn main() {
    // create the RL context.
    let (mut handle, thread, _audio) = Status::window();
    // create the Quiver state.
    let mut status = Status::new(&mut handle, &thread);

    loop {
        match status {
            // missing status: no info_quiver.json file is present.
            Status::Missing(ref mut window) => {
                if let Some(state) = Status::missing(&mut handle, &thread, window) {
                    status = state;
                }
            }
            // success status: standard state.
            Status::Success(ref mut script) => {
                if let Some(state) = Status::success(&mut handle, &thread, script) {
                    status = state;
                }
            }
            // failure status: an error has been thrown from Lua, show crash-handler.
            Status::Failure(ref mut window, ref mut script, ref error) => {
                if let Some(state) = Status::failure(&mut handle, &thread, window, script, error) {
                    status = state;
                }
            }
            // closure status: break the infinite loop and close.
            Status::Closure => break,
        }
    }
}
