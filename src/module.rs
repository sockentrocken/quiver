use mlua::prelude::*;
use serde::{Deserialize, Serialize};

//================================================================

pub struct Module {
    pub path: String,
    pub main: mlua::Function,
    pub step: mlua::Function,
    pub exit: mlua::Function,
}

impl Module {
    pub fn new(lua: &Lua, path: &str, global: &mlua::Table) -> mlua::Result<Self> {
        let main = global.get::<mlua::Function>("main")?;
        let step = global.get::<mlua::Function>("step")?;
        let exit = global.get::<mlua::Function>("exit")?;

        Ok(Self {
            path: path.to_string(),
            main,
            step,
            exit,
        })
    }
}

//================================================================

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct InfoModule {
    pub system: ModuleSystem,
    pub window: ModuleWindow,
}

impl InfoModule {
    pub const FILE_NAME: &'static str = "info.lua";

    pub fn new(lua: &Lua, path: &str) -> Result<Self, String> {
        let path = &format!("{}/{}", path, Self::FILE_NAME);

        let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

        let value = lua
            .load(data)
            .eval::<LuaValue>()
            .map_err(|e| e.to_string())?;

        lua.from_value::<Self>(value).map_err(|e| e.to_string())
    }
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
    pub rate: f32,
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
            rate: 60.0,
            icon: None,
            point: None,
            shape: (1024, 768),
            shape_min: None,
            shape_max: None,
            alpha: 1.0,
        }
    }
}
