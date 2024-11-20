/*
* ================================================================
* script.rs
* ================================================================
*/

use crate::engine::*;
use crate::status::*;
use crate::system::*;
use crate::window::*;
use crate::module::*;

//================================================================

use mlua::prelude::*;

//================================================================

#[derive(Default)]
pub struct Script {
    pub lua: Lua,
    pub system: ModuleSystem,
    pub window: ModuleWindow,
    pub module: Vec<Module>,
    pub main: Vec<ModuleFunction>,
    pub step: Vec<ModuleFunction>,
    pub exit: Vec<ModuleFunction>,
}

impl Script {
    pub fn new(
        info: &InfoEngine,
        status: StatusPointer,
        window: WindowPointer,
    ) -> Result<Self, String> {
        let lua = {
            /*
            if info.safe {
                Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new())
                    .expect("Error initializing Lua virtual machine.")
            } else {
                unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) }
            }
            */

            unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) }
        };

        let mut script = Self::default();

        general::set_global(&lua, &lua.globals(), status, window).map_err(|e| e.to_string())?;

        for name in &info.path {
            let module = Module::new(&lua, name).map_err(|e| e.to_string())?;

            if let Some(ref system) = module.info.system {
                script.system = system.clone();
            }
            if let Some(ref window) = module.info.window {
                script.window = window.clone();
            }

            if let Some(build) = &module.info.build {
                if let Some(min) = build.min {
                    if Engine::BUILD < min {
                        continue;
                    }
                }

                if let Some(max) = build.max {
                    if Engine::BUILD > max {
                        continue;
                    }
                }
            }

            script.module.push(module);
        }

        Self::standard(&lua, &script.system).map_err(|e| e.to_string())?;

        script.lua = lua;

        Ok(script)
    }

    fn standard(lua: &Lua, system: &ModuleSystem) -> mlua::Result<()> {
        let global = lua.globals();

        video::set_global(lua, &global, system)?;
        audio::set_global(lua, &global, system)?;
        input::set_global(lua, &global)?;
        interface::set_global(lua, &global)?;
        discord::set_global(lua, &global)?;
        steam::set_global(lua, &global)?;
        request::set_global(lua, &global)?;
        parry::set_global(lua, &global)?;
        rapier::set_global(lua, &global)?;

        Ok(())
    }

    pub async fn main(&mut self) -> Result<(), String> {
        let global = self.lua.globals();

        for module in &self.module {
            if let Some(entry) = &module.info.entry {
                let file = crate::utility::file::read(&format!("{}/{}.lua", module.path, entry.file))?;

                self.lua
                    .load(file)
                    .exec()
                    .map_err(|e| e.to_string())?;
            }

            if let Some(entry) = &module.info.entry {
                if let Some(name) = &entry.main {
                    self.main.push(ModuleFunction::new(&global, name)?);
                }

                if let Some(name) = &entry.step {
                    self.step.push(ModuleFunction::new(&global, name)?);
                }

                if let Some(name) = &entry.exit {
                    self.exit.push(ModuleFunction::new(&global, name)?);
                }
            }
        }

        for main in &self.main {
            main.call
                .call_async::<()>(())
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub fn step(&self) -> Result<(), String> {
        for step in &self.step {
            step.call.call::<()>(()).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub fn exit(&self) -> Result<(), String> {
        for exit in &self.exit {
            exit.call.call::<()>(()).map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}