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

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.data", "info": "The data API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, _info: &Info, table: &mlua::Table) -> mlua::Result<()> {
    let data = lua.create_table()?;

    // CompressData
    data.set("compress",       lua.create_function(self::compress)?)?;
    // DecompressData
    data.set("decompress",     lua.create_function(self::decompress)?)?;
    // EncodeDataBase64
    data.set("encode",         lua.create_function(self::encode)?)?;
    // DecodeDataBase64
    data.set("decode",         lua.create_function(self::decode)?)?;
    // ComputeCRC32/MD5/SHA1
    data.set("hash",           lua.create_function(self::hash)?)?;
    data.set("serialize",      lua.create_function(self::serialize)?)?;
    data.set("deserialize",    lua.create_function(self::deserialize)?)?;
    data.set("to_data",        lua.create_function(self::to_data)?)?;
    data.set("from_data",      lua.create_function(self::from_data)?)?;

    #[cfg(feature = "embed")]
    data.set("get_embed_file", lua.create_function(self::get_embed_file)?)?;

    #[cfg(feature = "embed")]
    data.set("get_embed_list", lua.create_function(self::get_embed_list)?)?;

    data.set("new",            lua.create_function(self::Data::<u8>::new)?)?;

    table.set("data", data)?;

    Ok(())
}

pub struct Data<T: Clone + IntoLua + Send + 'static>(pub Vec<T>);

impl<T: Clone + IntoLua + Send + 'static> Data<T> {
    pub fn new(_: &Lua, data: Vec<T>) -> mlua::Result<Self> {
        Ok(Self(data))
    }

    pub fn get_buffer(value: LuaValue) -> mlua::Result<LuaUserDataRef<Self>> {
        if let Some(data) = value.as_userdata() {
            if let Ok(data) = data.borrow::<Self>() {
                Ok(data)
            } else {
                Err(mlua::Error::RuntimeError(
                    "Data::get_buffer(): Error borrowing buffer.".to_string(),
                ))
            }
        } else {
            Err(mlua::Error::RuntimeError(
                "Data::get_buffer(): Value is not a Data user-data.".to_string(),
            ))
        }
    }

    pub fn get_buffer_mut(value: LuaValue) -> mlua::Result<LuaUserDataRefMut<Self>> {
        if let Some(data) = value.as_userdata() {
            if let Ok(data) = data.borrow_mut::<Self>() {
                Ok(data)
            } else {
                Err(mlua::Error::RuntimeError(
                    "Data::get_buffer(): Error borrowing buffer.".to_string(),
                ))
            }
        } else {
            Err(mlua::Error::RuntimeError(
                "Data::get_buffer(): Value is not a Data user-data.".to_string(),
            ))
        }
    }
}

impl<T: Clone + IntoLua + Send + 'static> mlua::UserData for Data<T> {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "data:foo",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_length", |_: &Lua, this, _: ()| Ok(this.0.len()));

        /* entry
        {
            "version": "1.0.0",
            "name": "data:get_buffer",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_buffer", |_: &Lua, this, _: ()| Ok(this.0.clone()));

        /* entry
        {
            "version": "1.0.0",
            "name": "data:get_slice",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "get_slice",
            |lua: &Lua, this, (index_a, index_b): (usize, usize)| {
                if (index_a <= this.0.len() && index_b <= this.0.len()) && (index_a <= index_b) {
                    let slice = &this.0[index_a..index_b];

                    let data = crate::system::data::Data::new(lua, slice.to_vec())?;

                    Ok(data)
                } else {
                    Err(mlua::Error::runtime("Data::get_slice(): Invalid index."))
                }
            },
        );
    }
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.compress",
    "info": "TO-DO",
    "test": "data/compress_decompress.lua"
}
*/
fn compress(lua: &Lua, data: LuaValue) -> mlua::Result<Data<u8>> {
    let data = Data::get_buffer(data)?;
    let data = &data.0;
    let mut out = 0;
    unsafe {
        let value = ffi::CompressData(data.as_ptr(), data.len() as i32, &mut out);
        let slice = std::slice::from_raw_parts(value, out as usize).to_vec();

        Data::new(lua, slice)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.decompress",
    "info": "TO-DO",
    "test": "data/compress_decompress.lua"
}
*/
fn decompress(lua: &Lua, data: LuaValue) -> mlua::Result<Data<u8>> {
    let data = Data::get_buffer(data)?;
    let data = &data.0;
    let mut out = 0;
    unsafe {
        let value = ffi::DecompressData(data.as_ptr(), data.len() as i32, &mut out);
        let slice = std::slice::from_raw_parts(value, out as usize).to_vec();

        Data::new(lua, slice)
    }
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.encode",
    "info": "TO-DO"
}
*/
fn encode(lua: &Lua, data: LuaValue) -> mlua::Result<Data<i8>> {
    let data = Data::get_buffer(data)?;
    let data = &data.0;
    let mut out = 0;
    unsafe {
        let value = ffi::EncodeDataBase64(data.as_ptr(), data.len() as i32, &mut out);
        let slice = std::slice::from_raw_parts(value, out as usize).to_vec();

        Data::new(lua, slice)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.decode",
    "info": "TO-DO"
}
*/
fn decode(lua: &Lua, data: LuaValue) -> mlua::Result<Data<u8>> {
    let data = Data::get_buffer(data)?;
    let data = &data.0;
    let mut out = 0;
    unsafe {
        let value = ffi::DecodeDataBase64(data.as_ptr(), &mut out);
        let slice = std::slice::from_raw_parts(value, out as usize).to_vec();

        Data::new(lua, slice)
    }
}

//================================================================

struct HashKind;

impl HashKind {
    const CRC32: i32 = 0;
    const MD5: i32 = 1;
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.hash",
    "info": "TO-DO"
}
*/
fn hash(lua: &Lua, (data, kind): (LuaValue, Option<i32>)) -> mlua::Result<LuaValue> {
    let mut data = Data::get_buffer_mut(data)?;
    let data = &mut data.0;
    let kind = kind.unwrap_or_default();

    unsafe {
        match kind {
            HashKind::CRC32 => {
                lua.to_value(&ffi::ComputeCRC32(data.as_mut_ptr(), data.len() as i32))
            }
            HashKind::MD5 => {
                let value = ffi::ComputeMD5(data.as_mut_ptr(), data.len() as i32);
                let value = vec![
                    *value.wrapping_add(0),
                    *value.wrapping_add(1),
                    *value.wrapping_add(2),
                    *value.wrapping_add(3),
                ];

                lua.to_value(&value)
            }
            _ => {
                let value = ffi::ComputeSHA1(data.as_mut_ptr(), data.len() as i32);
                let value = vec![
                    *value.wrapping_add(0),
                    *value.wrapping_add(1),
                    *value.wrapping_add(2),
                    *value.wrapping_add(3),
                    *value.wrapping_add(4),
                ];

                lua.to_value(&value)
            }
        }
    }
}

//================================================================

struct FormatKind;

impl FormatKind {
    const JSON: i32 = 0;
    const YAML: i32 = 1;
    const TOML: i32 = 2;
    const XML: i32 = 3;
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.serialize",
    "info": "Serialize a given Lua value as another format, in the form of a string.",
    "member": [
        { "name": "text", "info": "Lua value to serialize.", "kind": "any"    },
        { "name": "kind", "info": "Format.",                 "kind": "number" }
    ],
    "result": [
        { "name": "value", "info": "The value, in string form.", "kind": "string" }
    ]
}
*/
#[cfg(feature = "serialization")]
fn serialize(lua: &Lua, (text, kind): (LuaValue, Option<i32>)) -> mlua::Result<String> {
    let kind = kind.unwrap_or_default();

    match kind {
        FormatKind::JSON => {
            let text: serde_json::Value = lua.from_value(text)?;
            serde_json::to_string(&text).map_err(|e| mlua::Error::runtime(e.to_string()))
        }
        FormatKind::YAML => {
            let text: serde_json::Value = lua.from_value(text)?;
            serde_yaml::to_string(&text).map_err(|e| mlua::Error::runtime(e.to_string()))
        }
        FormatKind::TOML => {
            let text: serde_json::Value = lua.from_value(text)?;
            toml::to_string(&text).map_err(|e| mlua::Error::runtime(e.to_string()))
        }
        FormatKind::XML => {
            let text: serde_json::Value = lua.from_value(text)?;
            serde_xml_rs::to_string(&text).map_err(|e| mlua::Error::runtime(e.to_string()))
        }
        _ => {
            let text: serde_json::Value = lua.from_value(text)?;
            serde_ini::to_string(&text).map_err(|e| mlua::Error::runtime(e.to_string()))
        }
    }
}

#[cfg(not(feature = "serialization"))]
fn serialize(lua: &Lua, (text, _): (LuaValue, Option<i32>)) -> mlua::Result<String> {
    let text: serde_json::Value = lua.from_value(text)?;
    serde_json::to_string(&text).map_err(|e| mlua::Error::runtime(e.to_string()))
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.deserialize",
    "info": "Deserialize a given format string as a Lua value.",
    "member": [
        { "name": "text", "info": "String to deserialize.", "kind": "string" },
        { "name": "kind", "info": "Format.",                "kind": "number" }
    ],
    "result": [
        { "name": "value", "info": "The value, in Lua value form.", "kind": "any" }
    ]
}
*/
#[cfg(feature = "serialization")]
fn deserialize(lua: &Lua, (text, kind): (String, Option<i32>)) -> mlua::Result<LuaValue> {
    let kind = kind.unwrap_or_default();

    match kind {
        FormatKind::JSON => {
            let text: serde_json::Value =
                serde_json::from_str(&text).map_err(|e| mlua::Error::runtime(e.to_string()))?;
            lua.to_value(&text)
        }
        FormatKind::YAML => {
            let text: serde_json::Value =
                serde_yaml::from_str(&text).map_err(|e| mlua::Error::runtime(e.to_string()))?;
            lua.to_value(&text)
        }
        FormatKind::TOML => {
            let text: serde_json::Value =
                toml::from_str(&text).map_err(|e| mlua::Error::runtime(e.to_string()))?;
            lua.to_value(&text)
        }
        FormatKind::XML => {
            let text: serde_json::Value =
                serde_xml_rs::from_str(&text).map_err(|e| mlua::Error::runtime(e.to_string()))?;
            lua.to_value(&text)
        }
        _ => {
            let text: serde_json::Value =
                serde_ini::from_str(&text).map_err(|e| mlua::Error::runtime(e.to_string()))?;
            lua.to_value(&text)
        }
    }
}

#[cfg(not(feature = "serialization"))]
fn deserialize(lua: &Lua, (text, _): (String, Option<i32>)) -> mlua::Result<LuaValue> {
    let text: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| mlua::Error::runtime(e.to_string()))?;
    lua.to_value(&text)
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.to_data",
    "info": "TO-DO"
}
*/
fn to_data(lua: &Lua, (data, kind): (LuaValue, i32)) -> mlua::Result<Data<u8>> {
    match kind {
        0 => {
            let data: i32 = lua.from_value(data)?;
            Data::new(lua, data.to_ne_bytes().to_vec())
        }
        1 => {
            let data: f32 = lua.from_value(data)?;
            Data::new(lua, data.to_ne_bytes().to_vec())
        }
        _ => {
            let data: String = lua.from_value(data)?;
            Data::new(lua, data.into_bytes().to_vec())
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.from_data",
    "info": "TO-DO"
}
*/
fn from_data(lua: &Lua, (data, kind): (LuaValue, i32)) -> mlua::Result<LuaValue> {
    let data = Data::get_buffer(data)?;
    let data = &data.0;

    match kind {
        0 => {
            let data = i32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
            lua.to_value(&data)
        }
        1 => {
            let data = f32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
            lua.to_value(&data)
        }
        _ => {
            let data = String::from_utf8(data.to_vec()).unwrap();
            lua.to_value(&data)
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "feature": "embed",
    "name": "quiver.data.get_embed_file",
    "info": "TO-DO"
}
*/
#[cfg(feature = "embed")]
fn get_embed_file(lua: &Lua, path: String) -> mlua::Result<LuaValue> {
    if let Some(asset) = crate::status::Asset::get(&path) {
        let data = crate::system::data::Data::new(lua, asset.data.to_vec())?;
        let data = lua.create_userdata(data)?;

        Ok(mlua::Value::UserData(data))
    } else {
        Ok(mlua::Value::Nil)
    }
}

/* entry
{
    "version": "1.0.0",
    "feature": "embed",
    "name": "quiver.data.get_embed_list",
    "info": "TO-DO"
}
*/
#[cfg(feature = "embed")]
fn get_embed_list(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    let list: Vec<String> = crate::status::Asset::iter()
        .map(|i| i.to_string())
        .collect();

    lua.to_value(&list)
}
