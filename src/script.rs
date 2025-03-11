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
use serde::Deserialize;

#[cfg(feature = "zip")]
use std::io::Read;

use mlua::prelude::*;
use serde::Serialize;
use std::ffi::{CStr, CString};

//================================================================

pub static mut CALL_BACK_SAVE_FILE: Option<mlua::Function> = None;
pub static mut CALL_BACK_LOAD_FILE: Option<mlua::Function> = None;
pub static mut CALL_BACK_SAVE_TEXT: Option<mlua::Function> = None;
pub static mut CALL_BACK_LOAD_TEXT: Option<mlua::Function> = None;

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
    pub info: ScriptInfo,
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
    const CALL_INFO: &'static str = "info";
    const CALL_FAIL: &'static str = "fail";

    //================================================================

    #[allow(dead_code)]
    pub async fn new_test(path: &str) -> mlua::Result<()> {
        // initialize lua VM, depending on what safe flag is set.
        let lua = Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new())?;

        let status_info = StatusInfo {
            safe: true,
            path: "test/asset".to_string(),
        };

        let quiver = Self::set_environment(&lua, &status_info)?;

        let script_info = ScriptInfo::default();

        // set script data.
        lua.set_app_data(ScriptData::new(status_info.clone(), script_info.clone()));

        // set the standard Quiver library.
        Self::system(&lua, &quiver, &status_info, Some(&script_info))?;

        let main_data = std::fs::read(path)
            .unwrap_or_else(|_| panic!("Script::new_test(): Could not read file \"{path}\""));

        lua.load("quiver.general.load_base()").exec()?;

        lua.load(main_data).exec_async().await?;

        Ok(())
    }

    // get a new script instance.
    pub async fn new(status_info: &StatusInfo) -> mlua::Result<Self> {
        // initialize lua VM, depending on what safe flag is set.
        let lua = {
            if status_info.safe {
                // quiver is in safe mode, only load the safe standard Lua library.
                Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new())?
            } else {
                // quiver is in unsafe mode, load every Lua library and allow loading foreign native code.
                unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) }
            }
        };

        unsafe {
            // this is causing lua to be unable to load main.lua initially...?
            raylib::ffi::ChangeDirectory(Script::rust_to_c_string(&status_info.path)?.as_ptr());
        }

        let quiver = Self::set_environment(&lua, status_info)?;

        // set the standard Quiver library.
        Self::system(&lua, &quiver, status_info, None)?;

        lua.load(Self::get_main_data(status_info)?).exec()?;

        let script_info = Self::get_script_info(&lua, &quiver).await?;

        // set script data.
        lua.set_app_data(ScriptData::new(status_info.clone(), script_info.clone()));

        // set the standard Quiver library.
        Self::system(&lua, &quiver, status_info, Some(&script_info))?;

        // get the main function.
        let main: mlua::Function = quiver.get(Self::CALL_MAIN)?;

        // get the fail function.
        let fail: Option<mlua::Function> = quiver.get(Self::CALL_FAIL).unwrap_or(None);

        Ok(Self {
            lua,
            main,
            info: script_info,
            fail,
        })
    }

    // main Lua entry-point.
    pub async fn main(&self) -> Result<bool, String> {
        self.main
            .call_async::<bool>(())
            .await
            .map_err(|e| e.to_string())
    }

    // fail Lua entry-point.
    pub async fn fail(&self, message: &str) -> Result<bool, String> {
        if let Some(fail) = &self.fail {
            fail.call_async::<bool>(message)
                .await
                .map_err(|e| e.to_string())
        } else {
            Ok(false)
        }
    }

    // create a new info.json file at the given path, and dump main/base/meta.lua into the path.
    pub fn new_project(path: &str) {
        // dump main/base/meta.
        Self::dump(path);

        // dump info.json.
        StatusInfo {
            safe: true,
            path: path.to_string(),
        }
        .dump();
    }

    // create a new info.json file at the given path.
    pub fn load_project(path: &str) {
        // dump info.json.
        StatusInfo {
            safe: true,
            path: path.to_string(),
        }
        .dump();
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

    //================================================================

    // load every standard library into the quiver table.
    #[rustfmt::skip]
    fn system(
        lua: &Lua,
        quiver: &mlua::Table,
        status_info: &StatusInfo,
        script_info: Option<&ScriptInfo>,
    ) -> mlua::Result<()> {
        general::set_global(lua, quiver, status_info, script_info)?;
        window::set_global (lua, quiver, status_info, script_info)?;
        draw::set_global   (lua, quiver, status_info, script_info)?;
        input::set_global  (lua, quiver, status_info, script_info)?;
        model::set_global  (lua, quiver, status_info, script_info)?;
        texture::set_global(lua, quiver, status_info, script_info)?;
        image::set_global  (lua, quiver, status_info, script_info)?;
        sound::set_global  (lua, quiver, status_info, script_info)?;
        music::set_global  (lua, quiver, status_info, script_info)?;
        font::set_global   (lua, quiver, status_info, script_info)?;
        shader::set_global (lua, quiver, status_info, script_info)?;
        file::set_global   (lua, quiver, status_info, script_info)?;
        data::set_global   (lua, quiver, status_info, script_info)?;
        socket::set_global (lua, quiver, status_info, script_info)?;

        #[cfg(feature = "rapier3d")] rapier::set_global (lua, quiver, status_info, script_info)?;
        #[cfg(feature = "zip")]      zip::set_global    (lua, quiver, status_info, script_info)?;
        #[cfg(feature = "request")]  request::set_global(lua, quiver, status_info, script_info)?;
        #[cfg(feature = "steam")]    steam::set_global  (lua, quiver, status_info, script_info)?;
        #[cfg(feature = "discord")]  discord::set_global(lua, quiver, status_info, script_info)?;
        #[cfg(feature = "video")]    video::set_global  (lua, quiver, status_info, script_info)?;

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

    fn set_environment(lua: &Lua, _status_info: &StatusInfo) -> mlua::Result<mlua::Table> {
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
        //let package = global.get::<mlua::Table>("package")?;
        //package.set(
        //    "path",
        //    format!(
        //        "{:?};{}/?.lua",
        //        package.get::<mlua::String>("path")?,
        //        status_info.path
        //    ),
        //)?;

        #[cfg(feature = "embed")]
        {
            let package = global.get::<mlua::Table>("package")?;
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

        // get the global table.
        let global = lua.globals();

        // set the quiver table as a global value.
        global.set("quiver", lua.create_table()?)?;

        // get the quiver table.
        global.get("quiver")
    }

    #[allow(unused)]
    fn get_main_data(status_info: &StatusInfo) -> mlua::Result<String> {
        #[allow(unused_mut)]
        let mut main_data = format!("require \"{}\"", Self::CALL_MAIN);

        #[cfg(feature = "zip")]
        {
            // get the path to the main folder or file.
            //let main_path = format!("{}/{}", status_info.path, Self::CALL_MAIN);
            let main_path = Self::CALL_MAIN;
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
        if let Some(embed_file) = Asset::get(Self::NAME_MAIN) {
            main_data = String::from_utf8(embed_file.data.to_vec()).unwrap()
        }

        Ok(main_data)
    }

    async fn get_script_info(lua: &Lua, quiver: &mlua::Table) -> mlua::Result<ScriptInfo> {
        // get the info function.
        let script_info: Option<mlua::Function> = quiver.get(Self::CALL_INFO).unwrap_or(None);

        if let Some(script_info) = script_info {
            let script_info = script_info
                .call_async::<LuaValue>(())
                .await
                .map_err(|e| mlua::Error::runtime(e.to_string()))?;

            lua.from_value(script_info)
        } else {
            Ok(ScriptInfo::default())
        }
    }
}

impl Drop for Script {
    fn drop(&mut self) {
        unsafe {
            CALL_BACK_SAVE_FILE = None;
            CALL_BACK_LOAD_FILE = None;
            CALL_BACK_SAVE_TEXT = None;
            CALL_BACK_LOAD_TEXT = None;
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
    pub status_info: StatusInfo,
    pub script_info: ScriptInfo,
    pub version: String,
    pub feature: Feature,
    pub path_escape: bool,
}

impl ScriptData {
    pub fn new(status_info: StatusInfo, script_info: ScriptInfo) -> Self {
        Self {
            status_info,
            script_info,
            version: Status::VERSION.to_string(),
            feature: Feature::new(),
            path_escape: false,
        }
    }

    #[rustfmt::skip]
    pub fn get_path(lua: &Lua, path: &str) -> mlua::Result<String> {
        let script_data = lua.app_data_ref::<ScriptData>().unwrap();

        if script_data.status_info.safe {
            //let path = format!("{}/{path}", script_data.status_info.path);

            println!("================");
            println!("{path}");
            println!("================");

            // always disallow going up the directory in safe mode.
            let path = path.replace("../", "");
            let path = path.replace("..",  "");

            Ok(path)
        } else if script_data.path_escape {
            Ok(path.to_string())
        } else {
            Ok(format!("{}/{path}", script_data.status_info.path))
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

//================================================================

#[derive(Serialize, Deserialize, Clone)]
pub struct ScriptInfo {
    #[serde(default = "ScriptInfo::name")]
    pub name: String,
    #[serde(default = "ScriptInfo::icon")]
    pub icon: Option<String>,
    #[serde(default = "ScriptInfo::size")]
    pub size: (i32, i32),
    #[serde(default = "ScriptInfo::rate")]
    pub rate: u32,
    #[serde(default = "ScriptInfo::head")]
    pub head: bool,
    #[serde(default = "ScriptInfo::sync")]
    pub sync: bool,
    #[serde(default = "ScriptInfo::full")]
    pub full: bool,
    #[serde(default = "ScriptInfo::no_border")]
    pub no_border: bool,
    #[serde(default = "ScriptInfo::no_decor")]
    pub no_decor: bool,
    #[serde(default = "ScriptInfo::no_focus")]
    pub no_focus: bool,
    #[serde(default = "ScriptInfo::resizable")]
    pub resizable: bool,
    #[serde(default = "ScriptInfo::hidden")]
    pub hidden: bool,
    #[serde(default = "ScriptInfo::minimize")]
    pub minimize: bool,
    #[serde(default = "ScriptInfo::maximize")]
    pub maximize: bool,
    #[serde(default = "ScriptInfo::always_top")]
    pub always_top: bool,
    #[serde(default = "ScriptInfo::always_run")]
    pub always_run: bool,
    #[serde(default = "ScriptInfo::alpha")]
    pub alpha: bool,
    #[serde(default = "ScriptInfo::scale")]
    pub scale: bool,
    #[serde(default = "ScriptInfo::msaa")]
    pub msaa: bool,
    #[serde(default = "ScriptInfo::mouse_pass")]
    pub mouse_pass: bool,
    #[serde(default = "ScriptInfo::interlace")]
    pub interlace: bool,
}

#[rustfmt::skip]
impl ScriptInfo {
    fn name()       -> String         { "Quiver".to_string() }
    fn icon()       -> Option<String> { None }
    fn size()       -> (i32, i32)     { (1024, 768) }
    fn rate()       -> u32            { 60    }
    fn head()       -> bool           { true  }
    fn sync()       -> bool           { false }
    fn full()       -> bool           { false }
    fn no_border()  -> bool           { false }
    fn no_decor()   -> bool           { false }
    fn no_focus()   -> bool           { false }
    fn resizable()  -> bool           { false }
    fn hidden()     -> bool           { false }
    fn minimize()   -> bool           { false }
    fn maximize()   -> bool           { false }
    fn always_top() -> bool           { false }
    fn always_run() -> bool           { false }
    fn alpha()      -> bool           { false }
    fn scale()      -> bool           { false }
    fn msaa()       -> bool           { false }
    fn mouse_pass() -> bool           { false }
    fn interlace()  -> bool           { false }
}

#[rustfmt::skip]
impl Default for ScriptInfo {
    fn default() -> Self {
        Self {
            name:       Self::name(),
            icon:       Self::icon(),
            size:       Self::size(),
            rate:       Self::rate(),
            head:       Self::head(),
            sync:       Self::sync(),
            full:       Self::full(),
            no_border:  Self::no_border(),
            no_decor:   Self::no_decor(),
            no_focus:   Self::no_focus(),
            resizable:  Self::resizable(),
            hidden:     Self::hidden(),
            minimize:   Self::minimize(),
            maximize:   Self::maximize(),
            always_top: Self::always_top(),
            always_run: Self::always_run(),
            alpha:      Self::alpha(),
            scale:      Self::scale(),
            msaa:       Self::msaa(),
            mouse_pass: Self::mouse_pass(),
            interlace:  Self::interlace(),
        }
    }
}
