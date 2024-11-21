use crate::engine::*;
use crate::module::*;
use crate::status::*;

//================================================================

use imgui::InputTextCallbackHandler;
use raylib::texture::Texture2D;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

//================================================================

pub type WindowPointer = Rc<RefCell<Window>>;

#[derive(Default)]
pub struct Window {
    pub active: bool,
    pub logger: Logger,
    pub parser: Parser,
    pub wizard: Wizard,
    pub icon: Option<Texture2D>,
}

impl Window {
    pub fn draw(engine: &mut Engine, interface: &imgui::Ui, w_active: bool, l_active: bool) {
        let size = [
            interface.io().display_size[0] * 1.0,
            interface.io().display_size[1] * 0.5,
        ];

        if w_active {
            let mut result: Option<(mlua::Function, Vec<String>)> = None;

            interface
                .window("Debug Window")
                .menu_bar(true)
                .no_decoration()
                .position([0.0, 0.0], imgui::Condition::Always)
                .size(size, imgui::Condition::Always)
                .build(|| {
                    if let Some(_menu_bar) = interface.begin_menu_bar() {
                        if let Some(_menu) = interface.begin_menu("Engine") {
                            if interface.menu_item("Load Script") {
                                Status::set_restart(engine);
                            }
                            if interface.menu_item("Exit Engine") {
                                Status::set_closure(engine);
                            }
                            if interface.menu_item("Dump Lua API Documentation") {
                                //engine.script.help.dump("");
                            }
                        }
                        if let Some(_menu) = interface.begin_menu("Window") {
                            if interface.menu_item("Wipe Log") {
                                Logger::wipe(&mut engine.window.borrow_mut().logger);
                            }
                            if interface.menu_item("Dump Log") {
                                Logger::dump(&engine.window.borrow_mut().logger);
                            }
                            interface.checkbox(
                                "Show Log",
                                &mut engine.window.borrow_mut().logger.active,
                            );
                        }
                    }

                    engine
                        .window
                        .borrow_mut()
                        .logger
                        .draw(interface, true, true, [0.0, -24.0]);
                    result = Parser::draw(&mut engine.window.borrow_mut(), interface);
                });

            if let Some(result) = result {
                if let Err(err) = result.0.call::<()>(result.1) {
                    Status::set_failure(engine, err.to_string());
                }
            }
        } else if l_active {
            interface
                .window("Debug Window")
                .no_decoration()
                .position([0.0, 0.0], imgui::Condition::Always)
                .size([size[0], 80.0], imgui::Condition::Always)
                .bg_alpha(0.75)
                .build(|| {
                    engine
                        .window
                        .borrow_mut()
                        .logger
                        .draw(interface, false, false, [0.0, 0.0]);
                });
        }
    }
}

//================================================================

#[derive(Default)]
pub struct Logger {
    pub active: bool,
    pub buffer: Vec<LogLine>,
    pub scroll: bool,
}

impl Logger {
    pub fn draw(&mut self, interface: &imgui::Ui, border: bool, scroll: bool, size: [f32; 2]) {
        interface
            .child_window("Logger")
            .border(border)
            .scroll_bar(scroll)
            .size(size)
            .build(|| {
                for line in &self.buffer {
                    let style = interface.push_style_color(imgui::StyleColor::Text, line.color);

                    interface.text_wrapped(&line.label);

                    style.pop();
                }

                if self.scroll {
                    self.scroll = false;
                    interface.set_scroll_here_y_with_ratio(1.0);
                }
            });
    }

    pub fn push(&mut self, line: LogLine) {
        self.buffer.push(line);
        self.scroll = true;
    }

    pub fn dump(&self) {}

    pub fn wipe(&mut self) {
        self.buffer.clear();
    }

    pub fn show(&mut self, value: bool) {
        self.active = value;
    }
}

#[derive(Default)]
pub struct LogLine {
    pub label: String,
    pub color: [f32; 4],
}

impl LogLine {
    pub const COLOR_HISTORY: [f32; 4] = [0.75, 0.75, 0.75, 1.00];
    pub const COLOR_FAILURE: [f32; 4] = [1.00, 0.00, 0.00, 1.00];

    pub fn new(label: String, color: [f32; 4]) -> Self {
        Self { label, color }
    }
}

//================================================================

#[derive(Default)]
pub struct Parser {
    pub buffer: String,
    pub history: ParserBuffer<String>,
    pub suggest: ParserBuffer<ParserSuggest>,
    pub method: HashMap<String, ParserMethod>,
    pub dirty: bool,
}

impl Parser {
    pub fn print(window: &mut Window) -> Option<(mlua::Function, Vec<String>)> {
        window.logger.push(LogLine::new(
            window.parser.buffer.clone(),
            LogLine::COLOR_HISTORY,
        ));

        let clone = window.parser.buffer.clone();

        window
            .parser
            .history
            .buffer
            .push(window.parser.buffer.clone());
        window.parser.history.number = 0;

        window.parser.suggest.buffer.clear();
        window.parser.suggest.number = 0;

        window.parser.buffer.clear();
        window.parser.dirty = true;

        Parser::push(window, &clone)
    }

    pub fn draw(
        window: &mut Window,
        interface: &imgui::Ui,
    ) -> Option<(mlua::Function, Vec<String>)> {
        if !window.parser.suggest.buffer.is_empty() {
            let mut command: Option<String> = None;

            let size = interface.io().display_size;

            interface
                .window("Suggest")
                .position(
                    [0.0, interface.io().display_size[1] * 0.5],
                    imgui::Condition::Always,
                )
                .focus_on_appearing(false)
                .size(
                    [size[0], (window.parser.suggest.buffer.len() * 24) as f32],
                    imgui::Condition::Always,
                )
                .no_decoration()
                .build(|| {
                    for item in &window.parser.suggest.buffer {
                        if interface.menu_item(&item.text) {
                            command = Some(item.text.clone());
                        }
                        interface.same_line();

                        let style = interface
                            .push_style_color(imgui::StyleColor::Text, [0.5, 0.5, 0.5, 1.0]);

                        interface.text(&item.info);

                        style.pop();
                    }
                });

            if let Some(command) = command {
                window.parser.buffer = command;
                return Parser::print(window);
            }
        }

        interface.set_next_item_width(-1.0);

        if window.parser.dirty {
            interface.set_keyboard_focus_here();
            window.parser.dirty = false;
        }

        let callback = ParserCallback {
            history: &mut window.parser.history,
            suggest: &mut window.parser.suggest,
            method: &window.parser.method,
        };

        if interface
            .input_text("##Entry", &mut window.parser.buffer)
            .callback(imgui::InputTextCallback::all(), callback)
            .enter_returns_true(true)
            .build()
            && !window.parser.buffer.is_empty()
        {
            return Parser::print(window);
        }

        None
    }

    pub fn push(window: &mut Window, text: &str) -> Option<(mlua::Function, Vec<String>)> {
        let list: Vec<String> = text.split(";").map(str::to_string).collect();

        for line in list {
            let line = line.trim();

            if !line.is_empty() {
                let text: Vec<String> = line.split(" ").map(str::to_string).collect();

                if let Some(name) = text.first() {
                    if let Some(method) = window.parser.method.get(name) {
                        return Some((method.call.clone(), text));
                    } else {
                        window.logger.push(LogLine::new(
                            "Unknown command.".to_string(),
                            LogLine::COLOR_FAILURE,
                        ));
                    }
                }
            }
        }

        None
    }
}

#[derive(Default)]
pub struct ParserBuffer<T> {
    buffer: Vec<T>,
    number: i32,
}

#[derive(Default)]
pub struct ParserSuggest {
    pub text: String,
    pub info: String,
}

impl ParserSuggest {
    pub fn new(text: &str, info: &str) -> Self {
        Self {
            text: text.to_string(),
            info: info.to_string(),
        }
    }
}

pub struct ParserMethod {
    pub call: mlua::Function,
    pub info: String,
}

pub struct ParserCallback<'a> {
    history: &'a mut ParserBuffer<String>,
    suggest: &'a mut ParserBuffer<ParserSuggest>,
    method: &'a HashMap<String, ParserMethod>,
}

impl<'a> InputTextCallbackHandler for ParserCallback<'a> {
    fn on_history(
        &mut self,
        direction: imgui::HistoryDirection,
        mut data: imgui::TextCallbackData,
    ) {
        if !self.history.buffer.is_empty() {
            match direction {
                imgui::HistoryDirection::Up => {
                    self.history.number += 1;

                    if self.history.number > self.history.buffer.len() as i32 {
                        self.history.number = 1;
                    }
                }
                imgui::HistoryDirection::Down => {
                    self.history.number -= 1;

                    if self.history.number < 1 {
                        self.history.number = self.history.buffer.len() as i32;
                    }
                }
            }

            let index = (self.history.number - 1) as usize;

            data.clear();
            if let Some(index) = self.history.buffer.get(index) {
                data.push_str(index);
            }
        }
    }

    fn on_completion(&mut self, mut data: imgui::TextCallbackData) {
        if !self.suggest.buffer.is_empty() {
            self.suggest.number += 1;

            if self.suggest.number > self.suggest.buffer.len().try_into().unwrap() {
                self.suggest.number = 1;
            }

            let index = (self.suggest.number - 1) as usize;

            data.clear();
            if let Some(index) = self.suggest.buffer.get(index) {
                data.push_str(&index.text);
            }
        }
    }

    fn on_edit(&mut self, data: imgui::TextCallbackData) {
        self.suggest.buffer.clear();

        if !data.str().is_empty() {
            for (k, v) in self.method.iter() {
                if k.starts_with(data.str()) {
                    self.suggest.buffer.push(ParserSuggest::new(k, &v.info));
                }
            }
        }
    }
}

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

    pub fn draw(engine: &Engine, interface: &mut imgui::Ui) {
        let size = interface.io().display_size;

        let window = interface.push_style_var(imgui::StyleVar::WindowRounding(4.0));
        let frame = interface.push_style_var(imgui::StyleVar::FrameRounding(4.0));
        let grab = interface.push_style_var(imgui::StyleVar::GrabRounding(4.0));

        interface
            .window("Engine Wizard")
            .flags(
                imgui::WindowFlags::NO_MOVE
                    | imgui::WindowFlags::NO_RESIZE
                    | imgui::WindowFlags::NO_DECORATION,
            )
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(size, imgui::Condition::Always)
            .build(|| {
                let image_size: [f32; 2];

                let image = {
                    let tex = engine.window.borrow();
                    let tex = tex.icon.as_ref().unwrap();
                    let id = imgui::TextureId::new(tex.id as usize);
                    image_size = [tex.width as f32, tex.height as f32];
                    imgui::Image::new(id, image_size)
                };

                let state = engine.window.borrow().wizard.state.clone();
                let wizard = &mut engine.window.borrow_mut().wizard;

                match state {
                    WizardState::Main => {
                        interface.set_cursor_pos([
                            size[0] * 0.5 - image_size[0] * 0.5,
                            size[1] * 0.5 - image_size[1] * 0.5 - 88.0,
                        ]);

                        image.build(interface);

                        interface.set_cursor_pos([8.0, size[1] - 184.0]);

                        interface.separator();
                        interface.spacing();

                        if interface.button("New Module") {
                            wizard.state = WizardState::NewModule;
                        }
                        if interface.button("Load Module") {
                            use rfd::FileDialog;

                            let current = std::env::current_dir().unwrap();
                            let current = current.to_str().unwrap();

                            if let Some(folder) =
                                FileDialog::new().set_directory(current).pick_folders()
                            {
                                let folder = folder
                                    .iter()
                                    .map(|p| p.to_str().unwrap().to_string())
                                    .collect();

                                let info = InfoEngine {
                                    safe: true,
                                    path: folder,
                                };

                                info.dump("./").unwrap();

                                Status::set_restart(engine);
                            }
                        }
                        if interface.button("Exit Quiver") {
                            Status::set_closure(engine);
                        }
                        if interface.button("GitHub") {
                            raylib::misc::open_url("https://github.com/sockentrocken/quiver");
                        }
                        if interface.button("Discord") {
                            raylib::misc::open_url("https://discord.gg");
                        }
                        interface.text("1.0.0");
                    }
                    WizardState::NewModule => {
                        interface.text("Module Data");
                        interface.separator();
                        interface.spacing();

                        interface
                            .input_text("Module Path", &mut wizard.path)
                            .hint("my_module")
                            .build();
                        interface
                            .input_text("Module Name", &mut wizard.info.name)
                            .hint("My Module.")
                            .build();
                        interface
                            .input_text("Module Info", &mut wizard.info.info)
                            .hint("A module for Quiver.")
                            .build();

                        interface.set_cursor_pos([8.0, size[1] - 72.0]);

                        interface.separator();
                        interface.spacing();

                        if interface.button("Next") {
                            wizard.state = WizardState::NewSystem;
                        }
                        if interface.button("Back") {
                            wizard.state = WizardState::Main;
                        }
                    }
                    WizardState::NewSystem => {
                        interface.text("System Data");
                        interface.separator();
                        interface.spacing();

                        if let Some(system) = &mut wizard.info.system {
                            interface.checkbox("Model", &mut system.model);
                            interface.checkbox("Texture", &mut system.texture);
                            interface.checkbox("Image", &mut system.image);
                            interface.checkbox("Sound", &mut system.sound);
                            interface.checkbox("Music", &mut system.music);
                            interface.checkbox("Font", &mut system.font);
                            interface.checkbox("Shader", &mut system.shader);
                        }

                        interface.set_cursor_pos([8.0, size[1] - 72.0]);

                        interface.separator();
                        interface.spacing();

                        if interface.button("Next") {
                            wizard.state = WizardState::NewWindow;
                        }
                        if interface.button("Back") {
                            wizard.state = WizardState::NewModule;
                        }
                    }
                    WizardState::NewWindow => {
                        interface.text("Window Data");
                        interface.separator();
                        interface.spacing();

                        if let Some(window) = &mut wizard.info.window {
                            interface.checkbox("Full-Screen", &mut window.fullscreen);
                            interface.checkbox("Border-Less", &mut window.borderless);
                            interface.checkbox("Vertical-Sync", &mut window.sync);
                            interface.checkbox("MSAA", &mut window.msaa);
                            interface.checkbox("Resize", &mut window.resize);
                            interface.checkbox("Hidden", &mut window.hidden);
                            interface.checkbox("Minimized", &mut window.minimize);
                            interface.checkbox("Maximized", &mut window.maximize);
                            interface.checkbox("No-Decor", &mut window.no_decor);
                            interface.checkbox("No-Focus", &mut window.no_focus);
                            interface.checkbox("On-Front", &mut window.on_front);
                            interface.checkbox("Run Hidden", &mut window.run_hidden);
                            interface.checkbox("Mouse Pass", &mut window.mouse_pass);
                            interface.checkbox("Draw Alpha", &mut window.draw_alpha);
                            interface.checkbox("High Scale", &mut window.high_scale);
                            interface
                                .input_text("Window Name", &mut window.name)
                                .hint("My Module")
                                .build();
                            //interface.checkbox("", &mut window.icon);
                            interface.slider("Window Rate", 60, 240, &mut window.rate);
                            //interface.checkbox("", &mut window.point);
                            //interface.checkbox("", &mut window.shape);
                            //interface.checkbox("", &mut window.shape_min);
                            //interface.checkbox("", &mut window.shape_max);
                            //interface.checkbox("", &mut window.alpha);
                        }

                        interface.set_cursor_pos([8.0, size[1] - 72.0]);

                        interface.separator();
                        interface.spacing();

                        if interface.button("Make Module") {
                            let result = wizard.make();

                            match result {
                                Ok(_) => Status::set_restart(engine),
                                Err(error) => {
                                    wizard.warn = error;
                                    interface.open_popup("Error");
                                }
                            }
                        }

                        interface
                            .modal_popup_config("Error")
                            .always_auto_resize(true)
                            .build(|| {
                                interface.text(&wizard.warn);

                                if interface.button("Close") {
                                    interface.close_current_popup();
                                }
                            });
                        if interface.button("Back") {
                            wizard.state = WizardState::NewSystem;
                        }
                    }
                    WizardState::LoadModule => todo!(),
                }

                /*
                 */
            });
    }

    pub fn get_info(&self) -> String {
        let mut result = Self::FILE_INFO.to_string();
        result = result.replace("{name}", &self.info.name);
        result = result.replace("{info}", &self.info.info);
        result = result.replace("{path}", &self.path);

        if let Some(system) = &self.info.system {
            result = result.replace("{model}", &system.model.to_string());
            result = result.replace("{texture}", &system.texture.to_string());
            result = result.replace("{image}", &system.image.to_string());
            result = result.replace("{sound}", &system.sound.to_string());
            result = result.replace("{music}", &system.music.to_string());
            result = result.replace("{font}", &system.font.to_string());
            result = result.replace("{shader}", &system.shader.to_string());
        }

        if let Some(window) = &self.info.window {
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
        }

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

        let info = InfoEngine {
            safe: true,
            path: vec![self.path.clone()],
        };

        info.dump("./")?;

        raylib::core::misc::open_url(&self.path);

        Ok(())
    }
}
