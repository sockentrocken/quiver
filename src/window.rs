use crate::engine::*;
use crate::script::*;

//================================================================

use raylib::prelude::*;

//================================================================

pub struct Window {
    data: [widget::Data; 64],
    font: Font,
    logo: Texture2D,
    point: Vector2,
    focus: Option<i32>,
    count: i32,
}

impl Window {
    pub const COLOR_MAIN: Color = Color::new(255, 87, 34, 255);
    pub const COLOR_MAIN_HEAVY: Color = Color::new(230, 74, 25, 255);
    pub const COLOR_MAIN_LIGHT: Color = Color::new(255, 204, 108, 255);
    pub const COLOR_TEXT: Color = Color::new(255, 255, 255, 255);
    pub const COLOR_TEXT_MAIN: Color = Color::new(33, 33, 33, 255);
    pub const COLOR_TEXT_SIDE: Color = Color::new(117, 117, 117, 255);

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

    const TOGGLE_SHAPE: Vector2 = Vector2::new(24.0, 24.0);
    const TOGGLE_SHIFT: f32 = 8.0;

    //================================================================

    const SLIDER_CIRCLE_SHAPE: f32 = 8.0;
    const SLIDER_FOCUS_POINT: Vector2 = Vector2::new(0.0, -32.0);
    const SLIDER_FOCUS_SHAPE: f32 = 20.0;
    const SLIDER_SHAPE_MAX: Vector2 = Vector2::new(160.0, 24.0);
    const SLIDER_SHAPE_MIN: Vector2 = Vector2::new(160.0, 4.0);
    const SLIDER_SHIFT: f32 = 8.0;

    //================================================================

    const RECORD_SHAPE_MAX: Vector2 = Vector2::new(320.0, 24.0);
    const RECORD_SHAPE_MIN: Vector2 = Vector2::new(320.0, 4.0);
    const RECORD_SHAPE_CARET: Vector2 = Vector2::new(2.0, 16.0);
    const RECORD_SHIFT: f32 = 8.0;

    //================================================================

    const WIDGET_COUNT: usize = 64;

    //================================================================

    /// Create a new Window instance.
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // Load the font.
        let font = handle
            .load_font_from_memory(thread, ".ttf", Engine::FONT, Self::TEXT_SHAPE as i32, None)
            .expect("Window::new(): Could not load default font.");
        // Load the logo.
        let logo = handle
            .load_texture_from_image(
                &thread,
                &Image::load_image_from_mem(".png", Engine::LOGO)
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
            data.get_color(&Window::COLOR_MAIN_HEAVY),
        );
        self.font(draw, text, text_point, data.get_color(&Self::COLOR_TEXT));

        self.point.y += Self::BUTTON_SHAPE.y + Self::BUTTON_SHIFT;
        self.count += 1;

        state.click
    }

    // Draw a toggle.
    pub fn toggle(&mut self, draw: &mut RaylibDrawHandle, text: &str, value: &mut bool) {
        let rectangle_max = Rectangle::new(
            self.point.x,
            self.point.y,
            Self::TOGGLE_SHAPE.x,
            Self::TOGGLE_SHAPE.y,
        );
        let rectangle_min = Rectangle::new(
            self.point.x + (Self::TOGGLE_SHAPE.x * 0.25),
            self.point.y + (Self::TOGGLE_SHAPE.y * 0.25),
            Self::TOGGLE_SHAPE.x - (Self::TOGGLE_SHAPE.x * 0.5),
            Self::TOGGLE_SHAPE.y - (Self::TOGGLE_SHAPE.y * 0.5),
        );

        let state = widget::State::get(self, draw, rectangle_max);
        let data = widget::Data::get_mutable(self);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus);
        let data = widget::Data::get(self);

        let text_point = Vector2::new(
            rectangle_max.x + rectangle_max.width + Self::TOGGLE_SHIFT,
            rectangle_max.y - data.get_point(),
        );

        // Click event, change value.
        if state.click {
            *value = !*value;
        }

        self.card_round(
            draw,
            data.get_shape(&rectangle_max),
            data.get_color(&Window::COLOR_MAIN_HEAVY),
        );

        if *value {
            self.card_round(
                draw,
                data.get_shape(&rectangle_min),
                data.get_color(&Window::COLOR_MAIN_LIGHT),
            );
        }

        self.font(
            draw,
            text,
            text_point,
            data.get_color(&Self::COLOR_TEXT_MAIN),
        );

        self.point.y += Self::TOGGLE_SHAPE.y + Self::TOGGLE_SHIFT;
        self.count += 1;
    }

    // Draw a slider.
    pub fn slider(
        &mut self,
        draw: &mut RaylibDrawHandle,
        text: &str,
        value: &mut f32,
        min: f32,
        max: f32,
    ) {
        let percent = (*value - min) / (max - min);
        let rectangle_hit = Rectangle::new(
            self.point.x,
            self.point.y,
            Self::SLIDER_SHAPE_MAX.x,
            Self::SLIDER_SHAPE_MAX.y,
        );
        let rectangle_max = Rectangle::new(
            self.point.x,
            self.point.y + (Self::SLIDER_SHAPE_MAX.y - Self::SLIDER_SHAPE_MIN.y) * 0.5,
            Self::SLIDER_SHAPE_MIN.x,
            Self::SLIDER_SHAPE_MIN.y,
        );
        let rectangle_min = Rectangle::new(
            self.point.x,
            self.point.y + (Self::SLIDER_SHAPE_MAX.y - Self::SLIDER_SHAPE_MIN.y) * 0.5,
            Self::SLIDER_SHAPE_MIN.x * percent,
            Self::SLIDER_SHAPE_MIN.y,
        );

        let state = widget::State::get(self, draw, rectangle_hit);
        let data = widget::Data::get_mutable(self);
        data.set_hover(draw, state.hover || state.focus);
        data.set_focus(draw, state.focus);
        let data = widget::Data::get(self);

        let text_point = Vector2::new(
            self.point.x + Self::SLIDER_SHAPE_MAX.x + Self::SLIDER_SHIFT,
            self.point.y - data.get_point(),
        );

        if state.focus {
            let mouse = (draw.get_mouse_x() as f32 - rectangle_hit.x)
                / ((rectangle_hit.x + rectangle_hit.width) - rectangle_hit.x);

            let mouse = mouse.clamp(0.0, 1.0);

            *value = mouse * (max - min) + min;
        }

        self.card_sharp(
            draw,
            data.get_shape(&rectangle_max),
            data.get_color(&Window::COLOR_MAIN_LIGHT),
        );
        self.card_sharp(
            draw,
            data.get_shape(&rectangle_min),
            data.get_color(&Window::COLOR_MAIN_HEAVY),
        );

        if state.focus {
            let pin = Vector2::new(
                self.point.x + Self::SLIDER_FOCUS_POINT.x + rectangle_max.width * percent,
                self.point.y + Self::SLIDER_FOCUS_POINT.y - data.get_point(),
            );

            draw.draw_circle_v(
                pin,
                Self::SLIDER_FOCUS_SHAPE,
                data.get_color(&Self::COLOR_MAIN_HEAVY),
            );

            draw.draw_triangle(
                pin + Vector2::new(-Self::SLIDER_FOCUS_SHAPE, 0.0),
                pin + Vector2::new(0.0, -Self::SLIDER_FOCUS_POINT.y),
                pin + Vector2::new(Self::SLIDER_FOCUS_SHAPE, 0.0),
                data.get_color(&Self::COLOR_MAIN_HEAVY),
            );

            let value = &format!("{value:.0}");

            let measure = self.font_measure(value);

            self.font(draw, value, pin - measure * 0.5, Self::COLOR_TEXT_MAIN);
        }

        let pin = Vector2::new(
            self.point.x + Self::SLIDER_SHAPE_MAX.x * percent,
            self.point.y + Self::SLIDER_SHAPE_MAX.y * 0.5 - data.get_point(),
        );

        draw.draw_circle_v(
            pin,
            Self::SLIDER_CIRCLE_SHAPE,
            data.get_color(&Self::COLOR_MAIN_HEAVY),
        );

        self.font(draw, text, text_point, Self::COLOR_TEXT_MAIN);

        self.point.y += Self::SLIDER_SHAPE_MAX.y + Self::SLIDER_SHIFT;
        self.count += 1;
    }

    // Draw a record.
    pub fn record(&mut self, draw: &mut RaylibDrawHandle, text: &str, value: &mut String) {
        let rectangle_hit = Rectangle::new(
            self.point.x,
            self.point.y,
            Self::RECORD_SHAPE_MAX.x,
            Self::RECORD_SHAPE_MAX.y,
        );
        let rectangle_max = Rectangle::new(
            self.point.x,
            self.point.y + Self::RECORD_SHAPE_MAX.y - Self::RECORD_SHAPE_MIN.y,
            Self::RECORD_SHAPE_MIN.x,
            Self::RECORD_SHAPE_MIN.y,
        );

        let state = widget::State::get(self, draw, rectangle_hit);
        let data = widget::Data::get_mutable(self);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus);
        let data = widget::Data::get(self);

        let text_max_point = Vector2::new(
            self.point.x,
            self.point.y - data.get_point() - Self::RECORD_SHAPE_MIN.y,
        );
        let text_min_point = Vector2::new(
            self.point.x + Self::RECORD_SHAPE_MAX.x + Self::RECORD_SHIFT,
            self.point.y - data.get_point(),
        );

        self.card_sharp(
            draw,
            data.get_shape(&rectangle_max),
            data.get_color(&Window::COLOR_MAIN_HEAVY),
        );
        self.font(
            draw,
            value,
            text_max_point,
            data.get_color(&Self::COLOR_TEXT_SIDE),
        );
        self.font(
            draw,
            text,
            text_min_point,
            data.get_color(&Self::COLOR_TEXT_MAIN),
        );

        unsafe {
            if state.hover {
                let key = ffi::GetCharPressed();

                if draw.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
                    || ffi::IsKeyPressedRepeat(KeyboardKey::KEY_BACKSPACE as i32)
                {
                    value.pop();
                } else if key != 0 {
                    value.push(key as u8 as char);
                }

                let measure = self.font_measure(value);

                self.card_sharp(
                    draw,
                    data.get_shape(&Rectangle::new(
                        self.point.x + measure.x,
                        self.point.y,
                        Self::RECORD_SHAPE_CARET.x,
                        Self::RECORD_SHAPE_CARET.y,
                    )),
                    data.get_color(&Window::COLOR_MAIN_HEAVY),
                );
            }
        }

        self.point.y += Self::RECORD_SHAPE_MAX.y + Self::RECORD_SHIFT;
        self.count += 1;
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

mod widget {
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

        // Reset all state data.
        pub fn clear(window: &mut Window) {
            window.data.fill_with(Default::default);
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

#[derive(Default, Clone)]
pub enum WizardState {
    #[default]
    Main,
    NewModule,
    NewSystem,
    NewWindow,
    LoadModule,
}

#[derive(Default, Clone)]
pub struct Wizard {
    warn: String,
    path: String,
    info: InfoModule,
    state: WizardState,
}

impl Wizard {
    const FILE_INFO: &'static str = include_str!("asset/info.lua");
    const FILE_MAIN: &'static str = include_str!("asset/main.lua");
    const FILE_META: &'static str = include_str!("asset/meta.lua");
    const FILE_BASE: &'static str = include_str!("asset/base.lua");
    const NAME_MAIN: &'static str = "main.lua";
    const NAME_META: &'static str = "meta.lua";
    const NAME_BASE: &'static str = "base.lua";

    #[rustfmt::skip]
    pub fn draw(&mut self, engine: &mut Engine, draw: &mut RaylibDrawHandle, window: &mut Window) {
        window.begin();

        let size = draw.get_screen_width();

        match self.state {
            WizardState::Main => {
                let card_y = 160;

                window.card_sharp(
                    draw,
                    Rectangle::new(
                        0.0,
                        0.0,
                        draw.get_screen_width() as f32,
                        (draw.get_screen_height() - card_y) as f32,
                    ),
                    Window::COLOR_MAIN,
                );

                window.point(Vector2::new(
                    20.0,
                    (draw.get_screen_height() - card_y + 24) as f32,
                ));

                if window.button(draw, "New Module") {
                    self.state = WizardState::NewModule;
                    widget::Data::clear(window);
                }
                window.button(draw, "Load Module");
                if window.button(draw, "Exit Quiver") {
                    engine.status = Status::Closure;
                }

                draw.draw_texture_v(
                    &window.logo,
                    Vector2::new(
                        draw.get_screen_width() as f32 * 0.5 - window.logo.width as f32 * 0.5,
                        draw.get_screen_height() as f32 * 0.5
                            - window.logo.height as f32 * 0.5
                            - card_y as f32 * 0.5,
                    ),
                    Color::WHITE,
                );

            }
            WizardState::NewModule => {
                window.card_sharp(
                    draw,
                    Rectangle::new(0.0, 0.0, size as f32, 48.0),
                    Window::COLOR_MAIN,
                );

                window.point(Vector2::new(20.0, 12.0));
                window.text(draw, "Module Data", Window::COLOR_TEXT);

                window.point(Vector2::new(20.0, 72.0));
                window.record(draw, "Module Path", &mut self.path);

                window.point(Vector2::new(20.0, (draw.get_screen_height() - 96) as f32));
                if window.button(draw, "Next") {
                    self.state = WizardState::NewSystem;
                }
                if window.button(draw, "Back") {
                    self.state = WizardState::Main;
                }
            }
            WizardState::NewSystem => {
                window.card_sharp(
                    draw,
                    Rectangle::new(0.0, 0.0, size as f32, 48.0),
                    Window::COLOR_MAIN,
                );

                window.point(Vector2::new(20.0, 12.0));
                window.text(draw, "System Data", Window::COLOR_TEXT);

                window.point(Vector2::new(20.0, 72.0));
                window.toggle(draw, "Model",   &mut self.info.system.model);
                window.toggle(draw, "Texture", &mut self.info.system.texture);
                window.toggle(draw, "Image",   &mut self.info.system.image);
                window.toggle(draw, "Sound",   &mut self.info.system.sound);
                window.toggle(draw, "Music",   &mut self.info.system.music);
                window.toggle(draw, "Font",    &mut self.info.system.font);
                window.toggle(draw, "Shader",  &mut self.info.system.shader);

                window.point(Vector2::new(20.0, (draw.get_screen_height() - 96) as f32));
                if window.button(draw, "Next") {
                    self.state = WizardState::NewWindow;
                }
                if window.button(draw, "Back") {
                    self.state = WizardState::NewModule;
                }
            }
            WizardState::NewWindow => {
                window.card_sharp(
                    draw,
                    Rectangle::new(0.0, 0.0, size as f32, 48.0),
                    Window::COLOR_MAIN,
                );

                window.point(Vector2::new(20.0, 12.0));
                window.text(draw, "Window Data", Window::COLOR_TEXT);

                window.point(Vector2::new(20.0, 72.0));
                window.toggle(draw, "Fullscreen",    &mut self.info.window.fullscreen);
                window.toggle(draw, "Borderless",    &mut self.info.window.borderless);
                window.toggle(draw, "Vertical Sync", &mut self.info.window.sync);
                window.toggle(draw, "MSAA",          &mut self.info.window.msaa);
                window.toggle(draw, "Resize",        &mut self.info.window.resize);
                window.toggle(draw, "Hidden",        &mut self.info.window.hidden);
                window.toggle(draw, "Minimized",     &mut self.info.window.minimize);
                window.toggle(draw, "Maximized",     &mut self.info.window.maximize);
                window.toggle(draw, "No Decor",      &mut self.info.window.no_decor);
                window.toggle(draw, "No Focus",      &mut self.info.window.no_focus);
                window.toggle(draw, "On Front",      &mut self.info.window.on_front);
                window.toggle(draw, "Mouse Pass",    &mut self.info.window.mouse_pass);
                window.toggle(draw, "Draw Alpha",    &mut self.info.window.draw_alpha);
                window.toggle(draw, "High Scale",    &mut self.info.window.high_scale);
                window.record(draw, "Window Name",   &mut self.info.window.name);
                //window.record(draw, "Window Icon",   &mut self.info.window.icon);
                window.slider(draw, "Window Rate",   &mut self.info.window.rate, 60.0, 300.0);

                window.point(Vector2::new(20.0, (draw.get_screen_height() - 96) as f32));
                if window.button(draw, "Finish") {}
                if window.button(draw, "Back") {
                    self.state = WizardState::NewSystem;
                }
            }
            WizardState::LoadModule => todo!(),
        }
    }

    pub fn get_info(&self) -> String {
        let mut result = Self::FILE_INFO.to_string();
        result = result.replace("{path}", &self.path);

        let system = &self.info.system;

        result = result.replace("{model}", &system.model.to_string());
        result = result.replace("{texture}", &system.texture.to_string());
        result = result.replace("{image}", &system.image.to_string());
        result = result.replace("{sound}", &system.sound.to_string());
        result = result.replace("{music}", &system.music.to_string());
        result = result.replace("{font}", &system.font.to_string());
        result = result.replace("{shader}", &system.shader.to_string());

        let window = &self.info.window;

        //result = result.replace("{fullscreen}", &self.fullscreen.to_string());
        //result = result.replace("{borderless}", &self.borderless.to_string());
        //result = result.replace("{sync}", &self.sync.to_string());
        //result = result.replace("{msaa}", &self.msaa.to_string());
        //result = result.replace("{resize}", &self.resize.to_string());
        //result = result.replace("{hidden}", &self.hidden.to_string());
        //result = result.replace("{minimize}", &self.minimize.to_string());
        //result = result.replace("{maximize}", &self.maximize.to_string());
        //result = result.replace("{no_decor}", &self.no_decor.to_string());
        //result = result.replace("{no_focus}", &self.no_focus.to_string());
        //result = result.replace("{on_front}", &self.on_front.to_string());
        //result = result.replace("{run_hidden}", &self.run_hidden.to_string());
        //result = result.replace("{mouse_pass}", &self.mouse_pass.to_string());
        //result = result.replace("{draw_alpha}", &self.draw_alpha.to_string());
        //result = result.replace("{high_scale}", &self.high_scale.to_string());
        //result = result.replace("{window_name}", &self.name.to_string());
        //result = result.replace("{window_rate}", &self.rate.to_string());

        result
    }

    pub fn get_main(&self) -> String {
        let mut result = Self::FILE_MAIN.to_string();
        result = result.replace("{path}", &self.path);

        result
    }

    pub fn make(&self) -> Result<(), String> {
        if self.path.trim().is_empty() {
            return Err("Path cannot be empty.".to_string());
        }

        //folder::write(&self.path)?;

        /*
        file::write(
            &format!("{}/{}", self.path, InfoModule::FILE_NAME),
            self.get_info(),
        )?;
        file::write(
            &format!("{}/{}", self.path, Self::NAME_MAIN),
            self.get_main(),
        )?;
        file::write(
            &format!("{}/{}", self.path, Self::NAME_META),
            Self::FILE_META,
        )?;
        file::write(
            &format!("{}/{}", self.path, Self::NAME_BASE),
            Self::FILE_BASE,
        )?;
        */

        /*
        let info = InfoEngine {
            safe: true,
            path: self.path.clone(),
        };
        */

        //info.dump("./")?;

        raylib::core::misc::open_url(&self.path);

        Ok(())
    }
}
