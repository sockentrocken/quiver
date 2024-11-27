use crate::engine::*;
use crate::module::*;
use crate::status::*;
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
    pub fn new(info: &InfoEngine, status: StatusPointer) -> Result<Self, String> {
        let lua = {
            if info.safe {
                Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new())
                    .expect("Error initializing Lua virtual machine.")
            } else {
                unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) }
            }
        };

        let quiver = lua.create_table().map_err(|e| e.to_string())?;

        general::set_global(&lua, &quiver, status).map_err(|e| e.to_string())?;

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
        draw_3d::set_global(lua, table, system)?;
        draw_2d::set_global(lua, table, system)?;
        input::set_global(lua, table, system)?;
        window::set_global(lua, table, system)?;
        font::set_global(lua, table, system)?;
        music::set_global(lua, table, system)?;
        sound::set_global(lua, table, system)?;
        texture::set_global(lua, table, system)?;

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
