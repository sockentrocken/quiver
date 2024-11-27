use crate::engine::*;
use crate::system::*;

//================================================================

use mlua::prelude::*;

//================================================================

pub struct Script {
    pub lua: Lua,
    pub system: ModuleSystem,
    pub window: ModuleWindow,
    pub module: Module,
}

impl Script {
    pub fn new(info: &InfoEngine) -> Result<Self, String> {
        let lua = {
            if info.safe {
                Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new())
                    .expect("Error initializing Lua virtual machine.")
            } else {
                unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) }
            }
        };

        let quiver = lua.create_table().map_err(|e| e.to_string())?;

        general::set_global(&lua, &quiver).map_err(|e| e.to_string())?;

        let info_module = InfoModule::new(&lua, &info.path)?;

        Self::standard(&lua, &quiver, &info_module.system).map_err(|e| e.to_string())?;

        lua.globals()
            .set("quiver", quiver)
            .map_err(|e| e.to_string())?;

        let module = Module::new(&lua, &info.path, &lua.globals()).unwrap();

        Ok(Self {
            lua,
            system: info_module.system,
            window: info_module.window,
            module,
        })
    }

    fn standard(lua: &Lua, table: &mlua::Table, system: &ModuleSystem) -> mlua::Result<()> {
        draw::set_global(lua, table, system)?;
        input::set_global(lua, table, system)?;
        window::set_global(lua, table, system)?;

        texture::set_global(lua, table, system)?;
        sound::set_global(lua, table, system)?;
        music::set_global(lua, table, system)?;
        font::set_global(lua, table, system)?;

        Ok(())
    }

    pub fn main(&mut self) -> Result<(), String> {
        let file = crate::utility::file::read(&format!("{}/main.lua", self.module.path))?;

        self.lua.load(file).exec().map_err(|e| e.to_string())?;

        self.module.main.call::<()>(()).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn step(&self) -> Result<(), String> {
        self.module.step.call::<()>(()).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn exit(&self) -> Result<(), String> {
        self.module.exit.call::<()>(()).map_err(|e| e.to_string())?;

        Ok(())
    }
}

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
