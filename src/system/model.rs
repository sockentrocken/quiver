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
use std::collections::HashMap;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use serde::Deserialize;
use std::ffi::{CStr, CString};

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

#[derive(Deserialize)]
struct TransformBatch(usize, f32, f32, f32);

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
pub struct Model(
    pub RLModel,
    pub HashMap<usize, ffi::Matrix>,
    pub usize,
    pub Vec<TransformBatch>,
);

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
    fn new(lua: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(ScriptData::get_path(lua, &path)?)
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadModel(name.as_ptr());

            if ffi::IsModelValid(data) {
                Ok(Self(
                    RLModel::from_raw(data),
                    HashMap::new(),
                    0,
                    Vec::with_capacity(2048),
                ))
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
            "name": "model:insert_transform_list",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("insert_transform_list", |lua, this, point: LuaValue| {
            let point: Vector3 = lua.from_value(point)?;
            this.1
                .insert(this.2, Matrix::translate(point.x, point.y, point.z).into());
            this.2 += 1;
            Ok(this.2 - 1)
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "model:remove_transform_list",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("remove_transform_list", |_, this, index: usize| {
            this.1.remove(&index);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "model:clear_transform_list",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("clear_transform_list", |_, this, _: ()| {
            this.1.clear();
            this.2 = 0;
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "model:set_transform_list",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "set_transform_list",
            |lua, this, (index, point): (usize, LuaValue)| {
                let point: Vector3 = lua.from_value(point)?;
                if let Some(instance) = this.1.get_mut(&index) {
                    *instance = Matrix::translate(point.x, point.y, point.z).into();
                } else {
                    return Err(mlua::Error::runtime(
                        "set_transform_list(): Invalid instance index.",
                    ));
                }
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "model:set_transform_list_batch",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("set_transform_list_batch", |lua, this, batch: LuaValue| {
            this.3 = lua.from_value(batch)?;

            for b in &this.3 {
                if let Some(instance) = this.1.get_mut(&b.0) {
                    *instance = Matrix::translate(b.1, b.2, b.3).into();
                } else {
                    return Err(mlua::Error::runtime(
                        "set_transform_list_batch(): Invalid instance index.",
                    ));
                }
            }

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "model:bind",
            "info": "Bind a texture to the model.",
            "member": [
                { "name": "index",   "info": "", "kind": "number" },
                { "name": "which",   "info": "", "kind": "number" },
                { "name": "texture", "info": "Texture to bind to model.", "kind": "texture" }
            ]
        }
        */
        method.add_method_mut(
            "bind",
            |_, this, (index, which, texture): (usize, usize, LuaAnyUserData)| {
                if texture.is::<crate::system::texture::Texture>() {
                    let texture = texture.borrow::<crate::system::texture::Texture>().unwrap();
                    let texture = &*texture;

                    this.0.materials_mut()[index].maps_mut()[which].texture = texture.0;
                }

                Ok(())
            },
        );

        method.add_method_mut(
            "bind_shader",
            |_, this, (index, shader): (usize, LuaAnyUserData)| {
                if shader.is::<crate::system::shader::Shader>() {
                    let shader = shader.borrow::<crate::system::shader::Shader>().unwrap();
                    let shader = &*shader;

                    this.0.materials_mut()[index].shader = *shader.0;
                }

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "model:draw_mesh",
            "info": "TO-DO"
        }
        */
        method.add_method(
            "draw_mesh",
            |lua, this, (index, point, angle, scale): (usize, LuaValue, LuaValue, LuaValue)| unsafe {
                let mesh = &this.0.meshes()[index];
                let point: Vector3 = lua.from_value(point)?;
                let angle: Vector3 = lua.from_value(angle)?;
                let scale: Vector3 = lua.from_value(scale)?;
                let angle = Vector3::new(
                    angle.x * DEG2RAD as f32,
                    angle.y * DEG2RAD as f32,
                    angle.z * DEG2RAD as f32,
                );

                let transform =
                    (Matrix::translate(point.x, point.y, point.z) * Matrix::rotate_xyz(angle) * Matrix::scale(scale.x, scale.y, scale.z)).into();

                ffi::DrawMesh(**mesh, *this.0.materials()[0], transform);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "model:draw_mesh_instance",
            "info": "TO-DO"
        }
        */
        method.add_method("draw_mesh_instance", |_, this, index: usize| unsafe {
            let mesh = &this.0.meshes()[index];
            let transform = &this.1;
            let transform: Vec<ffi::Matrix> = transform.values().cloned().collect();

            ffi::DrawMeshInstanced(
                **mesh,
                *this.0.materials()[0],
                transform.as_ptr(),
                transform.len().try_into().unwrap(),
            );
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
            "name": "model:draw_wire",
            "info": "Draw the model (wire-frame).",
            "member": [
                { "name": "point", "info": "", "kind": "vector_3" },
                { "name": "scale", "info": "", "kind": "number"   },
                { "name": "color", "info": "", "kind": "color"    }
            ]
        }
        */
        method.add_method(
            "draw_wire",
            |lua, this, (point, scale, color): (LuaValue, f32, LuaValue)| unsafe {
                let point: Vector3 = lua.from_value(point)?;
                let color: Color = lua.from_value(color)?;

                ffi::DrawModelWires(*this.0, point.into(), scale, color.into());
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
                { "name": "angle", "info": "", "kind": "vector_3" },
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

                this.0.transform = ((Matrix::scale(scale.x, scale.y, scale.z) * angle.to_matrix()) * Matrix::translate(point.x, point.y, point.z)).into();

                ffi::DrawModel(*this.0, Vector3::zero().into(), 1.0, color.into());

                this.0.transform = Matrix::identity().into();

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "model:get_box_3",
            "info": "",
            "result": [
                { "name": "min_x", "info": "Minimum vector. (X)", "kind": "number" },
                { "name": "min_y", "info": "Minimum vector. (Y)", "kind": "number" },
                { "name": "min_z", "info": "Minimum vector. (Z)", "kind": "number" },
                { "name": "max_x", "info": "Maximum vector. (X)", "kind": "number" },
                { "name": "max_y", "info": "Maximum vector. (Y)", "kind": "number" },
                { "name": "max_z", "info": "Maximum vector. (Z)", "kind": "number" }
            ]
        }
        */
        method.add_method("get_box_3", |_, this, _: ()| unsafe {
            let value = ffi::GetModelBoundingBox(*this.0);
            Ok((
                value.min.x,
                value.min.y,
                value.min.z,
                value.max.x,
                value.max.y,
                value.max.z,
            ))
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

        /* entry
        {
            "version": "1.0.0",
            "name": "model:mesh_index",
            "info": "Get the index data of a specific mesh in the model.",
            "member": [
                { "name": "index", "info": "Index of mesh.", "kind": "number" }
            ],
            "result": [
                { "name": "table", "info": "Number table.", "kind": "table" }
            ]
        }
        */
        method.add_method("mesh_index", |lua, this, index: usize| {
            // bug with raylib-rs.
            //let mesh = &this.0.meshes()[index];
            //lua.to_value(mesh.indicies())

            let mesh = &this.0.meshes()[index];
            unsafe {
                let work = std::slice::from_raw_parts(
                    mesh.as_ref().indices as *const u16,
                    (mesh.as_ref().triangleCount * 3) as usize,
                );

                lua.to_value(&work)
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "model:mesh_triangle_count",
            "info": "Get the triangle count of a specific mesh in the model.",
            "member": [
                { "name": "index", "info": "Index of mesh.", "kind": "number" }
            ],
            "result": [
                { "name": "count", "info": "Triangle count.", "kind": "number" }
            ]
        }
        */
        method.add_method("mesh_triangle_count", |_, this, index: usize| {
            let mesh = &this.0.meshes()[index];
            Ok(mesh.triangleCount)
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
pub struct ModelAnimation(pub RLModelAnimation);

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
    fn new(lua: &Lua, path: String) -> mlua::Result<Vec<Self>> {
        let name = CString::new(ScriptData::get_path(lua, &path)?)
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let mut count = 0;
            let data = ffi::LoadModelAnimations(name.as_ptr(), &mut count);
            let mut list: Vec<Self> = Vec::new();

            if count == 0 {
                return Err(mlua::Error::RuntimeError(format!(
                    "ModelAnimation::new(): Could not load file \"{path}\"."
                )));
            }

            for x in 0..count {
                let animation = data.wrapping_add(x.try_into().unwrap());

                list.push(Self(RLModelAnimation::from_raw(*animation)));
            }

            Ok(list)
        }
    }
}

impl mlua::UserData for ModelAnimation {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("bone_count", |_, this| Ok(this.0.boneCount));
        field.add_field_method_get("frame_count", |_, this| Ok(this.0.frameCount));
        field.add_field_method_get("name", |_, this| unsafe {
            let name = this.0.name.as_ptr();
            Ok(CStr::from_ptr(name)
                .to_str()
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .to_string())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "model_animation:get_bone_info",
            "info": ""
        }
        */
        method.add_method("get_bone_info", |_, this, index: usize| {
            let bone = &this.0.bones()[index];
            unsafe {
                let name = CStr::from_ptr(bone.name.as_ptr())
                    .to_str()
                    .map_err(|e| mlua::Error::runtime(e.to_string()))?
                    .to_string();
                Ok((name, bone.parent))
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "model_animation:get_bone_info",
            "info": ""
        }
        */
        method.add_method(
            "get_bone_transform",
            |_, this, (frame, index): (usize, usize)| {
                let transform = this.0.frame_poses()[frame][index];
                Ok((
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "model_animation:update",
            "info": "Update model with new model animation data.",
            "member": [
                { "name": "model", "info": "", "kind": "model"  },
                { "name": "frame", "info": "", "kind": "number" }
            ]
        }
        */
        method.add_method(
            "update",
            |_, this, (model, frame): (LuaAnyUserData, usize)| {
                if model.is::<Model>() {
                    let model = model.borrow::<Model>().unwrap();

                    unsafe {
                        ffi::UpdateModelAnimation(*model.0, *this.0, frame.try_into().unwrap());
                    }
                } else {
                    panic!("not model");
                }

                Ok(())
            },
        );
    }
}
