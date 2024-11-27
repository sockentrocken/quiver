use crate::engine::*;
use crate::interface::*;

//================================================================

use raylib::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

//================================================================

pub type StatusPointer = Rc<RefCell<Status>>;

#[derive(Default, Clone)]
pub enum Status {
    #[default]
    Success,
    Failure(String),
    Wizard,
    Restart,
    Closure,
}

impl Status {
    pub fn success(engine: &mut Engine, handle: &mut RaylibHandle, thread: &RaylibThread) {
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);

        if let Some(script) = &mut engine.script {
            if let Err(error) = &mut script.step() {
                Status::set_failure(&engine, error.to_string());
            }
        }
    }

    pub fn failure(
        _engine: &mut Engine,
        _handle: &mut RaylibHandle,
        _thread: &RaylibThread,
        _text: &str,
    ) {
    }

    #[rustfmt::skip]
    pub fn wizard(
        engine: &mut Engine,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        interface: &mut Interface,
    ) {
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);

        Wizard::draw(engine, &mut draw, interface);
    }

    pub fn restart(engine: &mut Engine) {
        if let Some(script) = &mut engine.script {
            if let Err(error) = &mut script.exit() {
                Status::set_failure(&engine, error.to_string());
            }
        }

        *engine = Engine::new();

        if let Some(script) = &mut engine.script {
            if let Err(error) = &mut script.main() {
                Status::set_failure(&engine, error.to_string());
            }
        }
    }

    pub fn set_failure(engine: &Engine, text: String) {
        *engine.status.borrow_mut() = Status::Failure(text);

        unsafe {
            if ffi::IsWindowReady() {
                ffi::SetMouseOffset(0, 0);
                ffi::SetMouseScale(1.0, 1.0);
                ffi::EndMode3D();
                ffi::EndMode2D();
                ffi::EndShaderMode();
                ffi::EnableCursor();
            }
        }
    }

    pub fn set_wizard(engine: &Engine) {
        *engine.status.borrow_mut() = Status::Wizard;
    }

    pub fn _set_restart(engine: &Engine) {
        *engine.status.borrow_mut() = Status::Restart;
    }

    pub fn _set_closure(engine: &Engine) {
        *engine.status.borrow_mut() = Status::Closure;
    }
}
