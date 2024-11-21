use mlua::prelude::*;
use serde::{Deserialize, Serialize};

//================================================================

pub struct Module {
    pub path: String,
    pub info: InfoModule,
}

impl Module {
    pub fn new(lua: &Lua, path: &str) -> mlua::Result<Self> {
        Ok(Self {
            path: path.to_string(),
            info: InfoModule::new(lua, path)?,
        })
    }
}

//================================================================

pub struct ModuleFunction {
    pub call: mlua::Function,
}

impl ModuleFunction {
    pub fn new(global: &mlua::Table, name: &str) -> Result<Self, String> {
        let call = global
            .get::<mlua::Function>(name)
            .map_err(|_| format!("ModuleFunction::new(): Could not find function \"{name}\"."))?;

        Ok(Self { call })
    }
}

//================================================================

#[derive(Deserialize, Serialize, Clone)]
pub struct InfoModule {
    pub name: String,
    pub info: String,
    pub build: Option<ModuleBuild>,
    pub entry: Option<ModuleEntry>,
    pub system: Option<ModuleSystem>,
    pub window: Option<ModuleWindow>,
}

impl InfoModule {
    pub const FILE_NAME: &'static str = "info.lua";

    pub fn new(lua: &Lua, path: &str) -> mlua::Result<Self> {
        let path = &format!("{}/{}", path, Self::FILE_NAME);
        let data = std::path::Path::new(path);

        if data.is_file() {
            let data =
                std::fs::read_to_string(path).map_err(|e| mlua::Error::runtime(e.to_string()))?;

            let value = lua.load(data).eval::<LuaValue>()?;

            lua.from_value::<Self>(value)
        } else {
            Ok(Self::default())
        }
    }
}

impl Default for InfoModule {
    fn default() -> Self {
        Self {
            name: String::new(),
            info: String::new(),
            build: None,
            entry: Some(ModuleEntry::default()),
            system: Some(ModuleSystem::default()),
            window: Some(ModuleWindow::default()),
        }
    }
}

//================================================================

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct ModuleBuild {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

//================================================================

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct ModuleEntry {
    pub file: String,
    pub main: Option<String>,
    pub step: Option<String>,
    pub exit: Option<String>,
}

//================================================================

#[derive(Deserialize, Serialize, Clone)]
pub struct ModuleSystem {
    pub model: bool,
    pub texture: bool,
    pub image: bool,
    pub sound: bool,
    pub music: bool,
    pub font: bool,
    pub shader: bool,
}

impl Default for ModuleSystem {
    fn default() -> Self {
        Self {
            model: true,
            texture: true,
            image: true,
            sound: true,
            music: true,
            font: true,
            shader: true,
        }
    }
}

//================================================================

#[derive(Deserialize, Serialize, Clone)]
pub struct ModuleWindow {
    pub fullscreen: bool,
    pub borderless: bool,
    pub sync: bool,
    pub msaa: bool,
    pub resize: bool,
    pub hidden: bool,
    pub minimize: bool,
    pub maximize: bool,
    pub no_decor: bool,
    pub no_focus: bool,
    pub on_front: bool,
    pub run_hidden: bool,
    pub mouse_pass: bool,
    pub draw_alpha: bool,
    pub high_scale: bool,
    pub name: String,
    pub icon: Option<Vec<String>>,
    pub rate: u32,
    pub point: Option<(i32, i32)>,
    pub shape: (i32, i32),
    pub shape_min: Option<(i32, i32)>,
    pub shape_max: Option<(i32, i32)>,
    pub alpha: f32,
}

impl Default for ModuleWindow {
    fn default() -> Self {
        Self {
            fullscreen: false,
            borderless: false,
            sync: true,
            msaa: true,
            resize: true,
            hidden: false,
            minimize: false,
            maximize: false,
            no_decor: false,
            no_focus: false,
            on_front: false,
            run_hidden: false,
            mouse_pass: false,
            draw_alpha: false,
            high_scale: false,
            name: "Quiver".to_string(),
            rate: 60,
            icon: None,
            point: None,
            shape: (1024, 768),
            shape_min: None,
            shape_max: None,
            alpha: 1.0,
        }
    }
}
