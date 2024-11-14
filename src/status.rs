/*
* ================================================================
* status.rs
* ================================================================
*/

use crate::engine::*;
use crate::support::*;
use crate::window::*;

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
    pub fn success(
        engine: &mut Engine,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut RaylibImguiSupport,
    ) {
        let mut interface: Option<&mut imgui::Ui> = None;

        let w_active = engine.window.borrow().active;
        let l_active = engine.window.borrow().logger.active;

        if w_active || l_active {
            interface = Some(window.start_frame(handle));
        }

        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);

        if let Err(error) = engine.script.step() {
            Status::set_failure(&engine, error);
        }

        if let Some(interface) = interface {
            Window::draw(engine, interface, w_active, l_active);
            window.end_frame(&mut draw);
        }
    }

    pub fn failure(
        engine: &mut Engine,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut RaylibImguiSupport,
        text: &str,
    ) {
        let interface = window.start_frame(handle);

        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::BLACK);

        interface
            .window("Engine Failure")
            .flags(
                imgui::WindowFlags::NO_MOVE
                    | imgui::WindowFlags::NO_RESIZE
                    | imgui::WindowFlags::NO_DECORATION,
            )
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(interface.io().display_size, imgui::Condition::Always)
            .build(|| {
                interface.text(text);

                engine
                    .window
                    .borrow_mut()
                    .logger
                    .draw(interface, true, true, [0.0, -70.0]);

                if interface.button("Dump Report") {
                    todo!();
                }
                if interface.button("Load Script") {
                    Self::set_restart(engine);
                }
                if interface.button("Exit Engine") {
                    Self::set_closure(engine);
                }
            });

        window.end_frame(&mut draw);
    }

    pub fn wizard(
        engine: &mut Engine,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut RaylibImguiSupport,
    ) {
        let interface = window.start_frame(handle);

        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::BLACK);

        Wizard::draw(engine, interface);

        window.end_frame(&mut draw);
    }

    pub fn restart(
        engine: &mut Engine,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> RaylibImguiSupport {
        if let Err(error) = engine.script.exit() {
            Status::set_failure(engine, error);
        }

        *engine = Engine::new();

        if let Err(error) = engine.script.main() {
            Status::set_failure(engine, error);
        }

        RaylibImguiSupport::setup(handle, thread)
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

    pub fn set_restart(engine: &Engine) {
        *engine.status.borrow_mut() = Status::Restart;
    }

    pub fn set_closure(engine: &Engine) {
        *engine.status.borrow_mut() = Status::Closure;
    }
}
