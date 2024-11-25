use raylib::prelude::*;

use std::collections::HashMap;

#[derive(Default)]
pub struct Interface {
    data: HashMap<String, Widget>,
    mouse: Mouse,
}

#[derive(Default)]
pub struct Widget {
    dirty: bool,
    shift: f32,
}

#[derive(Default)]
pub struct Mouse {
    point: Vector2,
    press: bool,
    release: bool,
    up: bool,
    down: bool,
}

fn easeInOutSine(x: f32) -> f32 {
    return -((3.14 * x).cos() - 1.0) / 2.0;
}

impl Interface {
    fn check_mouse(&mut self, content: Rectangle) -> bool {
        unsafe { ffi::CheckCollisionPointRec(self.mouse.point.into(), content.into()) }
    }

    pub fn begin(&mut self, handle: &RaylibHandle) {
        self.mouse.point = handle.get_mouse_position();
        self.mouse.press = handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        self.mouse.release = handle.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);
        self.mouse.up = handle.is_mouse_button_up(MouseButton::MOUSE_BUTTON_LEFT);
        self.mouse.down = handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
    }

    pub fn button(&mut self, draw: &mut RaylibDrawHandle, text: &str, content: Rectangle) -> bool {
        let mut press = false;
        let check = self.check_mouse(content);
        let mut hash = self.data.get_mut(text);

        if let Some(widget) = &mut hash {
            if check {
                widget.shift += unsafe { ffi::GetFrameTime() * 8.0 };

                if self.mouse.press {
                    press = true;
                }
            } else {
                widget.shift -= unsafe { ffi::GetFrameTime() * 8.0 };
            }

            widget.shift = widget.shift.clamp(0.0, 1.0);

            let alpha = easeInOutSine(widget.shift);

            draw.draw_rectangle_rounded(
                content,
                0.5,
                4,
                Color::new(255, 255, 255, 127 + (alpha * 127.0) as u8),
            );
            draw.draw_text(
                &format!("{alpha}"),
                content.x as i32 + 4,
                content.y as i32 + 4,
                content.height as i32,
                Color::BLACK,
            );
        } else {
            self.data.insert(text.to_string(), Widget::default());
        }

        press
    }
}
