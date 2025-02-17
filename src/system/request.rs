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

use mlua::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.request", "info": "The request API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let request = lua.create_table()?;

    request.set("get",  lua.create_async_function(self::get)?)?;

    table.set("request", request)?;

    Ok(())
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "quiver.request.get",
    "info": ""
}
*/
async fn get(lua: Lua, (link, binary): (String, bool)) -> mlua::Result<LuaValue> {
    if binary {
        lua.to_value(
            &reqwest::get(link)
                .await
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .bytes()
                .await
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .to_vec(),
        )
    } else {
        lua.to_value(
            &reqwest::get(link)
                .await
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .text()
                .await
                .map_err(|e| mlua::Error::runtime(e.to_string()))?,
        )
    }
}
