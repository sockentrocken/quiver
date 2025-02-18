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

use mlua::prelude::*;
use raylib::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.data", "info": "The data API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let data = lua.create_table()?;

    // CompressData
    data.set("compress",    lua.create_function(self::compress)?)?;
    // DecompressData
    data.set("decompress", lua.create_function(self::decompress)?)?;
    // EncodeDataBase64
    data.set("encode",     lua.create_function(self::encode)?)?;
    // DecodeDataBase64
    data.set("decode",     lua.create_function(self::decode)?)?;
    // ComputeCRC32/MD5/SHA1
    data.set("hash",        lua.create_function(self::hash)?)?;
    data.set("serialize",   lua.create_function(self::serialize)?)?;
    data.set("deserialize", lua.create_function(self::deserialize)?)?;
    data.set("to_byte",   lua.create_function(self::to_byte)?)?;
    data.set("from_byte", lua.create_function(self::from_byte)?)?;

    table.set("data", data)?;

    Ok(())
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.compress",
    "info": "TO-DO"
}
*/
fn compress(_: &Lua, data: Vec<u8>) -> mlua::Result<Vec<u8>> {
    let mut out = 0;
    unsafe {
        let value = ffi::CompressData(data.as_ptr(), data.len() as i32, &mut out);
        let slice = std::slice::from_raw_parts(value, out as usize).to_vec();

        Ok(slice)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.decompress",
    "info": "TO-DO"
}
*/
fn decompress(_: &Lua, data: Vec<u8>) -> mlua::Result<Vec<u8>> {
    let mut out = 0;
    unsafe {
        let value = ffi::DecompressData(data.as_ptr(), data.len() as i32, &mut out);
        let slice = std::slice::from_raw_parts(value, out as usize).to_vec();

        Ok(slice)
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
fn encode(_: &Lua, data: Vec<u8>) -> mlua::Result<Vec<i8>> {
    let mut out = 0;
    unsafe {
        let value = ffi::EncodeDataBase64(data.as_ptr(), data.len() as i32, &mut out);
        let slice = std::slice::from_raw_parts(value, out as usize).to_vec();

        Ok(slice)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.decode",
    "info": "TO-DO"
}
*/
fn decode(_: &Lua, data: Vec<u8>) -> mlua::Result<Vec<u8>> {
    let mut out = 0;
    unsafe {
        let value = ffi::DecodeDataBase64(data.as_ptr(), &mut out);
        let slice = std::slice::from_raw_parts(value, out as usize).to_vec();

        Ok(slice)
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
fn hash(lua: &Lua, (mut data, kind): (Vec<u8>, Option<i32>)) -> mlua::Result<LuaValue> {
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

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.to_byte",
    "info": "TO-DO"
}
*/
fn to_byte(lua: &Lua, (data, kind): (LuaValue, i32)) -> mlua::Result<Vec<u8>> {
    match kind {
        0 => {
            let data: i32 = lua.from_value(data)?;
            Ok(data.to_ne_bytes().to_vec())
        }
        1 => {
            let data: f32 = lua.from_value(data)?;
            Ok(data.to_ne_bytes().to_vec())
        }
        _ => {
            let data: String = lua.from_value(data)?;
            Ok(data.into_bytes())
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.from_byte",
    "info": "TO-DO"
}
*/
fn from_byte(lua: &Lua, (data, kind): (Vec<u8>, i32)) -> mlua::Result<LuaValue> {
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
            let data = String::from_utf8(data).unwrap();
            lua.to_value(&data)
        }
    }
}
