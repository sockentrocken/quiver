/*
* BSD Zero Clause License
*
* Copyright (c) 2025 sockentrocken
*
* Permission to use, copy, modify, and/or distribute this software for any
* purpose with or without fee is hereby granted.
*
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
* REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
* AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
* INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
* LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
* OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
* PERFORMANCE OF THIS SOFTWARE.
*/

mod script;
mod status;
mod system;
mod window;

//================================================================

use crate::status::*;

//================================================================

// the main entry-point.
#[rustfmt::skip]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                if let Some(state) = Status::success(&mut handle, &thread, script).await {
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

    Ok(())

}
