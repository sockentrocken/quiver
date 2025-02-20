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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L64)
---@class quiver.sound
quiver.sound = {}

---An unique handle for sound in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L81)
---@class sound
sound = {}

---Create a new sound resource.
---@param path string # Path to sound file.
---@param alias number # OPTIONAL: The total sound alias count to load for the sound.
---@return sound sound # Sound resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L100)
function quiver.sound.new(path,alias) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L133)
function quiver.sound.new_from_memory() end

---Create a sound alias.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L200)
function sound:create_alias() end

---Remove a sound alias.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L210)
function sound:remove_alias() end

---Clear every sound alias.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L224)
function sound:remove_alias() end

---Play the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L237)
function sound:play() end

---Check if sound is currently playing.
---@return boolean state # State of the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L261)
function sound:get_playing() end

---Stop the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L278)
function sound:stop() end

---Pause the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L295)
function sound:pause() end

---Resume the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L312)
function sound:resume() end

---Set volume for the sound. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L338)
function sound:set_volume(volume) end

---Set pitch for the sound.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L367)
function sound:set_pitch(pitch) end

---Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L396)
function sound:set_pan(pan) end

---The input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L62)
---@class quiver.input
quiver.input = {}

---The board input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L76)
---@class quiver.input.board
quiver.input.board = {}

---The mouse input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L94)
---@class quiver.input.mouse
quiver.input.mouse = {}

---The pad input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L119)
---@class quiver.input.pad
quiver.input.pad = {}

---Set the clipboard text.
---@param text string # Clipboard text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L161)
function quiver.input.board.set_clipboard_text(text) end

---Get the clipboard text.
---@return string text # Clipboard text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L180)
function quiver.input.board.get_clipboard_text() end

---Get the last unicode glyph in the queue.
---@return number key_code # Key-code. If 0, queue is empty.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L200)
function quiver.input.board.get_key_code_queue() end

---Get the last unicode glyph in the queue.
---@return number uni_code # Uni-code. If 0, queue is empty.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L214)
function quiver.input.board.get_uni_code_queue() end

---Get the name of a given key.
---@param board input_board # The board button to get a name for.
---@return string name # The name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L231)
function quiver.input.board.get_name(board) end

---Get the state of an input (up).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L251)
function quiver.input.board.get_up(board) end

---Get the state of an input (down).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L265)
function quiver.input.board.get_down(board) end

---Get the state of an input (press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L279)
function quiver.input.board.get_press(board) end

---Get the state of an input (repeat-press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L293)
function quiver.input.board.get_press_repeat(board) end

---Get the state of an input (release).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L307)
function quiver.input.board.get_release(board) end

---Set the active state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L323)
function quiver.input.mouse.set_active(state) end

---Set the hidden state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L345)
function quiver.input.mouse.set_hidden(state) end

---Get the hidden state of the mouse.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L367)
function quiver.input.mouse.get_hidden() end

---Check if the mouse is currently over the screen.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L381)
function quiver.input.mouse.get_screen() end

---Get the current point of the mouse.
---@return number point_x # The point of the mouse (X).
---@return number point_y # The point of the mouse (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L396)
function quiver.input.mouse.get_point() end

---Set the current point of the mouse.
---@param point vector_2 # The point of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L413)
function quiver.input.mouse.set_point(point) end

---Get the current delta (i.e. mouse movement) of the mouse.
---@return number delta_x # The delta of the mouse (X).
---@return number delta_y # The delta of the mouse (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L432)
function quiver.input.mouse.get_delta() end

---Set the current shift of the mouse.
---@param shift vector_2 # The shift of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L449)
function quiver.input.mouse.set_shift(shift) end

---Set the current scale of the mouse.
---@param scale vector_2 # The scale of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L467)
function quiver.input.mouse.set_scale(scale) end

---Set the current cursor of the mouse.
---@param cursor cursor_mouse # The cursor of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L485)
function quiver.input.mouse.set_cursor(cursor) end

---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return number delta_x # The delta of the mouse wheel (X).
---@return number delta_y # The delta of the mouse wheel (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L503)
function quiver.input.mouse.get_wheel() end

---Get the state of an input (up).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L520)
function quiver.input.mouse.get_up(mouse) end

---Get the state of an input (down).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L534)
function quiver.input.mouse.get_down(mouse) end

---Get the state of an input (press).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L548)
function quiver.input.mouse.get_press(mouse) end

---Get the state of an input (release).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L562)
function quiver.input.mouse.get_release(mouse) end

---Get the state of a pad.
---@param index number # The index of the pad to check for.
---@return boolean state # The state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L581)
function quiver.input.pad.get_state(index) end

---Get the name of a pad.
---@param index number # The index of the pad to check for.
---@return string name # The name of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L598)
function quiver.input.pad.get_name(index) end

---Get the state of an input (press).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L618)
function quiver.input.pad.get_press(pad) end

---Get the state of an input (down).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L632)
function quiver.input.pad.get_down(pad) end

---Get the state of an input (release).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L646)
function quiver.input.pad.get_release(pad) end

---Get the state of an input (up).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L660)
function quiver.input.pad.get_up(pad) end

---Get the last pad button press.
---@return input_pad input # The last pad button press.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L674)
function quiver.input.pad.get_queue() end

---Get the axis count of a pad.
---@param index number # The index of the pad to check for.
---@return number axis_count # The axis count of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L691)
function quiver.input.pad.get_axis_count(index) end

---Get the axis state of a pad.
---@param index number # The index of the pad to check for.
---@param axis number # The axis of the pad to check for.
---@return number axis_state # The axis state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L709)
function quiver.input.pad.get_axis_state(index,axis) end

---Set the rumble of a pad.
---@param index number # The index of the pad to rumble.
---@param motor_a number # The intensity of the L. rumble motor.
---@param motor_b number # The intensity of the R. rumble motor.
---@param duration number # The duration of the rumble.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L726)
function quiver.input.pad.set_rumble(index,motor_a,motor_b,duration) end

---The Rapier API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L68)
---@class quiver.rapier
quiver.rapier = {}

---An unique handle for a Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L82)
---@class rapier
rapier = {}

---Create a new Rapier simulation.
---@return rapier rapier # Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L110)
function quiver.rapier.new() end

---Cast a ray.
---@param ray ray # Ray to cast.
---@param length number # Ray length.
---@param solid boolean # TO-DO
---@param exclude_rigid table # TO-DO
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L158)
function rapier:cast_ray(ray,length,solid,exclude_rigid) end

---Cast a ray, and also get the normal information..
---@param ray ray # Ray to cast.
---@param length number # Ray length.
---@param solid boolean # TO-DO
---@param exclude_rigid table # TO-DO
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L216)
function rapier:cast_ray_normal(ray,length,solid,exclude_rigid) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L270)
function rapier:test_intersect_cuboid_cuboid() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L314)
function rapier:test_intersect_cuboid() end

---Get the shape of a collider (cuboid).
---@param collider table # Collider handle.
---@return number half_shape_x # Half-shape of the cuboid. (X).
---@return number half_shape_y # Half-shape of the cuboid. (Y).
---@return number half_shape_z # Half-shape of the cuboid. (Z).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L385)
function rapier:get_collider_shape_cuboid(collider) end

---Set the shape of a collider (cuboid).
---@param collider table # Collider handle.
---@param half_shape vector_3 # Half-shape of cuboid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L421)
function rapier:set_collider_shape_cuboid(collider,half_shape) end

---Get the parent of a collider.
---@param collider table # Collider handle.
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L462)
function rapier:get_collider_parent(collider) end

---Get the position of a collider.
---@param collider table # Collider handle.
---@return number position_x # Collider position (X).
---@return number position_y # Collider position (Y).
---@return number position_z # Collider position (Z).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L493)
function rapier:get_collider_position(collider) end

---Set the position of a collider.
---@param collider table # Collider handle.
---@param position vector_3 # Collider position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L520)
function rapier:set_collider_position(collider,position) end

---Set the rotation of a collider.
---@param collider table # Collider handle.
---@param rotation vector_3 # Collider rotation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L548)
function rapier:set_collider_rotation(collider,rotation) end

---Set the sensor state of a collider.
---@param collider table # Collider handle.
---@param sensor boolean # Collider sensor state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L577)
function rapier:set_collider_sensor(collider,sensor) end

---Remove a collider.
---@param collider table # Collider handle.
---@param wake_parent boolean # Whether or not to wake up the rigid body parent this collider is bound to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L606)
function rapier:collider_remove(collider,wake_parent) end

---Remove a rigid body.
---@param rigid_body table # Rigid body handle.
---@param remove_collider boolean # Whether or not to remove every collider this rigid body is bound to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L633)
function rapier:rigid_body_remove(rigid_body,remove_collider) end

---Create a character controller.
---@return table character_controller # Character controller.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L661)
function rapier:character_controller() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L672)
function rapier:set_character_controller_up_vector() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L689)
function rapier:set_character_controller_slope() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L706)
function rapier:set_character_auto_step() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L753)
function rapier:set_character_snap_ground() end

---Move a character controller.
---@param step number # TO-DO
---@param character table # TO-DO
---@param collider table # TO-DO
---@param translation vector_3 # TO-DO
---@return number movement_x # Translation point (X).
---@return number movement_y # Translation point (Y).
---@return number movement_z # Translation point (Z).
---@return boolean floor # Currently on floor.
---@return boolean slope # Currently on slope.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L789)
function rapier:character_controller_move(step,character,collider,translation) end

---Create a rigid body.
---@param kind rigid_body_kind # Rigid body kind.
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L838)
function rapier:rigid_body(kind) end

---Get the user data of a rigid_body.
---@param rigid_body userdata # Rigid body handle.
---@return number user_data # Rigid body user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L862)
function rapier:get_rigid_body_user_data(rigid_body) end

---Set the user data of a rigid_body.
---@param rigid_body userdata # Rigid body handle.
---@param user_data number # Rigid body user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L888)
function rapier:set_rigid_body_user_data(rigid_body,user_data) end

---Set the position of a rigid_body.
---@param rigid_body userdata # rigid_body handle.
---@param position vector_3 # rigid_body position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L915)
function rapier:set_rigid_body_position(rigid_body,position) end

---Set the rotation of a rigid_body.
---@param rigid_body table # rigid_body handle.
---@param rotation vector_3 # rigid_body rotation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L944)
function rapier:set_rigid_body_rotation(rigid_body,rotation) end

---Get the user data of a collider.
---@param collider userdata # Collider handle.
---@return number user_data # Collider user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L979)
function rapier:get_collider_user_data(collider) end

---Set the user data of a collider.
---@param collider userdata # Collider handle.
---@param user_data number # Collider user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1002)
function rapier:set_collider_user_data(collider,user_data) end

---Create a collider builder (cuboid).
---@param half_shape vector_3 # Half-shape of cuboid.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1033)
function rapier:collider_builder_cuboid(half_shape) end

---Create a collider builder (tri-mesh).
---@param point_table table # The point array table.
---@param index_table table # The index array table.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1060)
function rapier:collider_builder_tri_mesh(point_table,index_table) end

---Create a collider builder (convex hull).
---@param vector_table table # A vector_3 vertex array table.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1108)
function rapier:collider_builder_convex_hull(vector_table) end

---Step the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1133)
function rapier:step() end

---Render the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1171)
function rapier:debug_render() end

---The thread API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/thread.rs#L69)
---@class quiver.thread
quiver.thread = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/thread.rs#L89)
---@class thread
thread = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/thread.rs#L137)
---@class thread
thread = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/thread.rs#L173)
---@class thread
thread = {}

---The request API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/request.rs#L58)
---@class quiver.request
quiver.request = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/request.rs#L78)
function quiver.request.get() end

---The texture API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L70)
---@class quiver.texture
quiver.texture = {}

---An unique handle for a texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L192)
---@class texture
---@field shape_x number # Shape of the texture (X).
---@field shape_y number # Shape of the texture (Y).
texture = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L209)
function texture:to_image() end

---Set the mipmap for a texture.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L220)
function texture:set_mipmap() end

---Set the filter for a texture.
---@param filter texture_filter # Texture filter.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L237)
function texture:set_filter(filter) end

---Set the wrap for a texture.
---@param wrap texture_wrap # Texture wrap.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L254)
function texture:set_wrap(wrap) end

---Draw a texture.
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L274)
function texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # TO-DO
---@param box_b box_2 # TO-DO
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L295)
function texture:draw_pro(box_a,box_b,point,angle,color) end

---Draw a billboard texture.
---@param camera camera_3d # TO-DO
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L326)
function texture:draw_billboard(camera,point,scale,color) end

---Draw a billboard texture (pro).
---@param camera camera_3d # TO-DO
---@param source box_3 # TO-DO
---@param point vector_3 # TO-DO
---@param up vector_3 # TO-DO
---@param scale vector_2 # TO-DO
---@param origin vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L355)
function texture:draw_billboard_pro(camera,source,point,up,scale,origin,angle,color) end

---Create a new texture resource.
---@param path string # Path to texture file.
---@return texture texture # Texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L394)
function quiver.texture.new(path) end

---An unique handle for a render texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L445)
---@class render_texture
---@field shape_x number # Shape of the texture (X).
---@field shape_y number # Shape of the texture (Y).
render_texture = {}

---Initialize drawing to the render texture.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L465)
function render_texture:begin(call) end

---Draw a texture.
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L490)
function render_texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # TO-DO
---@param box_b box_2 # TO-DO
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L514)
function render_texture:draw_pro(box_a,box_b,point,angle,color) end

---Create a new render texture resource.
---@param shape vector_2 # TO-DO
---@return render_texture render_texture # Render texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L548)
function quiver.render_texture.new(shape) end

---The ZIP API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L63)
---@class quiver.zip
quiver.zip = {}

---An unique handle to a ZIP in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L77)
---@class zip
zip = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L90)
function zip:get_data_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L103)
function zip:get_file() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L123)
function zip:is_file() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L137)
function zip:is_path() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L151)
function zip:is_system_link() end

---Create a new ZIP resource.
---@param path string # Path to ZIP file.
---@return zip zip # ZIP resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L174)
function quiver.zip.new(path) end

---The model API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L66)
---@class quiver.model
quiver.model = {}

---An unique handle for a model in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L100)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L123)
function quiver.model.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L161)
function model:insert_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L176)
function model:remove_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L188)
function model:clear_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L201)
function model:set_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L223)
function model:set_transform_list_batch() end

---Bind a texture to the model.
---@param index number # TO-DO
---@param which number # TO-DO
---@param texture texture # Texture to bind to model.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L251)
function model:bind(index,which,texture) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L286)
function model:draw_mesh() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L314)
function model:draw_mesh_instance() end

---Draw the model.
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L340)
function model:draw(point,scale,color) end

---Draw the model (wire-frame).
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L363)
function model:draw_wire(point,scale,color) end

---Draw the model with a transformation.
---@param point vector_3 # TO-DO
---@param angle vector_3 # TO-DO
---@param scale vector_3 # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L387)
function model:draw_transform(point,angle,scale,color) end

---TO-DO
---@return number min_x # Minimum vector. (X)
---@return number min_y # Minimum vector. (Y)
---@return number min_z # Minimum vector. (Z)
---@return number max_x # Maximum vector. (X)
---@return number max_y # Maximum vector. (Y)
---@return number max_z # Maximum vector. (Z)
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L420)
function model:get_box_3() end

---Get the vertex data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Vector3 table.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L445)
function model:mesh_vertex(index) end

---Get the index data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Number table.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L463)
function model:mesh_index(index) end

---Get the triangle count of a specific mesh in the model.
---@param index number # Index of mesh.
---@return number count # Triangle count.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L492)
function model:mesh_triangle_count(index) end

---An unique handle for a model animation in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L508)
---@class model_animation
model_animation = {}

---Create a new ModelAnimation resource.
---@param path string # Path to model file.
---@return model_animation model_animation # ModelAnimation resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L526)
function quiver.model_animation.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L573)
function model_animation:get_bone_info() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L591)
function model_animation:get_bone_info() end

---Update model with new model animation data.
---@param model model # TO-DO
---@param frame number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L614)
function model_animation:update(model,frame) end

---The drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L75)
---@class quiver.draw
quiver.draw = {}

---Initialize drawing to the screen.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L102)
function quiver.draw.begin(call) end

---Initialize drawing (blend mode) to the screen.
---@param call function # The draw code.
---@param mode function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L125)
function quiver.draw.begin_blend(call,mode) end

---Initialize drawing (scissor mode) to the screen.
---@param call function # The draw code.
---@param view box_2 # The clip test region.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L147)
function quiver.draw.begin_scissor(call,view) end

---Clear the screen with a color.
---@param color color # The color to use for clearing.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L174)
function quiver.draw.clear(color) end

---The 3D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L190)
---@class quiver.draw_3d
quiver.draw_3d = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L225)
function quiver.draw_3d.get_matrix_projection() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L245)
function quiver.draw_3d.get_matrix_model_view() end

---Update the 3D camera (pro).
---@param camera camera_3d # The camera to update.
---@param position vector_3 # TO-DO
---@param rotation vector_3 # TO-DO
---@param zoom number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L271)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L320)
function quiver.draw_3d.get_screen_to_world(camera,point,shape) end

---Get a 2D screen-space point for a 3D world-space point.
---@param camera camera_3d # The current camera.
---@param point vector_3 # The world-space point.
---@param shape vector_2 # The size of the view-port.
---@return number point_x # The 2D screen-space point (X).
---@return number point_y # The 2D screen-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L363)
function quiver.draw_3d.get_world_to_screen(camera,point,shape) end

---Initialize the 3D draw mode.
---@param call function # The draw code.
---@param camera camera_3d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L394)
function quiver.draw_3d.begin(call,camera) end

---Draw a grid.
---@param slice number # The slice count of the grid.
---@param space number # The space shift of the grid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L417)
function quiver.draw_3d.draw_grid(slice,space) end

---Draw a cube.
---@param point vector_3 # The point of the cube.
---@param shape vector_3 # The shape of the cube.
---@param color color # The color of the cube.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L435)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a ball.
---@param point vector_3 # The point of the ball.
---@param shape number # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L460)
function quiver.draw_3d.draw_ball(point,shape,color) end

---Draw a 3D box.
---@param shape box_3 # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L480)
function quiver.draw_3d.draw_box_3(shape,color) end

---Draw a ray.
---@param ray ray # The ray.
---@param color color # The color of the ray.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L500)
function quiver.draw_3d.draw_ray(ray,color) end

---Draw a line.
---@param point_a vector_3 # The point A of the line.
---@param point_b vector_3 # The point B of the line.
---@param color color # The color of the line.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L521)
function quiver.draw_3d.draw_line(point_a,point_b,color) end

---Set the current state of backface culling.
---@param state boolean # The new state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L544)
function quiver.draw_3d.set_backface_cull(state) end

---TO-DO
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L565)
function quiver.draw.begin_quad(call) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L592)
function quiver.draw.draw_quad_color() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L607)
function quiver.draw.draw_quad_normal() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L622)
function quiver.draw.draw_quad_coordinate() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L637)
function quiver.draw.draw_quad_vertex() end

---The 2D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L652)
---@class quiver.draw_2d
quiver.draw_2d = {}

---Get a world-space point for a 2D screen-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The screen-space point.
---@return number point_x # The 2D world-space point (X).
---@return number point_y # The 2D world-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L694)
function quiver.draw_2d.get_screen_to_world(camera,point) end

---Get a screen-space point for a 2D world-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The world-space point.
---@return number point_x # The 2D screen-space point (X).
---@return number point_y # The 2D screen-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L723)
function quiver.draw_2d.get_world_to_screen(camera,point) end

---Initialize the 2D draw mode.
---@param call function # The draw code.
---@param camera camera_2d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L748)
function quiver.draw_2d.begin(call,camera) end

---Draw pixel.
---@param point vector_2 # The point of the pixel.
---@param color color # The color of the pixel.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L772)
function quiver.draw_2d.draw_pixel(point,color) end

---Draw a line.
---@param point_a vector_2 # The point A of the line.
---@param point_b vector_2 # The point B of the line.
---@param thick number # The thickness of the line.
---@param color color # The color of the line.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L795)
function quiver.draw_2d.draw_line(point_a,point_b,thick,color) end

---Draw text.
---@param label string # The label of the text.
---@param point vector_2 # The point of the text.
---@param scale number # The angle of the text.
---@param color color # The color of the text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L822)
function quiver.draw_2d.draw_text(label,point,scale,color) end

---Draw a circle.
---@param point vector_2 # TO-DO
---@param radius number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L854)
function quiver.draw_2d.draw_circle(point,radius,color) end

---Draw the sector of a circle.
---@param point vector_2 # TO-DO
---@param radius number # TO-DO
---@param begin_angle number # TO-DO
---@param close_angle number # TO-DO
---@param segment_count number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L882)
function quiver.draw_2d.draw_circle_sector(point,radius,begin_angle,close_angle,segment_count,color) end

---Draw 2D box.
---@param shape box_2 # The shape of the box.
---@param point vector_2 # The point of the box.
---@param angle number # The angle of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L922)
function quiver.draw_2d.draw_box_2(shape,point,angle,color) end

---Draw 2D box with a gradient (X-direction).
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A of the box.
---@param color_b color # The color B of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L948)
function quiver.draw_2d.draw_box_2_gradient_x(shape,color_a,color_b) end

---Draw 2D box with a gradient (Y-direction).
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A of the box.
---@param color_b color # The color B of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L981)
function quiver.draw_2d.draw_box_2_gradient_y(shape,color_a,color_b) end

---Draw 2D box with a 4-point gradient.
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A (T.L.) of the box.
---@param color_b color # The color B (B.L.) of the box.
---@param color_c color # The color C (T.R.) of the box.
---@param color_d color # The color D (B.R.) of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1016)
function quiver.draw_2d.draw_box_2_gradient(shape,color_a,color_b,color_c,color_d) end

---Draw 2D box (out-line).
---@param shape box_2 # The shape of the box.
---@param thick number # The thickness of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1056)
function quiver.draw_2d.draw_box_2_line(shape,thick,color) end

---Draw 2D box (round).
---@param shape box_2 # The shape of the box.
---@param round number # The roundness of the box.
---@param count number # The segment count of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1082)
function quiver.draw_2d.draw_box_2_round(shape,round,count,color) end

---Draw 2D box (out-line, round).
---@param shape box_2 # The shape of the box.
---@param round number # The roundness of the box.
---@param count number # The segment count of the box.
---@param thick number # The thickness of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1109)
function quiver.draw_2d.draw_box_2_line_round(shape,round,count,thick,color) end

---Draw 2D triangle.
---@param point_a vector_2 # The point A of the triangle.
---@param point_b vector_2 # The point B of the triangle.
---@param point_c vector_2 # The point C of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1135)
function quiver.draw_2d.draw_triangle(point_a,point_b,point_c,color) end

---Draw 2D triangle (out-line).
---@param point_a vector_2 # The point A of the triangle.
---@param point_b vector_2 # The point B of the triangle.
---@param point_c vector_2 # The point C of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1163)
function quiver.draw_2d.draw_triangle_line(point_a,point_b,point_c,color) end

---The data API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L60)
---@class quiver.data
quiver.data = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L137)
function data:foo() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L146)
function data:get_buffer() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L155)
function data:get_slice() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L181)
function quiver.data.compress() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L200)
function quiver.data.decompress() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L221)
function quiver.data.encode() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L240)
function quiver.data.decode() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L268)
function quiver.data.hash() end

---Serialize a given Lua value as another format, in the form of a string.
---@param text any # Lua value to serialize.
---@param kind number # Format.
---@return string value # The value, in string form.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L330)
function quiver.data.serialize(text,kind) end

---Deserialize a given format string as a Lua value.
---@param text string # String to deserialize.
---@param kind number # Format.
---@return any value # The value, in Lua value form.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L371)
function quiver.data.deserialize(text,kind) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L412)
function quiver.data.to_data() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L436)
function quiver.data.from_data() end

---The file API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L64)
---@class quiver.file
quiver.file = {}

---Get the data of a file, in string format.
---@param path string # Path to file.
---@return string data # File data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L100)
function quiver.file.get(path) end

---Set the data of a file.
---@param path string # Path to file.
---@param data string # Data to copy.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L127)
function quiver.file.set(path,data) end

---Check if a file does exist.
---@param path string # Path to file.
---@return boolean exist # True if file does exist, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L155)
function quiver.file.get_file_exist(path) end

---Check if a path does exist.
---@param path string # Path.
---@return boolean exist # True if path does exist, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L175)
function quiver.file.get_path_exist(path) end

---Check if a file's extension is the same as a given one.
---@param path string # Path to file.
---@param extension string # Extension. MUST include dot (.png, .wav, etc.).
---@return boolean check # True if file extension is the same as the given one, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L196)
function quiver.file.get_file_extension_check(path,extension) end

---Get the size of a file.
---@param path string # Path to file.
---@return number size # File size.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L217)
function quiver.file.get_file_size(path) end

---Get the extension of a file.
---@param path string # Path to file.
---@return string extension # File extension.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L237)
function quiver.file.get_file_extension(path) end

---Get the name of a file.
---@param path string # Path to file.
---@param extension boolean # File extension. If true, will return file name with the extension.
---@return string name # File name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L264)
function quiver.file.get_file_name(path,extension) end

---Get the current work path.
---@return string path # Work path.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L295)
function quiver.file.get_work_directory() end

---Get the current application path.
---@return string path # Application path.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L315)
function quiver.file.get_application_directory() end

---Scan a path.
---@param path string # Path to scan.
---@param filter string # OPTIONAL: Extension filter. If filter is 'DIR', will includ every directory in the result.
---@param recursive boolean # Recursive toggle. If true, recursively scan the directory.
---@return table list # File list.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L340)
function quiver.file.scan_path(path,filter,recursive) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L384)
function quiver.file.get_path_escape() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L395)
function quiver.file.set_path_escape() end

---The general API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L61)
---@class quiver.general
quiver.general = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L91)
function quiver.general.test() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L102)
function quiver.general.set_log_level() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L116)
function quiver.general.open_link() end

---Load the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L126)
function quiver.general.load() end

---Exit the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L130)
function quiver.general.exit() end

---Set a key to exit Quiver.
---@param key input_board # Key to exit Quiver with.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L140)
function quiver.general.set_exit_key(key) end

---Get the current time. Will count up since the initialization of the window.
---@return number time # Current time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L156)
function quiver.general.get_time() end

---Get the current frame time.
---@return number frame_time # Current frame time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L169)
function quiver.general.get_frame_time() end

---Get the current frame rate.
---@return number frame_rate # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L182)
function quiver.general.get_frame_rate() end

---Set the current frame rate.
---@param frame_rate number # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L195)
function quiver.general.set_frame_rate(frame_rate) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L209)
function quiver.general.get_argument() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L222)
function quiver.general.get_system() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L236)
function quiver.general.get_memory() end

---The shader API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L64)
---@class quiver.shader
quiver.shader = {}

---An unique handle for a shader in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L81)
---@class shader
shader = {}

---Create a new shader resource.
---@param v_path string # Path to .vs file.
---@param f_path string # Path to .fs file.
---@return shader shader # Shader resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L100)
function quiver.shader.new(v_path,f_path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L141)
function quiver.shader.new_from_memory() end

---TO-DO
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L191)
function shader:begin(call) end

---TO-DO
---@param name string # TO-DO
---@return number location # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L216)
function shader:get_location_name(name) end

---TO-DO
---@param name string # TO-DO
---@return number location # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L233)
function shader:get_location_attribute_name(name) end

---TO-DO
---@param location number # TO-DO
---@return number location # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L250)
function shader:get_location(location) end

---TO-DO
---@param location number # TO-DO
---@param value number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L265)
function shader:set_location(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L284)
function shader:set_shader_integer(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L303)
function shader:set_shader_decimal(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value vector_3 # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L322)
function shader:set_shader_vector_3(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value vector_4 # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L342)
function shader:set_shader_vector_4(location,value) end

---The image API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L67)
---@class quiver.image
quiver.image = {}

---An unique handle for a image in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L90)
---@class image
---@field shape_x number # Shape of the image (X).
---@field shape_y number # Shape of the image (Y).
image = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L108)
function image:to_texture() end

---Create a new image resource.
---@param path string # Path to image file.
---@return image image # Image resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L128)
function quiver.image.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L154)
function quiver.image.new_from_memory() end

---The font API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L64)
---@class quiver.font
quiver.font = {}

---Set the vertical space between each line-break.
---@param space number # Vertical space.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L88)
function quiver.font.set_text_line_space(space) end

---An unique handle to a font in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L101)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L123)
function font:draw(label,point,scale,space,color) end

---Measure the size of a given text on screen, with a given font.
---@param label string # Label of font to measure.
---@param scale number # Scale of font to measure.
---@param space number # Space of font to measure.
---@return number size_x # Size of text (X).
---@return number size_y # Size of text (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L161)
function font:measure_text(label,scale,space) end

---Create a new font resource.
---@param path string # Path to font file.
---@param size number # Size for font.
---@return font font # Font resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L190)
function quiver.font.new(path,size) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L214)
function quiver.font.new_from_memory() end

---Create a new font resource (default font).
---@return font font # Font resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L249)
function quiver.font.new_default() end

---The window API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L67)
---@class quiver.window
quiver.window = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L158)
function quiver.window.file_dialog() end

---Get if the window should close.
---@return boolean close # True if the window should close.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L248)
function quiver.window.get_close() end

---Get the state of the window (full-screen).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L262)
function quiver.window.get_fullscreen() end

---Get the state of the window (hidden).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L276)
function quiver.window.get_hidden() end

---Get the state of the window (minimize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L290)
function quiver.window.get_minimize() end

---Get the state of the window (maximize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L304)
function quiver.window.get_maximize() end

---Get the state of the window (focus).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L318)
function quiver.window.get_focus() end

---Get the state of the window (resize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L332)
function quiver.window.get_resize() end

---Get the state of a window flag.
---@param flag window_flag # Window flag.
---@return boolean state # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L349)
function quiver.window.get_state(flag) end

---Set the state of a window flag.
---@param flag window_flag # Window flag.
---@param state boolean # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L364)
function quiver.window.set_state(flag,state) end

---Set the window to full-screen mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L379)
function quiver.window.set_fullscreen() end

---Set the window to border-less mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L389)
function quiver.window.set_borderless() end

---Minimize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L399)
function quiver.window.set_minimize() end

---Maximize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L409)
function quiver.window.set_maximize() end

---Restore the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L419)
function quiver.window.set_restore() end

---Set the window icon.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L429)
function quiver.window.set_icon() end

---Set the window name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L448)
function quiver.window.set_name() end

---Set the window point.
---@param point vector_2 # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L467)
function quiver.window.set_point(point) end

---Set the window monitor.
---@param index number # Index of monitor to move window to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L486)
function quiver.window.set_screen(index) end

---Set the minimum window shape.
---@param shape vector_2 # Minimum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L503)
function quiver.window.set_shape_min(shape) end

---Set the maximum window shape.
---@param shape vector_2 # Maximum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L522)
function quiver.window.set_shape_max(shape) end

---Set the current window shape.
---@param shape vector_2 # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L541)
function quiver.window.set_shape(shape) end

---Set the window alpha.
---@param alpha number # Alpha of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L560)
function quiver.window.set_alpha(alpha) end

---Focus the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L570)
function quiver.window.set_focus() end

---Get the shape of the window.
---@return number shape_x # Shape of the window (X).
---@return number shape_y # Shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L588)
function quiver.window.get_shape() end

---Get the shape of the current render view.
---@return number shape_x # Shape of the render view (X).
---@return number shape_y # Shape of the render view (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L603)
function quiver.window.get_render_shape() end

---Get the available monitor amount.
---@return number count # Monitor count.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L617)
function quiver.window.get_screen_count() end

---Get the current active monitor, where the window is.
---@return number index # Current active monitor index.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L631)
function quiver.window.get_screen_focus() end

---Get the point of the given monitor.
---@param index number # Index of the monitor.
---@return number point_x # Point of the monitor (X).
---@return number point_y # Point of the monitor (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L649)
function quiver.window.get_screen_point(index) end

---Get the shape of the given monitor.
---@param index number # Index of the monitor.
---@return number shape_x # Shape of the window (X).
---@return number shape_y # Shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L670)
function quiver.window.get_screen_shape(index) end

---Get the physical shape of the given monitor.
---@param index number # Index of the monitor.
---@return number shape_x # Physical shape of the window (X).
---@return number shape_y # Physical shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L688)
function quiver.window.get_screen_shape_physical(index) end

---Get the refresh rate of the given monitor.
---@param index number # Index of the monitor.
---@return number rate # Refresh rate of the monitor.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L710)
function quiver.window.get_screen_rate(index) end

---Get the point of the window.
---@return number point_x # Point of the window (X).
---@return number point_y # Point of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L725)
function quiver.window.get_point() end

---Get the DPI scale of the window.
---@return number scale_x # Scale of the window (X).
---@return number scale_y # Scale of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L744)
function quiver.window.get_scale() end

---Get the name of the given monitor.
---@param index number # Index of the monitor.
---@return string name # Name of the monitor.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L765)
function quiver.window.get_screen_name(index) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L782)
function quiver.window.get_screen_shot() end

---The music API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L64)
---@class quiver.music
quiver.music = {}

---An unique handle for music in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L81)
---@class music
music = {}

---Create a new music resource.
---@param path string # Path to music file.
---@return music music # Music resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L99)
function quiver.music.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L125)
function quiver.music.new_from_memory() end

---Play the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L165)
function music:play() end

---Check if music is currently playing.
---@return boolean state # State of the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L180)
function music:get_playing() end

---Stop the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L187)
function music:stop() end

---Pause the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L195)
function music:pause() end

---Resume the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L203)
function music:resume() end

---Set volume for the music. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L218)
function music:set_volume(volume) end

---Set pitch for the music.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L233)
function music:set_pitch(pitch) end

---Set pan for the music. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L248)
function music:set_pan(pan) end

---Update the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L256)
function music:update() end

---Set position for the music.
---@param position number # Current position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L271)
function music:set_position(position) end

---Get time length for the music.
---@return number length # Time length.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L286)
function music:get_length() end

---Get time played for the music.
---@return number played # Time played.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L300)
function music:get_played() end

