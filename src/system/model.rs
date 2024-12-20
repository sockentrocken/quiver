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

    let model_animation = lua.create_table()?;

    model_animation.set("new", lua.create_function(self::ModelAnimation::new)?)?;

    table.set("model_animation", model_animation)?;

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
            { "name": "model", "info": "Model resource.", "kind": "model" }
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
        {
            "version": "1.0.0",
            "name": "model:draw",
            "info": "Draw the model.",
            "member": [
                { "name": "point", "info": "", "kind": "vector_3" },
                { "name": "scale", "info": "", "kind": "number"   },
                { "name": "color", "info": "", "kind": "color"    }
            ]
        }
        */
        method.add_method(
            "draw",
            |lua, this, (point, scale, color): (LuaValue, f32, LuaValue)| unsafe {
                let point: Vector3 = lua.from_value(point)?;
                let color: Color = lua.from_value(color)?;

                ffi::DrawModel(*this.0, point.into(), scale, color.into());
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "model:draw_transform",
            "info": "Draw the model with a transformation.",
            "member": [
                { "name": "point", "info": "", "kind": "vector_3" },
                { "name": "angle", "info": "", "kind": "vector_4" },
                { "name": "scale", "info": "", "kind": "vector_3" },
                { "name": "color", "info": "", "kind": "color"    }
            ]
        }
        */
        method.add_method_mut(
            "draw_transform",
            |lua, this, (point, angle, scale, color): (LuaValue, LuaValue, LuaValue, LuaValue)| unsafe {
                let point: Vector3 = lua.from_value(point)?;
                let angle: Vector4 = lua.from_value(angle)?;
                let scale: Vector3 = lua.from_value(scale)?;
                let color: Color = lua.from_value(color)?;

                this.0.transform = (angle.to_matrix() * Matrix::scale(scale.x, scale.y, scale.z)).into();

                ffi::DrawModel(*this.0, point.into(), 1.0, color.into());

                this.0.transform = Matrix::identity().into();

                Ok(())
            },
        );

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

type RLModelAnimation = raylib::models::ModelAnimation;

/* class
{
    "version": "1.0.0",
    "name": "model_animation",
    "info": "An unique handle for a model animation in memory."
}
*/
pub struct ModelAnimation(pub Vec<RLModelAnimation>);

impl ModelAnimation {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.model_animation.new",
        "info": "Create a new ModelAnimation resource.",
        "member": [
            { "name": "path", "info": "Path to model file.", "kind": "string" }
        ],
        "result": [
            { "name": "model_animation", "info": "ModelAnimation resource.", "kind": "model_animation" }
        ]
    }
    */
    fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let mut count = 0;
            let data = ffi::LoadModelAnimations(name.as_ptr(), &mut count);
            let mut list: Vec<RLModelAnimation> = Vec::new();

            if count == 0 {
                return Err(mlua::Error::RuntimeError(format!(
                    "ModelAnimation::new(): Could not load file \"{path}\"."
                )));
            }

            for x in 0..count {
                let animation = data.wrapping_add(x.try_into().unwrap());

                println!("Pushing animation {x}");

                list.push(RLModelAnimation::from_raw(*animation));
            }

            Ok(Self(list))
        }
    }
}

impl mlua::UserData for ModelAnimation {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "model_animation:update",
            "info": "Update model with new model animation data.",
            "member": [
                { "name": "model", "info": "", "kind": "model"  },
                { "name": "index", "info": "", "kind": "number" },
                { "name": "frame", "info": "", "kind": "number" }
            ]
        }
        */
        method.add_method(
            "update",
            |_, this, (model, index, frame): (LuaAnyUserData, usize, usize)| {
                let animation = this.0.get(index).unwrap();

                if model.is::<Model>() {
                    let model = model.borrow::<Model>().unwrap();

                    unsafe {
                        ffi::UpdateModelAnimation(*model.0, **animation, frame.try_into().unwrap());
                    }
                } else {
                    panic!("not model");
                }

                Ok(())
            },
        );
    }
}
