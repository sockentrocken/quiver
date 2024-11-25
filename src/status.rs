use crate::engine::*;
use crate::interface::*;
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

static mut test_tog: bool = false;
static mut test_sli: f32 = 1.0;
static mut test_rec: String = String::new();

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



    #[rustfmt::skip]
    pub fn wizard(
        engine: &mut Engine,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        interface: &mut Interface,
    ) {
        //let interface = window.start_frame(handle);

        interface.begin();

        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::new(223, 223, 223, 255));

        /*
        let card_y = 160;

        draw.draw_rectangle_rec(Rectangle::new(0.0, 0.0, draw.get_screen_width() as f32, (draw.get_screen_height() - card_y) as f32), Interface::COLOR_PRIMARY);

        draw.draw_rectangle_gradient_v(0, draw.get_screen_height() - card_y, draw.get_screen_width(), 8, Color::new(0, 0, 0, 66), Color::new(0, 0, 0, 0));

        interface.point(Vector2::new(16.0, (draw.get_screen_height() - card_y + 24) as f32));
        interface.button(&mut draw, "New Module");
        interface.button(&mut draw, "Load Module");
        interface.button(&mut draw, "Exit Quiver");

        draw.draw_texture_v(&interface.card, Vector2::new(draw.get_screen_width() as f32 * 0.5 - interface.card.width as f32 * 0.5, draw.get_screen_height() as f32 * 0.5 - interface.card.height as f32 * 0.5 - card_y as f32 * 0.5), Color::WHITE);
        */


        let size = draw.get_screen_width();

        interface.card_sharp(&mut draw, Rectangle::new(0.0, 0.0, size as f32, 48.0));

        interface.point(Vector2::new(16.0, 12.0));
        interface.text(&mut draw, "New Module");

        unsafe {
            interface.point(Vector2::new(16.0, 72.0));
            interface.record(&mut draw, "Module Path", &mut test_rec);
            interface.record(&mut draw, "Module Name", &mut test_rec);
            interface.record(&mut draw, "Module Info", &mut test_rec);

            //interface.toggle(&mut draw, "Texture", &mut test_tog);
            //interface.toggle(&mut draw, "Sound",   &mut test_tog);
            //interface.toggle(&mut draw, "Music",   &mut test_tog);
            //interface.toggle(&mut draw, "Font",    &mut test_tog);
        }

        /*
        unsafe {
            interface.button(&mut draw, "Button");
            interface.toggle(&mut draw, "Toggle", &mut test_tog);
            interface.slider(&mut draw, "Slider", &mut test_sli, -1.0, 1.0);
            interface.record(&mut draw, "Record", &mut test_rec);
        }
        */
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
