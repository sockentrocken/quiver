/*
* Copyright (c) 2025 sockentrocken
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
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
    pub async fn missing(
        &mut self,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Option<Status> {
        while !handle.window_should_close() {
            let draw_shape = Vector2::new(
                handle.get_screen_width() as f32,
                handle.get_screen_height() as f32,
            );
            let logo_shape = Vector2::new(self.logo.width as f32, self.logo.height as f32);
            let logo_point = Vector2::new(
                (draw_shape.x * 0.5) - (logo_shape.x * 0.5),
                (draw_shape.y * 0.5) - (logo_shape.y * 0.5) - (Self::LOGO_SHAPE * 0.5),
            );
            let card_shape =
                Rectangle::new(0.0, 0.0, draw_shape.x, draw_shape.y - Self::LOGO_SHAPE);

            // begin drawing, clear screen, begin window frame.
            let mut draw = handle.begin_drawing(thread);
            draw.clear_background(Color::WHITE);
            self.begin();

            // card header.
            self.card_sharp(&mut draw, card_shape, Window::COLOR_PRIMARY_MAIN);
            draw.draw_texture_v(&self.logo, logo_point, Color::WHITE);

            // button footer.
            self.point(Vector2::new(20.0, draw_shape.y - Self::LOGO_SHAPE + 24.0));

            // create a new info file for a project, which doesn't exist yet.
            if self.button(&mut draw, "New Project") {
                let path = std::env::current_dir()
                    .map_err(|e| Status::panic(&e.to_string()))
                    .unwrap();

                let project = rfd::FileDialog::new().set_directory(path).pick_folder();

                if let Some(project) = project {
                    Script::new_project(&project.display().to_string());

                    drop(draw);
                    return Some(Status::new().await);
                }
            }

            // create a new info file for a project.
            if self.button(&mut draw, "Load Project") {
                let path = std::env::current_dir()
                    .map_err(|e| Status::panic(&e.to_string()))
                    .unwrap();

                let project = rfd::FileDialog::new().set_directory(path).pick_folder();

                if let Some(project) = project {
                    Script::load_project(&project.display().to_string());

                    drop(draw);
                    return Some(Status::new().await);
                }
            }

            // exit Quiver.
            if self.button(&mut draw, "Exit Quiver") {
                return Some(Status::Closure);
            }
        }

        Some(Status::Closure)
    }

    // draw failure window layout.
    pub async fn failure(
        &mut self,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        text: &str,
    ) -> Option<Status> {
        while !handle.window_should_close() {
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
            if self.button(&mut draw, "Load Project") {
                drop(draw);
                return Some(Status::new().await);
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
        }

        Some(Status::Closure)
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
            data.get_color(&Window::COLOR_PRIMARY_MAIN),
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
