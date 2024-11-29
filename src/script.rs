use crate::status::*;
use crate::system::*;

//================================================================

use mlua::prelude::*;

//================================================================

#[derive(Clone)]
pub struct Script {
    #[allow(dead_code)]
    pub lua: Lua,
    pub main: mlua::Function,
    pub step: mlua::Function,
    pub exit: mlua::Function,
    pub error: Option<mlua::Function>,
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
    const CALL_ERROR: &'static str = "error";

    pub fn new(info: &Info) -> mlua::Result<Self> {
        let lua = {
            if info.safe {
                Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new())?
            } else {
                unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) }
            }
        };
        let global = lua.globals();
        let quiver = lua.create_table()?;

        Self::system(&lua, &quiver)?;

        global.set("quiver", quiver)?;

        let package = global.get::<mlua::Table>("package")?;
        let path = package.get::<mlua::String>("path")?;
        package.set("path", format!("{path:?};{}/?.lua", info.path))?;

        lua.load(&format!("require \"{}\"", Self::CALL_MAIN))
            .exec()?;

        let quiver = global.get::<mlua::Table>("quiver")?;

        Ok(Self {
            lua,
            main: quiver.get::<mlua::Function>(Self::CALL_MAIN)?,
            step: quiver.get::<mlua::Function>(Self::CALL_STEP)?,
            exit: quiver.get::<mlua::Function>(Self::CALL_EXIT)?,
            error: {
                if quiver.contains_key(Self::CALL_ERROR)? {
                    Some(quiver.get::<mlua::Function>(Self::CALL_ERROR)?)
                } else {
                    None
                }
            },
        })
    }

    fn system(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
        general::set_global(lua, table)?;

        draw::set_global(lua, table)?;
        input::set_global(lua, table)?;
        window::set_global(lua, table)?;

        texture::set_global(lua, table)?;
        sound::set_global(lua, table)?;
        music::set_global(lua, table)?;
        font::set_global(lua, table)?;

        Ok(())
    }

    fn dump(path: &str) {
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

    pub fn main(&self) -> Result<(), String> {
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

    pub fn error(&self, message: &str) -> Result<(), String> {
        if let Some(error) = &self.error {
            error.call::<()>(message).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub fn new_module(path: &str) {
        Script::dump(&path);

        Info {
            safe: true,
            path: path.to_string(),
        }
        .dump();
    }

    pub fn load_module(path: &str) {
        Info {
            safe: true,
            path: path.to_string(),
        }
        .dump();
    }
}
