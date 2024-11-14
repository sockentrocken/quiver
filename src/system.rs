/*
* ================================================================
* system.rs
* ================================================================
*/

use crate::script::*;
use crate::status::*;
use crate::window::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::ffi::CString;

//================================================================

/* meta
---@class (exact) vector
---@field x number
---@field y number
---@field z number
---@field w number
---@field X number
---@field Y number
---@field Z number
---@field W number
---@field zero vector Constant vector with every component set to 0.
---@field one  vector Constant vector with every component set to 1.
---@field create    function
---@field magnitude function
---@field normalize function
---@field cross     function
---@field dot       function
---@field angle     function
---@field floor     function
---@field ceil      function
---@field abs       function
---@field sign      function
---@field clamp     function
---@field min       function
---@field max       function
vector = {}

---Create a new vector.
---@param x number The X component of the vector.
---@param y number The Y component of the vector.
---@param z number The Z component of the vector.
---@param w number The W component of the vector.
---@return vector # The new vector.
function vector.create(x, y, z, w) end

---Calculate the magnitude of a vector.
---@param vector vector The vector to calculate the magnitude of.
---@return number # The magnitude of the vector.
function vector.magnitude(vector) end

---Normalize a vector.
---@param vector vector The vector to normalize.
---@return vector # The unit vector version of the vector.
function vector.normalize(vector) end

---Calculate the cross product between *vector_1* and *vector_2*.
---@param vector_1 vector Vector 1.
---@param vector_2 vector Vector 2.
---@return vector # The cross product.
function vector.cross(vector_1, vector_2) end

---Calculate the dot product between *vector_1* and *vector_2*.
---@param vector_1 vector Vector 1.
---@param vector_2 vector Vector 2.
---@return number # The dot product.
function vector.dot(vector_1, vector_2) end

---Calculate the angle between *vector_1* and *vector_2*.
---@param vector_1 vector Vector 1.
---@param vector_2 vector Vector 2.
---@param axis? vector Axis vector. If set, will determine the sign of the angle.
---@return number # The radian angle.
function vector.angle(vector_1, vector_2, axis) end

---Round a vector down.
---@param vector vector The vector to round down.
---@return vector # The vector, with *floor* set on every component.
function vector.floor(vector) end

---Round a vector up.
---@param vector vector The vector to round up.
---@return vector # The vector, with *ceil* set on every component.
function vector.ceil(vector) end

---Calculate the absolute value of a vector.
---@param vector vector The vector to calculate the absolute value of.
---@return vector # The vector, with *abs* set on every component.
function vector.abs(vector) end

--- Calculate the sign of every component of a vector.
---@param vector vector The vector to calculate the sign of.
---@return vector # The vector, with *sign* set on every component.
function vector.sign(vector) end

---Apply a minimum/maximum value to every component of a vector.
---@param vector vector The vector to clamp to a minimum/maximum value of.
---@param min vector Minimum value vector.
---@param max vector Maximum value vector.
---@return vector # The vector, with *clamp* set on every component.
function vector.clamp(vector, min, max) end

---Apply a maximum value to every component of a vector.
---@param vector vector The vector to clamp to a maximum value of.
---@param max vector Maximum value vector.
---@return vector # The vector, with *max* set on every component.
function vector.max(vector, max) end

---Apply a minimum value to every component of a vector.
---@param vector vector The vector to clamp to a maximum value of.
---@param min vector Minimum value vector.
---@return vector # The vector, with *min* set on every component.
function vector.min(vector, min) end
*/

pub mod general {
    use super::*;
    use std::sync::Mutex;

    pub fn ffi_vect2(value: mlua::Vector) -> ffi::Vector2 {
        ffi::Vector2 {
            x: value.x(),
            y: value.y(),
        }
    }

    pub fn ffi_recta(min: mlua::Vector, max: mlua::Vector) -> ffi::Rectangle {
        ffi::Rectangle {
            x: min.x(),
            y: min.y(),
            width: max.x(),
            height: max.y(),
        }
    }

    pub fn ffi_rectn(value: mlua::Vector) -> ffi::Rectangle {
        ffi::Rectangle {
            x: value.x(),
            y: value.y(),
            width: value.z(),
            height: value.w(),
        }
    }

    pub fn ffi_vect3(value: mlua::Vector) -> ffi::Vector3 {
        ffi::Vector3 {
            x: value.x(),
            y: value.y(),
            z: value.z(),
        }
    }

    pub fn ffi_bound(min: mlua::Vector, max: mlua::Vector) -> ffi::BoundingBox {
        ffi::BoundingBox {
            min: ffi_vect3(min),
            max: ffi_vect3(max),
        }
    }

    pub fn ffi_color(value: mlua::Vector) -> ffi::Color {
        ffi::Color {
            r: (value.x() * 255.0) as u8,
            g: (value.y() * 255.0) as u8,
            b: (value.z() * 255.0) as u8,
            a: (value.w() * 255.0) as u8,
        }
    }

    pub fn ffi_cam3d(
        point: mlua::Vector,
        focus: mlua::Vector,
        up: mlua::Vector,
        zoom: f32,
    ) -> ffi::Camera3D {
        ffi::Camera3D {
            position: ffi_vect3(point),
            target: ffi_vect3(focus),
            up: ffi_vect3(up),
            fovy: zoom,
            projection: CameraProjection::CAMERA_PERSPECTIVE as i32,
        }
    }

    pub fn ffi_cam2d(
        point: mlua::Vector,
        focus: mlua::Vector,
        angle: f32,
        zoom: f32,
    ) -> ffi::Camera2D {
        ffi::Camera2D {
            target: ffi_vect2(point),
            offset: ffi_vect2(focus),
            rotation: angle,
            zoom,
        }
    }

    //================================================================

    /* meta
    ---@class (exact) file
    local file = {}

    ---An unique file handle for a file in memory.
    ---@param path string Path to file.
    ---@return file # The user-data object.
    function File(path) end
    */
    use std::{
        io::{BufRead, BufReader, BufWriter, Read, Write},
        sync::Arc,
    };

    pub struct File {
        pub reader: BufReader<std::fs::File>,
        pub writer: BufWriter<std::fs::File>,
    }

    impl mlua::UserData for File {
        fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

        fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
            method.add_method_mut("get_all_string", |_: &Lua, this, _: ()| {
                let mut data = String::new();
                this.reader.read_to_string(&mut data)?;

                Ok(data)
            });

            method.add_method_mut("get_string", |_: &Lua, this, _: ()| {
                let mut data = String::new();
                this.reader.read_line(&mut data)?;

                Ok(data)
            });

            method.add_method_mut("set_string", |_: &Lua, this, data: String| {
                this.writer.write_all(data.as_bytes())?;
                this.writer.flush()?;

                Ok(())
            });
        }
    }

    impl File {
        pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
            if !std::path::Path::new(&path).exists() {
                std::fs::File::create(&path)?;
            }

            let file = std::fs::File::options().read(true).write(true).open(path)?;

            Ok(Self {
                reader: BufReader::new(file.try_clone().unwrap()),
                writer: BufWriter::new(file.try_clone().unwrap()),
            })
        }
    }

    //================================================================

    use notify::{Config, PollWatcher, RecursiveMode, Watcher};
    use std::path::Path;

    pub struct FileWatcher(PollWatcher, Arc<Mutex<bool>>);

    impl mlua::UserData for FileWatcher {
        fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

        fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
            method.add_method("watch", |_: &Lua, this, _: ()| {
                this.0.poll().unwrap();
                let mut sent = this.1.lock().unwrap();

                // event happened, send true.
                if *sent {
                    *sent = false;
                    Ok(true)
                }
                // event did not happen, send false.
                else {
                    Ok(false)
                }
            });
        }
    }

    impl FileWatcher {
        pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher = PollWatcher::new(tx, Config::default().with_manual_polling())
                .map_err(|e| mlua::Error::runtime(e.to_string()))?;

            // Add a path to be watched. All files and directories at that path and
            // below will be monitored for changes.
            watcher
                .watch(Path::new(&path), RecursiveMode::Recursive)
                .map_err(|e| mlua::Error::runtime(e.to_string()))?;

            let sentinel = Arc::new(Mutex::new(false));
            let clone = sentinel.clone();

            // run event receiver on a different thread, we want this one for user input
            std::thread::spawn(move || {
                for res in rx {
                    match res {
                        Ok(_) => {
                            let mut sent = clone.lock().unwrap();
                            *sent = true;
                        }
                        Err(e) => println!("watch error: {:?}", e),
                    }
                }
            });

            Ok(Self(watcher, sentinel.clone()))
        }
    }

    //================================================================

    #[rustfmt::skip]
    pub fn set_global(lua: &Lua, global: &mlua::Table, status: StatusPointer, window : WindowPointer) -> mlua::Result<()> {
        /* meta
        ---Load the engine.
        function engine_load() end
        */
        let clone = status.clone();
        global.set("engine_load", lua.create_function(move |_, _ : ()| {
            *clone.borrow_mut() = Status::Restart;
            Ok(())
        })?)?;

        /* meta
        ---Exit the engine.
        function engine_exit() end
        */
        let clone = status.clone();
        global.set("engine_exit", lua.create_function(move |_, _ : ()| {
            *clone.borrow_mut() = Status::Closure;
            Ok(())
        })?)?;

        /* meta
       ---Get the current state of the debug window.
        function get_debug() end
        */
        let clone = window.clone();
        global.set("get_debug", lua.create_function(move |_, _ : ()| {
            Ok(clone.borrow().active)
        })?)?;

        /* meta
        ---Set the current state of the debug window.
        ---@param value boolean New state.
        function set_debug(value) end
        */
        let clone = window.clone();
        global.set("set_debug", lua.create_function(move |_, value : bool| {
            let mut clone = clone.borrow_mut();
            clone.active = value;
            clone.parser.dirty = value;

            Ok(())
        })?)?;

        /* meta
        ---Get the current state of the debug logger.
        function get_logger() end
        */
        let clone = window.clone();
        global.set("get_logger", lua.create_function(move |_, _ : ()| {
            Ok(clone.borrow_mut().logger.active)
        })?)?;

        /* meta
        ---Set the current state of the debug logger.
        ---@param value boolean New state.
        function set_logger(value) end
        */
        let clone = window.clone();
        global.set("set_logger", lua.create_function(move |_, value : bool| {
            clone.borrow_mut().logger.active = value;
            Ok(())
        })?)?;

        /* meta
        ---Wipe the debug logger text.
        function wipe_logger() end
        */
        let clone = window.clone();
        global.set("wipe_logger", lua.create_function(move |_, _ : ()| {
            Logger::wipe(&mut clone.borrow_mut().logger);
            Ok(())
        })?)?;

        /* meta
        ---Show the debug logger text.
        ---@param value boolean New state.
        function show_logger(value) end
        */
        let clone = window.clone();
        global.set("show_logger", lua.create_function(move |_, value: bool| {
            Logger::show(&mut clone.borrow_mut().logger, value);
            Ok(())
        })?)?;

        /* meta
        ---Push a new string to the debug logger.
        ---@param label string Label for line to print.
        ---@param color? vector Color for line to print.
        function push_logger(label, color) end
        */
        let clone = window.clone();
        global.set("push_logger", lua.create_function(move |_, (label, color) : (String, Option<[f32; 4]>)| {
            println!("{label}");
            clone.borrow_mut().logger.push(LogLine::new(label, color.unwrap_or([1.0, 1.0, 1.0, 1.0])));
            Ok(())
        })?)?;

        /* meta
        ---Push a new method to the debug parser.
        ---@param name string Name for method to push.
        ---@param info string Info for method to push.
        ---@param call function Function call-back for method to push.
        function push_parser(name, info, call) end
        */
        let clone = window.clone();
        global.set("push_parser", lua.create_function(move |_, (name, info, call) : (String, String, mlua::Function)| {
            clone.borrow_mut().parser.method.insert(name, ParserMethod { call, info });
            Ok(())
        })?)?;

        global.set("File",           lua.create_function(self::File::new)?)?;
        global.set("FileWatcher",    lua.create_function(self::FileWatcher::new)?)?;
        //global.set("Automation",    lua.create_function(self::Automation::new)?)?;

        global.set("get_file_exist",   lua.create_function(self::get_file_exist)?)?;
        global.set("set_exit_key",   lua.create_function(self::set_exit_key)?)?;
        global.set("get_time",       lua.create_function(self::get_time)?)?;
        global.set("get_frame_time", lua.create_function(self::get_frame_time)?)?;
        global.set("get_frame_rate", lua.create_function(self::get_frame_rate)?)?;
        global.set("set_frame_rate", lua.create_function(self::set_frame_rate)?)?;
        global.set("json_to_table",  lua.create_function(self::json_to_table)?)?;
        global.set("table_to_json",  lua.create_function(self::table_to_json)?)?;
        //global.set("open_link",  lua.create_function(self::open_link)?)?;
        //global.set("compress_data",  lua.create_function(self::compress_data)?)?;
        //global.set("decompress_data",  lua.create_function(self::decompress_data)?)?;
        //global.set("encode_data_base64",  lua.create_function(self::encode_data_base64)?)?;
        //global.set("decode_data_base64",  lua.create_function(self::decode_data_base64)?)?;

        Ok(())
    }

    //================================================================

    /* meta
    ---Set a key to exit Quiver.
    ---@param key input_board Key to exit Quiver with.
    function set_exit_key(key) end
    */
    fn set_exit_key(_: &Lua, value: i32) -> mlua::Result<()> {
        if (input::BOARD_RANGE_LOWER..=input::BOARD_RANGE_UPPER).contains(&value) {
            unsafe {
                ffi::SetExitKey(value);
                Ok(())
            }
        } else {
            Err(mlua::Error::runtime("set_exit_key(): Unknown key value."))
        }
    }

    /* meta
    ---Get the current time. Will count up since the initialization of the window.
    ---@return number # Current time.
    function get_time() end
    */
    fn get_time(_: &Lua, _: ()) -> mlua::Result<f64> {
        unsafe { Ok(ffi::GetTime()) }
    }

    /* meta
    ---Get the current frame time.
    ---@return number # Current frame time.
    function get_frame_time() end
    */
    fn get_frame_time(_: &Lua, _: ()) -> mlua::Result<f32> {
        unsafe { Ok(ffi::GetFrameTime()) }
    }

    /* meta
    ---Get the current frame rate.
    ---@return number # Current frame rate.
    function get_frame_rate() end
    */
    fn get_frame_rate(_: &Lua, _: ()) -> mlua::Result<i32> {
        unsafe { Ok(ffi::GetFPS()) }
    }

    /* meta
    ---Set the current frame rate.
    ---@param value number Value to set the frame rate to.
    function set_frame_rate(value) end
    */
    fn set_frame_rate(_: &Lua, rate: i32) -> mlua::Result<()> {
        unsafe {
            ffi::SetTargetFPS(rate);
            Ok(())
        }
    }

    /* meta
    ---Convert a table to a JSON string.
    ---@param value table Table to convert to a JSON string.
    ---@return string # JSON conversion of table.
    function table_to_json(value) end
    */
    fn json_to_table(lua: &Lua, data: String) -> mlua::Result<LuaValue> {
        let json: serde_json::Value =
            serde_json::from_str(&data).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        lua.to_value(&json)
    }

    /* meta
    ---Convert a JSON string to a table.
    ---@param value string JSON string to convert to a table.
    ---@return table # Table conversion of a JSON string.
    function json_to_table(value) end
    */
    fn table_to_json(_: &Lua, data: LuaValue) -> mlua::Result<String> {
        let json = serde_json::to_string(&data).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        Ok(json)
    }

    /* meta
    ---Check if a file does exist in disk.
    ---@param path string Path for file to check.
    ---@return boolean # True if file does exist, false otherwise.
    function get_file_exist(path) end
    */
    fn get_file_exist(_: &Lua, path: String) -> mlua::Result<bool> {
        Ok(std::path::Path::new(&path).exists())
    }
}

//================================================================

pub mod video {
    use super::*;

    //================================================================

    type RLModel = raylib::core::models::Model;
    type RLModelAnimation = raylib::core::models::ModelAnimation;
    type RLTexture = raylib::core::texture::Texture2D;
    type RLRenderTexture = raylib::core::texture::RenderTexture2D;
    type RLImage = raylib::core::texture::Image;
    type RLFont = raylib::core::text::Font;
    type RLShader = raylib::core::shaders::Shader;

    //================================================================

    /* meta
    ---@class (exact) model
    local model = {}

    ---An unique handle for a model in memory.
    ---@param path string Path to file.
    ---@return model # The user-data object.
    function Model(path) end
    */
    pub struct Model(RLModel);

    impl mlua::UserData for Model {
        fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

        fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
            method.add_method(
                "draw",
                |_: &Lua, this, (point, scale, color): (mlua::Vector, f32, mlua::Vector)| unsafe {
                    let point = general::ffi_vect3(point);
                    let color = general::ffi_color(color);

                    ffi::DrawModel(*this.0, point, scale, color);
                    Ok(())
                },
            );

            method.add_method_mut(
                "draw_transform",
                |_: &Lua,
                 this,
                 (point, angle, scale, color): (
                    mlua::Vector,
                    mlua::Vector,
                    mlua::Vector,
                    mlua::Vector,
                )| {
                    let point = general::ffi_vect3(point);
                    let angle = general::ffi_vect3(angle);
                    let scale = general::ffi_vect3(scale);
                    let color = general::ffi_color(color);

                    this.0.transform = (Matrix::rotate_xyz(raylib::prelude::Vector3::new(
                        angle.x * DEG2RAD as f32,
                        angle.y * DEG2RAD as f32,
                        angle.z * DEG2RAD as f32,
                    )) * Matrix::scale(scale.x, scale.y, scale.z))
                    .into();

                    unsafe {
                        ffi::DrawModel(*this.0, point, 1.0, color);
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
        pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
            let name =
                CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

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
            let name =
                CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

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
        _: &Lua,
        (texture, point, angle, scale, color): (
            &ffi::Texture,
            mlua::Vector,
            f32,
            f32,
            mlua::Vector,
        ),
    ) -> mlua::Result<()> {
        let point = general::ffi_vect2(point);
        let color = general::ffi_color(color);

        unsafe {
            ffi::DrawTextureEx(*texture, point, angle, scale, color);
            Ok(())
        }
    }

    fn texture_pro_draw(
        _: &Lua,
        (texture, rec_a, rec_b, point, angle, color): (
            &ffi::Texture,
            mlua::Vector,
            mlua::Vector,
            mlua::Vector,
            f32,
            mlua::Vector,
        ),
    ) -> mlua::Result<()> {
        let rec_a = general::ffi_rectn(rec_a);
        let rec_b = general::ffi_rectn(rec_b);
        let point = general::ffi_vect2(point);
        let color = general::ffi_color(color);

        unsafe {
            ffi::DrawTexturePro(*texture, rec_a, rec_b, point, angle, color);
            Ok(())
        }
    }

    /* meta
    ---@class (exact) texture
    local texture = {}

    ---An unique handle for a texture in memory.
    ---@param path string Path to file.
    ---@return texture # The user-data object.
    function Texture(path) end
    */
    pub struct Texture(RLTexture);

    impl mlua::UserData for Texture {
        fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
            field.add_field_method_get("size", |_: &Lua, this| {
                Ok(mlua::Vector::new(
                    this.0.width as f32,
                    this.0.height as f32,
                    0.0,
                    0.0,
                ))
            });
        }

        fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
            /* meta
            ---Draw the texture.
            ---@param point The point of the texture.
            ---@param angle The point of the texture.
            ---@param scale The point of the texture.
            ---@param color The point of the texture.
            function texture:draw(point, angle, scale, color) end
            */
            method.add_method(
                "draw",
                |lua: &Lua,
                 this,
                 (point, angle, scale, color): (
                    mlua::Vector,
                    f32,
                    f32,
                    mlua::Vector,
                )| {
                    Ok(texture_draw(
                        lua,
                        (&this.0, point, angle, scale, color),
                    ))
                },
            );

            /* meta
            ---Draw the texture (pro variant).
            ---@param rec_a The "source" rectangle of the texture.
            ---@param rec_b The "target" rectangle of the texture.
            ---@param angle The point of the texture.
            ---@param scale The point of the texture.
            ---@param color The point of the texture.
            function texture:draw(point, angle, scale, color) end
            */
            method.add_method(
                "draw_pro",
                |lua: &Lua,
                 this,
                 (rec_a, rec_b, point, angle, color): (
                    mlua::Vector,
                    mlua::Vector,
                    mlua::Vector,
                    f32,
                    mlua::Vector,
                )| {
                    Ok(texture_pro_draw(
                        lua,
                        (&this.0, rec_a, rec_b, point, angle, color),
                    ))
                },
            );
        }
    }

    impl Texture {
        pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
            let name =
                CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

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
    ---@class (exact) render_texture
    local render_texture = {}

    ---An unique handle for a render texture in memory.
    ---@param path string Path to file.
    ---@return render_texture # The user-data object.
    function RenderTexture(path) end
    */
    pub struct RenderTexture(RLRenderTexture);

    impl mlua::UserData for RenderTexture {
        fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
            field.add_field_method_get("size", |_: &Lua, this| {
                Ok(mlua::Vector::new(
                    this.0.texture.width as f32,
                    this.0.texture.height as f32,
                    0.0,
                    0.0,
                ))
            });
        }

        fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
            method.add_method(
                "draw",
                |lua: &Lua,
                 this,
                 (point, rotation, scale, color): (
                    mlua::Vector,
                    f32,
                    f32,
                    mlua::Vector,
                )| {
                    Ok(texture_draw(
                        lua,
                        (&this.0.texture, point, rotation, scale, color),
                    ))
                },
            );

            method.add_method(
                "draw_pro",
                |lua: &Lua,
                 this,
                 (rec_a, rec_b, point, rotation, color): (
                    mlua::Vector,
                    mlua::Vector,
                    mlua::Vector,
                    f32,
                    mlua::Vector,
                )| {
                    Ok(texture_pro_draw(
                        lua,
                        (&this.0.texture, rec_a, rec_b, point, rotation, color),
                    ))
                },
            );
        }
    }

    impl RenderTexture {
        pub fn new(_: &Lua, size: mlua::Vector) -> mlua::Result<Self> {
            unsafe {
                let data = ffi::LoadRenderTexture(size.x() as i32, size.y() as i32);

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

    pub struct Image(RLImage);

    impl mlua::UserData for Image {
        fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
            field.add_field_method_get("size", |_: &Lua, this| {
                Ok(mlua::Vector::new(
                    this.0.width as f32,
                    this.0.height as f32,
                    0.0,
                    0.0,
                ))
            });
        }

        fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
            method.add_method_mut(
                "draw",
                |_: &Lua,
                 this,
                 (image, rec_src_min, rec_src_max, rec_dst_min, rec_dst_max, color): (
                    mlua::AnyUserData,
                    mlua::Vector,
                    mlua::Vector,
                    mlua::Vector,
                    mlua::Vector,
                    mlua::Vector,
                )| {
                    if image.is::<Image>() {
                        if let Ok(image) = image.borrow::<Image>() {
                            let src = general::ffi_recta(rec_src_min, rec_src_max);
                            let dst = general::ffi_recta(rec_dst_min, rec_dst_max);
                            let col = general::ffi_color(color);

                            unsafe {
                                ffi::ImageDraw(&mut *this.0, *image.0, src, dst, col);
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

    /* meta
    ---@class (exact) image
    local image = {}

    ---An unique handle for an image in memory.
    ---@param path string Path to file.
    ---@return image # The user-data object.
    function Image(path) end
    */
    impl Image {
        pub fn new_path(_: &Lua, path: String) -> mlua::Result<Self> {
            let name =
                CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

            unsafe {
                let data = ffi::LoadImage(name.as_ptr());

                if ffi::IsImageReady(data) {
                    Ok(Self(RLImage::from_raw(data)))
                } else {
                    Err(mlua::Error::RuntimeError(format!(
                        "Image::new_path(): Could not load file \"{path}\"."
                    )))
                }
            }
        }

        pub fn new_color(
            _: &Lua,
            (shape, color): (mlua::Vector, mlua::Vector),
        ) -> mlua::Result<Self> {
            unsafe {
                let data = ffi::GenImageColor(
                    shape.x() as i32,
                    shape.y() as i32,
                    general::ffi_color(color),
                );

                if ffi::IsImageReady(data) {
                    Ok(Self(RLImage::from_raw(data)))
                } else {
                    Err(mlua::Error::RuntimeError(format!(
                        "Image::new_color(): Could not generate image."
                    )))
                }
            }
        }
    }

    //================================================================

    /* meta
    ---@class (exact) font
    local font = {}

    ---An unique handle for a font in memory.
    ---@param path string Path to file.
    ---@return font # The user-data object.
    function Font(path) end
    */
    pub struct Font(RLFont);

    impl mlua::UserData for Font {
        fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

        fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
            method.add_method(
                "draw",
                |_: &Lua,
                 this,
                 (text, point, scale, space, color): (
                    String,
                    mlua::Vector,
                    f32,
                    f32,
                    mlua::Vector,
                )| {
                    let point = general::ffi_vect2(point);
                    let color = general::ffi_color(color);
                    let text =
                        CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))?;
                    let text = text.as_ptr();

                    unsafe {
                        ffi::DrawTextEx(*this.0, text, point, scale, space, color);
                        Ok(())
                    }
                },
            );
        }
    }

    impl Font {
        pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
            let name =
                CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

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
    ---@class (exact) shader
    local shader = {}

    ---An unique handle for a shader in memory.
    ---@param path string Path to file.
    ---@return shader # The user-data object.
    function Shader(path) end
    */
    pub struct Shader(RLShader);

    impl mlua::UserData for Shader {
        fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

        fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
            method.add_method("begin", |_: &Lua, this, _: ()| unsafe {
                ffi::BeginShaderMode(*this.0);
                Ok(())
            });

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

    use steamworks::AppId;
    use steamworks::Client;
    use steamworks::FriendFlags;
    use steamworks::PersonaStateChange;
    use steamworks::SingleClient;

    /* meta
    ---@class (exact) steam
    local steam = {}

    ---A handle into the Steam API.
    ---@return steam # The user-data object.
    function Steam() end
    */
    pub struct Steam {
        client: Client,
        single: SingleClient,
    }

    impl mlua::UserData for Steam {
        fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

        fn add_methods<M: mlua::UserDataMethods<Self>>(_: &mut M) {}
    }

    impl Steam {
        #[rustfmt::skip]
        pub fn new(
            _: &Lua, _ : ()
        ) -> mlua::Result<Self> {
            let (client, single) = Client::init_app(480).unwrap();

            Ok(Self {
                client,
                single
            })
        }
    }

    //================================================================

    #[rustfmt::skip]
    pub fn set_global(lua: &Lua, global: &mlua::Table, system : &ModuleSystem) -> mlua::Result<()> {
        global.set("Steam", lua.create_function(self::Steam::new)?)?;

        if system.model {
            global.set("Model",          lua.create_function(self::Model::new)?)?;
            global.set("ModelAnimation", lua.create_function(self::ModelAnimation::new)?)?;
        }

        if system.texture {
            global.set("Texture",        lua.create_function(self::Texture::new)?)?;
            global.set("RenderTexture",  lua.create_function(self::RenderTexture::new)?)?;
        }

        if system.image {
            global.set("ImagePath",  lua.create_function(self::Image::new_path)?)?;
            global.set("ImageColor", lua.create_function(self::Image::new_color)?)?;
        }

        if system.font   { global.set("Font",   lua.create_function(self::Font::new)?)?;   }
        if system.shader { global.set("Shader", lua.create_function(self::Shader::new)?)?; }

        //global.set("set_screen_image", lua.create_function(self::set_screen_image)?)?;
        //global.set("set_screen_color", lua.create_function(self::set_screen_color)?)?;

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
        //global.set("get_window_minimize", lua.create_function(self::get_window_minimize)?)?;
        //global.set("get_window_maximize", lua.create_function(self::get_window_maximize)?)?;
        //global.set("get_window_focus", lua.create_function(self::get_window_focus)?)?;
        //global.set("get_window_resize", lua.create_function(self::get_window_resized)?)?;
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
        global.set("draw_bounding_box", lua.create_function(self::draw_bounding_box)?)?;

        global.set("begin_mode_2d", lua.create_function(self::begin_mode_2d)?)?;
        global.set("close_mode_2d", lua.create_function(self::close_mode_2d)?)?;
        global.set("get_screen_to_world", lua.create_function(self::get_screen_to_world)?)?;
        global.set("get_world_to_screen", lua.create_function(self::get_world_to_screen)?)?;
        global.set("draw_rectangle", lua.create_function(self::draw_rectangle)?)?;
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
    ---@param color vector color to clear the screen with.
    function clear_screen(color) end
    */
    fn clear_screen(_: &Lua, color: mlua::Vector) -> mlua::Result<()> {
        let color = general::ffi_color(color);

        unsafe {
            ffi::ClearBackground(color);
            Ok(())
        }
    }

    /* meta
    ---Get the size of the screen.
    ---@return vector # Screen size.
    function get_window_shape() end
    */
    fn get_window_shape(_: &Lua, _: ()) -> mlua::Result<mlua::Vector> {
        unsafe {
            Ok(mlua::Vector::new(
                ffi::GetScreenWidth() as f32,
                ffi::GetScreenHeight() as f32,
                0.0,
                0.0,
            ))
        }
    }

    /* meta
    ---Get the state of the window (minimized).
    ---@return boolean # True if minimized, false otherwise.
    function get_window_minimized() end
    */
    fn get_window_minimized(_: &Lua, _: ()) -> mlua::Result<bool> {
        unsafe { Ok(ffi::IsWindowMinimized()) }
    }

    /* meta
    ---Get the state of the window (maximized).
    ---@return boolean # True if maximized, false otherwise.
    function get_window_maximized() end
    */
    fn get_window_maximized(_: &Lua, _: ()) -> mlua::Result<bool> {
        unsafe { Ok(ffi::IsWindowMaximized()) }
    }

    /* meta
    ---Get the state of the window (focused).
    ---@return boolean # True if focused, false otherwise.
    function get_window_focused() end
    */
    fn get_window_focused(_: &Lua, _: ()) -> mlua::Result<bool> {
        unsafe { Ok(ffi::IsWindowFocused()) }
    }

    /* meta
    ---Get the state of the window (resized).
    ---@return boolean # True if resized, false otherwise.
    function get_window_resized() end
    */
    fn get_window_resized(_: &Lua, _: ()) -> mlua::Result<bool> {
        unsafe { Ok(ffi::IsWindowResized()) }
    }

    //================================================================

    /* meta
    ---Initialize the 3D draw mode. **MUST** call *close_mode_3d* after 3D drawing is done.
    ---@param point vector The point of the camera.
    ---@param focus vector The focus of the camera.
    ---@param up vector The direction pointing "up" of the camera.
    ---@param zoom number The zoom of the camera.
    function begin_mode_3d(point, focus, up, zoom) end
    */
    fn begin_mode_3d(
        _: &Lua,
        (point, focus, up, zoom): (mlua::Vector, mlua::Vector, mlua::Vector, f32),
    ) -> mlua::Result<()> {
        let value = general::ffi_cam3d(point, focus, up, zoom);

        unsafe {
            ffi::BeginMode3D(value);
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
    ---*3D mode operation.* Draw a grid.
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
    ---*3D mode operation.* Draw a cube.
    ---@param point vector The point of the cube.
    ---@param shape vector The shape of the cube.
    ---@param color vector The color of the cube.
    function draw_cube(point, shape, color) end
    */
    fn draw_cube(
        _: &Lua,
        (point, shape, color): (mlua::Vector, mlua::Vector, mlua::Vector),
    ) -> mlua::Result<()> {
        let point = general::ffi_vect3(point);
        let shape = general::ffi_vect3(shape);
        let color = general::ffi_color(color);

        unsafe {
            ffi::DrawCubeV(point, shape, color);
            Ok(())
        }
    }

    /* meta
    --- foo
    */
    fn draw_ball(
        _: &Lua,
        (point, shape, color): (mlua::Vector, f32, mlua::Vector),
    ) -> mlua::Result<()> {
        let point = general::ffi_vect3(point);
        let color = general::ffi_color(color);

        unsafe {
            ffi::DrawSphere(point, shape, color);
            Ok(())
        }
    }

    /* meta
    --- foo
    */
    fn draw_bounding_box(
        _: &Lua,
        (min, max, color): (mlua::Vector, mlua::Vector, mlua::Vector),
    ) -> mlua::Result<()> {
        let bound = general::ffi_bound(min, max);
        let color = general::ffi_color(color);

        unsafe {
            ffi::DrawBoundingBox(bound, color);
            Ok(())
        }
    }

    //================================================================

    /* meta
    ---Initialize the 2D draw mode. **MUST** call *close_mode_2d* after 2D drawing is done.
    ---@param point vector The point of the camera.
    ---@param focus vector The focus of the camera.
    ---@param angle number The angle of the camera.
    ---@param zoom number The zoom of the camera.
    function begin_mode_2d(point, focus, angle, zoom) end
    */
    fn begin_mode_2d(
        _: &Lua,
        (point, focus, angle, zoom): (mlua::Vector, mlua::Vector, f32, f32),
    ) -> mlua::Result<()> {
        let value = general::ffi_cam2d(point, focus, angle, zoom);

        unsafe {
            ffi::BeginMode2D(value);
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
    ---*2D mode operation.* Get the world-space point for a screen-space point.
    ---@param point vector Screen-space point to convert from.
    ---@param c_point vector The point of the camera.
    ---@param c_focus vector The focus of the camera.
    ---@param c_angle number The angle of the camera.
    ---@param c_zoom number The zoom of the camera.
    ---@return vector # World-space point.
    function get_screen_to_world(point, c_point, c_focus, c_angle, c_zoom) end
    */
    fn get_screen_to_world(
        _: &Lua,
        (point, cam_point, cam_focus, cam_angle, cam_zoom): (
            mlua::Vector,
            mlua::Vector,
            mlua::Vector,
            f32,
            f32,
        ),
    ) -> mlua::Result<mlua::Vector> {
        let point = general::ffi_vect2(point);
        let camera = general::ffi_cam2d(cam_point, cam_focus, cam_angle, cam_zoom);

        unsafe {
            let value = ffi::GetScreenToWorld2D(point, camera);

            Ok(mlua::Vector::new(value.x, value.y, 0.0, 0.0))
        }
    }

    /* meta
    ---*2D mode operation.* Get the screen-space point for a world-space point.
    ---@param point vector World-space point to convert from.
    ---@param c_point vector The point of the camera.
    ---@param c_focus vector The focus of the camera.
    ---@param c_angle number The angle of the camera.
    ---@param c_zoom number The zoom of the camera.
    ---@return vector # Screen-space point.
    function get_world_to_screen(point, c_point, c_focus, c_angle, c_zoom) end
    */
    fn get_world_to_screen(
        _: &Lua,
        (point, cam_point, cam_focus, cam_angle, cam_zoom): (
            mlua::Vector,
            mlua::Vector,
            mlua::Vector,
            f32,
            f32,
        ),
    ) -> mlua::Result<mlua::Vector> {
        let point = general::ffi_vect2(point);
        let camera = general::ffi_cam2d(cam_point, cam_focus, cam_angle, cam_zoom);

        unsafe {
            let value = ffi::GetWorldToScreen2D(point, camera);

            Ok(mlua::Vector::new(value.x, value.y, 0.0, 0.0))
        }
    }

    /* meta
    ---*2D mode operation.* Draw a rectangle.
    ---@param shape vector Shape for the rectangle.
    ---@param point vector Point for the rectangle.
    ---@param angle vector Angle for the rectangle.
    ---@param color vector Color for the rectangle.
    function draw_rectangle(shape, point, angle, color) end
    */
    fn draw_rectangle(
        _: &Lua,
        (shape, point, angle, color): (mlua::Vector, mlua::Vector, f32, mlua::Vector),
    ) -> mlua::Result<()> {
        let recta = general::ffi_rectn(shape);
        let point = general::ffi_vect2(point);
        let color = general::ffi_color(color);

        unsafe {
            ffi::DrawRectanglePro(recta, point, angle, color);
            Ok(())
        }
    }

    /* meta
    ---*2D mode operation.* Draw text.
    ---@param text string Text to draw.
    ---@param point vector Point for the text.
    ---@param scale number Scale for the text.
    ---@param color vector Color for the text.
    function draw_text(text, point, scale, color) end
    */
    fn draw_text(
        _: &Lua,
        (text, point, scale, color): (String, mlua::Vector, i32, mlua::Vector),
    ) -> mlua::Result<()> {
        let text = CString::new(text).map_err(|e| mlua::Error::runtime(e.to_string()))?;
        let point = general::ffi_vect2(point);
        let color = general::ffi_color(color);

        unsafe {
            ffi::DrawText(text.as_ptr(), point.x as i32, point.y as i32, scale, color);
            Ok(())
        }
    }
}

//================================================================

pub mod audio {
    use super::*;

    //================================================================

    type RLSound = ffi::Sound;
    type RLMusic = ffi::Music;

    //================================================================

    /* meta
    ---@class (exact) sound
    local sound = {}

    ---An unique handle for sound in memory.
    ---@param path string Path to file.
    ---@return sound # The user-data object.
    function Sound(path) end
    */
    pub struct Sound(RLSound);

    impl mlua::UserData for Sound {
        fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

        fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
            /* meta
            --- foo
            */
            method.add_method("play", |_, this, ()| unsafe {
                ffi::PlaySound(this.0);
                Ok(())
            });
        }
    }

    impl Sound {
        pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
            let name =
                CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

            unsafe {
                let data = ffi::LoadSound(name.as_ptr());

                if ffi::IsSoundReady(data) {
                    Ok(Self(data))
                } else {
                    Err(mlua::Error::RuntimeError(format!(
                        "Sound::new(): Could not load file \"{path}\"."
                    )))
                }
            }
        }
    }

    impl Drop for Sound {
        fn drop(&mut self) {
            unsafe {
                ffi::UnloadSound(self.0);
            }
        }
    }

    //================================================================

    /* meta
    ---@class (exact) music
    local music = {}

    ---An unique handle for music in memory.
    ---@param path string Path to file.
    ---@return music # The user-data object.
    function Music(path) end
    */
    pub struct Music(RLMusic);

    impl mlua::UserData for Music {
        fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

        fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
            /* meta
            ---Play music.
            function music:play() end
            */
            method.add_method("play", |_, this, ()| unsafe {
                ffi::PlayMusicStream(this.0);
                Ok(())
            });

            /* meta
            ---Update music stream.
            function music:update() end
            */
            method.add_method("update", |_, this, ()| unsafe {
                ffi::UpdateMusicStream(this.0);
                Ok(())
            });
        }
    }

    impl Music {
        pub fn new(_: &Lua, path: String) -> mlua::Result<Self> {
            let name =
                CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

            unsafe {
                let data = ffi::LoadMusicStream(name.as_ptr());

                if ffi::IsMusicReady(data) {
                    Ok(Self(data))
                } else {
                    Err(mlua::Error::RuntimeError(format!(
                        "Music::new(): Could not load file \"{path}\"."
                    )))
                }
            }
        }
    }

    impl Drop for Music {
        fn drop(&mut self) {
            unsafe {
                ffi::UnloadMusicStream(self.0);
            }
        }
    }

    //================================================================

    #[rustfmt::skip]
    pub fn set_global(lua: &Lua, global: &mlua::Table, system: &ModuleSystem) -> mlua::Result<()> {
        if system.sound { global.set("Sound", lua.create_function(self::Sound::new)?)?; }
        if system.music { global.set("Music", lua.create_function(self::Music::new)?)?; }

        Ok(())
    }
}

pub mod input {
    use super::*;

    pub const BOARD_RANGE_LOWER: i32 = 0;
    pub const BOARD_RANGE_UPPER: i32 = 384;
    pub const MOUSE_RANGE_LOWER: i32 = 0;
    pub const MOUSE_RANGE_UPPER: i32 = 6;
    pub const CURSOR_RANGE_LOWER: i32 = 0;
    pub const CURSOR_RANGE_UPPER: i32 = 10;
    pub const PAD_RANGE_LOWER: i32 = 0;
    pub const PAD_RANGE_UPPER: i32 = 17;

    //================================================================

    #[rustfmt::skip]
    pub fn set_global(lua: &Lua, global: &mlua::Table) -> mlua::Result<()> {
        //global.set("set_clipboard_text", lua.create_function(self::set_clipboard_text)?)?;
        //global.set("get_clipboard_text", lua.create_function(self::get_clipboard_text)?)?;

        //global.set("get_board_key_queue",      lua.create_function(self::get_keycode_queue)?)?;
        //global.set("get_board_uni_queue",      lua.create_function(self::get_unicode_queue)?)?;
        global.set("get_board_up",      lua.create_function(self::get_board_up)?)?;
        global.set("get_board_down",    lua.create_function(self::get_board_down)?)?;
        global.set("get_board_press",   lua.create_function(self::get_board_press)?)?;
        global.set("get_board_release", lua.create_function(self::get_board_release)?)?;

        global.set("set_mouse_active",  lua.create_function(self::set_mouse_active)?)?;
        global.set("set_mouse_hidden",  lua.create_function(self::set_mouse_hidden)?)?;
        global.set("get_mouse_hidden",  lua.create_function(self::get_mouse_hidden)?)?;
        global.set("get_mouse_screen",  lua.create_function(self::get_mouse_screen)?)?;
        global.set("get_mouse_point",   lua.create_function(self::get_mouse_point)?)?;
        global.set("get_mouse_delta",   lua.create_function(self::get_mouse_delta)?)?;
        global.set("set_mouse_point",      lua.create_function(self::set_mouse_point)?)?;
        global.set("set_mouse_shift",      lua.create_function(self::set_mouse_shift)?)?;
        global.set("set_mouse_scale",      lua.create_function(self::set_mouse_scale)?)?;
        global.set("set_mouse_cursor",     lua.create_function(self::set_mouse_cursor)?)?;
        global.set("get_mouse_wheel",      lua.create_function(self::get_mouse_wheel)?)?;
        global.set("get_mouse_up",      lua.create_function(self::get_mouse_up)?)?;
        global.set("get_mouse_down",    lua.create_function(self::get_mouse_down)?)?;
        global.set("get_mouse_press",   lua.create_function(self::get_mouse_press)?)?;
        global.set("get_mouse_release", lua.create_function(self::get_mouse_release)?)?;

        //global.set("get_pad_state",   lua.create_function(self::get_pad_state)?)?;
        //global.set("get_pad_name",    lua.create_function(self::get_pad_name)?)?;
        //global.set("get_pad_queue",   lua.create_function(self::get_pad_queue)?)?;
        //global.set("get_pad_axis_count",    lua.create_function(self::get_pad_axis_count)?)?;
        //global.set("get_pad_axis_state",    lua.create_function(self::get_pad_axis_state)?)?;
        global.set("get_pad_up",      lua.create_function(self::get_pad_up)?)?;
        global.set("get_pad_down",    lua.create_function(self::get_pad_down)?)?;
        global.set("get_pad_press",   lua.create_function(self::get_pad_press)?)?;
        global.set("get_pad_release", lua.create_function(self::get_pad_release)?)?;

        global.set("set_interface_alpha",  lua.create_function(self::set_interface_alpha)?)?;
        global.set("interface_button",     lua.create_function(self::interface_button)?)?;
        global.set("interface_toggle",     lua.create_function(self::interface_toggle)?)?;
        global.set("interface_check_box",  lua.create_function(self::interface_check_box)?)?;
        global.set("interface_combo_box",  lua.create_function(self::interface_combo_box)?)?;
        global.set("interface_spinner",    lua.create_function(self::interface_spinner)?)?;
        global.set("interface_slider",     lua.create_function(self::interface_slider)?)?;
        global.set("interface_slider_bar", lua.create_function(self::interface_slider_bar)?)?;

        Ok(())
    }

    //================================================================

    /* meta
    ---Set the interface alpha.
    ---@param value number The alpha of the interface.
    function set_interface_alpha(value) end
    */
    fn set_interface_alpha(_: &Lua, value: f32) -> mlua::Result<()> {
        unsafe { Ok(ffi::GuiSetAlpha(value)) }
    }

    /* meta
    ---Draw an interface button.
    ---@param shape vector The shape of the button. X/Y for position, Z/W for width/height.
    ---@param label string The label of the button.
    ---@return boolean # True on button click.
    function interface_button(shape, label) end
    */
    fn interface_button(_: &Lua, (shape, label): (mlua::Vector, String)) -> mlua::Result<bool> {
        let shape = general::ffi_rectn(shape);
        let label = CString::new(label).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe { Ok(ffi::GuiButton(shape, label.as_ptr()) > 0) }
    }

    /* meta
    ---Draw an interface toggle.
    ---@param shape vector  The shape of the slider. X/Y for position, Z/W for width/height.
    ---@param label string  The label of the slider.
    ---@param value boolean The value of the slider.
    ---@return boolean # The new value of *value*, if any.
    function interface_toggle(shape, label, value) end
    */
    fn interface_toggle(
        _: &Lua,
        (shape, label, mut value): (mlua::Vector, String, bool),
    ) -> mlua::Result<bool> {
        let shape = general::ffi_rectn(shape);
        let label = CString::new(label).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            ffi::GuiToggle(shape, label.as_ptr(), &mut value);
            Ok(value)
        }
    }

    /* meta
    ---Draw an interface check box.
    ---@param shape vector  The shape of the check box. X/Y for position, Z/W for width/height.
    ---@param label string  The label of the check box.
    ---@param value boolean The value of the check box.
    ---@return boolean # The new value of *value*, if any.
    function interface_check_box(shape, label, value) end
    */
    fn interface_check_box(
        _: &Lua,
        (shape, label, mut value): (mlua::Vector, String, bool),
    ) -> mlua::Result<bool> {
        let shape = general::ffi_rectn(shape);
        let label = CString::new(label).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            ffi::GuiCheckBox(shape, label.as_ptr(), &mut value);
            Ok(value)
        }
    }

    /* meta
    ---Draw an interface spinner.
    ---@param shape vector  The shape of the spinner. X/Y for position, Z/W for width/height.
    ---@param label string  The label of the spinner.
    ---@param value number  The value of the spinner.
    ---@param min   number  The minimum value of the spinner.
    ---@param max   number  The maximum value of the spinner.
    ---@param edit  boolean The edit mode value of the spinner.
    ---@return number # The new value of *value*, if any.
    function interface_spinner(shape, label, value, min, max, edit) end
    */
    fn interface_spinner(
        _: &Lua,
        (shape, label, mut value, min, max, edit): (mlua::Vector, String, i32, i32, i32, bool),
    ) -> mlua::Result<i32> {
        let shape = general::ffi_rectn(shape);
        let label = CString::new(label).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            ffi::GuiSpinner(shape, label.as_ptr(), &mut value, min, max, edit);
            Ok(value)
        }
    }

    /* meta
    ---Draw an interface combo box.
    ---@param shape vector The shape of the combo box. X/Y for position, Z/W for width/height.
    ---@param label string The label of the combo box.
    ---@param value number The value of the combo box.
    ---@return number # The new value of *value*, if any.
    function interface_combo_box(shape, label, value) end
    */
    fn interface_combo_box(
        _: &Lua,
        (shape, label, mut value): (mlua::Vector, String, i32),
    ) -> mlua::Result<i32> {
        let shape = general::ffi_rectn(shape);
        let label = CString::new(label).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            ffi::GuiComboBox(shape, label.as_ptr(), &mut value);
            Ok(value)
        }
    }

    /* meta
    ---Draw an interface slider.
    ---@param shape   vector The shape of the slider. X/Y for position, Z/W for width/height.
    ---@param label_a string The label of the slider.
    ---@param label_b string The label of the slider.
    ---@param value   number The value of the slider.
    ---@param min     number The minimum value of the slider.
    ---@param max     number The maximum value of the slider.
    ---@return number # The new value of *value*, if any.
    function interface_slider(shape, label_a, label_b, value, min, max) end
    */
    fn interface_slider(
        _: &Lua,
        (shape, label_a, label_b, mut value, min, max): (
            mlua::Vector,
            String,
            String,
            f32,
            f32,
            f32,
        ),
    ) -> mlua::Result<f32> {
        let shape = general::ffi_rectn(shape);
        let label_a = CString::new(label_a).map_err(|e| mlua::Error::runtime(e.to_string()))?;
        let label_b = CString::new(label_b).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            ffi::GuiSlider(
                shape,
                label_a.as_ptr(),
                label_b.as_ptr(),
                &mut value,
                min,
                max,
            );
            Ok(value)
        }
    }

    /* meta
    ---Draw an interface slider bar.
    ---@param shape   vector The shape of the slider bar. X/Y for position, Z/W for width/height.
    ---@param label_a string The label of the slider bar.
    ---@param label_b string The label of the slider bar.
    ---@param value   number The value of the slider bar.
    ---@param min     number The minimum value of the slider bar.
    ---@param max     number The maximum value of the slider bar.
    ---@return number # The new value of *value*, if any.
    function interface_slider_bar(shape, label_a, label_b, value, min, max) end
    */
    fn interface_slider_bar(
        _: &Lua,
        (shape, label_a, label_b, mut value, min, max): (
            mlua::Vector,
            String,
            String,
            f32,
            f32,
            f32,
        ),
    ) -> mlua::Result<f32> {
        let shape = general::ffi_rectn(shape);
        let label_a = CString::new(label_a).map_err(|e| mlua::Error::runtime(e.to_string()))?;
        let label_b = CString::new(label_b).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            ffi::GuiSliderBar(
                shape,
                label_a.as_ptr(),
                label_b.as_ptr(),
                &mut value,
                min,
                max,
            );
            Ok(value)
        }
    }

    /* meta
    ---Set the active state of the mouse.
    ---@param value boolean New state. Active if true, inactive if false.
    function set_mouse_active(value) end
    */
    fn set_mouse_active(_: &Lua, value: bool) -> mlua::Result<()> {
        unsafe {
            if value {
                ffi::EnableCursor();
                Ok(())
            } else {
                ffi::DisableCursor();
                Ok(())
            }
        }
    }

    /* meta
    ---Set the hidden state of the mouse.
    ---@param value boolean New state. Hide if true, show if false.
    function set_mouse_hidden(value) end
    */
    fn set_mouse_hidden(_: &Lua, value: bool) -> mlua::Result<()> {
        unsafe {
            if value {
                ffi::HideCursor();
                Ok(())
            } else {
                ffi::ShowCursor();
                Ok(())
            }
        }
    }

    /* meta
    ---Check if the mouse is currently hidden.
    ---@return boolean # The hidden state of the mouse.
    function get_mouse_hidden() end
    */
    fn get_mouse_hidden(_: &Lua, _: ()) -> mlua::Result<bool> {
        unsafe { Ok(ffi::IsCursorHidden()) }
    }

    /* meta
    ---Check if the mouse is currently over the screen.
    ---@return boolean # The screen state of the mouse.
    function get_mouse_screen() end
    */
    fn get_mouse_screen(_: &Lua, _: ()) -> mlua::Result<bool> {
        unsafe { Ok(ffi::IsCursorOnScreen()) }
    }

    /* meta
    ---Get the current point of the mouse.
    ---@return vector # The point of the mouse.
    function get_mouse_point() end
    */
    fn get_mouse_point(_: &Lua, _: ()) -> mlua::Result<mlua::Vector> {
        unsafe {
            let value = ffi::GetMousePosition();
            Ok(mlua::Vector::new(value.x, value.y, 0.0, 0.0))
        }
    }

    /* meta
    ---Get the current delta (i.e. mouse movement) of the mouse.
    ---@return vector # The delta of the mouse.
    function get_mouse_delta() end
    */
    fn get_mouse_delta(_: &Lua, _: ()) -> mlua::Result<mlua::Vector> {
        unsafe {
            let value = ffi::GetMouseDelta();
            Ok(mlua::Vector::new(value.x, value.y, 0.0, 0.0))
        }
    }

    /* meta
    ---Set the current point of the mouse.
    ---@param value vector New point.
    function set_mouse_point(value) end
    */
    fn set_mouse_point(_: &Lua, point: mlua::Vector) -> mlua::Result<()> {
        unsafe {
            ffi::SetMousePosition(point.x() as i32, point.y() as i32);
            Ok(())
        }
    }

    /* meta
    ---Set the current shift of the mouse.
    ---@param value vector New shift.
    function set_mouse_shift(value) end
    */
    fn set_mouse_shift(_: &Lua, point: mlua::Vector) -> mlua::Result<()> {
        unsafe {
            ffi::SetMouseOffset(point.x() as i32, point.y() as i32);
            Ok(())
        }
    }

    /* meta
    ---Set the current scale of the mouse.
    ---@param value vector New scale.
    function set_mouse_scale(value) end
    */
    fn set_mouse_scale(_: &Lua, point: mlua::Vector) -> mlua::Result<()> {
        unsafe {
            ffi::SetMouseScale(point.x(), point.y());
            Ok(())
        }
    }

    /* meta
    ---Set the current cursor of the mouse.
    ---@param value cursor_mouse New cursor.
    function set_mouse_cursor(value) end
    */
    fn set_mouse_cursor(_: &Lua, value: i32) -> mlua::Result<()> {
        if (self::CURSOR_RANGE_LOWER..=self::CURSOR_RANGE_LOWER).contains(&value) {
            unsafe {
                ffi::SetMouseCursor(value);
                Ok(())
            }
        } else {
            Err(mlua::Error::runtime(
                "set_mouse_cursor(): Unknown cursor value.",
            ))
        }
    }

    /* meta
    ---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
    ---@return vector # The delta of the mouse wheel.
    function get_mouse_wheel() end
    */
    fn get_mouse_wheel(_: &Lua, _: ()) -> mlua::Result<mlua::Vector> {
        unsafe {
            let value = ffi::GetMouseWheelMoveV();
            Ok(mlua::Vector::new(value.x, value.y, 0.0, 0.0))
        }
    }

    /* meta
    ---Get the state of an input (up).
    ---@param value input_mouse The input to check for.
    function get_mouse_up(value) end
    */
    fn get_mouse_up(_: &Lua, value: i32) -> mlua::Result<bool> {
        if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsMouseButtonUp(value)) }
        } else {
            Err(mlua::Error::runtime("get_mouse_up(): Unknown key value."))
        }
    }

    /* meta
    ---Get the state of an input (down).
    ---@param value input_mouse The input to check for.
    function get_mouse_down(value) end
    */
    fn get_mouse_down(_: &Lua, value: i32) -> mlua::Result<bool> {
        if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsMouseButtonDown(value)) }
        } else {
            Err(mlua::Error::runtime("get_mouse_up(): Unknown key value."))
        }
    }

    /* meta
    ---Get the state of an input (press).
    ---@param value input_mouse The input to check for.
    function get_mouse_press(value) end
    */
    fn get_mouse_press(_: &Lua, value: i32) -> mlua::Result<bool> {
        if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsMouseButtonPressed(value)) }
        } else {
            Err(mlua::Error::runtime("get_mouse_up(): Unknown key value."))
        }
    }

    /* meta
    ---Get the state of an input (release).
    ---@param value input_mouse The input to check for.
    function get_mouse_release(value) end
    */
    fn get_mouse_release(_: &Lua, value: i32) -> mlua::Result<bool> {
        if (self::MOUSE_RANGE_LOWER..=self::MOUSE_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsMouseButtonReleased(value)) }
        } else {
            Err(mlua::Error::runtime("get_mouse_up(): Unknown key value."))
        }
    }

    //================================================================

    /* meta
    ---Get the state of an input (up).
    ---@param value input_board The input to check for.
    function get_board_up(value) end
    */
    fn get_board_up(_: &Lua, value: i32) -> mlua::Result<bool> {
        if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsKeyUp(value)) }
        } else {
            Err(mlua::Error::runtime("get_board_up(): Unknown key value."))
        }
    }

    /* meta
    ---Get the state of an input (down).
    ---@param value input_board The input to check for.
    function get_board_down(value) end
    */
    fn get_board_down(_: &Lua, value: i32) -> mlua::Result<bool> {
        if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsKeyDown(value)) }
        } else {
            Err(mlua::Error::runtime("get_board_up(): Unknown key value."))
        }
    }

    /* meta
    ---Get the state of an input (press).
    ---@param value input_board The input to check for.
    function get_board_press(value) end
    */
    fn get_board_press(_: &Lua, value: i32) -> mlua::Result<bool> {
        if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsKeyPressed(value)) }
        } else {
            Err(mlua::Error::runtime("get_board_up(): Unknown key value."))
        }
    }

    /* meta
    ---Get the state of an input (release).
    ---@param value input_board The input to check for.
    function get_board_release(value) end
    */
    fn get_board_release(_: &Lua, value: i32) -> mlua::Result<bool> {
        if (self::BOARD_RANGE_LOWER..=self::BOARD_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsKeyReleased(value)) }
        } else {
            Err(mlua::Error::runtime("get_board_up(): Unknown key value."))
        }
    }

    //================================================================

    /* meta
    ---Get the state of an input (up).
    ---@param index integer The index of the pad to check for.
    ---@param value input_pad The input to check for.
    function get_pad_up(index, value) end
    */
    fn get_pad_up(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
        if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsGamepadButtonUp(index, value)) }
        } else {
            Err(mlua::Error::runtime("get_pad_up(): Unknown key value."))
        }
    }

    /* meta
    ---Get the state of an input (down).
    ---@param index integer The index of the pad to check for.
    ---@param value input_pad The input to check for.
    function get_pad_down(index, value) end
    */
    fn get_pad_down(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
        if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsGamepadButtonDown(index, value)) }
        } else {
            Err(mlua::Error::runtime("get_pad_up(): Unknown key value."))
        }
    }

    /* meta
    ---Get the state of an input (press).
    ---@param index integer The index of the pad to check for.
    ---@param value input_pad The input to check for.
    function get_pad_press(index, value) end
    */
    fn get_pad_press(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
        if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsGamepadButtonPressed(index, value)) }
        } else {
            Err(mlua::Error::runtime("get_pad_up(): Unknown key value."))
        }
    }

    /* meta
    ---Get the state of an input (release).
    ---@param index integer The index of the pad to check for.
    ---@param value input_pad The input to check for.
    function get_pad_release(index, value) end
    */
    fn get_pad_release(_: &Lua, (index, value): (i32, i32)) -> mlua::Result<bool> {
        if (self::PAD_RANGE_LOWER..=self::PAD_RANGE_UPPER).contains(&value) {
            unsafe { Ok(ffi::IsGamepadButtonReleased(index, value)) }
        } else {
            Err(mlua::Error::runtime("get_pad_up(): Unknown key value."))
        }
    }
}

pub mod collision {
    use super::*;

    //================================================================

    #[rustfmt::skip]
    pub fn set_global(_: &Lua, _: &mlua::Table) -> mlua::Result<()> {
        Ok(())
    }
}
