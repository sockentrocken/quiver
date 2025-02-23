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
    data.set("decompress",  lua.create_function(self::decompress)?)?;
    // EncodeDataBase64
    data.set("encode",      lua.create_function(self::encode)?)?;
    // DecodeDataBase64
    data.set("decode",      lua.create_function(self::decode)?)?;
    // ComputeCRC32/MD5/SHA1
    data.set("hash",        lua.create_function(self::hash)?)?;
    data.set("serialize",   lua.create_function(self::serialize)?)?;
    data.set("deserialize", lua.create_function(self::deserialize)?)?;
    data.set("to_data",     lua.create_function(self::to_data)?)?;
    data.set("from_data",   lua.create_function(self::from_data)?)?;
    data.set("from_data",   lua.create_function(self::from_data)?)?;
    data.set("new",         lua.create_function(self::Data::<u8>::new)?)?;

    table.set("data", data)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "quiver.data.compress",
    "info": "TO-DO"
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
    "info": "TO-DO"
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
