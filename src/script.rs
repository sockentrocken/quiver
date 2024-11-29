use crate::status::*;
use crate::system::*;

//================================================================

use mlua::prelude::*;

//================================================================

#[derive(Default)]
pub struct Script {
    pub lua: Lua,
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

        Self::standard(&lua, &quiver).map_err(|e| e.to_string())?;

        lua.globals()
            .set("quiver", quiver)
            .map_err(|e| e.to_string())?;

        let module = Module::new(&lua, &info.path, &lua.globals()).unwrap();

        Ok(Self { lua, module })
    }

    fn standard(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
        draw::set_global(lua, table)?;
        input::set_global(lua, table)?;
        window::set_global(lua, table)?;

        texture::set_global(lua, table)?;
        sound::set_global(lua, table)?;
        music::set_global(lua, table)?;
        font::set_global(lua, table)?;

        Ok(())
    }

    pub fn main(&mut self) -> Result<(), String> {
        //let file = crate::utility::file::read(&format!("{}/main.lua", self.module.path))?;

        //self.lua.load(file).exec().map_err(|e| e.to_string())?;

        if let Some(main) = &self.module.main {
            main.call::<()>(()).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub fn step(&self) -> Result<(), String> {
        if let Some(step) = &self.module.step {
            step.call::<()>(()).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub fn exit(&self) -> Result<(), String> {
        if let Some(exit) = &self.module.exit {
            exit.call::<()>(()).map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}

use serde::{Deserialize, Serialize};

//================================================================

#[derive(Default)]
pub struct Module {
    pub path: String,
    pub main: Option<mlua::Function>,
    pub step: Option<mlua::Function>,
    pub exit: Option<mlua::Function>,
}

impl Module {
    pub fn new(lua: &Lua, path: &str, global: &mlua::Table) -> mlua::Result<Self> {
        let main = Some(global.get::<mlua::Function>("main")?);
        let step = Some(global.get::<mlua::Function>("step")?);
        let exit = Some(global.get::<mlua::Function>("exit")?);

        Ok(Self {
            path: path.to_string(),
            main,
            step,
            exit,
        })
    }
}
