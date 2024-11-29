use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

/* class
{ "name": "quiver.model", "info": "The model API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let model = lua.create_table()?;

    model.set("new", lua.create_function(self::Model::new)?)?;

    table.set("model", model)?;

    Ok(())
}

type RLModel = ffi::Model;

/* class
{ "name": "model", "info": "An unique handle for a model in memory." }
*/
pub struct Model(pub RLModel);

impl Model {
    /* entry
    {
        "name": "quiver.model.new",
        "info": "Create a new Model resource.",
        "member": [
            { "name": "path", "info": "Path to model file.", "kind": "string" }
        ],
        "result": [
            { "name": "Model", "info": "Model resource.", "kind": "Model" }
        ]
    }
    */
    fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadModel(name.as_ptr());

            if ffi::IsModelReady(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Model::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadModel(self.0);
        }
    }
}

impl mlua::UserData for Model {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        { "name": "model:draw_wire", "info": "Draw the model (wire-frame render)." }
        */
        method.add_method("draw_wire", |_, this, ()| unsafe {
            ffi::DrawModelWires(this.0, Vector3::zero().into(), 1.0, Color::RED.into());
            Ok(())
        });
    }
}
