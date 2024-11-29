use crate::status::*;
use crate::system::*;

//================================================================

use mlua::prelude::*;

//================================================================

pub struct Script {
    #[allow(dead_code)]
    pub lua: Lua,
    pub main: mlua::Function,
    pub step: mlua::Function,
    pub exit: mlua::Function,
}

impl Script {
    const FILE_MAIN: &'static str = include_str!("asset/main.lua");
    const FILE_BASE: &'static str = include_str!("asset/base.lua");
    const FILE_META: &'static str = include_str!("asset/meta.lua");
    const NAME_MAIN: &'static str = "main.lua";
    const NAME_BASE: &'static str = "base.lua";
    const NAME_META: &'static str = "meta.lua";
    const CALL_MAIN: &'static str = "main";
    const CALL_STEP: &'static str = "step";
    const CALL_EXIT: &'static str = "exit";

    pub fn new(info: &InfoEngine) -> mlua::Result<Self> {
        let lua = {
            if info.safe {
                Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new())?
            } else {
                unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) }
            }
        };

        let quiver = lua.create_table()?;

        general::set_global(&lua, &quiver)?;

        Self::standard(&lua, &quiver)?;

        let global = lua.globals();

        global.set("quiver", quiver)?;

        let package = global.get::<mlua::Table>("package")?;
        let path = package.get::<mlua::String>("path")?;
        package.set("path", format!("{path:?};{}/?.lua", info.path))?;

        let file = std::fs::read(&format!("{}/{}", info.path, Self::NAME_MAIN))?;

        lua.load(file).exec()?;

        let quiver = global.get::<mlua::Table>("quiver")?;

        let main = quiver.get::<mlua::Function>(Self::CALL_MAIN)?;
        let step = quiver.get::<mlua::Function>(Self::CALL_STEP)?;
        let exit = quiver.get::<mlua::Function>(Self::CALL_EXIT)?;

        Ok(Self {
            lua,
            main,
            step,
            exit,
        })
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
        self.main.call::<()>(()).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn step(&self) -> Result<(), String> {
        self.step.call::<()>(()).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn exit(&self) -> Result<(), String> {
        self.exit.call::<()>(()).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn dump(path: &str) {
        std::fs::write(format!("{path}/{}", Self::NAME_MAIN), Self::FILE_MAIN)
            .map_err(|e| Status::panic(&e.to_string()))
            .unwrap();
        std::fs::write(format!("{path}/{}", Self::NAME_BASE), Self::FILE_BASE)
            .map_err(|e| Status::panic(&e.to_string()))
            .unwrap();
        std::fs::write(format!("{path}/{}", Self::NAME_META), Self::FILE_META)
            .map_err(|e| Status::panic(&e.to_string()))
            .unwrap();
    }
}
