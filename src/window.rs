use crate::{script::Script, status::*};

//================================================================

use raylib::prelude::*;

//================================================================

pub struct Window {
    data: [widget::Data; 64],
    font: Font,
    pub logo: Texture2D,
    point: Vector2,
    focus: Option<i32>,
    count: i32,
}

impl Window {
    pub const COLOR_PRIMARY_MAIN: Color = Color::new(255, 87, 34, 255);
    pub const COLOR_PRIMARY_SIDE: Color = Color::new(230, 74, 25, 255);
    pub const COLOR_TEXT: Color = Color::new(255, 255, 255, 255);

    //================================================================

    const GRADIENT_POINT_Y: f32 = 4.0;
    const GRADIENT_SHAPE_Y: i32 = 6;
    const GRADIENT_COLOR_MAX: Color = Color::new(0, 0, 0, 99);
    const GRADIENT_COLOR_MIN: Color = Color::new(0, 0, 0, 0);

    //================================================================

    const CARD_ROUND_SHAPE: f32 = 0.25;
    const CARD_ROUND_COUNT: i32 = 4;

    //================================================================

    const TEXT_SHAPE: f32 = 24.0;
    const TEXT_SPACE: f32 = 1.0;
    const TEXT_SHIFT: f32 = 8.0;

    //================================================================

    const BUTTON_SHAPE: Vector2 = Vector2::new(160.0, 32.0);
    const BUTTON_TEXT_SHIFT: Vector2 = Vector2::new(8.0, 4.0);
    const BUTTON_SHIFT: f32 = 8.0;

    //================================================================

    const WIDGET_COUNT: usize = 64;

    //================================================================

    pub fn draw(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) -> Option<Status> {
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);

        self.begin();

        let card = 160.0;
        let draw_size = Vector2::new(
            draw.get_screen_width() as f32,
            draw.get_screen_height() as f32,
        );
        let logo_size = Vector2::new(self.logo.width as f32, self.logo.height as f32);
        let card_shape = Rectangle::new(0.0, 0.0, draw_size.x, draw_size.y - card);
        let logo_point = Vector2::new(
            draw_size.x * 0.5 - logo_size.x * 0.5,
            draw_size.y * 0.5 - logo_size.y * 0.5 - card * 0.5,
        );

        self.card_sharp(&mut draw, card_shape, Window::COLOR_PRIMARY_MAIN);

        draw.draw_texture_v(&self.logo, logo_point, Color::WHITE);

        self.point(Vector2::new(20.0, draw_size.y - card + 24.0));

        if self.button(&mut draw, "New Module") {
            let module = rfd::FileDialog::new().set_directory("/").pick_folder();

            if let Some(module) = module {
                let module = module.display().to_string();

                Script::dump(&module);

                InfoEngine {
                    safe: true,
                    path: module,
                }
                .dump();

                drop(draw);

                return Some(Status::new(handle, thread));
            }
        }
        if self.button(&mut draw, "Load Module") {
            let module = rfd::FileDialog::new().set_directory("/").pick_folder();

            if let Some(module) = module {
                let module = module.display().to_string();

                InfoEngine {
                    safe: true,
                    path: module,
                }
                .dump();

                drop(draw);

                return Some(Status::new(handle, thread));
            }
        }
        if self.button(&mut draw, "Exit Quiver") {
            return Some(Status::Closure);
        }

        None
    }

    /// Create a new Window instance.
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // Load the font.
        let font = handle
            .load_font_from_memory(thread, ".ttf", Status::FONT, Self::TEXT_SHAPE as i32, None)
            .expect("Window::new(): Could not load default font.");
        // Load the logo.
        let logo = handle
            .load_texture_from_image(
                thread,
                &Image::load_image_from_mem(".png", Status::LOGO)
                    .expect("Window::new(): Could not load texture."),
            )
            .expect("Window::new(): Could not load texture.");

        Self {
            data: [widget::Data::default(); Self::WIDGET_COUNT],
            font,
            logo,
            point: Vector2::default(),
            focus: None,
            count: i32::default(),
        }
    }

    /// Begin a new frame to reset the per-frame state.
    pub fn begin(&mut self) {
        self.point = Vector2::default();
        self.count = i32::default();
    }

    // Change the current draw cursor.
    pub fn point(&mut self, point: Vector2) {
        self.point = point;
    }

    /// Draw a sharp card.
    pub fn card_sharp(&self, draw: &mut RaylibDrawHandle, rectangle: Rectangle, color: Color) {
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

    /// Draw a round card.
    pub fn card_round(&self, draw: &mut RaylibDrawHandle, rectangle: Rectangle, color: Color) {
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

    /// Draw text.
    pub fn text(&mut self, draw: &mut RaylibDrawHandle, text: &str, color: Color) {
        self.font(draw, text, self.point, color);

        self.point.y += self.font_measure(text).y + Self::TEXT_SHIFT;
    }

    /// Draw a button. Will return true on click.
    pub fn button(&mut self, draw: &mut RaylibDrawHandle, text: &str) -> bool {
        let rectangle = Rectangle::new(
            self.point.x,
            self.point.y,
            Self::BUTTON_SHAPE.x,
            Self::BUTTON_SHAPE.y,
        );

        let state = widget::State::get(self, draw, rectangle);
        let data = widget::Data::get_mutable(self);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus);
        let data = widget::Data::get(self);

        let text_point = Vector2::new(
            rectangle.x + Self::BUTTON_TEXT_SHIFT.x,
            rectangle.y + Self::BUTTON_TEXT_SHIFT.y - data.get_point(),
        );

        self.card_round(
            draw,
            data.get_shape(&rectangle),
            data.get_color(&Window::COLOR_PRIMARY_SIDE),
        );
        self.font(draw, text, text_point, data.get_color(&Self::COLOR_TEXT));

        self.point.y += Self::BUTTON_SHAPE.y + Self::BUTTON_SHIFT;
        self.count += 1;

        state.click
    }

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

    fn font_measure(&self, text: &str) -> Vector2 {
        self.font
            .measure_text(text, Self::TEXT_SHAPE, Self::TEXT_SPACE)
    }
}

pub mod widget {
    use super::*;

    #[derive(Default, Debug)]
    pub struct State {
        pub hover: bool,
        pub focus: bool,
        pub click: bool,
    }

    impl State {
        pub fn get(window: &mut Window, draw: &RaylibDrawHandle, rectangle: Rectangle) -> Self {
            let mut state = Self::default();
            let hover = rectangle.check_collision_point_rec(draw.get_mouse_position());

            if hover {
                if let None = window.focus {
                    if draw.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                        window.focus = Some(window.count);
                    }
                }

                state.hover = true;
            }

            if let Some(focus) = window.focus {
                if focus == window.count {
                    if draw.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                        if hover {
                            state.click = true;
                        }

                        window.focus = None;
                    }

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

        // Get a reference to widget data. WARNING: will panic on widget overflow.
        pub fn get(window: &Window) -> &Self {
            window
                .data
                .get(window.count as usize)
                .expect("Data::get(): Widget overflow.")
        }

        // Get a mutable reference to widget data. WARNING: will panic on widget overflow.
        pub fn get_mutable(window: &mut Window) -> &mut Self {
            window
                .data
                .get_mut(window.count as usize)
                .expect("Data::get_mutable(): Widget overflow.")
        }

        pub fn get_point(&self) -> f32 {
            ((self.hover - 1.0) + (1.0 - self.focus)) * Self::POINT_SHIFT
        }

        pub fn get_shape(&self, rectangle: &Rectangle) -> Rectangle {
            Rectangle::new(
                rectangle.x,
                rectangle.y - self.get_point(),
                rectangle.width,
                rectangle.height,
            )
        }

        pub fn get_color(&self, color: &Color) -> Color {
            Color::new(
                (color.r as f32 * ((self.hover * Self::COLOR_UPPER) + Self::COLOR_LOWER)) as u8,
                (color.g as f32 * ((self.hover * Self::COLOR_UPPER) + Self::COLOR_LOWER)) as u8,
                (color.b as f32 * ((self.hover * Self::COLOR_UPPER) + Self::COLOR_LOWER)) as u8,
                color.a,
            )
        }

        pub fn set_hover(&mut self, draw: &RaylibDrawHandle, value: bool) {
            if value {
                self.hover += draw.get_frame_time() * Self::HOVER_SPEED;
            } else {
                self.hover -= draw.get_frame_time() * Self::HOVER_SPEED;
            }

            self.hover = self.hover.clamp(0.0, 1.0);
        }

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
