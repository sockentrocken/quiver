/*
* Copyright (c) 2025 sockentrocken
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use crate::status::*;
use crate::system::*;

//================================================================

#[cfg(feature = "zip")]
use ::zip::ZipArchive;

#[cfg(feature = "zip")]
use std::io::Read;

use mlua::prelude::*;
use serde::Serialize;
use std::ffi::{CStr, CString};

//================================================================

pub struct BaseFile {
    pub name: &'static str,
    pub data: &'static str,
}

impl BaseFile {
    const fn new(name: &'static str, data: &'static str) -> Self {
        Self { name, data }
    }
}

#[derive(Clone)]
pub struct Script {
    #[allow(dead_code)]
    pub lua: Lua,
    pub main: mlua::Function,
    pub fail: Option<mlua::Function>,
}

impl Script {
    const FILE_MAIN: &'static str = include_str!("asset/main.lua");
    #[rustfmt::skip]
    pub const FILE_BASE: [BaseFile; 10] = [
        BaseFile::new("base/constant.lua",    include_str!(concat!(env!("OUT_DIR"), "/constant.lua"))),
        BaseFile::new("base/extension.lua",   include_str!(concat!(env!("OUT_DIR"), "/extension.lua"))),
        BaseFile::new("base/allocator.lua",   include_str!(concat!(env!("OUT_DIR"), "/allocator.lua"))),
        BaseFile::new("base/primitive.lua",   include_str!(concat!(env!("OUT_DIR"), "/primitive.lua"))),
        BaseFile::new("base/scheduler.lua",   include_str!(concat!(env!("OUT_DIR"), "/scheduler.lua"))),
        BaseFile::new("base/action.lua",      include_str!(concat!(env!("OUT_DIR"), "/action.lua"))),
        BaseFile::new("base/logger.lua",      include_str!(concat!(env!("OUT_DIR"), "/logger.lua"))),
        BaseFile::new("base/window.lua",      include_str!(concat!(env!("OUT_DIR"), "/window.lua"))),
        BaseFile::new("base/system.lua",      include_str!(concat!(env!("OUT_DIR"), "/system.lua"))),
        BaseFile::new("base/scene.lua",       include_str!(concat!(env!("OUT_DIR"), "/scene.lua"))),
    ];
    const FILE_BASE_MAIN: BaseFile =
        BaseFile::new("base/main.lua", include_str!("asset/base/main.lua"));
    const FILE_META: &'static str = include_str!("asset/meta.lua");
    const NAME_MAIN: &'static str = "main.lua";
    const NAME_META: &'static str = "meta.lua";
    const CALL_MAIN: &'static str = "main";
    const CALL_FAIL: &'static str = "fail";

    //================================================================

    #[allow(dead_code)]
    pub async fn new_test(path: &str) -> mlua::Result<()> {
        // initialize lua VM, depending on what safe flag is set.
        let lua = Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new())?;

        let info = Info {
            head: true,
            safe: true,
            path: "test/asset".to_string(),
        };

        // set script data.
        lua.set_app_data(ScriptData::new(info.clone()));

        // set the standard Quiver library.
        Self::system(&lua, &info)?;

        let main_data = std::fs::read(path)
            .unwrap_or_else(|_| panic!("Script::new_test(): Could not read file \"{path}\""));

        lua.load("quiver.general.load_base()").exec()?;

        lua.load(main_data).exec_async().await?;

        Ok(())
    }

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

        #[allow(unused_mut)]
        let mut main_data = format!("require \"{}\"", Self::CALL_MAIN);

        #[cfg(feature = "zip")]
        {
            // get the path to the main folder or file.
            let main_path = format!("{}/{}", info.path, Self::CALL_MAIN);
            let main_path = std::path::Path::new(&main_path);

            if main_path.is_file() {
                let file = std::fs::File::open(main_path)?;
                let mut file =
                    ZipArchive::new(file).map_err(|e| mlua::Error::runtime(e.to_string()))?;
                if let Ok(mut value) = file.by_name(Self::NAME_MAIN) {
                    let mut buffer = String::new();
                    value.read_to_string(&mut buffer)?;

                    main_data = buffer;
                };
            }
        }

        #[cfg(feature = "embed")]
        if let Some(embed_file) = Asset::get("main.lua") {
            main_data = String::from_utf8(embed_file.data.to_vec()).unwrap()
        }

        lua.load(main_data).exec()?;

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
    pub async fn main(&self) -> Result<bool, String> {
        self.main
            .call_async::<bool>(())
            .await
            .map_err(|e| e.to_string())
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
    pub fn new_project(path: &str) {
        // dump main/base/meta.
        Self::dump(path);

        // dump info_quiver.json.
        Info {
            head: true,
            safe: true,
            path: path.to_string(),
        }
        .dump();
    }

    // create a new info_quiver.json file at the given path.
    pub fn load_project(path: &str) {
        // dump info_quiver.json.
        Info {
            head: true,
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
                if let Ok(text) = text.to_string() {
                    println!("{}", text);
                } else {
                    println!("{:?}", text);
                }
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

        #[cfg(feature = "embed")]
        {
            let loader: mlua::Table = package.get("loaders")?;
            loader.push(lua.create_function(|lua, path: String| {
                let path = format!("{path}.lua");

                if let Some(asset) = Asset::get(&path) {
                    if let Ok(asset) = String::from_utf8(asset.data.to_vec()) {
                        Ok(mlua::Value::Function(lua.load(asset).into_function()?))
                    } else {
                        Err(mlua::Error::runtime(format!(
                            "File '\"{path}\"' did not contain valid UTF-8 data."
                        )))
                    }
                } else {
                    lua.to_value(&format!("\n\tno file '\"{path}\"' in embed data"))
                }
            })?)?;
        }

        // create the quiver table.
        let quiver = lua.create_table()?;

        // set the standard Quiver library.
        general::set_global(lua, info, &quiver)?;
        window::set_global(lua, info, &quiver)?;
        draw::set_global(lua, info, &quiver)?;
        input::set_global(lua, info, &quiver)?;
        model::set_global(lua, info, &quiver)?;
        texture::set_global(lua, info, &quiver)?;
        image::set_global(lua, info, &quiver)?;
        sound::set_global(lua, info, &quiver)?;
        music::set_global(lua, info, &quiver)?;
        font::set_global(lua, info, &quiver)?;
        shader::set_global(lua, info, &quiver)?;
        file::set_global(lua, info, &quiver)?;
        data::set_global(lua, info, &quiver)?;

        #[cfg(feature = "rapier3d")]
        rapier::set_global(lua, info, &quiver)?;

        #[cfg(feature = "zip")]
        zip::set_global(lua, info, &quiver)?;

        #[cfg(feature = "request")]
        request::set_global(lua, info, &quiver)?;

        #[cfg(feature = "steam")]
        steam::set_global(lua, info, &quiver)?;

        #[cfg(feature = "discord")]
        discord::set_global(lua, info, &quiver)?;

        #[cfg(feature = "video")]
        video::set_global(lua, info, &quiver)?;

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

        std::fs::create_dir(format!("{path}/base")).unwrap();

        // dump base library.
        for base in Self::FILE_BASE {
            std::fs::write(format!("{path}/{}", base.name), base.data)
                .map_err(|e| Status::panic(&e.to_string()))
                .unwrap();
        }

        std::fs::write(
            format!("{path}/{}", Self::FILE_BASE_MAIN.name),
            Self::FILE_BASE_MAIN.data,
        )
        .map_err(|e| Status::panic(&e.to_string()))
        .unwrap();

        // dump meta.lua.
        std::fs::write(format!("{path}/{}", Self::NAME_META), Self::FILE_META)
            .map_err(|e| Status::panic(&e.to_string()))
            .unwrap();
    }

    pub fn rust_to_c_string(text: &str) -> mlua::Result<CString> {
        CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))
    }

    pub fn c_to_rust_string(text: *const i8) -> mlua::Result<String> {
        unsafe {
            Ok(CStr::from_ptr(text)
                .to_str()
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .to_string())
        }
    }
}

//================================================================

#[derive(Serialize)]
pub struct Feature {
    serialization: bool,
    system_info: bool,
    file_notify: bool,
    rapier3d: bool,
    rapier2d: bool,
    zip: bool,
    request: bool,
    steam: bool,
    discord: bool,
    embed: bool,
    video: bool,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            serialization: cfg!(feature = "serialization"),
            system_info: cfg!(feature = "system_info"),
            file_notify: cfg!(feature = "file_notify"),
            rapier3d: cfg!(feature = "rapier3d"),
            rapier2d: cfg!(feature = "rapier2d"),
            zip: cfg!(feature = "zip"),
            request: cfg!(feature = "request"),
            steam: cfg!(feature = "steam"),
            discord: cfg!(feature = "discord"),
            embed: cfg!(feature = "embed"),
            video: cfg!(feature = "video"),
        }
    }
}

#[derive(Serialize)]
pub struct ScriptData {
    pub info: Info,
    pub version: String,
    pub feature: Feature,
    pub path_escape: bool,
}

impl ScriptData {
    pub fn new(info: Info) -> Self {
        Self {
            info,
            version: Status::VERSION.to_string(),
            feature: Feature::new(),
            path_escape: false,
        }
    }

    #[rustfmt::skip]
    pub fn get_path(lua: &Lua, path: &str) -> mlua::Result<String> {
        let script_data = lua.app_data_ref::<ScriptData>().unwrap();

        if script_data.info.safe {
            let path = format!("{}/{path}", script_data.info.path);

            // always disallow going up the directory in safe mode.
            let path = path.replace("../", "");
            let path = path.replace("..",  "");

            Ok(path)
        } else if script_data.path_escape {
            Ok(path.to_string())
        } else {
            Ok(format!("{}/{path}", script_data.info.path))
        }
    }

    pub fn get_path_escape(lua: &Lua) -> mlua::Result<bool> {
        let script_data = lua.app_data_ref::<ScriptData>().unwrap();

        Ok(script_data.path_escape)
    }

    pub fn set_path_escape(lua: &Lua, state: bool) -> mlua::Result<()> {
        let mut script_data = lua.app_data_mut::<ScriptData>().unwrap();

        script_data.path_escape = state;

        Ok(())
    }
}
