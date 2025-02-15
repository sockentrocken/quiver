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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L31)
---@class quiver.sound
quiver.sound = {}

---An unique handle for sound in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L47)
---@class sound
sound = {}

---Create a new sound resource.
---@param path string # Path to sound file.
---@return sound sound # Sound resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L63)
function quiver.sound.new(path) end

---Play the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L96)
function sound:play() end

---Check if sound is currently playing.
---@return boolean state # State of the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L111)
function sound:get_playing() end

---Stop the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L118)
function sound:stop() end

---Pause the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L126)
function sound:pause() end

---Resume the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L134)
function sound:resume() end

---Set volume for the sound. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L149)
function sound:set_volume(volume) end

---Set pitch for the sound.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L164)
function sound:set_pitch(pitch) end

---Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L179)
function sound:set_pan(pan) end

---The input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L27)
---@class quiver.input
quiver.input = {}

---The board input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L39)
---@class quiver.input.board
quiver.input.board = {}

---The mouse input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L57)
---@class quiver.input.mouse
quiver.input.mouse = {}

---The pad input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L82)
---@class quiver.input.pad
quiver.input.pad = {}

---Set the clipboard text.
---@param text string # Clipboard text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L124)
function quiver.input.board.set_clipboard_text(text) end

---Get the clipboard text.
---@return string text # Clipboard text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L143)
function quiver.input.board.get_clipboard_text() end

---Get the last unicode glyph in the queue.
---@return number key_code # Key-code. If 0, queue is empty.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L163)
function quiver.input.board.get_key_code_queue() end

---Get the last unicode glyph in the queue.
---@return number uni_code # Uni-code. If 0, queue is empty.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L177)
function quiver.input.board.get_uni_code_queue() end

---Get the name of a given key.
---@param board input_board # The board button to get a name for.
---@return string name # The name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L194)
function quiver.input.board.get_name(board) end

---Get the state of an input (up).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L214)
function quiver.input.board.get_up(board) end

---Get the state of an input (down).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L228)
function quiver.input.board.get_down(board) end

---Get the state of an input (press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L242)
function quiver.input.board.get_press(board) end

---Get the state of an input (repeat-press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L256)
function quiver.input.board.get_press_repeat(board) end

---Get the state of an input (release).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L270)
function quiver.input.board.get_release(board) end

---Set the active state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L286)
function quiver.input.mouse.set_active(state) end

---Set the hidden state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L308)
function quiver.input.mouse.set_hidden(state) end

---Get the hidden state of the mouse.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L330)
function quiver.input.mouse.get_hidden() end

---Check if the mouse is currently over the screen.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L344)
function quiver.input.mouse.get_screen() end

---Get the current point of the mouse.
---@return number point_x # The point of the mouse (X).
---@return number point_y # The point of the mouse (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L359)
function quiver.input.mouse.get_point() end

---Set the current point of the mouse.
---@param point vector_2 # The point of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L376)
function quiver.input.mouse.set_point(point) end

---Get the current delta (i.e. mouse movement) of the mouse.
---@return number delta_x # The delta of the mouse (X).
---@return number delta_y # The delta of the mouse (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L395)
function quiver.input.mouse.get_delta() end

---Set the current shift of the mouse.
---@param shift vector_2 # The shift of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L412)
function quiver.input.mouse.set_shift(shift) end

---Set the current scale of the mouse.
---@param scale vector_2 # The scale of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L430)
function quiver.input.mouse.set_scale(scale) end

---Set the current cursor of the mouse.
---@param cursor cursor_mouse # The cursor of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L448)
function quiver.input.mouse.set_cursor(cursor) end

---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return number delta_x # The delta of the mouse wheel (X).
---@return number delta_y # The delta of the mouse wheel (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L466)
function quiver.input.mouse.get_wheel() end

---Get the state of an input (up).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L483)
function quiver.input.mouse.get_up(mouse) end

---Get the state of an input (down).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L497)
function quiver.input.mouse.get_down(mouse) end

---Get the state of an input (press).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L511)
function quiver.input.mouse.get_press(mouse) end

---Get the state of an input (release).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L525)
function quiver.input.mouse.get_release(mouse) end

---Get the state of a pad.
---@param index number # The index of the pad to check for.
---@return boolean state # The state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L544)
function quiver.input.pad.get_state(index) end

---Get the name of a pad.
---@param index number # The index of the pad to check for.
---@return string name # The name of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L561)
function quiver.input.pad.get_name(index) end

---Get the state of an input (press).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L581)
function quiver.input.pad.get_press(pad) end

---Get the state of an input (down).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L595)
function quiver.input.pad.get_down(pad) end

---Get the state of an input (release).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L609)
function quiver.input.pad.get_release(pad) end

---Get the state of an input (up).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L623)
function quiver.input.pad.get_up(pad) end

---Get the last pad button press.
---@return input_pad input # The last pad button press.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L637)
function quiver.input.pad.get_queue() end

---Get the axis count of a pad.
---@param index number # The index of the pad to check for.
---@return number axis_count # The axis count of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L654)
function quiver.input.pad.get_axis_count(index) end

---Get the axis state of a pad.
---@param index number # The index of the pad to check for.
---@param axis number # The axis of the pad to check for.
---@return number axis_state # The axis state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L672)
function quiver.input.pad.get_axis_state(index,axis) end

---Set the rumble of a pad.
---@param index number # The index of the pad to rumble.
---@param motor_a number # The intensity of the L. rumble motor.
---@param motor_b number # The intensity of the R. rumble motor.
---@param duration number # The duration of the rumble.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L689)
function quiver.input.pad.set_rumble(index,motor_a,motor_b,duration) end

---The Rapier API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L35)
---@class quiver.rapier
quiver.rapier = {}

---An unique handle for a Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L49)
---@class rapier
rapier = {}

---Create a new Rapier simulation.
---@return rapier rapier # Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L77)
function quiver.rapier.new() end

---Cast a ray.
---@param ray ray # Ray to cast.
---@param length number # Ray length.
---@param solid boolean # 
---@param exclude_rigid table # 
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L125)
function rapier:cast_ray(ray,length,solid,exclude_rigid) end

---Cast a ray, and also get the normal information..
---@param ray ray # Ray to cast.
---@param length number # Ray length.
---@param solid boolean # 
---@param exclude_rigid table # 
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L183)
function rapier:cast_ray_normal(ray,length,solid,exclude_rigid) end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L237)
function rapier:test_intersect_cuboid_cuboid() end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L281)
function rapier:test_intersect_cuboid() end

---Get the shape of a collider (cuboid).
---@param collider table # Collider handle.
---@return number half_shape_x # Half-shape of the cuboid. (X).
---@return number half_shape_y # Half-shape of the cuboid. (Y).
---@return number half_shape_z # Half-shape of the cuboid. (Z).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L352)
function rapier:get_collider_shape_cuboid(collider) end

---Set the shape of a collider (cuboid).
---@param collider table # Collider handle.
---@param half_shape vector_3 # Half-shape of cuboid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L388)
function rapier:set_collider_shape_cuboid(collider,half_shape) end

---Get the parent of a collider.
---@param collider table # Collider handle.
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L429)
function rapier:get_collider_parent(collider) end

---Get the position of a collider.
---@param collider table # Collider handle.
---@return number position_x # Collider position (X).
---@return number position_y # Collider position (Y).
---@return number position_z # Collider position (Z).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L460)
function rapier:get_collider_position(collider) end

---Set the position of a collider.
---@param collider table # Collider handle.
---@param position vector_3 # Collider position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L487)
function rapier:set_collider_position(collider,position) end

---Set the rotation of a collider.
---@param collider table # Collider handle.
---@param rotation vector_3 # Collider rotation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L515)
function rapier:set_collider_rotation(collider,rotation) end

---Set the sensor state of a collider.
---@param collider table # Collider handle.
---@param sensor boolean # Collider sensor state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L544)
function rapier:set_collider_sensor(collider,sensor) end

---Remove a collider.
---@param collider table # Collider handle.
---@param wake_parent boolean # Whether or not to wake up the rigid body parent this collider is bound to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L573)
function rapier:collider_remove(collider,wake_parent) end

---Remove a rigid body.
---@param rigid_body table # Rigid body handle.
---@param remove_collider boolean # Whether or not to remove every collider this rigid body is bound to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L600)
function rapier:rigid_body_remove(rigid_body,remove_collider) end

---Create a character controller.
---@return table character_controller # Character controller.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L628)
function rapier:character_controller() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L639)
function rapier:set_character_controller_up_vector() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L656)
function rapier:set_character_controller_slope() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L673)
function rapier:set_character_auto_step() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L720)
function rapier:set_character_snap_ground() end

---Move a character controller.
---@param step number # 
---@param character table # 
---@param collider table # 
---@param translation vector_3 # 
---@return number movement_x # Translation point (X).
---@return number movement_y # Translation point (Y).
---@return number movement_z # Translation point (Z).
---@return boolean floor # Currently on floor.
---@return boolean slope # Currently on slope.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L756)
function rapier:character_controller_move(step,character,collider,translation) end

---Create a rigid body.
---@param kind rigid_body_kind # Rigid body kind.
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L805)
function rapier:rigid_body(kind) end

---Get the user data of a rigid_body.
---@param rigid_body userdata # Rigid body handle.
---@return number user_data # Rigid body user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L829)
function rapier:get_rigid_body_user_data(rigid_body) end

---Set the user data of a rigid_body.
---@param rigid_body userdata # Rigid body handle.
---@param user_data number # Rigid body user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L855)
function rapier:set_rigid_body_user_data(rigid_body,user_data) end

---Set the position of a rigid_body.
---@param rigid_body userdata # rigid_body handle.
---@param position vector_3 # rigid_body position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L882)
function rapier:set_rigid_body_position(rigid_body,position) end

---Set the rotation of a rigid_body.
---@param rigid_body table # rigid_body handle.
---@param rotation vector_3 # rigid_body rotation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L911)
function rapier:set_rigid_body_rotation(rigid_body,rotation) end

---Get the user data of a collider.
---@param collider userdata # Collider handle.
---@return number user_data # Collider user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L946)
function rapier:get_collider_user_data(collider) end

---Set the user data of a collider.
---@param collider userdata # Collider handle.
---@param user_data number # Collider user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L969)
function rapier:set_collider_user_data(collider,user_data) end

---Create a collider builder (cuboid).
---@param half_shape vector_3 # Half-shape of cuboid.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1000)
function rapier:collider_builder_cuboid(half_shape) end

---Create a collider builder (tri-mesh).
---@param point_table table # The point array table.
---@param index_table table # The index array table.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1027)
function rapier:collider_builder_tri_mesh(point_table,index_table) end

---Create a collider builder (convex hull).
---@param vector_table table # A vector_3 vertex array table.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1075)
function rapier:collider_builder_convex_hull(vector_table) end

---Step the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1100)
function rapier:step() end

---Render the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1138)
function rapier:debug_render() end

---The texture API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L37)
---@class quiver.texture
quiver.texture = {}

---An unique handle for a texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L159)
---@class texture
---@field shape_x number # Shape of the texture (X).
---@field shape_y number # Shape of the texture (Y).
texture = {}

---Set the mipmap for a texture.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L176)
function texture:set_mipmap() end

---Set the filter for a texture.
---@param filter texture_filter # Texture filter.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L193)
function texture:set_filter(filter) end

---Set the wrap for a texture.
---@param wrap texture_wrap # Texture wrap.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L210)
function texture:set_wrap(wrap) end

---Draw a texture.
---@param point vector_2 # 
---@param angle number # 
---@param scale number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L230)
function texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # 
---@param box_b box_2 # 
---@param point vector_2 # 
---@param angle number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L251)
function texture:draw_pro(box_a,box_b,point,angle,color) end

---Draw a billboard texture.
---@param camera camera_3d # 
---@param point vector_3 # 
---@param scale number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L282)
function texture:draw_billboard(camera,point,scale,color) end

---Draw a billboard texture (pro).
---@param camera camera_3d # 
---@param source box_3 # 
---@param point vector_3 # 
---@param up vector_3 # 
---@param scale vector_2 # 
---@param origin vector_2 # 
---@param angle number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L311)
function texture:draw_billboard_pro(camera,source,point,up,scale,origin,angle,color) end

---Create a new texture resource.
---@param path string # Path to texture file.
---@return texture texture # Texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L350)
function quiver.texture.new(path) end

---An unique handle for a render texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L387)
---@class render_texture
---@field shape_x number # Shape of the texture (X).
---@field shape_y number # Shape of the texture (Y).
render_texture = {}

---Initialize drawing to the render texture.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L407)
function render_texture:begin(call) end

---Draw a texture.
---@param point vector_2 # 
---@param angle number # 
---@param scale number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L432)
function render_texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # 
---@param box_b box_2 # 
---@param point vector_2 # 
---@param angle number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L456)
function render_texture:draw_pro(box_a,box_b,point,angle,color) end

---Create a new render texture resource.
---@param shape vector_2 # 
---@return render_texture render_texture # Render texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L490)
function quiver.render_texture.new(shape) end

---The ZIP API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L31)
---@class quiver.zip
quiver.zip = {}

---An unique handle to a ZIP in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L45)
---@class zip
zip = {}

---Get a file from a ZIP file as binary data.
---@param path string # Path to file in ZIP file.
---@return table data # Binary data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L64)
function zip:get_binary(path) end

---Get a file from a ZIP file as string data.
---@param path string # Path to file in ZIP file.
---@return string data # String data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L88)
function zip:get_string(path) end

---Create a new ZIP resource.
---@param path string # Path to ZIP file.
---@return zip zip # ZIP resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L115)
function quiver.zip.new(path) end

---The model API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L33)
---@class quiver.model
quiver.model = {}

---An unique handle for a model in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L67)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L88)
function quiver.model.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L126)
function model:insert_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L141)
function model:remove_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L153)
function model:clear_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L166)
function model:set_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L188)
function model:set_transform_list_batch() end

---Bind a texture to the model.
---@param index number # 
---@param which number # 
---@param texture texture # Texture to bind to model.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L216)
function model:bind(index,which,texture) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L251)
function model:draw_mesh() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L279)
function model:draw_mesh_instance() end

---Draw the model.
---@param point vector_3 # 
---@param scale number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L305)
function model:draw(point,scale,color) end

---Draw the model (wire-frame).
---@param point vector_3 # 
---@param scale number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L328)
function model:draw_wire(point,scale,color) end

---Draw the model with a transformation.
---@param point vector_3 # 
---@param angle vector_3 # 
---@param scale vector_3 # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L352)
function model:draw_transform(point,angle,scale,color) end

---
---@return number min_x # Minimum vector. (X)
---@return number min_y # Minimum vector. (Y)
---@return number min_z # Minimum vector. (Z)
---@return number max_x # Maximum vector. (X)
---@return number max_y # Maximum vector. (Y)
---@return number max_z # Maximum vector. (Z)
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L385)
function model:get_box_3() end

---Get the vertex data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Vector3 table.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L410)
function model:mesh_vertex(index) end

---Get the index data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Number table.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L428)
function model:mesh_index(index) end

---Get the triangle count of a specific mesh in the model.
---@param index number # Index of mesh.
---@return number count # Triangle count.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L457)
function model:mesh_triangle_count(index) end

---An unique handle for a model animation in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L473)
---@class model_animation
model_animation = {}

---Create a new ModelAnimation resource.
---@param path string # Path to model file.
---@return model_animation model_animation # ModelAnimation resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L489)
function quiver.model_animation.new(path) end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L536)
function model_animation:get_bone_info() end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L554)
function model_animation:get_bone_info() end

---Update model with new model animation data.
---@param model model # 
---@param frame number # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L577)
function model_animation:update(model,frame) end

---The drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L42)
---@class quiver.draw
quiver.draw = {}

---Initialize drawing to the screen.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L66)
function quiver.draw.begin(call) end

---Initialize drawing (blend mode) to the screen.
---@param call function # The draw code.
---@param mode function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L89)
function quiver.draw.begin_blend(call,mode) end

---Initialize drawing (scissor mode) to the screen.
---@param call function # The draw code.
---@param view box_2 # The clip test region.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L111)
function quiver.draw.begin_scissor(call,view) end

---Clear the screen with a color.
---@param color color # The color to use for clearing.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L138)
function quiver.draw.clear(color) end

---The 3D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L154)
---@class quiver.draw_3d
quiver.draw_3d = {}

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L189)
function quiver.draw_3d.get_matrix_projection() end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L209)
function quiver.draw_3d.get_matrix_model_view() end

---Update the 3D camera (pro).
---@param camera camera_3d # The camera to update.
---@param position vector_3 # 
---@param rotation vector_3 # 
---@param zoom number # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L235)
function quiver.draw_3d.update_camera_pro(camera,position,rotation,zoom) end

---Get a ray for a 2D screen-space point.
---@param camera camera_3d # The current camera.
---@param point vector_2 # The screen-space point.
---@param shape vector_2 # The size of the view-port.
---@return number position_x # The 3D ray position. (X).
---@return number position_y # The 3D ray position. (Y).
---@return number position_z # The 3D ray position. (Z).
---@return number direction_x # The 3D ray direction. (X).
---@return number direction_y # The 3D ray direction. (Y).
---@return number direction_z # The 3D ray direction. (Z).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L284)
function quiver.draw_3d.get_screen_to_world(camera,point,shape) end

---Get a 2D screen-space point for a 3D world-space point.
---@param camera camera_3d # The current camera.
---@param point vector_3 # The world-space point.
---@param shape vector_2 # The size of the view-port.
---@return number point_x # The 2D screen-space point (X).
---@return number point_y # The 2D screen-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L327)
function quiver.draw_3d.get_world_to_screen(camera,point,shape) end

---Initialize the 3D draw mode.
---@param call function # The draw code.
---@param camera camera_3d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L358)
function quiver.draw_3d.begin(call,camera) end

---Draw a grid.
---@param slice number # The slice count of the grid.
---@param space number # The space shift of the grid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L381)
function quiver.draw_3d.draw_grid(slice,space) end

---Draw a cube.
---@param point vector_3 # The point of the cube.
---@param shape vector_3 # The shape of the cube.
---@param color color # The color of the cube.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L399)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a ball.
---@param point vector_3 # The point of the ball.
---@param shape number # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L424)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a 3D box.
---@param shape box_3 # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L444)
function quiver.draw_3d.draw_box_3(shape,color) end

---Draw a ray.
---@param ray ray # The ray.
---@param color color # The color of the ray.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L464)
function quiver.draw_3d.draw_ray(ray,color) end

---Draw a line.
---@param point_a vector_3 # The point A of the line.
---@param point_b vector_3 # The point B of the line.
---@param color color # The color of the line.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L485)
function quiver.draw_3d.draw_line(point_a,point_b,color) end

---Set the current state of backface culling.
---@param state boolean # The new state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L508)
function quiver.draw_3d.set_backface_cull(state) end

---
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L529)
function quiver.draw.begin_quad(call) end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L556)
function quiver.draw.draw_quad_color() end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L571)
function quiver.draw.draw_quad_normal() end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L586)
function quiver.draw.draw_quad_coordinate() end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L601)
function quiver.draw.draw_quad_vertex() end

---The 2D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L616)
---@class quiver.draw_2d
quiver.draw_2d = {}

---Get a world-space point for a 2D screen-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The screen-space point.
---@return number point_x # The 2D world-space point (X).
---@return number point_y # The 2D world-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L658)
function quiver.draw_2d.get_screen_to_world(camera,point) end

---Get a screen-space point for a 2D world-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The world-space point.
---@return number point_x # The 2D screen-space point (X).
---@return number point_y # The 2D screen-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L687)
function quiver.draw_2d.get_world_to_screen(camera,point) end

---Initialize the 2D draw mode.
---@param call function # The draw code.
---@param camera camera_2d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L712)
function quiver.draw_2d.begin(call,camera) end

---Draw pixel.
---@param point vector_2 # The point of the pixel.
---@param color color # The color of the pixel.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L736)
function quiver.draw_2d.draw_pixel(point,color) end

---Draw a line.
---@param point_a vector_2 # The point A of the line.
---@param point_b vector_2 # The point B of the line.
---@param thick number # The thickness of the line.
---@param color color # The color of the line.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L759)
function quiver.draw_2d.draw_line(point_a,point_b,thick,color) end

---Draw text.
---@param label string # The label of the text.
---@param point vector_2 # The point of the text.
---@param scale number # The angle of the text.
---@param color color # The color of the text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L786)
function quiver.draw_2d.draw_text(label,point,scale,color) end

---Draw a circle.
---@param point vector_2 # 
---@param radius number # 
---@param color color # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L818)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L846)
function quiver.draw_2d.draw_circle_sector(point,radius,begin_angle,close_angle,segment_count,color) end

---Draw 2D box.
---@param shape box_2 # The shape of the box.
---@param point vector_2 # The point of the box.
---@param angle number # The angle of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L886)
function quiver.draw_2d.draw_box_2(shape,point,angle,color) end

---Draw 2D box with a gradient (X-direction).
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A of the box.
---@param color_b color # The color B of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L912)
function quiver.draw_2d.draw_box_2_gradient_x(shape,color_a,color_b) end

---Draw 2D box with a gradient (Y-direction).
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A of the box.
---@param color_b color # The color B of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L945)
function quiver.draw_2d.draw_box_2_gradient_y(shape,color_a,color_b) end

---Draw 2D box with a 4-point gradient.
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A (T.L.) of the box.
---@param color_b color # The color B (B.L.) of the box.
---@param color_c color # The color C (T.R.) of the box.
---@param color_d color # The color D (B.R.) of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L980)
function quiver.draw_2d.draw_box_2_gradient(shape,color_a,color_b,color_c,color_d) end

---Draw 2D box (out-line).
---@param shape box_2 # The shape of the box.
---@param thick number # The thickness of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1020)
function quiver.draw_2d.draw_box_2_line(shape,thick,color) end

---Draw 2D box (round).
---@param shape box_2 # The shape of the box.
---@param round number # The roundness of the box.
---@param count number # The segment count of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1046)
function quiver.draw_2d.draw_box_2_round(shape,round,count,color) end

---Draw 2D box (out-line, round).
---@param shape box_2 # The shape of the box.
---@param round number # The roundness of the box.
---@param count number # The segment count of the box.
---@param thick number # The thickness of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1073)
function quiver.draw_2d.draw_box_2_line_round(shape,round,count,thick,color) end

---Draw 2D triangle.
---@param point_a vector_2 # The point A of the triangle.
---@param point_b vector_2 # The point B of the triangle.
---@param point_c vector_2 # The point C of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1099)
function quiver.draw_2d.draw_triangle(point_a,point_b,point_c,color) end

---Draw 2D triangle (out-line).
---@param point_a vector_2 # The point A of the triangle.
---@param point_b vector_2 # The point B of the triangle.
---@param point_c vector_2 # The point C of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1127)
function quiver.draw_2d.draw_triangle_line(point_a,point_b,point_c,color) end

---The file API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L31)
---@class quiver.file
quiver.file = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L61)
function quiver.file.to_binary() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L82)
function quiver.file.from_binary() end

---Get the data of a file, in string format.
---@param path string # Path to file.
---@return string data # File data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L109)
function quiver.file.get(path) end

---Set the data of a file.
---@param path string # Path to file.
---@param data string # Data to copy.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L134)
function quiver.file.set(path,data) end

---Check if a file does exist.
---@param path string # Path to file.
---@return boolean exist # True if file does exist, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L161)
function quiver.file.get_file_exist(path) end

---Check if a path does exist.
---@param path string # Path.
---@return boolean exist # True if path does exist, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L181)
function quiver.file.get_path_exist(path) end

---Check if a file's extension is the same as a given one.
---@param path string # Path to file.
---@param extension string # Extension. MUST include dot (.png, .wav, etc.).
---@return boolean check # True if file extension is the same as the given one, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L202)
function quiver.file.get_file_extension_check(path,extension) end

---Get the size of a file.
---@param path string # Path to file.
---@return number size # File size.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L223)
function quiver.file.get_file_size(path) end

---Get the extension of a file.
---@param path string # Path to file.
---@return string extension # File extension.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L243)
function quiver.file.get_file_extension(path) end

---Get the name of a file.
---@param path string # Path to file.
---@param extension boolean # File extension. If true, will return file name with the extension.
---@return string name # File name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L270)
function quiver.file.get_file_name(path,extension) end

---Get the current work path.
---@return string path # Work path.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L301)
function quiver.file.get_work_directory() end

---Get the current application path.
---@return string path # Application path.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L321)
function quiver.file.get_application_directory() end

---Scan a path.
---@param path string # Path to scan.
---@param filter string # OPTIONAL: Extension filter. If filter is 'DIR', will includ every directory in the result.
---@param recursive boolean # Recursive toggle. If true, recursively scan the directory.
---@return table list # File list.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L346)
function quiver.file.scan_path(path,filter,recursive) end

---The general API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L31)
---@class quiver.general
quiver.general = {}

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L64)
function quiver.general.set_log_level() end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L78)
function quiver.general.open_link() end

---Serialize a given Lua value as a JSON string.
---@param value any # Lua value to serialize.
---@return string value # The value, in string form.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L98)
function quiver.general.serialize(value) end

---Deserialize a given JSON string as a Lua value.
---@param value string # String to deserialize.
---@return any value # The value, in Lua value form.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L116)
function quiver.general.deserialize(value) end

---Load the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L125)
function quiver.general.load() end

---Exit the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L129)
function quiver.general.exit() end

---Set a key to exit Quiver.
---@param key input_board # Key to exit Quiver with.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L139)
function quiver.general.set_exit_key(key) end

---Get the current time. Will count up since the initialization of the window.
---@return number time # Current time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L155)
function quiver.general.get_time() end

---Get the current frame time.
---@return number frame_time # Current frame time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L168)
function quiver.general.get_frame_time() end

---Get the current frame rate.
---@return number frame_rate # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L181)
function quiver.general.get_frame_rate() end

---set the current frame rate.
---@param frame_rate number # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L194)
function quiver.general.set_frame_rate(frame_rate) end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L208)
function quiver.general.get_memory() end

---Get the current info manifest data.
---@return boolean safe # Safe mode.
---@return string path # Main path.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L223)
function quiver.general.get_info() end

---The shader API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L31)
---@class quiver.shader
quiver.shader = {}

---An unique handle for a shader in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L47)
---@class shader
shader = {}

---Create a new shader resource.
---@param v_path string # Path to .vs file.
---@param f_path string # Path to .fs file.
---@return shader shader # Shader resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L64)
function quiver.shader.new(v_path,f_path) end

---
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L113)
function shader:begin(call) end

---
---@param name string # 
---@return number location # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L138)
function shader:get_location_name(name) end

---
---@param name string # 
---@return number location # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L155)
function shader:get_location_attribute_name(name) end

---
---@param location number # 
---@return number location # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L172)
function shader:get_location(location) end

---
---@param location number # 
---@param value number # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L187)
function shader:set_location(location,value) end

---
---@param location number # 
---@param value number # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L206)
function shader:set_shader_integer(location,value) end

---
---@param location number # 
---@param value number # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L225)
function shader:set_shader_decimal(location,value) end

---
---@param location number # 
---@param value vector_3 # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L244)
function shader:set_shader_vector_3(location,value) end

---
---@param location number # 
---@param value vector_4 # 
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L264)
function shader:set_shader_vector_4(location,value) end

---The image API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L36)
---@class quiver.image
quiver.image = {}

---An unique handle for a image in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L58)
---@class image
---@field shape_x number # Shape of the image (X).
---@field shape_y number # Shape of the image (Y).
image = {}

---Create a new image resource.
---@param path string # Path to image file.
---@return image image # Image resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L83)
function quiver.image.new(path) end

---The font API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L31)
---@class quiver.font
quiver.font = {}

---Set the vertical space between each line-break.
---@param space number # Vertical space.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L54)
function quiver.font.set_text_line_space(space) end

---An unique handle to a font in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L67)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L87)
function font:draw(label,point,scale,space,color) end

---Measure the size of a given text on screen, with a given font.
---@param label string # Label of font to measure.
---@param scale number # Scale of font to measure.
---@param space number # Space of font to measure.
---@return number size_x # Size of text (X).
---@return number size_y # Size of text (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L125)
function font:measure_text(label,scale,space) end

---Create a new font resource.
---@param path string # Path to font file.
---@param size number # Size for font.
---@return font font # Font resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L154)
function quiver.font.new(path,size) end

---Create a new font resource (default font).
---@return font font # Font resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L181)
function quiver.font.new_default() end

---The window API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L27)
---@class quiver.window
quiver.window = {}

---Get if the window should close.
---@return boolean close # True if the window should close.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L115)
function quiver.window.get_close() end

---Get the state of the window (full-screen).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L129)
function quiver.window.get_fullscreen() end

---Get the state of the window (hidden).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L143)
function quiver.window.get_hidden() end

---Get the state of the window (minimize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L157)
function quiver.window.get_minimize() end

---Get the state of the window (maximize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L171)
function quiver.window.get_maximize() end

---Get the state of the window (focus).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L185)
function quiver.window.get_focus() end

---Get the state of the window (resize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L199)
function quiver.window.get_resize() end

---Get the state of a window flag.
---@param flag window_flag # Window flag.
---@return boolean state # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L216)
function quiver.window.get_state(flag) end

---Set the state of a window flag.
---@param flag window_flag # Window flag.
---@param state boolean # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L231)
function quiver.window.set_state(flag,state) end

---Set the window to full-screen mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L246)
function quiver.window.set_fullscreen() end

---Set the window to border-less mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L256)
function quiver.window.set_borderless() end

---Minimize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L266)
function quiver.window.set_minimize() end

---Maximize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L276)
function quiver.window.set_maximize() end

---Restore the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L286)
function quiver.window.set_restore() end

---Set the window icon.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L296)
function quiver.window.set_icon() end

---Set the window name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L315)
function quiver.window.set_name() end

---Set the window point.
---@param point vector_2 # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L334)
function quiver.window.set_point(point) end

---Set the window monitor.
---@param index number # Index of monitor to move window to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L353)
function quiver.window.set_screen(index) end

---Set the minimum window shape.
---@param shape vector_2 # Minimum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L370)
function quiver.window.set_shape_min(shape) end

---Set the maximum window shape.
---@param shape vector_2 # Maximum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L389)
function quiver.window.set_shape_max(shape) end

---Set the current window shape.
---@param shape vector_2 # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L408)
function quiver.window.set_shape(shape) end

---Set the window alpha.
---@param alpha number # Alpha of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L427)
function quiver.window.set_alpha(alpha) end

---Focus the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L437)
function quiver.window.set_focus() end

---Get the shape of the window.
---@return number shape_x # Shape of the window (X).
---@return number shape_y # Shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L455)
function quiver.window.get_shape() end

---Get the shape of the current render view.
---@return number shape_x # Shape of the render view (X).
---@return number shape_y # Shape of the render view (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L470)
function quiver.window.get_render_shape() end

---Get the available monitor amount.
---@return number count # Monitor count.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L484)
function quiver.window.get_screen_count() end

---Get the current active monitor, where the window is.
---@return number index # Current active monitor index.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L498)
function quiver.window.get_screen_focus() end

---Get the point of the given monitor.
---@param index number # Index of the monitor.
---@return number point_x # Point of the monitor (X).
---@return number point_y # Point of the monitor (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L516)
function quiver.window.get_screen_point(index) end

---Get the shape of the given monitor.
---@param index number # Index of the monitor.
---@return number shape_x # Shape of the window (X).
---@return number shape_y # Shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L537)
function quiver.window.get_screen_shape(index) end

---Get the physical shape of the given monitor.
---@param index number # Index of the monitor.
---@return number shape_x # Physical shape of the window (X).
---@return number shape_y # Physical shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L555)
function quiver.window.get_screen_shape_physical(index) end

---Get the refresh rate of the given monitor.
---@param index number # Index of the monitor.
---@return number rate # Refresh rate of the monitor.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L577)
function quiver.window.get_screen_rate(index) end

---Get the point of the window.
---@return number point_x # Point of the window (X).
---@return number point_y # Point of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L592)
function quiver.window.get_point() end

---Get the DPI scale of the window.
---@return number scale_x # Scale of the window (X).
---@return number scale_y # Scale of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L611)
function quiver.window.get_scale() end

---Get the name of the given monitor.
---@param index number # Index of the monitor.
---@return string name # Name of the monitor.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L632)
function quiver.window.get_screen_name(index) end

---The music API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L31)
---@class quiver.music
quiver.music = {}

---An unique handle for music in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L47)
---@class music
music = {}

---Create a new music resource.
---@param path string # Path to music file.
---@return music music # Music resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L63)
function quiver.music.new(path) end

---Play the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L96)
function music:play() end

---Check if music is currently playing.
---@return boolean state # State of the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L111)
function music:get_playing() end

---Stop the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L118)
function music:stop() end

---Pause the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L126)
function music:pause() end

---Resume the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L134)
function music:resume() end

---Set volume for the music. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L149)
function music:set_volume(volume) end

---Set pitch for the music.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L164)
function music:set_pitch(pitch) end

---Set pan for the music. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L179)
function music:set_pan(pan) end

---Update the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L187)
function music:update() end

---Set position for the music.
---@param position number # Current position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L202)
function music:set_position(position) end

---Get time length for the music.
---@return number length # Time length.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L217)
function music:get_length() end

---Get time played for the music.
---@return number played # Time played.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L231)
function music:get_played() end

