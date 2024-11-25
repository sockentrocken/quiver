use crate::engine::*;

use raylib::prelude::*;

use std::collections::HashMap;

pub struct Interface {
    font: Font,
    pub card: Texture2D,
    check: Texture2D,
    point: Vector2,
    shape: Vector2,
    focus: Option<i32>,
    count: i32,
}

fn easeInOutSine(x: f32) -> f32 {
    return -((3.14 * x).cos() - 1.0) / 2.0;
}

#[derive(Default, Debug)]
pub struct State {
    hover: bool,
    focus: bool,
    click: bool,
}

impl Interface {
    pub const COLOR_PRIMARY: Color = Color::new(255, 152, 0, 255);
    pub const COLOR_LIGHT_PRIMARY: Color = Color::new(255, 224, 178, 255);
    pub const COLOR_HEAVY_PRIMARY: Color = Color::new(245, 124, 0, 255);
    pub const COLOR_ACCENT: Color = Color::new(255, 87, 34, 255);
    pub const COLOR_TEXT: Color = Color::new(255, 255, 255, 255);

    pub const SPACE: Vector2 = Vector2::new(8.0, 8.0);

    fn load_texture(handle: &mut RaylibHandle, thread: &RaylibThread, texture: &[u8]) -> Texture2D {
        handle
            .load_texture_from_image(
                &thread,
                &Image::load_image_from_mem(".png", texture)
                    .expect("Interface::new(): Could not load texture."),
            )
            .expect("Interface::new(): Could not load texture.")
    }

    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let font = handle
            .load_font_from_memory(&thread, ".ttf", Engine::FONT, 24, None)
            .expect("Interface::new(): Could not load default font.");

        Self {
            font,
            card: Self::load_texture(handle, thread, Engine::CARD),
            check: Self::load_texture(handle, thread, Engine::CHECK),
            point: Vector2::default(),
            shape: Vector2::default(),
            focus: None,
            count: i32::default(),
        }
    }

    pub fn point(&mut self, point: Vector2) {
        self.point = point;
    }

    pub fn begin(&mut self) {
        self.point = Vector2::default();
        self.count = i32::default();
    }

    pub fn card_sharp(&self, draw: &mut RaylibDrawHandle, content: Rectangle) {
        draw.draw_rectangle_gradient_v(
            content.x as i32,
            (content.y + content.height - 4.0) as i32,
            content.width as i32,
            8,
            Color::new(0, 0, 0, 66),
            Color::new(0, 0, 0, 0),
        );

        draw.draw_rectangle_rec(content, Interface::COLOR_PRIMARY);
    }

    pub fn card_round(&self, draw: &mut RaylibDrawHandle, content: Rectangle) {
        draw.draw_rectangle_gradient_v(
            content.x as i32,
            (content.y + content.height - 4.0) as i32,
            content.width as i32,
            8,
            Color::new(0, 0, 0, 66),
            Color::new(0, 0, 0, 0),
        );

        draw.draw_rectangle_rounded(content, 0.25, 4, Interface::COLOR_PRIMARY);
    }

    pub fn font(&mut self, draw: &mut RaylibDrawHandle, text: &str, point: Vector2) {
        draw.draw_text_ex(&self.font, text, point, 24.0, 1.0, Self::COLOR_TEXT);
    }

    pub fn font_measure(&mut self, text: &str) -> Vector2 {
        self.font.measure_text(text, 24.0, 1.0)
    }

    pub fn text(&mut self, draw: &mut RaylibDrawHandle, text: &str) {
        self.font(draw, text, self.point);

        self.point.y += self.font_measure(text).y + 8.0;
    }

    pub fn state(&mut self, draw: &RaylibDrawHandle, content: Rectangle) -> State {
        let mut state = State::default();
        let hover = content.check_collision_point_rec(draw.get_mouse_position());

        if hover {
            if self.focus.is_none() && draw.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            {
                self.focus = Some(self.count);
            }

            state.hover = true;
        }

        if let Some(focus) = self.focus {
            if focus == self.count {
                if draw.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                    if hover {
                        state.click = true;
                    }

                    self.focus = None;
                }

                state.focus = true;
            }
        }

        state
    }

    pub fn button(&mut self, draw: &mut RaylibDrawHandle, text: &str) -> bool {
        let measure = self.font_measure(text);
        let content = Rectangle::new(
            self.point.x,
            self.point.y,
            measure.x + 16.0,
            measure.y + 8.0,
        );

        let result = self.state(draw, content);

        self.card_round(draw, content);
        self.font(draw, text, Vector2::new(content.x + 8.0, content.y + 4.0));

        self.point.y += content.height + 8.0;
        self.count += 1;

        result.click
    }

    pub fn toggle(&mut self, draw: &mut RaylibDrawHandle, text: &str, value: &mut bool) {
        let content_max = Rectangle::new(self.point.x, self.point.y, 24.0, 24.0);
        let state = self.state(draw, content_max);

        if state.click {
            *value = !*value;
        }

        self.card_round(draw, content_max);

        if *value {
            draw.draw_texture_pro(
                &self.check,
                Rectangle::new(0.0, 0.0, self.check.width as f32, self.check.height as f32),
                Rectangle::new(
                    content_max.x,
                    content_max.y,
                    content_max.width,
                    content_max.height,
                ),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }

        self.font(
            draw,
            text,
            Vector2::new(
                content_max.width + content_max.x + Self::SPACE.x,
                content_max.y,
            ),
        );

        self.point.y += content_max.height + Self::SPACE.y;
        self.count += 1;
    }

    pub fn slider(
        &mut self,
        draw: &mut RaylibDrawHandle,
        text: &str,
        value: &mut f32,
        min: f32,
        max: f32,
    ) {
        let percent = (*value - min) / (max - min);
        let content_hit = Rectangle::new(self.point.x, self.point.y, 96.0, 24.0);
        let content_max = Rectangle::new(self.point.x, self.point.y + 8.0, 96.0, 4.0);
        let content_min = Rectangle::new(self.point.x, self.point.y + 8.0, 96.0 * percent, 4.0);
        let state = self.state(draw, content_hit);

        if state.focus {
            let mouse = (draw.get_mouse_x() as f32 - content_hit.x)
                / ((content_hit.x + content_hit.width) - content_hit.x);

            let mouse = mouse.clamp(0.0, 1.0);

            *value = mouse * (max - min) + min;
        }

        self.card_sharp(draw, content_max);
        self.card_sharp(draw, content_min);
        draw.draw_circle_v(
            Vector2::new(
                self.point.x + 8.0 + (content_max.width - 16.0) * percent,
                self.point.y + 10.0,
            ),
            8.0,
            Self::COLOR_HEAVY_PRIMARY,
        );

        self.font(
            draw,
            text,
            Vector2::new(
                content_max.width + content_max.x + Self::SPACE.x,
                self.point.y,
            ),
        );

        self.point.y += content_hit.height + Self::SPACE.y;
        self.count += 1;
    }

    pub fn record(&mut self, draw: &mut RaylibDrawHandle, text: &str, value: &mut String) {
        let content_hit = Rectangle::new(self.point.x, self.point.y, 160.0, 24.0);
        let content_max = Rectangle::new(self.point.x, self.point.y + 20.0, 160.0, 4.0);
        let state = self.state(draw, content_hit);

        self.card_sharp(draw, content_max);
        self.font(draw, value, self.point + Vector2::new(0.0, -4.0));
        self.font(
            draw,
            text,
            self.point + Vector2::new(content_hit.width + Self::SPACE.x, 0.0),
        );

        unsafe {
            if state.hover {
                let key = ffi::GetCharPressed();

                if draw.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
                    || ffi::IsKeyPressedRepeat(KeyboardKey::KEY_BACKSPACE as i32)
                {
                    value.pop();
                } else {
                    if key != 0 {
                        value.push(key as u8 as char);
                    }
                }

                let measure = self.font_measure(value);

                self.card_sharp(
                    draw,
                    Rectangle::new(self.point.x + measure.x, self.point.y, 2.0, 16.0),
                );
            }
        }

        self.point.y += content_hit.height + Self::SPACE.y;
        self.count += 1;
    }
}
