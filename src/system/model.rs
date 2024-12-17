/*
* MIT License
*
* Copyright (c) 2024 sockentrocken
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.model", "info": "The model API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let model = lua.create_table()?;

    model.set("new", lua.create_function(self::Model::new)?)?;

    table.set("model", model)?;

    Ok(())
}

type RLModel = raylib::models::Model;

/* class
{
    "version": "1.0.0",
    "name": "model",
    "info": "An unique handle for a model in memory.",
    "member": [
        { "name": "mesh_count",     "info": "Mesh count.",     "kind": "number" },
        { "name": "bone_count",     "info": "Bone count.",     "kind": "number" },
        { "name": "material_count", "info": "Material count.", "kind": "number" }
    ]
}
*/
pub struct Model(pub RLModel);

impl Model {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.model.new",
        "info": "Create a new Model resource.",
        "member": [
            { "name": "path", "info": "Path to model file.", "kind": "string" }
        ],
        "result": [
            { "name": "Model", "info": "Model resource.", "kind": "model" }
        ]
    }
    */
    fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadModel(name.as_ptr());

            if ffi::IsModelValid(data) {
                Ok(Self(RLModel::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Model::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

impl mlua::UserData for Model {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("mesh_count", |_, this| Ok(this.0.meshCount));
        field.add_field_method_get("material_count", |_, this| Ok(this.0.materialCount));
        field.add_field_method_get("bone_count", |_, this| Ok(this.0.boneCount));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "model:bind",
            "info": "Bind a texture to the model.",
            "member": [
                { "name": "texture", "info": "Texture to bind to model.", "kind": "texture" }
            ]
        }
        */
        method.add_method_mut("bind", |_, this, texture: LuaAnyUserData| {
            if texture.is::<crate::system::texture::Texture>() {
                let texture = texture.borrow::<crate::system::texture::Texture>().unwrap();
                let texture = &*texture;

                this.0.materials_mut()[0].maps_mut()
                    [MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
                    .texture = *texture.0;
            }

            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "model:draw", "info": "Draw the model." }
        */
        method.add_method("draw", |_, this, ()| unsafe {
            ffi::DrawModel(*this.0, Vector3::zero().into(), 1.0, Color::WHITE.into());
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "model:mesh_vertex",
            "info": "Get the vertex data of a specific mesh in the model.",
            "member": [
                { "name": "index", "info": "Index of mesh.", "kind": "number" }
            ],
            "result": [
                { "name": "table", "info": "Vector3 table.", "kind": "table" }
            ]
        }
        */
        method.add_method("mesh_vertex", |lua, this, index: usize| {
            let mesh = &this.0.meshes()[index];
            lua.to_value(mesh.vertices())
        });
    }
}
