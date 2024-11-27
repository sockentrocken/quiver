use crate::engine::*;
use crate::module::*;

//================================================================

use raylib::prelude::*;

//================================================================

#[derive(Default, Clone)]
pub enum WizardState {
    #[default]
    Main,
    NewModule,
    NewSystem,
    NewWindow,
    LoadModule,
}

#[derive(Default)]
pub struct Wizard {
    pub warn: String,
    pub path: String,
    pub info: InfoModule,
    pub state: WizardState,
}

use crate::utility::*;

impl Wizard {
    pub const FILE_INFO: &'static str = include_str!("asset/module/info.lua");
    pub const FILE_MAIN: &'static str = include_str!("asset/module/main.lua");
    pub const FILE_META: &'static str = include_str!("asset/module/meta.lua");
    pub const FILE_BASE: &'static str = include_str!("asset/module/base.lua");
    pub const NAME_MAIN: &'static str = "main.lua";
    pub const NAME_META: &'static str = "meta.lua";
    pub const NAME_BASE: &'static str = "base.lua";

    #[rustfmt::skip]
    pub fn draw(engine: &Engine, draw: &mut RaylibDrawHandle, interface: &mut Interface) {
        interface.begin();

        let size = draw.get_screen_width();

        interface.card_sharp(
            draw,
            Rectangle::new(0.0, 0.0, size as f32, 48.0),
            Interface::COLOR_PRIMARY,
        );

        match interface.wizard.state {
            WizardState::Main => {
                let card_y = 160;

                interface.card_sharp(
                    draw,
                    Rectangle::new(
                        0.0,
                        0.0,
                        draw.get_screen_width() as f32,
                        (draw.get_screen_height() - card_y) as f32,
                    ),
                    Interface::COLOR_PRIMARY,
                );

                interface.point(Vector2::new(
                    20.0,
                    (draw.get_screen_height() - card_y + 24) as f32,
                ));

                if interface.button(draw, "New Module") {
                    interface.wizard.state = WizardState::NewModule;
                    interface.data_clear();
                }
                interface.button(draw, "Load Module");
                interface.button(draw, "Exit Quiver");

                draw.draw_texture_v(
                    &interface.card,
                    Vector2::new(
                        draw.get_screen_width() as f32 * 0.5 - interface.card.width as f32 * 0.5,
                        draw.get_screen_height() as f32 * 0.5
                            - interface.card.height as f32 * 0.5
                            - card_y as f32 * 0.5,
                    ),
                    Color::WHITE,
                );
            }
            WizardState::NewModule => {
                interface.point(Vector2::new(20.0, 12.0));
                interface.text(draw, "New Module", Interface::COLOR_TEXT);

                interface.point(Vector2::new(20.0, 72.0));
                //interface.record(draw, "Module Path", &mut interface.wizard.path);
                //interface.record(draw, "Module Name", &mut interface.wizard.info.name);
                //interface.record(draw, "Module Info", &mut interface.wizard.info.info);

                interface.point(Vector2::new(20.0, (draw.get_screen_height() - 96) as f32));
                if interface.button(draw, "Next") {
                    interface.wizard.state = WizardState::NewSystem;
                    interface.data_clear();
                }
                if interface.button(draw, "Back") {
                    interface.wizard.state = WizardState::Main;
                    interface.data_clear();
                }
            }
            WizardState::NewSystem => {
                interface.point(Vector2::new(20.0, 12.0));
                interface.text(draw, "System Data", Interface::COLOR_TEXT);

                interface.point(Vector2::new(20.0, 72.0));
                //interface.toggle(draw, "Model",   &mut system.model);
                //interface.toggle(draw, "Texture", &mut system.texture);
                //interface.toggle(draw, "Image",   &mut system.image);
                //interface.toggle(draw, "Sound",   &mut system.sound);
                //interface.toggle(draw, "Music",   &mut system.music);
                //interface.toggle(draw, "Font",    &mut system.font);
                //interface.toggle(draw, "Shader",  &mut system.shader);

                interface.point(Vector2::new(20.0, (draw.get_screen_height() - 96) as f32));
                if interface.button(draw, "Next") {
                    interface.wizard.state = WizardState::NewWindow;
                }
                if interface.button(draw, "Back") {
                    interface.wizard.state = WizardState::NewModule;
                }
            }
            WizardState::NewWindow => {
                interface.point(Vector2::new(20.0, 12.0));
                interface.text(draw, "New Window", Interface::COLOR_TEXT);

                interface.point(Vector2::new(20.0, 72.0));
                //interface.toggle(draw, "Fullscreen",    &mut window.fullscreen);
                //interface.toggle(draw, "Borderless",    &mut window.borderless);
                //interface.toggle(draw, "Vertical Sync", &mut window.sync);
                //interface.toggle(draw, "MSAA",          &mut window.msaa);
                //interface.toggle(draw, "Resize",        &mut window.resize);
                //interface.toggle(draw, "Hidden",        &mut window.hidden);
                //interface.toggle(draw, "Minimized",     &mut window.minimize);
                //interface.toggle(draw, "Maximized",     &mut window.maximize);
                //interface.toggle(draw, "No Decor",      &mut window.no_decor);
                //interface.toggle(draw, "No Focus",      &mut window.no_focus);
                //interface.toggle(draw, "On Front",      &mut window.on_front);
                //interface.toggle(draw, "Mouse Pass",    &mut window.mouse_pass);
                //interface.toggle(draw, "Draw Alpha",    &mut window.draw_alpha);
                //interface.toggle(draw, "High Scale",    &mut window.high_scale);
                //interface.record(draw, "Window Name",   &mut window.name);
                //interface.record(draw, "Window Icon",   &mut window.icon);
                //interface.slider(draw, "Window Rate",   &mut window.rate, 60.0, 300.0);

                interface.point(Vector2::new(20.0, (draw.get_screen_height() - 96) as f32));
                if interface.button(draw, "Next") {}
                if interface.button(draw, "Back") {
                    interface.wizard.state = WizardState::NewSystem;
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

        result = result.replace("{fullscreen}", &window.fullscreen.to_string());
        result = result.replace("{borderless}", &window.borderless.to_string());
        result = result.replace("{sync}", &window.sync.to_string());
        result = result.replace("{msaa}", &window.msaa.to_string());
        result = result.replace("{resize}", &window.resize.to_string());
        result = result.replace("{hidden}", &window.hidden.to_string());
        result = result.replace("{minimize}", &window.minimize.to_string());
        result = result.replace("{maximize}", &window.maximize.to_string());
        result = result.replace("{no_decor}", &window.no_decor.to_string());
        result = result.replace("{no_focus}", &window.no_focus.to_string());
        result = result.replace("{on_front}", &window.on_front.to_string());
        result = result.replace("{run_hidden}", &window.run_hidden.to_string());
        result = result.replace("{mouse_pass}", &window.mouse_pass.to_string());
        result = result.replace("{draw_alpha}", &window.draw_alpha.to_string());
        result = result.replace("{high_scale}", &window.high_scale.to_string());
        result = result.replace("{window_name}", &window.name.to_string());
        result = result.replace("{window_rate}", &window.rate.to_string());

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

        folder::write(&self.path)?;

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

        let info = InfoEngine {
            safe: true,
            path: self.path.clone(),
        };

        info.dump("./")?;

        raylib::core::misc::open_url(&self.path);

        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Widget {
    alpha: f32,
    scale: f32,
}

impl Default for Widget {
    fn default() -> Self {
        Self {
            alpha: 0.0,
            scale: 1.0,
        }
    }
}

pub struct Interface {
    data: [Widget; 64],
    font: Font,
    pub card: Texture2D,
    check: Texture2D,
    point: Vector2,
    focus: Option<i32>,
    count: i32,
    wizard: Wizard,
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
    pub const COLOR_PRIMARY: Color = Color::new(255, 87, 34, 255);
    pub const COLOR_HEAVY_PRIMARY: Color = Color::new(230, 74, 25, 255);
    pub const COLOR_LIGHT_PRIMARY: Color = Color::new(255, 204, 108, 255);
    pub const COLOR_ACCENT: Color = Color::new(255, 152, 0, 255);
    pub const COLOR_TEXT: Color = Color::new(255, 255, 255, 255);
    pub const COLOR_TEXT_MAIN: Color = Color::new(33, 33, 33, 255);
    pub const COLOR_TEXT_SIDE: Color = Color::new(117, 117, 117, 255);

    //================================================================

    const GRADIENT_POINT_Y: f32 = 2.0;
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

    const BUTTON_SHAPE: Vector2 = Vector2::new(48.0, 24.0);
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
            data: [Widget::default(); 64],
            font,
            card: Self::load_texture(handle, thread, Engine::CARD),
            check: Self::load_texture(handle, thread, Engine::CHECK),
            point: Vector2::default(),
            focus: None,
            count: i32::default(),
            wizard: Wizard::default(),
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
            .expect("Interface::data_get(): Widget overflow.")
    }

    pub fn data_get_mut(&mut self) -> &mut Widget {
        self.data
            .get_mut(self.count as usize)
            .expect("Interface::data_get_mut(): Widget overflow.")
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
            data.alpha += draw.get_frame_time() * 10.0;
        } else {
            data.alpha -= draw.get_frame_time() * 10.0;
        }

        data.alpha = data.alpha.clamp(0.0, 1.0);
    }

    fn set_scale(&mut self, draw: &RaylibDrawHandle, value: bool) {
        let data = self.data_get_mut();

        if value {
            data.scale -= draw.get_frame_time() * 10.0;
        } else {
            data.scale += draw.get_frame_time() * 10.0;
        }

        data.scale = data.scale.clamp(0.0, 1.0);
    }

    pub fn button(&mut self, draw: &mut RaylibDrawHandle, text: &str) -> bool {
        let measure = self.font_measure(text);
        let content = Rectangle::new(
            self.point.x,
            self.point.y,
            measure.x + Self::BUTTON_TEXT_SHIFT.x * 2.0,
            measure.y + Self::BUTTON_TEXT_SHIFT.y * 2.0,
        );

        let result = self.state(draw, content);

        self.set_alpha(draw, result.hover);
        self.set_scale(draw, result.focus);

        let data = self.data_get();

        self.card_round(
            draw,
            Self::get_scale(&content, data.scale, data.alpha),
            Self::get_alpha(&Interface::COLOR_HEAVY_PRIMARY, data.alpha),
        );
        self.font(
            draw,
            text,
            Vector2::new(
                content.x + Self::BUTTON_TEXT_SHIFT.x,
                content.y - Self::get_point(data.scale, data.alpha) + Self::BUTTON_TEXT_SHIFT.y,
            ),
            Self::get_alpha(&Self::COLOR_TEXT_MAIN, data.alpha),
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
            Self::get_alpha(&Interface::COLOR_HEAVY_PRIMARY, data.alpha),
        );

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
                Self::get_alpha(&Color::WHITE, data.alpha),
            );
        }

        self.font(
            draw,
            text,
            Vector2::new(
                content_max.width + content_max.x + Self::TOGGLE_SHIFT,
                content_max.y,
            ),
            Self::get_alpha(&Self::COLOR_TEXT_MAIN, data.alpha),
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
            Self::get_alpha(&Interface::COLOR_LIGHT_PRIMARY, data.alpha),
        );
        self.card_sharp(
            draw,
            content_min,
            Self::get_alpha(&Interface::COLOR_HEAVY_PRIMARY, data.alpha),
        );
        /*
        draw.draw_circle_v(
            Vector2::new(
                self.point.x + 8.0 + (content_max.width - 16.0) * percent,
                self.point.y + 10.0,
            ),
            8.0,
            Self::COLOR_HEAVY_PRIMARY,
        );
        */

        if state.focus {
            let pin = Vector2::new(
                self.point.x + content_max.width * percent,
                self.point.y - 32.0,
            );

            draw.draw_circle_v(
                pin,
                20.0,
                Self::get_alpha(&Self::COLOR_HEAVY_PRIMARY, data.alpha),
            );

            draw.draw_triangle(
                pin + Vector2::new(-20.0, 6.0),
                pin + Vector2::new(0.0, 32.0),
                pin + Vector2::new(20.0, 6.0),
                Self::get_alpha(&Self::COLOR_HEAVY_PRIMARY, data.alpha),
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
                Self::get_alpha(&Self::COLOR_HEAVY_PRIMARY, data.alpha),
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

        self.card_sharp(draw, content_max, Interface::COLOR_HEAVY_PRIMARY);
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
                } else {
                    if key != 0 {
                        value.push(key as u8 as char);
                    }
                }

                let measure = self.font_measure(value);

                self.card_sharp(
                    draw,
                    Rectangle::new(self.point.x + measure.x, self.point.y, 2.0, 16.0),
                    Interface::COLOR_HEAVY_PRIMARY,
                );
            }
        }

        self.point.y += content_hit.height + Self::SLIDER_SHIFT;
        self.count += 1;
    }
}
