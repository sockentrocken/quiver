use crate::engine::*;
use crate::script::*;

//================================================================

use raylib::prelude::*;

//================================================================

pub struct Window {
    data: [Widget; 64],
    font: Font,
    pub card: Texture2D,
    point: Vector2,
    focus: Option<i32>,
    count: i32,
}

#[derive(Default, Debug)]
pub struct State {
    hover: bool,
    focus: bool,
    click: bool,
}

#[derive(Copy, Clone, Default)]
pub struct Widget {
    hover: f32,
    focus: f32,
}

impl Window {
    pub const COLOR_MAIN: Color = Color::new(255, 87, 34, 255);
    pub const COLOR_MAIN_HEAVY: Color = Color::new(230, 74, 25, 255);
    pub const COLOR_MAIN_LIGHT: Color = Color::new(255, 204, 108, 255);
    pub const COLOR_TEXT: Color = Color::new(255, 255, 255, 255);
    pub const COLOR_TEXT_MAIN: Color = Color::new(33, 33, 33, 255);
    pub const COLOR_TEXT_SIDE: Color = Color::new(117, 117, 117, 255);

    //================================================================

    const GRADIENT_POINT_Y: f32 = 12.0;
    const GRADIENT_SHAPE_Y: i32 = 6;
    const GRADIENT_COLOR_MAX: Color = Color::new(0, 0, 0, 66);
    const GRADIENT_COLOR_MIN: Color = Color::new(0, 0, 0, 0);

    //================================================================

    const CARD_ROUND_SHAPE: f32 = 0.25;
    const CARD_ROUND_COUNT: i32 = 4;

    //================================================================

    const TEXT_SHAPE: f32 = 24.0;
    const TEXT_SPACE: f32 = 1.0;
    const TEXT_SHIFT: f32 = 8.0;

    //================================================================

    const BUTTON_SHAPE: Vector2 = Vector2::new(96.0, 24.0);
    const BUTTON_TEXT_SHIFT: Vector2 = Vector2::new(8.0, 4.0);
    const BUTTON_SHIFT: f32 = 8.0;

    //================================================================

    const TOGGLE_SHAPE: Vector2 = Vector2::new(24.0, 24.0);
    const TOGGLE_SHIFT: f32 = 8.0;

    //================================================================

    const SLIDER_SHAPE: Vector2 = Vector2::new(96.0, 24.0);
    const SLIDER_SHIFT: f32 = 8.0;

    //================================================================

    const RECORD_SHAPE: Vector2 = Vector2::new(96.0, 24.0);
    const RECORD_SHIFT: f32 = 8.0;

    //================================================================

    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let font = handle
            .load_font_from_memory(&thread, ".ttf", Engine::FONT, Self::TEXT_SHAPE as i32, None)
            .expect("window::new(): Could not load default font.");

        Self {
            data: [Widget::default(); 64],
            font,
            card: Self::load_texture(handle, thread, Engine::CARD),
            point: Vector2::default(),
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

    pub fn data_get(&self) -> &Widget {
        self.data
            .get(self.count as usize)
            .expect("window::data_get(): Widget overflow.")
    }

    pub fn data_get_mut(&mut self) -> &mut Widget {
        self.data
            .get_mut(self.count as usize)
            .expect("window::data_get_mut(): Widget overflow.")
    }

    pub fn data_clear(&mut self) {
        self.data.fill_with(Default::default);
    }

    pub fn card_sharp(&self, draw: &mut RaylibDrawHandle, content: Rectangle, color: Color) {
        draw.draw_rectangle_gradient_v(
            content.x as i32,
            (content.y + content.height) as i32,
            content.width as i32,
            Self::GRADIENT_SHAPE_Y,
            Self::GRADIENT_COLOR_MAX,
            Self::GRADIENT_COLOR_MIN,
        );

        draw.draw_rectangle_rec(content, color);
    }

    pub fn card_round(&self, draw: &mut RaylibDrawHandle, content: Rectangle, color: Color) {
        draw.draw_rectangle_gradient_v(
            content.x as i32,
            (content.y + content.height - Self::GRADIENT_POINT_Y) as i32,
            content.width as i32,
            Self::GRADIENT_SHAPE_Y + Self::GRADIENT_POINT_Y as i32,
            Self::GRADIENT_COLOR_MAX,
            Self::GRADIENT_COLOR_MIN,
        );

        draw.draw_rectangle_rounded(
            content,
            Self::CARD_ROUND_SHAPE,
            Self::CARD_ROUND_COUNT,
            color,
        );
    }

    pub fn font(&mut self, draw: &mut RaylibDrawHandle, text: &str, point: Vector2, color: Color) {
        draw.draw_text_ex(
            &self.font,
            text,
            point,
            Self::TEXT_SHAPE,
            Self::TEXT_SPACE,
            color,
        );
    }

    pub fn font_measure(&mut self, text: &str) -> Vector2 {
        self.font
            .measure_text(text, Self::TEXT_SHAPE, Self::TEXT_SPACE)
    }

    pub fn text(&mut self, draw: &mut RaylibDrawHandle, text: &str, color: Color) {
        self.font(draw, text, self.point, color);

        self.point.y += self.font_measure(text).y + Self::TEXT_SHIFT;
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
        let content = Rectangle::new(
            self.point.x,
            self.point.y,
            128.0 + Self::BUTTON_TEXT_SHIFT.x * 2.0,
            24.0 + Self::BUTTON_TEXT_SHIFT.y * 2.0,
        );

        let result = self.state(draw, content);

        self.set_alpha(draw, result.hover);
        self.set_scale(draw, result.focus);

        let data = self.data_get();

        self.card_round(
            draw,
            Self::get_scale(&content, data.focus, data.hover),
            Self::get_alpha(&Window::COLOR_MAIN_HEAVY, data.hover),
        );
        self.font(
            draw,
            text,
            Vector2::new(
                content.x + Self::BUTTON_TEXT_SHIFT.x,
                content.y - Self::get_point(data.focus, data.hover) + Self::BUTTON_TEXT_SHIFT.y,
            ),
            Self::get_alpha(&Self::COLOR_TEXT, data.hover),
        );

        self.point.y += content.height + Self::BUTTON_SHIFT;
        self.count += 1;

        result.click
    }

    pub fn toggle(&mut self, draw: &mut RaylibDrawHandle, text: &str, value: &mut bool) {
        let content_max = Rectangle::new(self.point.x, self.point.y, 24.0, 24.0);
        let state = self.state(draw, content_max);

        self.set_alpha(draw, state.hover);
        self.set_scale(draw, state.focus);

        let data = self.data_get();

        if state.click {
            *value = !*value;
        }

        self.card_round(
            draw,
            content_max,
            Self::get_alpha(&Window::COLOR_MAIN_HEAVY, data.hover),
        );

        if *value {
            /*
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
                Self::get_alpha(&Color::WHITE, data.hover),
            );
            */
        }

        self.font(
            draw,
            text,
            Vector2::new(
                content_max.width + content_max.x + Self::TOGGLE_SHIFT,
                content_max.y,
            ),
            Self::get_alpha(&Self::COLOR_TEXT_MAIN, data.hover),
        );

        self.point.y += content_max.height + Self::TOGGLE_SHIFT;
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

        self.set_alpha(draw, state.hover || state.focus);
        self.set_scale(draw, state.focus);

        let data = self.data_get();

        if state.focus {
            let mouse = (draw.get_mouse_x() as f32 - content_hit.x)
                / ((content_hit.x + content_hit.width) - content_hit.x);

            let mouse = mouse.clamp(0.0, 1.0);

            *value = mouse * (max - min) + min;
        }

        self.card_sharp(
            draw,
            content_max,
            Self::get_alpha(&Window::COLOR_MAIN_LIGHT, data.hover),
        );
        self.card_sharp(
            draw,
            content_min,
            Self::get_alpha(&Window::COLOR_MAIN_HEAVY, data.hover),
        );

        if state.focus {
            let pin = Vector2::new(
                self.point.x + content_max.width * percent,
                self.point.y - 32.0,
            );

            draw.draw_circle_v(
                pin,
                20.0,
                Self::get_alpha(&Self::COLOR_MAIN_HEAVY, data.hover),
            );

            draw.draw_triangle(
                pin + Vector2::new(-20.0, 6.0),
                pin + Vector2::new(0.0, 32.0),
                pin + Vector2::new(20.0, 6.0),
                Self::get_alpha(&Self::COLOR_MAIN_HEAVY, data.hover),
            );

            let value = &format!("{value:.0}");

            let measure = self.font_measure(value);

            self.font(draw, value, pin - measure * 0.5, Self::COLOR_TEXT_MAIN);
        } else {
            let pin = Vector2::new(
                self.point.x + content_max.width * percent,
                self.point.y + 10.0,
            );

            draw.draw_circle_v(
                pin,
                8.0,
                Self::get_alpha(&Self::COLOR_MAIN_HEAVY, data.hover),
            );
        }

        self.font(
            draw,
            text,
            Vector2::new(
                content_max.width + content_max.x + Self::SLIDER_SHIFT * 2.0,
                self.point.y,
            ),
            Self::COLOR_TEXT_MAIN,
        );

        self.point.y += content_hit.height + Self::SLIDER_SHIFT;
        self.count += 1;
    }

    pub fn record(&mut self, draw: &mut RaylibDrawHandle, text: &str, value: &mut String) {
        let content_hit = Rectangle::new(self.point.x, self.point.y, 160.0, 24.0);
        let content_max = Rectangle::new(self.point.x, self.point.y + 20.0, 160.0, 4.0);
        let state = self.state(draw, content_hit);

        self.card_sharp(draw, content_max, Window::COLOR_MAIN_HEAVY);
        self.font(
            draw,
            value,
            self.point + Vector2::new(0.0, -4.0),
            Self::COLOR_TEXT_SIDE,
        );
        self.font(
            draw,
            text,
            self.point + Vector2::new(content_hit.width + Self::RECORD_SHIFT, 0.0),
            Self::COLOR_TEXT_MAIN,
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
                    Rectangle::new(self.point.x + measure.x, self.point.y, 2.0, 16.0),
                    Window::COLOR_MAIN_HEAVY,
                );
            }
        }

        self.point.y += content_hit.height + Self::SLIDER_SHIFT;
        self.count += 1;
    }

    fn load_texture(handle: &mut RaylibHandle, thread: &RaylibThread, texture: &[u8]) -> Texture2D {
        handle
            .load_texture_from_image(
                &thread,
                &Image::load_image_from_mem(".png", texture)
                    .expect("window::new(): Could not load texture."),
            )
            .expect("window::new(): Could not load texture.")
    }

    fn get_point(scale: f32, alpha: f32) -> f32 {
        ((alpha - 1.0) + scale) * 4.0
    }

    fn get_alpha(color: &Color, alpha: f32) -> Color {
        Color::new(
            (color.r as f32 * ((alpha * 0.25) + 0.75)) as u8,
            (color.g as f32 * ((alpha * 0.25) + 0.75)) as u8,
            (color.b as f32 * ((alpha * 0.25) + 0.75)) as u8,
            color.a,
        )
    }

    fn get_scale(content: &Rectangle, scale: f32, alpha: f32) -> Rectangle {
        Rectangle::new(
            content.x,
            content.y - Self::get_point(scale, alpha),
            content.width,
            content.height,
        )
    }

    fn set_alpha(&mut self, draw: &RaylibDrawHandle, value: bool) {
        let data = self.data_get_mut();

        if value {
            data.hover += draw.get_frame_time() * 16.0;
        } else {
            data.hover -= draw.get_frame_time() * 16.0;
        }

        data.hover = data.hover.clamp(0.0, 1.0);
    }

    fn set_scale(&mut self, draw: &RaylibDrawHandle, value: bool) {
        let data = self.data_get_mut();

        if value {
            data.focus -= draw.get_frame_time() * 16.0;
        } else {
            data.focus += draw.get_frame_time() * 16.0;
        }

        data.focus = data.focus.clamp(0.0, 1.0);
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
    pub fn draw(&mut self, engine: &Engine, draw: &mut RaylibDrawHandle, window: &mut Window) {
        window.begin();

        let size = draw.get_screen_width();

        match self.state {
            WizardState::Main => {
                let card_y = 240;

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
                    window.data_clear();
                }
                window.button(draw, "Load Module");
                window.button(draw, "Exit Quiver");
                window.button(draw, "GitHub");
                window.button(draw, "Discord");

                draw.draw_texture_v(
                    &window.card,
                    Vector2::new(
                        draw.get_screen_width() as f32 * 0.5 - window.card.width as f32 * 0.5,
                        draw.get_screen_height() as f32 * 0.5
                            - window.card.height as f32 * 0.5
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
