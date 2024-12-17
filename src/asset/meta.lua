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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L10)
---@class quiver.sound
quiver.sound = {}

---An unique handle for sound in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L26)
---@class sound
sound = {}

---Create a new sound resource.
---@param path string # Path to sound file.
---@return sound sound # Sound resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L42)
function quiver.sound.new(path) end

---Play the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L74)
function sound:play() end

---Check if sound is currently playing.
---@return boolean state # State of the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L89)
function sound:get_playing() end

---Stop the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L96)
function sound:stop() end

---Pause the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L104)
function sound:pause() end

---Resume the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L112)
function sound:resume() end

---Set volume for the sound. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L127)
function sound:set_volume(volume) end

---Set pitch for the sound.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L142)
function sound:set_pitch(pitch) end

---Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L157)
function sound:set_pan(pan) end

---The input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L21)
---@class quiver.input
quiver.input = {}

---The board input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L33)
---@class quiver.input.board
quiver.input.board = {}

---The mouse input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L49)
---@class quiver.input.mouse
quiver.input.mouse = {}

---The pad input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L74)
---@class quiver.input.pad
quiver.input.pad = {}

---Set the active state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L104)
function quiver.input.mouse.set_active(state) end

---Set the hidden state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L125)
function quiver.input.mouse.set_hidden(state) end

---Get the hidden state of the mouse.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L146)
function quiver.input.mouse.get_hidden() end

---Check if the mouse is currently over the screen.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L159)
function quiver.input.mouse.get_screen() end

---Get the current point of the mouse.
---@return vector_2 point # The point of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L172)
function quiver.input.mouse.get_point() end

---Set the current point of the mouse.
---@param point vector_2 # The point of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L188)
function quiver.input.mouse.set_point(point) end

---Get the current delta (i.e. mouse movement) of the mouse.
---@return vector_2 delta # The delta of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L205)
function quiver.input.mouse.get_delta() end

---Set the current shift of the mouse.
---@param shift vector_2 # The shift of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L221)
function quiver.input.mouse.set_shift(shift) end

---Set the current scale of the mouse.
---@param scale vector_2 # The scale of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L238)
function quiver.input.mouse.set_scale(scale) end

---Set the current cursor of the mouse.
---@param cursor cursor_mouse # The cursor of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L255)
function quiver.input.mouse.set_cursor(cursor) end

---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return vector_2 delta # The delta of the mouse wheel.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L277)
function quiver.input.mouse.get_wheel() end

---Get the state of an input (up).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L293)
function quiver.input.mouse.get_up(mouse) end

---Get the state of an input (down).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L310)
function quiver.input.mouse.get_down(mouse) end

---Get the state of an input (press).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L327)
function quiver.input.mouse.get_press(mouse) end

---Get the state of an input (release).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L344)
function quiver.input.mouse.get_release(mouse) end

---Get the state of an input (up).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L363)
function quiver.input.board.get_up(board) end

---Get the state of an input (down).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L380)
function quiver.input.board.get_down(board) end

---Get the state of an input (press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L397)
function quiver.input.board.get_press(board) end

---Get the state of an input (release).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L414)
function quiver.input.board.get_release(board) end

---Get the state of a pad.
---@param index number # The index of the pad to check for.
---@return boolean state # The state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L436)
function quiver.input.pad.get_state(index) end

---Get the name of a pad.
---@param index number # The index of the pad to check for.
---@return string name # The name of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L452)
function quiver.input.pad.get_name(index) end

---Get the last pad button press.
---@return input_pad input # The last pad button press.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L468)
function quiver.input.pad.get_queue() end

---Get the axis count of a pad.
---@param index number # The index of the pad to check for.
---@return number axis_count # The axis count of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L484)
function quiver.input.pad.get_axis_count(index) end

---Get the axis state of a pad.
---@param index number # The index of the pad to check for.
---@param axis number # The axis of the pad to check for.
---@return number axis_state # The axis state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L501)
function quiver.input.pad.get_axis_state(index,axis) end

---Get the state of an input (up).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L514)
function quiver.input.pad.get_up(pad) end

---Get the state of an input (down).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L531)
function quiver.input.pad.get_down(pad) end

---Get the state of an input (press).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L548)
function quiver.input.pad.get_press(pad) end

---Get the state of an input (release).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L565)
function quiver.input.pad.get_release(pad) end

---The Rapier API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L11)
---@class quiver.rapier
quiver.rapier = {}

---An unique handle for a Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L25)
---@class rapier
rapier = {}

---Create a new Rapier simulation.
---@return rapier rapier # Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L52)
function quiver.rapier.new() end

---Cast a ray.
---@param ray ray # 
---@param time number # 
---@param filter table # 
---@return boolean pick # 
---@return number time # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L77)
function rapier:cast_ray(ray,time,filter) end

---Set the collider shape.
---@param collider table # Collider.
---@param shape vector_3 # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L112)
function rapier:collider_set_shape(collider,shape) end

---Create a kinematic character controller.
---@return table controller # Controller.
---@return table collider # Collider.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L142)
function rapier:character_controller() end

---Move a kinematic character controller.
---@param controller table # Controller.
---@param collider table # Collider.
---@param velocity vector_3 # Velocity.
---@param time_step number # Time step.
---@return vector_3 translation # Translation.
---@return boolean floor # Currently on floor.
---@return boolean slide # Currently on slide.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L180)
function rapier:move_character_controller(controller,collider,velocity,time_step) end

---Create a convex mesh collider out of a point cloud.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L226)
function rapier:collider_convex_mesh() end

---Step the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L244)
function rapier:step() end

---Render the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L271)
function rapier:debug_render() end

---The texture API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L14)
---@class quiver.texture
quiver.texture = {}

---An unique handle for a texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L77)
---@class texture
---@field shape vector_2 # Shape of the texture.
texture = {}

---Draw a texture.
---@param point vector_2 # 
---@param angle number # 
---@param scale number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L100)
function texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # 
---@param box_b box_2 # 
---@param point vector_2 # 
---@param angle number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L121)
function texture:draw_pro(box_a,box_b,point,angle,color) end

---Create a new texture resource.
---@param path string # Path to texture file.
---@return texture texture # Texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L155)
function quiver.texture.new(path) end

---The model API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L10)
---@class quiver.model
quiver.model = {}

---An unique handle for a model in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L35)
---@class model
---@field mesh_count number # Mesh count.
---@field bone_count number # Bone count.
---@field material_count number # Material count.
model = {}

---Create a new Model resource.
---@param path string # Path to model file.
---@return model Model # Model resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L51)
function quiver.model.new(path) end

---Bind a texture to the model.
---@param texture texture # Texture to bind to model.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L86)
function model:bind(texture) end

---Draw the model.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L102)
function model:draw() end

---Get the vertex data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Vector3 table.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L120)
function model:mesh_vertex(index) end

---The drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L23)
---@class quiver.draw
quiver.draw = {}

---Initialize drawing to the screen.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L51)
function quiver.draw.begin(call) end

---Initialize drawing (blend mode) to the screen.
---@param call function # The draw code.
---@param mode function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L74)
function quiver.draw.begin_blend(call,mode) end

---Initialize drawing (scissor mode) to the screen.
---@param call function # The draw code.
---@param view box_2 # The clip test region.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L96)
function quiver.draw.begin_scissor(call,view) end

---Get a ray for a 2D screen-space point.
---@param camera camera_3d # The current camera.
---@param point vector_2 # The screen-space point.
---@param shape vector_2 # The size of the view-port.
---@return ray ray # The 3D ray, beginning at the screen-space point, in 3D space.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L129)
function quiver.draw.get_screen_to_world_3d(camera,point,shape) end

---Get a 2D screen-space point for a 3D world-space point.
---@param camera camera_3d # The current camera.
---@param point vector_3 # The world-space point.
---@param shape vector_2 # The size of the view-port.
---@return vector_2 point # The 2D screen-space point.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L164)
function quiver.draw.get_world_to_screen_3d(camera,point,shape) end

---Get a world-space point for a 2D screen-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The screen-space point.
---@return vector_2 point # The 2D world-space point.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L198)
function quiver.draw.get_screen_to_world_2d(camera,point) end

---Get a screen-space point for a 2D world-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The world-space point.
---@return vector_2 point # The 2D screen-space point.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L226)
function quiver.draw.get_world_to_screen_2d(camera,point) end

---Clear the screen with a color.
---@param color color # The color to use for clearing.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L249)
function quiver.draw.clear(color) end

---The 2D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L265)
---@class quiver.draw_2d
quiver.draw_2d = {}

---Initialize the 2D draw mode.
---@param call function # The draw code.
---@param camera camera_2d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L291)
function quiver.draw_2d.begin(call,camera) end

---Draw 2D box.
---@param shape box_2 # The shape of the box.
---@param point vector_2 # The point of the box.
---@param angle number # The angle of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L316)
function quiver.draw_2d.draw_box_2(shape,point,angle,color) end

---Draw text.
---@param label string # The label of the text.
---@param point vector_2 # The point of the text.
---@param scale number # The angle of the text.
---@param color color # The color of the text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L342)
function quiver.draw_2d.draw_text(label,point,scale,color) end

---Draw a circle.
---@param point vector_2 # 
---@param radius number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L374)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L402)
function quiver.draw_2d.draw_circle_sector(point,radius,begin_angle,close_angle,segment_count,color) end

---The 3D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L436)
---@class quiver.draw_3d
quiver.draw_3d = {}

---Initialize the 3D draw mode.
---@param call function # The draw code.
---@param camera camera_3d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L462)
function quiver.draw_3d.begin(call,camera) end

---Draw a grid.
---@param slice number # The slice count of the grid.
---@param space number # The space shift of the grid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L485)
function quiver.draw_3d.draw_grid(slice,space) end

---Draw a cube.
---@param point vector_3 # The point of the cube.
---@param shape vector_3 # The shape of the cube.
---@param color color # The color of the cube.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L503)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a ball.
---@param point vector_3 # The point of the ball.
---@param shape number # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L528)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a 3D box.
---@param shape box_3 # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L548)
function quiver.draw_3d.draw_box_3(shape,color) end

---The general API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L10)
---@class quiver.general
quiver.general = {}

---Load the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L33)
function quiver.general.load() end

---Exit the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L37)
function quiver.general.exit() end

---Set a key to exit Quiver.
---@param key input_board # Key to exit Quiver with.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L47)
function quiver.general.set_exit_key(key) end

---Get the current time. Will count up since the initialization of the window.
---@return number time # Current time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L69)
function quiver.general.get_time() end

---Get the current frame time.
---@return number frame_time # Current frame time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L82)
function quiver.general.get_frame_time() end

---Get the current frame rate.
---@return number frame_rate # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L95)
function quiver.general.get_frame_rate() end

---set the current frame rate.
---@param frame_rate number # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L108)
function quiver.general.set_frame_rate(frame_rate) end

---The font API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L10)
---@class quiver.font
quiver.font = {}

---An unique handle to a font in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L26)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L45)
function font.draw(label,point,scale,space,color) end

---Create a new font resource.
---@param path string # Path to font file.
---@return font font # Font resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L82)
function quiver.font.new(path) end

---The window API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L10)
---@class quiver.window
quiver.window = {}

---Set the window to full-screen mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L46)
function quiver.window.set_fullscreen() end

---Set the window to border-less mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L56)
function quiver.window.set_borderless() end

---Minimize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L66)
function quiver.window.set_minimize() end

---Maximize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L76)
function quiver.window.set_maximize() end

---Focus the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L86)
function quiver.window.set_focus() end

---Restore the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L96)
function quiver.window.set_restore() end

---Set the window name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L106)
function quiver.window.set_name() end

---Set the window monitor.
---@param index number # Index of monitor to move window to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L125)
function quiver.window.set_monitor(index) end

---Set the current window shape.
---@param shape vector_2 # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L142)
function quiver.window.set_shape(shape) end

---Set the minimum window shape.
---@param shape vector_2 # Minimum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L161)
function quiver.window.set_shape_min(shape) end

---Set the maximum window shape.
---@param shape vector_2 # Maximum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L180)
function quiver.window.set_shape_max(shape) end

---Set the window alpha.
---@param alpha number # Alpha of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L199)
function quiver.window.set_alpha(alpha) end

---Set the window point.
---@param point vector_2 # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L216)
function quiver.window.set_point(point) end

---Get the state of the window (full-screen).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L237)
function quiver.window.get_fullscreen() end

---Get the state of the window (minimize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L251)
function quiver.window.get_minimize() end

---Get the state of the window (maximize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L265)
function quiver.window.get_maximize() end

---Get the state of the window (focus).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L279)
function quiver.window.get_focus() end

---Get the state of the window (resize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L293)
function quiver.window.get_resize() end

---Get the state of the window (hidden).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L307)
function quiver.window.get_hidden() end

---Get the shape of the window.
---@return vector_2 shape # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L321)
function quiver.window.get_shape() end

---Get the point of the window.
---@return vector_2 point # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L340)
function quiver.window.get_point() end

---Get the DPI scale of the window.
---@return number scale # Scale of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L358)
function quiver.window.get_scale() end

---Get if the window should close.
---@return boolean close # True if the window should close.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L376)
function quiver.window.get_close() end

---The music API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L10)
---@class quiver.music
quiver.music = {}

---An unique handle for music in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L26)
---@class music
music = {}

---Create a new music resource.
---@param path string # Path to music file.
---@return music music # Music resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L42)
function quiver.music.new(path) end

---Play the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L74)
function music:play() end

---Check if music is currently playing.
---@return boolean state # State of the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L89)
function music:get_playing() end

---Stop the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L96)
function music:stop() end

---Pause the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L104)
function music:pause() end

---Resume the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L112)
function music:resume() end

---Set volume for the music. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L127)
function music:set_volume(volume) end

---Set pitch for the music.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L142)
function music:set_pitch(pitch) end

---Set pan for the music. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L157)
function music:set_pan(pan) end

---Update the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L165)
function music:update() end

---Set position for the music.
---@param position number # Current position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L180)
function music:set_position(position) end

---Get time length for the music.
---@return number length # Time length.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L195)
function music:get_length() end

---Get time played for the music.
---@return number played # Time played.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L209)
function music:get_played() end

