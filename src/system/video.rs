use crate::module::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

type RLModel = raylib::core::models::Model;
type RLModelAnimation = raylib::core::models::ModelAnimation;
type RLTexture = raylib::core::texture::Texture2D;
type RLRenderTexture = raylib::core::texture::RenderTexture2D;
type RLImage = raylib::core::texture::Image;
type RLFont = raylib::core::text::Font;
type RLShader = raylib::core::shaders::Shader;

//================================================================

/* meta
---@class model
local model = {}
*/
pub struct Model(RLModel);

impl mlua::UserData for Model {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method(
            "draw",
            |lua: &Lua, this, (point, scale, color): (LuaValue, f32, LuaValue)| unsafe {
                let point: crate::system::general::Vector3 = lua.from_value(point)?;
                let color: crate::system::general::Color = lua.from_value(color)?;

                ffi::DrawModel(*this.0, point.into(), scale, color.into());
                Ok(())
            },
        );

        method.add_method_mut(
            "draw_transform",
            |lua: &Lua,
             this,
             (point, angle, scale, color): (LuaValue, LuaValue, LuaValue, LuaValue)| {
                let point: crate::system::general::Vector3 = lua.from_value(point)?;
                let angle: crate::system::general::Vector3 = lua.from_value(angle)?;
                let scale: crate::system::general::Vector3 = lua.from_value(scale)?;
                let color: crate::system::general::Color = lua.from_value(color)?;

                this.0.transform = (Matrix::rotate_xyz(raylib::prelude::Vector3::new(
                    angle.x * DEG2RAD as f32,
                    angle.y * DEG2RAD as f32,
                    angle.z * DEG2RAD as f32,
                )) * Matrix::scale(scale.x, scale.y, scale.z))
                .into();

                unsafe {
                    ffi::DrawModel(*this.0, point.into(), 1.0, color.into());
                    Ok(())
                }
            },
        );

        method.add_method_mut("bind_texture", |_: &Lua, this, texture: LuaAnyUserData| {
            if let Ok(texture) = texture.borrow::<Texture>() {
                this.0.materials_mut()[0]
                    .set_material_texture(MaterialMapIndex::MATERIAL_MAP_ALBEDO, &texture.0);
                Ok(())
            } else {
                Err(mlua::Error::runtime(
                    "Model::bind_texture(): Error binding texture.",
                ))
            }
        });
    }
}

impl Model {
    /* meta
    ---An unique handle for a model in memory.
    ---@param path string Path to file.
    ---@return model # The user-data object.
    function Model(path) end
    */
    pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadModel(name.as_ptr());

            if ffi::IsModelReady(data) {
                Ok(Self(RLModel::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Model::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

//================================================================

pub struct ModelAnimation(Vec<RLModelAnimation>);

impl mlua::UserData for ModelAnimation {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method_mut(
                "update_model",
                |_: &Lua, this, (model, index, frame): (LuaAnyUserData, usize, i32)| {
                    if let Ok(model) = model.borrow::<Model>() {
                        if let Some(animation) = this.0.get(index) {
                            if model.0.is_model_animation_valid(animation) {
                                unsafe {
                                    ffi::UpdateModelAnimation(*model.0, **animation, frame);
                                    Ok(())
                                }
                            } else {
                                Err(mlua::Error::runtime(
                                    "ModelAnimation::update_model(): Model animation is invalid for model.",
                                ))
                            }
                        } else {
                            Err(mlua::Error::runtime(
                                "ModelAnimation::update_model(): Frame index is invalid.",
                            ))
                        }
                    } else {
                        Err(mlua::Error::runtime(
                            "ModelAnimation::update_model(): Error updating model animation.",
                        ))
                    }
                },
            );
    }
}

impl ModelAnimation {
    pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let mut count = 0;
            let data = ffi::LoadModelAnimations(name.as_ptr(), &mut count);

            if count <= 0 {
                return Err(mlua::Error::RuntimeError(format!(
                    "ModelAnimation::new(): No animation data found in \"{path}\"."
                )));
            }

            let mut result: Vec<RLModelAnimation> = Vec::new();

            for x in 0..count {
                result.push(RLModelAnimation::from_raw(*data.offset(x as isize)));
            }

            Ok(Self(result))
        }
    }
}

//================================================================

fn texture_draw(
    lua: &Lua,
    (texture, point, angle, scale, color): (&ffi::Texture, LuaValue, f32, f32, LuaValue),
) -> mlua::Result<()> {
    let point: crate::system::general::Vector2 = lua.from_value(point)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawTextureEx(*texture, point.into(), angle, scale, color.into());
        Ok(())
    }
}

fn texture_pro_draw(
    lua: &Lua,
    (texture, rec_a, rec_b, point, angle, color): (
        &ffi::Texture,
        LuaValue,
        LuaValue,
        LuaValue,
        f32,
        LuaValue,
    ),
) -> mlua::Result<()> {
    let rec_a: crate::system::general::Box2 = lua.from_value(rec_a)?;
    let rec_b: crate::system::general::Box2 = lua.from_value(rec_b)?;
    let point: crate::system::general::Vector2 = lua.from_value(point)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawTexturePro(
            *texture,
            rec_a.into(),
            rec_b.into(),
            point.into(),
            angle,
            color.into(),
        );
        Ok(())
    }
}

/* meta
---@class texture
---@field size vector_2 # Size of the texture.
local texture = {}
*/
pub struct Texture(RLTexture);

impl mlua::UserData for Texture {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("size", |lua: &Lua, this| {
            Ok(lua.to_value(&crate::system::general::Vector2::new(
                this.0.width as f32,
                this.0.height as f32,
            ))?)
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* meta
        ---Draw the texture.
        ---@param point vector_2 The point of the texture.
        ---@param angle number   The angle of the texture.
        ---@param scale number   The scale of the texture.
        ---@param color color    The color of the texture.
        function texture:draw(point, angle, scale, color) end
        */
        method.add_method(
            "draw",
            |lua: &Lua, this, (point, angle, scale, color): (LuaValue, f32, f32, LuaValue)| {
                Ok(texture_draw(lua, (&this.0, point, angle, scale, color)))
            },
        );

        /* meta
        ---Draw the texture (pro variant).
        ---@param box_a box_2    The source rectangle of the texture.
        ---@param box_b box_2    The target rectangle of the texture.
        ---@param point vector_2 The point of the texture.
        ---@param angle number   The angle of the texture.
        ---@param color color    The color of the texture.
        function texture:draw_pro(box_a, box_b, point, angle, color) end
        */
        method.add_method(
                "draw_pro",
                |lua: &Lua,
                 this,
                 (box_a, box_b, point, angle, color): (
                    LuaValue,
                    LuaValue,
                    LuaValue,
                    f32,
                    LuaValue,
                )| {
                    Ok(texture_pro_draw(
                        lua,
                        (&this.0, box_a, box_b, point, angle, color),
                    ))
                },
            );
    }
}

impl Texture {
    /* meta
    ---An unique handle for a texture in memory.
    ---@param path string Path to file.
    ---@return texture # The user-data object.
    function Texture(path) end
    */
    pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadTexture(name.as_ptr());

            if ffi::IsTextureReady(data) {
                Ok(Self(RLTexture::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Texture::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

//================================================================

/* meta
---@class render_texture
---@field size vector_2 # Size of the texture.
local render_texture = {}
*/
pub struct RenderTexture(RLRenderTexture);

impl mlua::UserData for RenderTexture {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("size", |lua: &Lua, this| {
            Ok(lua.to_value(&crate::system::general::Vector2::new(
                this.0.texture.width as f32,
                this.0.texture.height as f32,
            ))?)
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* meta
        ---Draw the texture.
        ---@param point vector_2 The point of the texture.
        ---@param angle number   The angle of the texture.
        ---@param scale number   The scale of the texture.
        ---@param color color    The color of the texture.
        function render_texture:draw(point, angle, scale, color) end
        */
        method.add_method(
            "draw",
            |lua: &Lua, this, (point, angle, scale, color): (LuaValue, f32, f32, LuaValue)| {
                Ok(texture_draw(
                    lua,
                    (&this.0.texture, point, angle, scale, color),
                ))
            },
        );

        /* meta
        ---Draw the texture (pro variant).
        ---@param box_a box_2    The source rectangle of the texture.
        ---@param box_b box_2    The target rectangle of the texture.
        ---@param point vector_2 The point of the texture.
        ---@param angle number   The angle of the texture.
        ---@param color color    The color of the texture.
        function render_texture:draw_pro(box_a, box_b, point, angle, color) end
        */
        method.add_method(
                "draw_pro",
                |lua: &Lua,
                 this,
                 (box_a, box_b, point, angle, color): (
                    LuaValue,
                    LuaValue,
                    LuaValue,
                    f32,
                    LuaValue,
                )| {
                    Ok(texture_pro_draw(
                        lua,
                        (&this.0.texture, box_a, box_b, point, angle, color),
                    ))
                },
            );
    }
}

impl RenderTexture {
    /* meta
    ---An unique handle for a render texture in memory.
    ---@param path string Path to file.
    ---@return render_texture # The user-data object.
    function RenderTexture(path) end
    */
    pub fn new(lua: &Lua, size: LuaValue) -> mlua::Result<Self> {
        unsafe {
            let size: crate::system::general::Vector2 = lua.from_value(size)?;
            let data = ffi::LoadRenderTexture(size.x as i32, size.y as i32);

            if ffi::IsRenderTextureReady(data) {
                Ok(Self(RLRenderTexture::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(
                    "RenderTexture::new(): Could not load render texture.".to_string(),
                ))
            }
        }
    }
}

//================================================================

/* meta
---@class image
---@field size vector_2 # Size of the texture.
local image = {}
*/
pub struct Image(RLImage);

impl mlua::UserData for Image {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("size", |lua: &Lua, this| {
            Ok(lua.to_value(&crate::system::general::Vector2::new(
                this.0.width as f32,
                this.0.height as f32,
            ))?)
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method_mut(
                "draw",
                |lua: &Lua,
                 this,
                 (image, box_a, box_b, color): (
                    mlua::AnyUserData,
                    LuaValue,
                    LuaValue,
                    LuaValue,
                )| {
                    if image.is::<Image>() {
                        if let Ok(image) = image.borrow::<Image>() {
                            let src: crate::system::general::Box2 = lua.from_value(box_a)?;
                            let dst: crate::system::general::Box2 = lua.from_value(box_b)?;
                            let col: crate::system::general::Color = lua.from_value(color)?;

                            unsafe {
                                ffi::ImageDraw(
                                    &mut *this.0,
                                    *image.0,
                                    src.into(),
                                    dst.into(),
                                    col.into(),
                                );
                            }

                            Ok(())
                        } else {
                            Err(mlua::Error::runtime(
                                "Image::draw(): Error borrowing image.".to_string(),
                            ))
                        }
                    } else {
                        Err(mlua::Error::runtime(
                            "Image::draw(): Incorrect image type.".to_string(),
                        ))
                    }
                },
            );

        method.add_method_mut("texture", |_: &Lua, this, _: ()| unsafe {
            let data = ffi::LoadTextureFromImage(*this.0);
            Ok(Texture(RLTexture::from_raw(data)))
        });
    }
}

impl Image {
    /* meta
    ---An unique handle for an image in memory.
    ---@param path string Path to file.
    ---@return image # The user-data object.
    function Image(path) end
    */
    pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadImage(name.as_ptr());

            if ffi::IsImageReady(data) {
                Ok(Self(RLImage::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Image::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

//================================================================

/* meta
---@class font
local font = {}
*/
pub struct Font(RLFont);

impl mlua::UserData for Font {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* meta
        ---Draw a font.
        ---@param text  string   Text of font to draw.
        ---@param point vector_2 Point of font to draw.
        ---@param scale number   Scale of font to draw.
        ---@param space number   Space of font to draw.
        ---@param color color    Color of font to draw.
        function font:draw(text, point, scale, space, color) end
        */
        method.add_method(
                "draw",
                |lua: &Lua,
                 this,
                 (text, point, scale, space, color): (
                    String,
                    LuaValue,
                    f32,
                    f32,
                    LuaValue,
                )| {
                    let point : crate::system::general::Vector2 = lua.from_value(point)?;
                    let color : crate::system::general::Color   = lua.from_value(color)?;
                    let text = CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))?;

                    unsafe {
                        ffi::DrawTextEx(*this.0, text.as_ptr(), point.into(), scale, space, color.into());
                        Ok(())
                    }
                },
            );
    }
}

impl Font {
    /* meta
    ---An unique handle for a font in memory.
    ---@param path string Path to file.
    ---@return font # The user-data object.
    function Font(path) end
    */
    pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadFont(name.as_ptr());

            if ffi::IsFontReady(data) {
                Ok(Self(RLFont::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Font::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

//================================================================

/* meta
---@class shader
local shader = {}
*/
pub struct Shader(RLShader);

impl mlua::UserData for Shader {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* meta
        ---Initialize the shader draw mode. **MUST** call *close* after shader drawing is done.
        function shader:begin() end
        */
        method.add_method("begin", |_: &Lua, this, _: ()| unsafe {
            ffi::BeginShaderMode(*this.0);
            Ok(())
        });

        /* meta
        ---Finalize the shader draw mode.
        function shader:close() end
        */
        method.add_method("close", |_: &Lua, _, _: ()| unsafe {
            ffi::EndShaderMode();
            Ok(())
        });

        //method.add_method("get_uniform", |_: &Lua, this, _: ()| unsafe { Ok(()) });
        //method.add_method("get_attribute", |_: &Lua, this, _: ()| unsafe { Ok(()) });
        //method.add_method("set_value", |_: &Lua, this, _: ()| unsafe { Ok(()) });
        //method.add_method("set_vector", |_: &Lua, this, _: ()| unsafe { Ok(()) });
        //method.add_method("set_matrix", |_: &Lua, this, _: ()| unsafe { Ok(()) });
        //method.add_method("set_texture", |_: &Lua, this, _: ()| unsafe { Ok(()) });
    }
}

impl Shader {
    /* meta
    ---An unique handle for a shader in memory.
    ---@param v_path? string Path to shader .vs file.
    ---@param f_path? string Path to shader .fs file.
    ---@return shader # The user-data object.
    function Shader(v_path, f_path) end
    */
        #[rustfmt::skip]
    pub fn new(
            _: &Lua,
            (v_path, f_path): (Option<String>, Option<String>),
        ) -> mlua::Result<Self> {
            let v_path = v_path.map(|f| CString::new(f).unwrap());
            let f_path = f_path.map(|f| CString::new(f).unwrap());

            unsafe {
                let shader = match (v_path, f_path) {
                    (Some(vs), Some(fs)) =>
                    ffi::LoadShader(vs.as_ptr(), fs.as_ptr()),
                    (None, Some(fs)) =>
                    ffi::LoadShader(std::ptr::null(), fs.as_ptr()),
                    (Some(vs), None) =>
                    ffi::LoadShader(vs.as_ptr(), std::ptr::null()),
                    (None, None) =>
                    ffi::LoadShader(std::ptr::null(), std::ptr::null()),
                };

                if ffi::IsShaderReady(shader) {
                    Ok(Self(RLShader::from_raw(shader)))
                } else {
                    Err(mlua::Error::RuntimeError("Shader::new(): Could not load file.".to_string()))
                }
            }
        }
}

//================================================================

#[rustfmt::skip]
    pub fn set_global(lua: &Lua, global: &mlua::Table, system : &ModuleSystem) -> mlua::Result<()> {
        if system.model {
            global.set("Model",          lua.create_function(self::Model::new)?)?;
            global.set("ModelAnimation", lua.create_function(self::ModelAnimation::new)?)?;
        }

        if system.texture {
            global.set("Texture",        lua.create_function(self::Texture::new)?)?;
            global.set("RenderTexture",  lua.create_function(self::RenderTexture::new)?)?;
        }

        if system.image {
            global.set("Image",  lua.create_function(self::Image::new)?)?;
        }

        if system.font   { global.set("Font",   lua.create_function(self::Font::new)?)?;   }
        if system.shader { global.set("Shader", lua.create_function(self::Shader::new)?)?; }

        //global.set("set_screen_image", lua.create_function(self::set_screen_image)?)?;
        global.set("set_screen_color", lua.create_function(self::set_screen_color)?)?;

        //global.set("set_window_fullscreen", lua.create_function(self::set_window_fullscreen)?)?;
        //global.set("set_window_borderless", lua.create_function(self::set_window_borderless)?)?;
        //global.set("set_window_minimize", lua.create_function(self::set_window_minimize)?)?;
        //global.set("set_window_maximize", lua.create_function(self::set_window_maximize)?)?;
        //global.set("set_window_focus", lua.create_function(self::set_window_focus)?)?;
        //global.set("set_window_restore", lua.create_function(self::set_window_restore)?)?;
        //global.set("set_window_icon", lua.create_function(self::set_window_icon)?)?;
        //global.set("set_window_name", lua.create_function(self::set_window_name)?)?;
        //global.set("set_window_monitor", lua.create_function(self::set_window_monitor)?)?;
        //global.set("set_window_shape_min", lua.create_function(self::set_window_shape_min)?)?;
        //global.set("set_window_shape_max", lua.create_function(self::set_window_shape_max)?)?;
        //global.set("set_window_point", lua.create_function(self::set_window_point)?)?;
        //global.set("set_window_shape", lua.create_function(self::set_window_shape)?)?;
        //global.set("set_window_alpha", lua.create_function(self::set_window_alpha)?)?;

        //global.set("get_window_fullscreen", lua.create_function(self::get_window_fullscreen)?)?;
        global.set("get_window_minimize", lua.create_function(self::get_window_minimize)?)?;
        global.set("get_window_maximize", lua.create_function(self::get_window_maximize)?)?;
        global.set("get_window_focus", lua.create_function(self::get_window_focus)?)?;
        global.set("get_window_resize", lua.create_function(self::get_window_resize)?)?;
        //global.set("get_window_hidden", lua.create_function(self::get_window_hidden)?)?;
        //global.set("get_window_point", lua.create_function(self::get_window_point)?)?;
        global.set("get_window_shape", lua.create_function(self::get_window_shape)?)?;
        //global.set("get_window_scale", lua.create_function(self::get_window_scale)?)?;

        //global.set("get_render_shape", lua.create_function(self::get_render_shape)?)?;

        //global.set("get_monitor_count", lua.create_function(self::get_monitor_count)?)?;
        //global.set("get_monitor_active", lua.create_function(self::get_monitor_active)?)?;
        //global.set("get_monitor_rate", lua.create_function(self::get_monitor_rate)?)?;
        //global.set("get_monitor_name", lua.create_function(self::get_monitor_name)?)?;
        //global.set("get_monitor_point", lua.create_function(self::get_monitor_point)?)?;
        //global.set("get_monitor_shape", lua.create_function(self::get_monitor_shape)?)?;
        //global.set("get_monitor_shape_physical", lua.create_function(self::get_monitor_shape_physical)?)?;

        global.set("begin_mode_3d", lua.create_function(self::begin_mode_3d)?)?;
        global.set("close_mode_3d", lua.create_function(self::close_mode_3d)?)?;
        global.set("draw_grid", lua.create_function(self::draw_grid)?)?;
        global.set("draw_cube", lua.create_function(self::draw_cube)?)?;
        global.set("draw_ball", lua.create_function(self::draw_ball)?)?;
        global.set("draw_box_3", lua.create_function(self::draw_box_3)?)?;

        global.set("begin_mode_2d", lua.create_function(self::begin_mode_2d)?)?;
        global.set("close_mode_2d", lua.create_function(self::close_mode_2d)?)?;
        global.set("get_screen_to_world", lua.create_function(self::get_screen_to_world)?)?;
        global.set("get_world_to_screen", lua.create_function(self::get_world_to_screen)?)?;
        global.set("draw_box_2", lua.create_function(self::draw_box_2)?)?;
        global.set("draw_text", lua.create_function(self::draw_text)?)?;

        //global.set("begin_blend_mode", lua.create_function(self::begin_blend_mode)?)?;
        //global.set("close_blend_mode", lua.create_function(self::close_blend_mode)?)?;
        //global.set("begin_clip_mode", lua.create_function(self::begin_clip_mode)?)?;
        //global.set("close_clip_mode", lua.create_function(self::close_clip_mode)?)?;

        //global.set("update_camera", lua.create_function(self::update_camera)?)?;
        //global.set("update_camera_pro", lua.create_function(self::update_camera_pro)?)?;

        Ok(())
    }

//================================================================

/* meta
---Clear the screen with a color.
---@param color color Color to clear the screen with.
function set_screen_color(color) end
*/
fn set_screen_color(lua: &Lua, color: LuaValue) -> mlua::Result<()> {
    let color: crate::system::general::Color = lua.from_value(color)?;

    unsafe {
        ffi::ClearBackground(color.into());
        Ok(())
    }
}

/* meta
---Get the size of the screen.
---@return vector_2 # Screen size.
function get_window_shape() end
*/
fn get_window_shape(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    unsafe {
        lua.to_value(&crate::system::general::Vector2::new(
            ffi::GetScreenWidth() as f32,
            ffi::GetScreenHeight() as f32,
        ))
    }
}

/* meta
---Get the state of the window (minimized).
---@return boolean # True if minimized, false otherwise.
function get_window_minimize() end
*/
fn get_window_minimize(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowMinimized()) }
}

/* meta
---Get the state of the window (maximized).
---@return boolean # True if maximized, false otherwise.
function get_window_maximize() end
*/
fn get_window_maximize(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowMaximized()) }
}

/* meta
---Get the state of the window (focused).
---@return boolean # True if focused, false otherwise.
function get_window_focus() end
*/
fn get_window_focus(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowFocused()) }
}

/* meta
---Get the state of the window (resized).
---@return boolean # True if resized, false otherwise.
function get_window_resize() end
*/
fn get_window_resize(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowResized()) }
}

//================================================================

/* meta
---Initialize the 3D draw mode. **MUST** call *close_mode_3d* after 3D drawing is done.
---@param camera camera_3d The 3D camera to use for drawing.
function begin_mode_3d(camera) end
*/
fn begin_mode_3d(lua: &Lua, camera: LuaValue) -> mlua::Result<()> {
    let value: crate::system::general::Camera3D = lua.from_value(camera)?;

    unsafe {
        ffi::BeginMode3D(value.into());
        Ok(())
    }
}

/* meta
---Finalize the 3D draw mode.
function close_mode_3d() end
*/
fn close_mode_3d(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::EndMode3D();
        Ok(())
    }
}

/* meta
---Draw a grid.
---@param slice number The slice count of the grid.
---@param space number The space shift of the grid.
function draw_grid(slice, space) end
*/
fn draw_grid(_: &Lua, (slice, space): (i32, f32)) -> mlua::Result<()> {
    unsafe {
        ffi::DrawGrid(slice, space);
        Ok(())
    }
}

/* meta
---Draw a cube.
---@param point vector_3 The point of the cube.
---@param shape vector_3 The shape of the cube.
---@param color color    The color of the cube.
function draw_cube(point, shape, color) end
*/
fn draw_cube(lua: &Lua, (point, shape, color): (LuaValue, LuaValue, LuaValue)) -> mlua::Result<()> {
    let point: crate::system::general::Vector3 = lua.from_value(point)?;
    let shape: crate::system::general::Vector3 = lua.from_value(shape)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawCubeV(point.into(), shape.into(), color.into());
        Ok(())
    }
}

/* meta
---Draw a ball.
---@param point vector_3 The point of the ball.
---@param shape number   The shape of the ball.
---@param color color    The color of the ball.
function draw_ball(point, shape, color) end
*/
fn draw_ball(lua: &Lua, (point, shape, color): (LuaValue, f32, LuaValue)) -> mlua::Result<()> {
    let point: crate::system::general::Vector3 = lua.from_value(point)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawSphere(point.into(), shape, color.into());
        Ok(())
    }
}

/* meta
---Draw a 3D box.
---@param box_3 box_3 Box to draw.
---@param color color The color of the box to draw.
function draw_ball(point, shape, color) end
*/
fn draw_box_3(lua: &Lua, (box_3, color): (LuaValue, LuaValue)) -> mlua::Result<()> {
    let box_3: crate::system::general::Box3 = lua.from_value(box_3)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawBoundingBox(box_3.into(), color.into());
        Ok(())
    }
}

//================================================================

/* meta
---Initialize the 2D draw mode. **MUST** call *close_mode_2d* after 2D drawing is done.
---@param camera camera_2d The 2D camera to use for drawing.
function begin_mode_2d(camera) end
*/
fn begin_mode_2d(lua: &Lua, camera: LuaValue) -> mlua::Result<()> {
    let value: crate::system::general::Camera2D = lua.from_value(camera)?;

    unsafe {
        ffi::BeginMode2D(value.into());
        Ok(())
    }
}

/* meta
---Finalize the 2D draw mode.
function close_mode_2d() end
*/
fn close_mode_2d(_: &Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::EndMode2D();
        Ok(())
    }
}

/* meta
---Get the world-space point for a screen-space point.
---@param point  vector_2  Screen-space point to convert from.
---@param camera camera_2d Camera to convert from.
---@return vector_2 # World-space point.
function get_screen_to_world(point, camera) end
*/
fn get_screen_to_world(lua: &Lua, (point, camera): (LuaValue, LuaValue)) -> mlua::Result<LuaValue> {
    let point: crate::system::general::Vector2 = lua.from_value(point)?;
    let camera: crate::system::general::Camera2D = lua.from_value(camera)?;

    unsafe {
        let value = ffi::GetScreenToWorld2D(point.into(), camera.into());
        lua.to_value(&crate::system::general::Vector2::new(value.x, value.y))
    }
}

/* meta
---Get the screen-space point for a world-space point.
---@param point  vector_2  World-space point to convert from.
---@param camera camera_2d Camera to convert from.
---@return vector_2 # Screen-space point.
function get_world_to_screen(point, camera) end
*/
fn get_world_to_screen(lua: &Lua, (point, camera): (LuaValue, LuaValue)) -> mlua::Result<LuaValue> {
    let point: crate::system::general::Vector2 = lua.from_value(point)?;
    let camera: crate::system::general::Camera2D = lua.from_value(camera)?;

    unsafe {
        let value = ffi::GetWorldToScreen2D(point.into(), camera.into());
        lua.to_value(&crate::system::general::Vector2::new(value.x, value.y))
    }
}

/* meta
---Draw a 2D box.
---@param shape box_2    Box to draw.
---@param point vector_2 The point of the box.
---@param angle number   The angle of the box.
---@param color color    The color of the box.
function draw_box_2(shape, point, angle, color) end
*/
fn draw_box_2(
    lua: &Lua,
    (shape, point, angle, color): (LuaValue, LuaValue, f32, LuaValue),
) -> mlua::Result<()> {
    let shape: crate::system::general::Box2 = lua.from_value(shape)?;
    let point: crate::system::general::Vector2 = lua.from_value(point)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawRectanglePro(shape.into(), point.into(), angle, color.into());
        Ok(())
    }
}

/* meta
---Draw text.
---@param text  string   Text to draw.
---@param point vector_2 Point for the text.
---@param scale number   Scale for the text.
---@param color color    Color for the text.
function draw_text(text, point, scale, color) end
*/
fn draw_text(
    lua: &Lua,
    (text, point, scale, color): (String, LuaValue, i32, LuaValue),
) -> mlua::Result<()> {
    let text = CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))?;
    let point: crate::system::general::Vector2 = lua.from_value(point)?;
    let color: crate::system::general::Color = lua.from_value(color)?;

    unsafe {
        ffi::DrawText(
            text.as_ptr(),
            point.x as i32,
            point.y as i32,
            scale,
            color.into(),
        );
        Ok(())
    }
}
