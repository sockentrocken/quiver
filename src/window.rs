/*
* MIT License
*
* Copyright (c) 2024 sockentrocken
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use crate::{script::Script, status::*};

//================================================================

use raylib::prelude::*;

//================================================================

// window structure, responsible for drawing the missing/failure interface.
pub struct Window {
    data: [gizmo::Data; Self::GIZMO_COUNT],
    font: Font,
    logo: Texture2D,
    point: Vector2,
    focus: Option<i32>,
    count: i32,
}

impl Window {
    const COLOR_PRIMARY_MAIN: Color = Color::new(255, 87, 34, 255);
    const COLOR_PRIMARY_SIDE: Color = Color::new(255, 152, 0, 255);
    const COLOR_TEXT_WHITE: Color = Color::new(255, 255, 255, 255);
    const COLOR_TEXT_BLACK: Color = Color::new(33, 33, 33, 255);

    //================================================================

    const GRADIENT_POINT_Y: f32 = 4.0;
    const GRADIENT_SHAPE_Y: i32 = 6;
    const GRADIENT_COLOR_MAX: Color = Color::new(0, 0, 0, 99);
    const GRADIENT_COLOR_MIN: Color = Color::new(0, 0, 0, 0);

    //================================================================

    const LOGO_SHAPE: f32 = 160.0;

    //================================================================

    const CARD_ROUND_SHAPE: f32 = 0.25;
    const CARD_ROUND_COUNT: i32 = 4;

    //================================================================

    const TEXT_SHAPE: f32 = 24.0;
    const TEXT_SPACE: f32 = 1.0;

    //================================================================

    const BUTTON_SHAPE: Vector2 = Vector2::new(160.0, 32.0);
    const BUTTON_TEXT_SHIFT: Vector2 = Vector2::new(8.0, 4.0);
    const BUTTON_SHIFT: f32 = 8.0;

    //================================================================

    const GIZMO_COUNT: usize = 64;

    //================================================================

    // get a new window instance.
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // load font.
        let font = handle
            .load_font_from_memory(thread, ".ttf", Status::FONT, Self::TEXT_SHAPE as i32, None)
            .map_err(|e| Status::panic(&e.to_string()))
            .unwrap();

        // load logo.
        let logo = handle
            .load_texture_from_image(
                thread,
                &Image::load_image_from_mem(".png", Status::LOGO)
                    .map_err(|e| Status::panic(&e.to_string()))
                    .unwrap(),
            )
            .map_err(|e| Status::panic(&e.to_string()))
            .unwrap();

        Self {
            data: [gizmo::Data::default(); Self::GIZMO_COUNT],
            font,
            logo,
            point: Vector2::default(),
            focus: None,
            count: i32::default(),
        }
    }

    // draw missing window layout.
    pub fn missing(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) -> Option<Status> {
        let draw_shape = Vector2::new(
            handle.get_screen_width() as f32,
            handle.get_screen_height() as f32,
        );
        let logo_shape = Vector2::new(self.logo.width as f32, self.logo.height as f32);
        let logo_point = Vector2::new(
            (draw_shape.x * 0.5) - (logo_shape.x * 0.5),
            (draw_shape.y * 0.5) - (logo_shape.y * 0.5) - (Self::LOGO_SHAPE * 0.5),
        );
        let card_shape = Rectangle::new(0.0, 0.0, draw_shape.x, draw_shape.y - Self::LOGO_SHAPE);

        // begin drawing, clear screen, begin window frame.
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);
        self.begin();

        // card header.
        self.card_sharp(&mut draw, card_shape, Window::COLOR_PRIMARY_MAIN);
        draw.draw_texture_v(&self.logo, logo_point, Color::WHITE);

        // button footer.
        self.point(Vector2::new(20.0, draw_shape.y - Self::LOGO_SHAPE + 24.0));

        // create a new info file for a module, which doesn't exist yet.
        if self.button(&mut draw, "New Module") {
            let path = std::env::current_dir()
                .map_err(|e| Status::panic(&e.to_string()))
                .unwrap();

            let module = rfd::FileDialog::new().set_directory(path).pick_folder();

            if let Some(module) = module {
                Script::new_module(&module.display().to_string());

                drop(draw);
                return Some(Status::new(handle, thread));
            }
        }

        // create a new info file for a module.
        if self.button(&mut draw, "Load Module") {
            let path = std::env::current_dir()
                .map_err(|e| Status::panic(&e.to_string()))
                .unwrap();

            let module = rfd::FileDialog::new().set_directory(path).pick_folder();

            if let Some(module) = module {
                Script::load_module(&module.display().to_string());

                drop(draw);
                return Some(Status::new(handle, thread));
            }
        }

        // exit Quiver.
        if self.button(&mut draw, "Exit Quiver") {
            return Some(Status::Closure);
        }

        // if window should close, exit Quiver.
        if draw.window_should_close() {
            Some(Status::Closure)
        } else {
            None
        }
    }

    // draw failure window layout.
    pub fn failure(
        &mut self,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        text: &str,
    ) -> Option<Status> {
        let draw_shape = Vector2::new(
            handle.get_screen_width() as f32,
            handle.get_screen_height() as f32,
        );
        let card_shape = Rectangle::new(0.0, 0.0, draw_shape.x, 48.0);

        // begin drawing, clear screen, begin window frame.
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);
        self.begin();

        // card header.
        self.card_sharp(&mut draw, card_shape, Window::COLOR_PRIMARY_MAIN);
        self.font(
            &mut draw,
            "Fatal Error",
            Vector2::new(20.0, 12.0),
            Self::COLOR_TEXT_WHITE,
        );
        self.font(
            &mut draw,
            text,
            Vector2::new(20.0, 72.0),
            Self::COLOR_TEXT_BLACK,
        );

        // button footer.
        self.point(Vector2::new(20.0, draw_shape.y - 136.0));

        // reload Quiver.
        if self.button(&mut draw, "Load Module") {
            drop(draw);
            return Some(Status::new(handle, thread));
        }

        // copy report to clipboard.
        if self.button(&mut draw, "Copy Report") {
            draw.set_clipboard_text(text)
                .map_err(|e| Status::panic(&e.to_string()))
                .unwrap();
        }

        // exit Quiver.
        if self.button(&mut draw, "Exit Quiver") {
            return Some(Status::Closure);
        }

        // if window should close, exit Quiver.
        if draw.window_should_close() {
            Some(Status::Closure)
        } else {
            None
        }
    }

    //================================================================

    // begin a new frame for the window.
    fn begin(&mut self) {
        self.point = Vector2::default();
        self.count = i32::default();
    }

    // set the draw cursor point.
    fn point(&mut self, point: Vector2) {
        self.point = point;
    }

    // draw a card with a drop shadow (sharp).
    fn card_sharp(&self, draw: &mut RaylibDrawHandle, rectangle: Rectangle, color: Color) {
        draw.draw_rectangle_gradient_v(
            rectangle.x as i32,
            (rectangle.y + rectangle.height) as i32,
            rectangle.width as i32,
            Self::GRADIENT_SHAPE_Y,
            Self::GRADIENT_COLOR_MAX,
            Self::GRADIENT_COLOR_MIN,
        );

        draw.draw_rectangle_rec(rectangle, color);
    }

    // draw a card with a drop shadow (round).
    fn card_round(&self, draw: &mut RaylibDrawHandle, rectangle: Rectangle, color: Color) {
        draw.draw_rectangle_gradient_v(
            rectangle.x as i32,
            (rectangle.y + rectangle.height - Self::GRADIENT_POINT_Y) as i32,
            rectangle.width as i32,
            Self::GRADIENT_SHAPE_Y + Self::GRADIENT_POINT_Y as i32,
            Self::GRADIENT_COLOR_MAX,
            Self::GRADIENT_COLOR_MIN,
        );

        draw.draw_rectangle_rounded(
            rectangle,
            Self::CARD_ROUND_SHAPE,
            Self::CARD_ROUND_COUNT,
            color,
        );
    }

    // draw a button.
    fn button(&mut self, draw: &mut RaylibDrawHandle, text: &str) -> bool {
        // get the point and shape of the gizmo.
        let rectangle = Rectangle::new(
            self.point.x,
            self.point.y,
            Self::BUTTON_SHAPE.x,
            Self::BUTTON_SHAPE.y,
        );

        // get state, and data of the widget.
        let state = gizmo::State::get(self, draw, rectangle);
        let data = gizmo::Data::get_mutable(self);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus);
        let data = gizmo::Data::get(self);

        // get location of text.
        let text_point = Vector2::new(
            rectangle.x + Self::BUTTON_TEXT_SHIFT.x,
            rectangle.y + Self::BUTTON_TEXT_SHIFT.y - data.get_point(),
        );

        // draw card and text.
        self.card_round(
            draw,
            data.get_shape(&rectangle),
            data.get_color(&Window::COLOR_PRIMARY_SIDE),
        );
        self.font(
            draw,
            text,
            text_point,
            data.get_color(&Self::COLOR_TEXT_WHITE),
        );

        // increment the point of the next gizmo.
        self.point.y += Self::BUTTON_SHAPE.y + Self::BUTTON_SHIFT;
        self.count += 1;

        state.click
    }

    // draw text.
    fn font(&self, draw: &mut RaylibDrawHandle, text: &str, point: Vector2, color: Color) {
        draw.draw_text_ex(
            &self.font,
            text,
            point,
            Self::TEXT_SHAPE,
            Self::TEXT_SPACE,
            color,
        );
    }
}

pub mod gizmo {
    use super::*;

    #[derive(Default, Debug)]
    pub struct State {
        pub hover: bool,
        pub focus: bool,
        pub click: bool,
    }

    impl State {
        // get the state of a gizmo.
        pub fn get(window: &mut Window, draw: &RaylibDrawHandle, rectangle: Rectangle) -> Self {
            let mut state = State::default();
            // check if the cursor is over the gizmo's shape.
            let hover = rectangle.check_collision_point_rec(draw.get_mouse_position());

            // cursor is currently over the gizmo...
            if hover {
                // no focus is set, and the mouse button has been set off, set current gizmo as the focus.
                if window.focus.is_none()
                    && draw.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                {
                    window.focus = Some(window.count);
                }

                // set hover.
                state.hover = true;
            }

            // focus is set...
            if let Some(focus) = window.focus {
                // current gizmo is the current focus!
                if focus == window.count {
                    // the mouse button has been set off...
                    if draw.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                        // if the mouse was hovering over the gizmo, set off click event.
                        if hover {
                            state.click = true;
                        }

                        // set window focus as none.
                        window.focus = None;
                    }

                    // set focus.
                    state.focus = true;
                }
            }

            state
        }
    }

    #[derive(Copy, Clone, Default)]
    pub struct Data {
        hover: f32,
        focus: f32,
    }

    impl Data {
        const POINT_SHIFT: f32 = 4.0;
        const COLOR_UPPER: f32 = 0.25;
        const COLOR_LOWER: f32 = 0.75;
        const HOVER_SPEED: f32 = 16.0;
        const FOCUS_SPEED: f32 = 16.0;

        // borrow a data instance.
        pub fn get(window: &Window) -> &Self {
            window
                .data
                .get(window.count as usize)
                .expect("Data::get(): gizmo overflow.")
        }

        // borrow a data instance mutably.
        pub fn get_mutable(window: &mut Window) -> &mut Self {
            window
                .data
                .get_mut(window.count as usize)
                .expect("Data::get_mutable(): gizmo overflow.")
        }

        // get a point depending on the value of hover.
        pub fn get_point(&self) -> f32 {
            ((self.hover - 1.0) + (1.0 - self.focus)) * Self::POINT_SHIFT
        }

        // get a shape depending on the value of hover.
        pub fn get_shape(&self, rectangle: &Rectangle) -> Rectangle {
            Rectangle::new(
                rectangle.x,
                rectangle.y - self.get_point(),
                rectangle.width,
                rectangle.height,
            )
        }

        // get a color depending on the value of hover.
        pub fn get_color(&self, color: &Color) -> Color {
            Color::new(
                (color.r as f32 * ((self.hover * Self::COLOR_UPPER) + Self::COLOR_LOWER)) as u8,
                (color.g as f32 * ((self.hover * Self::COLOR_UPPER) + Self::COLOR_LOWER)) as u8,
                (color.b as f32 * ((self.hover * Self::COLOR_UPPER) + Self::COLOR_LOWER)) as u8,
                color.a,
            )
        }

        // adjust the hover variable.
        pub fn set_hover(&mut self, draw: &RaylibDrawHandle, value: bool) {
            if value {
                self.hover += draw.get_frame_time() * Self::HOVER_SPEED;
            } else {
                self.hover -= draw.get_frame_time() * Self::HOVER_SPEED;
            }

            self.hover = self.hover.clamp(0.0, 1.0);
        }

        // adjust the focus variable.
        pub fn set_focus(&mut self, draw: &RaylibDrawHandle, value: bool) {
            if value {
                self.focus += draw.get_frame_time() * Self::FOCUS_SPEED;
            } else {
                self.focus -= draw.get_frame_time() * Self::FOCUS_SPEED;
            }

            self.focus = self.focus.clamp(0.0, 1.0);
        }
    }
}
