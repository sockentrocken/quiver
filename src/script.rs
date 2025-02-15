/*
* BSD Zero Clause License
*
* Copyright (c) 2025 sockentrocken
*
* Permission to use, copy, modify, and/or distribute this software for any
* purpose with or without fee is hereby granted.
*
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
* REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
* AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
* INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
* LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
* OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
* PERFORMANCE OF THIS SOFTWARE.
*/

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
    pub fail: Option<mlua::Function>,
}

impl Script {
    const FILE_MAIN: &'static str = include_str!("asset/main.lua");
    const FILE_BASE: &'static str = include_str!("asset/base.lua");
    const FILE_META: &'static str = include_str!("asset/meta.lua");
    const NAME_MAIN: &'static str = "main.lua";
    const NAME_BASE: &'static str = "base.lua";
    const NAME_META: &'static str = "meta.lua";
    const CALL_MAIN: &'static str = "main";
    const CALL_FAIL: &'static str = "fail";

    //================================================================

    // get a new script instance.
    pub fn new(info: &Info) -> mlua::Result<Self> {
        // initialize lua VM, depending on what safe flag is set.
        let lua = {
            if info.safe {
                // quiver is in safe mode, only load the safe standard Lua library.
                Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new())?
            } else {
                // quiver is in unsafe mode, load every Lua library and allow loading foreign native code.
                unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) }
            }
        };

        // set script data.
        lua.set_app_data(ScriptData::new(info.clone()));

        // set the standard Quiver library.
        Self::system(&lua, info)?;

        // load the main entry-point file, which should add a "quiver.main" entry-point to the quiver table.
        lua.load(format!("require \"{}\"", Self::CALL_MAIN))
            .exec()?;

        // get the global table.
        let global = lua.globals();
        // get the quiver table.
        let quiver = global.get::<mlua::Table>("quiver")?;

        // get the main function.
        let main: mlua::Function = quiver.get(Self::CALL_MAIN)?;
        // get the fail function. note that it may not exist in the lua space, so we use our own crash-handler instead.
        let fail = {
            if quiver.contains_key(Self::CALL_FAIL)? {
                Some(quiver.get::<mlua::Function>(Self::CALL_FAIL)?)
            } else {
                None
            }
        };

        Ok(Self { lua, main, fail })
    }

    // main Lua entry-point.
    pub fn main(&self) -> Result<bool, String> {
        self.main.call::<bool>(()).map_err(|e| e.to_string())
    }

    // fail Lua entry-point.
    pub fn fail(&self, message: &str) -> Result<bool, String> {
        self.fail
            .as_ref()
            .expect("Script::fail(): Unwrapping without a function.")
            .call::<bool>(message)
            .map_err(|e| e.to_string())
    }

    // create a new info_quiver.json file at the given path, and dump main/base/meta.lua into the path.
    pub fn new_module(path: &str) {
        // dump main/base/meta.
        Self::dump(path);

        // dump info_quiver.json.
        Info {
            safe: true,
            path: path.to_string(),
        }
        .dump();
    }

    // create a new info_quiver.json file at the given path.
    pub fn load_module(path: &str) {
        // dump info_quiver.json.
        Info {
            safe: true,
            path: path.to_string(),
        }
        .dump();
    }

    //================================================================

    // load every standard library into the quiver table.
    fn system(lua: &Lua, info: &Info) -> mlua::Result<()> {
        // get the global lua table.
        let global = lua.globals();
        // over-load print to use rust's println instead. otherwise, RL will consume the Lua print.
        global.set(
            "print",
            lua.create_function(|_, text: LuaValue| {
                println!("{:?}", text);
                Ok(())
            })?,
        )?;

        // set the lua package loader to also consider the current game path.
        let package = global.get::<mlua::Table>("package")?;
        package.set(
            "path",
            format!(
                "{:?};{}/?.lua",
                package.get::<mlua::String>("path")?,
                info.path
            ),
        )?;

        // create the quiver table.
        let quiver = lua.create_table()?;

        // set the standard Quiver library.
        general::set_global(lua, &quiver)?;
        rapier::set_global(lua, &quiver)?;
        window::set_global(lua, &quiver)?;
        draw::set_global(lua, &quiver)?;
        input::set_global(lua, &quiver)?;
        model::set_global(lua, &quiver)?;
        texture::set_global(lua, &quiver)?;
        image::set_global(lua, &quiver)?;
        sound::set_global(lua, &quiver)?;
        music::set_global(lua, &quiver)?;
        font::set_global(lua, &quiver)?;
        shader::set_global(lua, &quiver)?;
        file::set_global(lua, &quiver)?;
        zip::set_global(lua, &quiver)?;

        // set the quiver table as a global value.
        global.set("quiver", quiver)?;

        Ok(())
    }

    // dump main.lua/base.lua/meta.lua into a given directory.
    fn dump(path: &str) {
        // dump main.lua.
        std::fs::write(format!("{path}/{}", Self::NAME_MAIN), Self::FILE_MAIN)
            .map_err(|e| Status::panic(&e.to_string()))
            .unwrap();

        // dump base.lua.
        std::fs::write(format!("{path}/{}", Self::NAME_BASE), Self::FILE_BASE)
            .map_err(|e| Status::panic(&e.to_string()))
            .unwrap();

        // dump meta.lua.
        std::fs::write(format!("{path}/{}", Self::NAME_META), Self::FILE_META)
            .map_err(|e| Status::panic(&e.to_string()))
            .unwrap();
    }
}

//================================================================

pub struct ScriptData {
    pub info: Info,
}

impl ScriptData {
    pub fn new(info: Info) -> Self {
        Self { info }
    }

    pub fn get_path(lua: &Lua, path: &str) -> mlua::Result<String> {
        let script_data = lua.app_data_ref::<ScriptData>().unwrap();

        //        if script_data.info.safe {
        // any path in safe mode must be done within the info file's given path.

        Ok(format!("{}/{path}", script_data.info.path))

        //        } else {
        //            // un-safe mode does not have this restriction.
        //            Ok(path.to_string())
        //        }
    }
}
