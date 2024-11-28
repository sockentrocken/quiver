use crate::script::*;
use crate::status::*;
use crate::window::*;

use raylib::prelude::*;

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
    pub fn draw(&mut self, draw: &mut RaylibDrawHandle, window: &mut Window) -> Option<Status> {
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
                    return Some(Status::Closure);
                }
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

        None
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

#[derive(Default, Clone)]
pub enum WizardState {
    #[default]
    Main,
    NewModule,
    NewSystem,
    NewWindow,
    LoadModule,
}
