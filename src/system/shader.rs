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

use crate::script::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.shader", "info": "The shader API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let shader = lua.create_table()?;

    shader.set("new", lua.create_function(self::Shader::new)?)?;

    table.set("shader", shader)?;

    Ok(())
}

pub type RLShader = raylib::shaders::Shader;

/* class
{ "version": "1.0.0", "name": "shader", "info": "An unique handle for a shader in memory." }
*/
pub struct Shader(pub RLShader);

impl Shader {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.shader.new",
        "info": "Create a new shader resource.",
        "member": [
            { "name": "v_path", "info": "Path to .vs file.", "kind": "string" },
            { "name": "f_path", "info": "Path to .fs file.", "kind": "string" }
        ],
        "result": [
            { "name": "shader", "info": "Shader resource.", "kind": "shader" }
        ]
    }
    */
    fn new(lua: &Lua, (v_path, f_path): (Option<String>, Option<String>)) -> mlua::Result<Self> {
        let v_path = match v_path {
            Some(name) => {
                let pointer = CString::new(ScriptData::get_path(lua, &name)?)
                    .map_err(|e| mlua::Error::runtime(e.to_string()))?;

                pointer.into_raw()
            }
            None => std::ptr::null(),
        };

        let f_path = match f_path {
            Some(name) => {
                let pointer = CString::new(ScriptData::get_path(lua, &name)?)
                    .map_err(|e| mlua::Error::runtime(e.to_string()))?;

                pointer.into_raw()
            }
            None => std::ptr::null(),
        };

        unsafe {
            let data = ffi::LoadShader(v_path, f_path);

            if ffi::IsShaderValid(data) {
                Ok(Self(RLShader::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(
                    "Shader::new(): Could not load file.".to_string(),
                ))
            }
        }
    }
}

impl mlua::UserData for Shader {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "shader:begin",
            "info": "",
            "member": [
                { "name": "call", "info": "The draw code.", "kind": "function" }
            ]
        }
        */
        method.add_method("begin", |_: &Lua, this, call: mlua::Function| {
            unsafe {
                ffi::BeginShaderMode(*this.0);

                call.call::<()>(())?;

                ffi::EndShaderMode();
            }

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:get_location_name",
            "info": "",
            "member": [
                { "name": "name", "info": "", "kind": "string" }
            ],
            "result": [
                { "name": "location", "info": "", "kind": "number" }
            ]
        }
        */
        method.add_method("get_location_name", |_, this, name: String| {
            Ok(this.0.get_shader_location(&name))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:get_location_attribute_name",
            "info": "",
            "member": [
                { "name": "name", "info": "", "kind": "string" }
            ],
            "result": [
                { "name": "location", "info": "", "kind": "number" }
            ]
        }
        */
        method.add_method("get_location_attribute_name", |_, this, name: String| {
            Ok(this.0.get_shader_location_attribute(&name))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:get_location",
            "info": "",
            "member": [
                { "name": "location", "info": "", "kind": "number" }
            ],
            "result": [
                { "name": "location", "info": "", "kind": "number" }
            ]
        }
        */
        method.add_method("get_location", |_, this, location: usize| {
            Ok(this.0.locs()[location])
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:set_location",
            "info": "",
            "member": [
                { "name": "location", "info": "", "kind": "number" },
                { "name": "value",    "info": "", "kind": "number" }
            ]
        }
        */
        method.add_method_mut(
            "set_location",
            |_, this, (location, value): (usize, i32)| {
                this.0.locs_mut()[location] = value;
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:set_shader_integer",
            "info": "",
            "member": [
                { "name": "location", "info": "", "kind": "number" },
                { "name": "value",    "info": "", "kind": "number" }
            ]
        }
        */
        method.add_method_mut(
            "set_shader_integer",
            |_, this, (location, value): (i32, i32)| {
                this.0.set_shader_value(location, value);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:set_shader_decimal",
            "info": "",
            "member": [
                { "name": "location", "info": "", "kind": "number" },
                { "name": "value",    "info": "", "kind": "number" }
            ]
        }
        */
        method.add_method_mut(
            "set_shader_decimal",
            |_, this, (location, value): (i32, f32)| {
                this.0.set_shader_value(location, value);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:set_shader_vector_3",
            "info": "",
            "member": [
                { "name": "location", "info": "", "kind": "number"   },
                { "name": "value",    "info": "", "kind": "vector_3" }
            ]
        }
        */
        method.add_method_mut(
            "set_shader_vector_3",
            |lua, this, (location, value): (i32, LuaValue)| {
                this.0
                    .set_shader_value(location, lua.from_value::<Vector3>(value)?);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:set_shader_vector_4",
            "info": "",
            "member": [
                { "name": "location", "info": "", "kind": "number"   },
                { "name": "value",    "info": "", "kind": "vector_4" }
            ]
        }
        */
        method.add_method_mut(
            "set_shader_vector_4",
            |lua, this, (location, value): (i32, LuaValue)| {
                this.0
                    .set_shader_value(location, lua.from_value::<Vector4>(value)?);
                Ok(())
            },
        );
    }
}
