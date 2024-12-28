---@meta

---The Quiver API.
---@class quiver
quiver = {}

---Main entry-point. Quiver will call this on initialization.
---@alias quiver.main fun()

---Fail entry-point. Quiver will call this on a script error, with the script error message as the argument. Note that this function is OPTIONAL, and Quiver will use a default crash handler if missing.
---@alias quiver.fail fun(error : string)

---The sound API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L34)
---@class quiver.sound
quiver.sound = {}

---An unique handle for sound in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L50)
---@class sound
sound = {}

---Create a new sound resource.
---@param path string # Path to sound file.
---@return sound sound # Sound resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L66)
function quiver.sound.new(path) end

---Play the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L98)
function sound:play() end

---Check if sound is currently playing.
---@return boolean state # State of the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L113)
function sound:get_playing() end

---Stop the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L120)
function sound:stop() end

---Pause the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L128)
function sound:pause() end

---Resume the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L136)
function sound:resume() end

---Set volume for the sound. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L151)
function sound:set_volume(volume) end

---Set pitch for the sound.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L166)
function sound:set_pitch(pitch) end

---Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L181)
function sound:set_pan(pan) end

---The input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L45)
---@class quiver.input
quiver.input = {}

---The board input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L57)
---@class quiver.input.board
quiver.input.board = {}

---The mouse input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L73)
---@class quiver.input.mouse
quiver.input.mouse = {}

---The pad input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L98)
---@class quiver.input.pad
quiver.input.pad = {}

---Set the active state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L128)
function quiver.input.mouse.set_active(state) end

---Set the hidden state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L149)
function quiver.input.mouse.set_hidden(state) end

---Get the hidden state of the mouse.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L170)
function quiver.input.mouse.get_hidden() end

---Check if the mouse is currently over the screen.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L183)
function quiver.input.mouse.get_screen() end

---Get the current point of the mouse.
---@return vector_2 point # The point of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L196)
function quiver.input.mouse.get_point() end

---Set the current point of the mouse.
---@param point vector_2 # The point of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L212)
function quiver.input.mouse.set_point(point) end

---Get the current delta (i.e. mouse movement) of the mouse.
---@return vector_2 delta # The delta of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L229)
function quiver.input.mouse.get_delta() end

---Set the current shift of the mouse.
---@param shift vector_2 # The shift of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L245)
function quiver.input.mouse.set_shift(shift) end

---Set the current scale of the mouse.
---@param scale vector_2 # The scale of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L262)
function quiver.input.mouse.set_scale(scale) end

---Set the current cursor of the mouse.
---@param cursor cursor_mouse # The cursor of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L279)
function quiver.input.mouse.set_cursor(cursor) end

---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return vector_2 delta # The delta of the mouse wheel.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L301)
function quiver.input.mouse.get_wheel() end

---Get the state of an input (up).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L317)
function quiver.input.mouse.get_up(mouse) end

---Get the state of an input (down).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L334)
function quiver.input.mouse.get_down(mouse) end

---Get the state of an input (press).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L351)
function quiver.input.mouse.get_press(mouse) end

---Get the state of an input (release).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L368)
function quiver.input.mouse.get_release(mouse) end

---Get the state of an input (up).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L387)
function quiver.input.board.get_up(board) end

---Get the state of an input (down).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L404)
function quiver.input.board.get_down(board) end

---Get the state of an input (press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L421)
function quiver.input.board.get_press(board) end

---Get the state of an input (release).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L438)
function quiver.input.board.get_release(board) end

---Get the state of a pad.
---@param index number # The index of the pad to check for.
---@return boolean state # The state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L460)
function quiver.input.pad.get_state(index) end

---Get the name of a pad.
---@param index number # The index of the pad to check for.
---@return string name # The name of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L476)
function quiver.input.pad.get_name(index) end

---Get the last pad button press.
---@return input_pad input # The last pad button press.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L492)
function quiver.input.pad.get_queue() end

---Get the axis count of a pad.
---@param index number # The index of the pad to check for.
---@return number axis_count # The axis count of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L508)
function quiver.input.pad.get_axis_count(index) end

---Get the axis state of a pad.
---@param index number # The index of the pad to check for.
---@param axis number # The axis of the pad to check for.
---@return number axis_state # The axis state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L525)
function quiver.input.pad.get_axis_state(index,axis) end

---Get the state of an input (up).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L538)
function quiver.input.pad.get_up(pad) end

---Get the state of an input (down).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L555)
function quiver.input.pad.get_down(pad) end

---Get the state of an input (press).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L572)
function quiver.input.pad.get_press(pad) end

---Get the state of an input (release).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L589)
function quiver.input.pad.get_release(pad) end

---The Rapier API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L36)
---@class quiver.rapier
quiver.rapier = {}

---An unique handle for a Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L50)
---@class rapier
rapier = {}

---Create a new Rapier simulation.
---@return rapier rapier # Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L77)
function quiver.rapier.new() end

---Cast a ray.
---@param ray ray # 
---@param time number # 
---@param filter_rigid table # 
---@param filter_collider table # 
---@return boolean pick # 
---@return number time # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L103)
function rapier:cast_ray(ray,time,filter_rigid,filter_collider) end

---Create a rigid body.
---@param data rigid_body_info # Rigid body data.
---@return table rigid_body # Rigid body.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L169)
function rapier:create_rigid_body(data) end

---Create a collider.
---@param data collider_info # Collider data.
---@return table collider # Collider.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L242)
function rapier:create_collider(data) end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L287)
function rapier:create_sphere() end

---Set the collider shape.
---@param collider table # Collider.
---@param shape vector_3 # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L317)
function rapier:collider_set_shape(collider,shape) end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L343)
function rapier:set_rigid_body_data() end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L364)
function rapier:get_rigid_body_data() end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L377)
function rapier:get_collider_data() end

---Create a kinematic character controller.
---@return table controller # Controller.
---@return table collider # Collider.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L394)
function rapier:character_controller() end

---Move a kinematic character controller.
---@param controller table # Controller.
---@param collider table # Collider.
---@param velocity vector_3 # Velocity.
---@param time_step number # Time step.
---@return vector_3 translation # Translation.
---@return boolean floor # Currently on floor.
---@return boolean slide # Currently on slide.
---@return table collision # Collision list.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L426)
function rapier:move_character_controller(controller,collider,velocity,time_step) end

---Create a convex mesh collider out of a point cloud.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L497)
function rapier:collider_convex_mesh() end

---Step the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L515)
function rapier:step() end

---Render the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L542)
function rapier:debug_render() end

---The texture API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L39)
---@class quiver.texture
quiver.texture = {}

---An unique handle for a texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L108)
---@class texture
---@field shape vector_2 # Shape of the texture.
texture = {}

---Set the mipmap for a texture.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L125)
function texture:set_mipmap() end

---Set the filter for a texture.
---@param filter texture_filter # Texture filter.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L142)
function texture:set_filter(filter) end

---Set the wrap for a texture.
---@param wrap texture_wrap # Texture wrap.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L159)
function texture:set_wrap(wrap) end

---Draw a texture.
---@param point vector_2 # 
---@param angle number # 
---@param scale number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L179)
function texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # 
---@param box_b box_2 # 
---@param point vector_2 # 
---@param angle number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L200)
function texture:draw_pro(box_a,box_b,point,angle,color) end

---Create a new texture resource.
---@param path string # Path to texture file.
---@return texture texture # Texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L234)
function quiver.texture.new(path) end

---An unique handle for a render texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L269)
---@class render_texture
---@field shape vector_2 # Shape of the texture.
render_texture = {}

---Initialize drawing to the render texture.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L292)
function render_texture:begin(call) end

---Draw a texture.
---@param point vector_2 # 
---@param angle number # 
---@param scale number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L317)
function render_texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # 
---@param box_b box_2 # 
---@param point vector_2 # 
---@param angle number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L341)
function render_texture:draw_pro(box_a,box_b,point,angle,color) end

---Create a new render texture resource.
---@param shape vector_2 # 
---@return render_texture render_texture # Render texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L375)
function quiver.render_texture.new(shape) end

---The model API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L34)
---@class quiver.model
quiver.model = {}

---An unique handle for a model in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L65)
---@class model
---@field mesh_count number # Mesh count.
---@field bone_count number # Bone count.
---@field material_count number # Material count.
model = {}

---Create a new Model resource.
---@param path string # Path to model file.
---@return model model # Model resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L81)
function quiver.model.new(path) end

---Bind a texture to the model.
---@param texture texture # Texture to bind to model.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L116)
function model:bind(texture) end

---Draw the model.
---@param point vector_3 # 
---@param scale number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L152)
function model:draw(point,scale,color) end

---Draw the model with a transformation.
---@param point vector_3 # 
---@param angle vector_4 # 
---@param scale vector_3 # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L176)
function model:draw_transform(point,angle,scale,color) end

---Get the vertex data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Vector3 table.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L207)
function model:mesh_vertex(index) end

---An unique handle for a model animation in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L223)
---@class model_animation
model_animation = {}

---Create a new ModelAnimation resource.
---@param path string # Path to model file.
---@return model_animation model_animation # ModelAnimation resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L239)
function quiver.model_animation.new(path) end

---Update model with new model animation data.
---@param model model # 
---@param index number # 
---@param frame number # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L282)
function model_animation:update(model,index,frame) end

---The drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L47)
---@class quiver.draw
quiver.draw = {}

---Initialize drawing to the screen.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L75)
function quiver.draw.begin(call) end

---Initialize drawing (blend mode) to the screen.
---@param call function # The draw code.
---@param mode function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L98)
function quiver.draw.begin_blend(call,mode) end

---Initialize drawing (scissor mode) to the screen.
---@param call function # The draw code.
---@param view box_2 # The clip test region.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L120)
function quiver.draw.begin_scissor(call,view) end

---Get a ray for a 2D screen-space point.
---@param camera camera_3d # The current camera.
---@param point vector_2 # The screen-space point.
---@param shape vector_2 # The size of the view-port.
---@return ray ray # The 3D ray, beginning at the screen-space point, in 3D space.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L153)
function quiver.draw.get_screen_to_world_3d(camera,point,shape) end

---Get a 2D screen-space point for a 3D world-space point.
---@param camera camera_3d # The current camera.
---@param point vector_3 # The world-space point.
---@param shape vector_2 # The size of the view-port.
---@return number point_x # The 2D screen-space point (X).
---@return number point_y # The 2D screen-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L189)
function quiver.draw.get_world_to_screen_3d(camera,point,shape) end

---Get a world-space point for a 2D screen-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The screen-space point.
---@return vector_2 point # The 2D world-space point.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L223)
function quiver.draw.get_screen_to_world_2d(camera,point) end

---Get a screen-space point for a 2D world-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The world-space point.
---@return vector_2 point # The 2D screen-space point.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L251)
function quiver.draw.get_world_to_screen_2d(camera,point) end

---Clear the screen with a color.
---@param color color # The color to use for clearing.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L274)
function quiver.draw.clear(color) end

---The 2D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L290)
---@class quiver.draw_2d
quiver.draw_2d = {}

---Initialize the 2D draw mode.
---@param call function # The draw code.
---@param camera camera_2d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L316)
function quiver.draw_2d.begin(call,camera) end

---Draw 2D box.
---@param shape box_2 # The shape of the box.
---@param point vector_2 # The point of the box.
---@param angle number # The angle of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L341)
function quiver.draw_2d.draw_box_2(shape,point,angle,color) end

---Draw text.
---@param label string # The label of the text.
---@param point vector_2 # The point of the text.
---@param scale number # The angle of the text.
---@param color color # The color of the text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L367)
function quiver.draw_2d.draw_text(label,point,scale,color) end

---Draw a circle.
---@param point vector_2 # 
---@param radius number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L399)
function quiver.draw_2d.draw_circle(point,radius,color) end

---Draw the sector of a circle.
---@param point vector_2 # 
---@param radius number # 
---@param begin_angle number # 
---@param close_angle number # 
---@param segment_count number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L427)
function quiver.draw_2d.draw_circle_sector(point,radius,begin_angle,close_angle,segment_count,color) end

---The 3D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L461)
---@class quiver.draw_3d
quiver.draw_3d = {}

---Initialize the 3D draw mode.
---@param call function # The draw code.
---@param camera camera_3d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L488)
function quiver.draw_3d.begin(call,camera) end

---Draw a grid.
---@param slice number # The slice count of the grid.
---@param space number # The space shift of the grid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L511)
function quiver.draw_3d.draw_grid(slice,space) end

---Draw a cube.
---@param point vector_3 # The point of the cube.
---@param shape vector_3 # The shape of the cube.
---@param color color # The color of the cube.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L529)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a ball.
---@param point vector_3 # The point of the ball.
---@param shape number # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L554)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a 3D box.
---@param shape box_3 # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L574)
function quiver.draw_3d.draw_box_3(shape,color) end

---Draw a ray.
---@param ray ray # The ray.
---@param color color # The color of the ray.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L594)
function quiver.draw_3d.draw_ray(ray,color) end

---The general API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L34)
---@class quiver.general
quiver.general = {}

---Load the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L58)
function quiver.general.load() end

---Exit the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L62)
function quiver.general.exit() end

---Set a key to exit Quiver.
---@param key input_board # Key to exit Quiver with.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L72)
function quiver.general.set_exit_key(key) end

---Get the current time. Will count up since the initialization of the window.
---@return number time # Current time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L94)
function quiver.general.get_time() end

---Get the current frame time.
---@return number frame_time # Current frame time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L107)
function quiver.general.get_frame_time() end

---Get the current frame rate.
---@return number frame_rate # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L120)
function quiver.general.get_frame_rate() end

---set the current frame rate.
---@param frame_rate number # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L133)
function quiver.general.set_frame_rate(frame_rate) end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L147)
function quiver.general.get_memory() end

---The shader API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L34)
---@class quiver.shader
quiver.shader = {}

---An unique handle for a shader in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L50)
---@class shader
shader = {}

---Create a new shader resource.
---@param v_path string # Path to .vs file.
---@param f_path string # Path to .fs file.
---@return shader shader # Shader resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L67)
function quiver.shader.new(v_path,f_path) end

---
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L116)
function shader:begin(call) end

---
---@param name string # 
---@return number location # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L141)
function shader:get_location_name(name) end

---
---@param location number # 
---@return number location # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L158)
function shader:get_location(location) end

---
---@param location number # 
---@param value number # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L173)
function shader:set_location(location,value) end

---
---@param location number # 
---@param value number # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L192)
function shader:set_shader_number(location,value) end

---
---@param location number # 
---@param value vector_3 # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L211)
function shader:set_shader_vector_3(location,value) end

---
---@param location number # 
---@param value vector_4 # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L231)
function shader:set_shader_vector_4(location,value) end

---The font API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L34)
---@class quiver.font
quiver.font = {}

---Set the vertical space between each line-break.
---@param space number # Vertical space.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L57)
function quiver.font.set_text_line_space(space) end

---An unique handle to a font in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L70)
---@class font
font = {}

---Draw a font.
---@param label string # Label of font to draw.
---@param point vector_2 # Point of font to draw.
---@param scale number # Scale of font to draw.
---@param space number # Space of font to draw.
---@param color color # Color of font to draw.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L90)
function font.draw(label,point,scale,space,color) end

---Measure the size of a given text on screen, with a given font.
---@param label string # Label of font to measure.
---@param scale number # Scale of font to measure.
---@param space number # Space of font to measure.
---@return number size_x # Size of text (X).
---@return number size_y # Size of text (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L128)
function font.measure_text(label,scale,space) end

---Create a new font resource.
---@param path string # Path to font file.
---@return font font # Font resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L156)
function quiver.font.new(path) end

---Create a new font resource (default font).
---@return font font # Font resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L182)
function quiver.font.new_default() end

---The window API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L34)
---@class quiver.window
quiver.window = {}

---Get the state of a window flag.
---@param flag window_flag # Window flag.
---@return boolean state # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L83)
function quiver.window.get_state(flag) end

---Set the state of a window flag.
---@param flag window_flag # Window flag.
---@param state boolean # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L98)
function quiver.window.set_state(flag,state) end

---Set the window to full-screen mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L113)
function quiver.window.set_fullscreen() end

---Set the window to border-less mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L123)
function quiver.window.set_borderless() end

---Minimize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L133)
function quiver.window.set_minimize() end

---Maximize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L143)
function quiver.window.set_maximize() end

---Focus the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L153)
function quiver.window.set_focus() end

---Restore the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L163)
function quiver.window.set_restore() end

---Set the window name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L173)
function quiver.window.set_name() end

---Set the window monitor.
---@param index number # Index of monitor to move window to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L192)
function quiver.window.set_monitor(index) end

---Set the current window shape.
---@param shape vector_2 # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L209)
function quiver.window.set_shape(shape) end

---Set the minimum window shape.
---@param shape vector_2 # Minimum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L228)
function quiver.window.set_shape_min(shape) end

---Set the maximum window shape.
---@param shape vector_2 # Maximum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L247)
function quiver.window.set_shape_max(shape) end

---Set the window alpha.
---@param alpha number # Alpha of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L266)
function quiver.window.set_alpha(alpha) end

---Set the window point.
---@param point vector_2 # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L283)
function quiver.window.set_point(point) end

---Get the state of the window (full-screen).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L304)
function quiver.window.get_fullscreen() end

---Get the state of the window (minimize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L318)
function quiver.window.get_minimize() end

---Get the state of the window (maximize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L332)
function quiver.window.get_maximize() end

---Get the state of the window (focus).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L346)
function quiver.window.get_focus() end

---Get the state of the window (resize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L360)
function quiver.window.get_resize() end

---Get the state of the window (hidden).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L374)
function quiver.window.get_hidden() end

---Get the shape of the window.
---@return vector_2 shape # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L388)
function quiver.window.get_shape() end

---Get the point of the window.
---@return vector_2 point # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L407)
function quiver.window.get_point() end

---Get the DPI scale of the window.
---@return number scale # Scale of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L425)
function quiver.window.get_scale() end

---Get if the window should close.
---@return boolean close # True if the window should close.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L443)
function quiver.window.get_close() end

---The music API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L34)
---@class quiver.music
quiver.music = {}

---An unique handle for music in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L50)
---@class music
music = {}

---Create a new music resource.
---@param path string # Path to music file.
---@return music music # Music resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L66)
function quiver.music.new(path) end

---Play the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L98)
function music:play() end

---Check if music is currently playing.
---@return boolean state # State of the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L113)
function music:get_playing() end

---Stop the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L120)
function music:stop() end

---Pause the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L128)
function music:pause() end

---Resume the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L136)
function music:resume() end

---Set volume for the music. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L151)
function music:set_volume(volume) end

---Set pitch for the music.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L166)
function music:set_pitch(pitch) end

---Set pan for the music. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L181)
function music:set_pan(pan) end

---Update the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L189)
function music:update() end

---Set position for the music.
---@param position number # Current position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L204)
function music:set_position(position) end

---Get time length for the music.
---@return number length # Time length.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L219)
function music:get_length() end

---Get time played for the music.
---@return number played # Time played.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L233)
function music:get_played() end

