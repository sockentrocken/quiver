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
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L64)
---@class quiver.sound
quiver.sound = {}

---An unique handle for sound in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L81)
---@class sound
sound = {}

---Create a new sound resource.
---@param path string # Path to sound file.
---@param alias number? # OPTIONAL: The total sound alias count to load for the sound.
---@return sound sound # Sound resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L101)
function quiver.sound.new(path,alias) end

---Create a new sound resource, from memory.
---@param data data # The data buffer.
---@param alias number? # OPTIONAL: The total sound alias count to load for the sound.
---@param kind string # The kind of sound file (.wav, etc.).
---@return sound music # Sound resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L142)
function quiver.sound.new_from_memory(data,alias,kind) end

---Create a sound alias.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L209)
function sound:create_alias() end

---Remove a sound alias.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L219)
function sound:remove_alias() end

---Clear every sound alias.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L233)
function sound:remove_alias() end

---Play the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L246)
function sound:play() end

---Check if sound is currently playing.
---@return boolean state # State of the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L270)
function sound:get_playing() end

---Stop the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L287)
function sound:stop() end

---Pause the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L304)
function sound:pause() end

---Resume the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L321)
function sound:resume() end

---Set volume for the sound. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L347)
function sound:set_volume(volume) end

---Set pitch for the sound.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L376)
function sound:set_pitch(pitch) end

---Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/sound.rs#L405)
function sound:set_pan(pan) end

---The input API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L65)
---@class quiver.input
quiver.input = {}

---The board input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L78)
---@class quiver.input.board
quiver.input.board = {}

---The mouse input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L96)
---@class quiver.input.mouse
quiver.input.mouse = {}

---The pad input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L122)
---@class quiver.input.pad
quiver.input.pad = {}

---Set a key to exit Quiver.
---@param key input_board # Key to exit Quiver with.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L163)
function quiver.input.set_exit_key(key) end

---Set the clipboard text.
---```lua
---quiver.input.set_clipboard_text("hello, world!")
---local text = quiver.input.get_clipboard_text()
---assert(text == "hello, world!")
---
---```
---@param text string # Clipboard text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L181)
function quiver.input.board.set_clipboard_text(text) end

---Get the clipboard text.
---```lua
---quiver.input.set_clipboard_text("hello, world!")
---local text = quiver.input.get_clipboard_text()
---assert(text == "hello, world!")
---
---```
---@return string text # Clipboard text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L201)
function quiver.input.board.get_clipboard_text() end

---Get the last unicode glyph in the queue.
---@return number key_code # Key-code. If 0, queue is empty.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L218)
function quiver.input.board.get_key_code_queue() end

---Get the last unicode glyph in the queue.
---@return number uni_code # Uni-code. If 0, queue is empty.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L232)
function quiver.input.board.get_uni_code_queue() end

---Get the name of a given key.
---```lua
---local name = quiver.input.board.get_name(INPUT_BOARD.A)
---assert(name == "a")
---
---```
---@param board input_board # The board button to get a name for.
---@return string name # The name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L250)
function quiver.input.board.get_name(board) end

---Get the state of an input (up).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L272)
function quiver.input.board.get_up(board) end

---Get the state of an input (down).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L286)
function quiver.input.board.get_down(board) end

---Get the state of an input (press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L300)
function quiver.input.board.get_press(board) end

---Get the state of an input (repeat-press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L314)
function quiver.input.board.get_press_repeat(board) end

---Get the state of an input (release).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L328)
function quiver.input.board.get_release(board) end

---Set the active state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L344)
function quiver.input.mouse.set_active(state) end

---Set the hidden state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L366)
function quiver.input.mouse.set_hidden(state) end

---Get the hidden state of the mouse.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L388)
function quiver.input.mouse.get_hidden() end

---Check if the mouse is currently over the screen.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L402)
function quiver.input.mouse.get_screen() end

---Get the current point of the mouse.
---@return number point_x # The point of the mouse (X).
---@return number point_y # The point of the mouse (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L417)
function quiver.input.mouse.get_point() end

---Set the current point of the mouse.
---@param point vector_2 # The point of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L434)
function quiver.input.mouse.set_point(point) end

---Get the current delta (i.e. mouse movement) of the mouse.
---@return number delta_x # The delta of the mouse (X).
---@return number delta_y # The delta of the mouse (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L453)
function quiver.input.mouse.get_delta() end

---Set the current shift of the mouse.
---@param shift vector_2 # The shift of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L470)
function quiver.input.mouse.set_shift(shift) end

---Set the current scale of the mouse.
---@param scale vector_2 # The scale of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L488)
function quiver.input.mouse.set_scale(scale) end

---Set the current cursor of the mouse.
---@param cursor cursor_mouse # The cursor of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L506)
function quiver.input.mouse.set_cursor(cursor) end

---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return number delta_x # The delta of the mouse wheel (X).
---@return number delta_y # The delta of the mouse wheel (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L524)
function quiver.input.mouse.get_wheel() end

---Get the last mouse button press.
---@return input_mouse input # The last mouse button press.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L541)
function quiver.input.mouse.get_mouse_queue() end

---Get the state of an input (up).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L565)
function quiver.input.mouse.get_up(mouse) end

---Get the state of an input (down).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L579)
function quiver.input.mouse.get_down(mouse) end

---Get the state of an input (press).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L593)
function quiver.input.mouse.get_press(mouse) end

---Get the state of an input (release).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L607)
function quiver.input.mouse.get_release(mouse) end

---Get the state of a pad.
---@param index number # The index of the pad to check for.
---@return boolean state # The state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L626)
function quiver.input.pad.get_state(index) end

---Get the name of a pad.
---@param index number # The index of the pad to check for.
---@return string name # The name of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L643)
function quiver.input.pad.get_name(index) end

---Get the state of an input (press).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L663)
function quiver.input.pad.get_press(pad) end

---Get the state of an input (down).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L677)
function quiver.input.pad.get_down(pad) end

---Get the state of an input (release).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L691)
function quiver.input.pad.get_release(pad) end

---Get the state of an input (up).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L705)
function quiver.input.pad.get_up(pad) end

---Get the last pad button press.
---@return input_pad input # The last pad button press.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L719)
function quiver.input.pad.get_queue() end

---Get the axis count of a pad.
---@param index number # The index of the pad to check for.
---@return number axis_count # The axis count of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L736)
function quiver.input.pad.get_axis_count(index) end

---Get the axis state of a pad.
---@param index number # The index of the pad to check for.
---@param axis number # The axis of the pad to check for.
---@return number axis_state # The axis state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L754)
function quiver.input.pad.get_axis_state(index,axis) end

---Set the rumble of a pad.
---@param index number # The index of the pad to rumble.
---@param motor_a number # The intensity of the L. rumble motor.
---@param motor_b number # The intensity of the R. rumble motor.
---@param duration number # The duration of the rumble.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/input.rs#L771)
function quiver.input.pad.set_rumble(index,motor_a,motor_b,duration) end

---The Rapier API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L72)
---@class quiver.rapier
quiver.rapier = {}

---An unique handle for a Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L86)
---@class rapier
rapier = {}

---Create a new Rapier simulation.
---@return rapier rapier # Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L114)
function quiver.rapier.new() end

---Cast a ray.
---@param ray ray # Ray to cast.
---@param length number # Ray length.
---@param solid boolean # TO-DO
---@param exclude_rigid table # TO-DO
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L162)
function rapier:cast_ray(ray,length,solid,exclude_rigid) end

---Cast a ray, and also get the normal information..
---@param ray ray # Ray to cast.
---@param length number # Ray length.
---@param solid boolean # TO-DO
---@param exclude_rigid table # TO-DO
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L220)
function rapier:cast_ray_normal(ray,length,solid,exclude_rigid) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L274)
function rapier:test_intersect_cuboid_cuboid() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L318)
function rapier:test_intersect_cuboid() end

---Get the shape of a collider (cuboid).
---@param collider table # Collider handle.
---@return number half_shape_x # Half-shape of the cuboid. (X).
---@return number half_shape_y # Half-shape of the cuboid. (Y).
---@return number half_shape_z # Half-shape of the cuboid. (Z).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L389)
function rapier:get_collider_shape_cuboid(collider) end

---Set the shape of a collider (cuboid).
---@param collider table # Collider handle.
---@param half_shape vector_3 # Half-shape of cuboid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L425)
function rapier:set_collider_shape_cuboid(collider,half_shape) end

---Get the parent of a collider.
---@param collider table # Collider handle.
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L466)
function rapier:get_collider_parent(collider) end

---Get the position of a collider.
---@param collider table # Collider handle.
---@return number position_x # Collider position (X).
---@return number position_y # Collider position (Y).
---@return number position_z # Collider position (Z).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L497)
function rapier:get_collider_position(collider) end

---Set the position of a collider.
---@param collider table # Collider handle.
---@param position vector_3 # Collider position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L524)
function rapier:set_collider_position(collider,position) end

---Set the rotation of a collider.
---@param collider table # Collider handle.
---@param rotation vector_3 # Collider rotation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L552)
function rapier:set_collider_rotation(collider,rotation) end

---Set the sensor state of a collider.
---@param collider table # Collider handle.
---@param sensor boolean # Collider sensor state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L581)
function rapier:set_collider_sensor(collider,sensor) end

---Remove a collider.
---@param collider table # Collider handle.
---@param wake_parent boolean # Whether or not to wake up the rigid body parent this collider is bound to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L610)
function rapier:collider_remove(collider,wake_parent) end

---Remove a rigid body.
---@param rigid_body table # Rigid body handle.
---@param remove_collider boolean # Whether or not to remove every collider this rigid body is bound to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L637)
function rapier:rigid_body_remove(rigid_body,remove_collider) end

---Create a character controller.
---@return table character_controller # Character controller.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L665)
function rapier:character_controller() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L676)
function rapier:set_character_controller_up_vector() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L693)
function rapier:set_character_controller_slope() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L710)
function rapier:set_character_auto_step() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L757)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L793)
function rapier:character_controller_move(step,character,collider,translation) end

---Create a rigid body.
---@param kind rigid_body_kind # Rigid body kind.
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L842)
function rapier:rigid_body(kind) end

---Get the user data of a rigid_body.
---@param rigid_body userdata # Rigid body handle.
---@return number user_data # Rigid body user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L866)
function rapier:get_rigid_body_user_data(rigid_body) end

---Set the user data of a rigid_body.
---@param rigid_body userdata # Rigid body handle.
---@param user_data number # Rigid body user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L892)
function rapier:set_rigid_body_user_data(rigid_body,user_data) end

---Set the position of a rigid_body.
---@param rigid_body userdata # rigid_body handle.
---@param position vector_3 # rigid_body position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L919)
function rapier:set_rigid_body_position(rigid_body,position) end

---Set the rotation of a rigid_body.
---@param rigid_body table # rigid_body handle.
---@param rotation vector_3 # rigid_body rotation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L948)
function rapier:set_rigid_body_rotation(rigid_body,rotation) end

---Get the user data of a collider.
---@param collider userdata # Collider handle.
---@return number user_data # Collider user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L983)
function rapier:get_collider_user_data(collider) end

---Set the user data of a collider.
---@param collider userdata # Collider handle.
---@param user_data number # Collider user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L1006)
function rapier:set_collider_user_data(collider,user_data) end

---Create a collider builder (cuboid).
---@param half_shape vector_3 # Half-shape of cuboid.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L1037)
function rapier:collider_builder_cuboid(half_shape) end

---Create a collider builder (tri-mesh).
---@param point_table table # The point array table.
---@param index_table table # The index array table.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L1064)
function rapier:collider_builder_tri_mesh(point_table,index_table) end

---Create a collider builder (convex hull).
---@param vector_table table # A vector_3 vertex array table.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L1112)
function rapier:collider_builder_convex_hull(vector_table) end

---Step the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L1137)
function rapier:step() end

---Render the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/rapier.rs#L1175)
function rapier:debug_render() end

---The Steam API.
---
--- ---
---*Available with compile feature: `steam`.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L64)
---@class quiver.steam
quiver.steam = {}

---An unique handle to a Steam client.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L78)
---@class steam
steam = {}

---Update the Steam state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L136)
function steam:update() end

---Get the current AppID.
---@return number ID # The current AppID.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L155)
function steam:get_app_ID() end

---Get the current country code from the user's IP.
---@return string code # The current country code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L169)
function steam:get_IP_country() end

---Get the current language for the Steam client.
---@return string language # The current language.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L183)
function steam:get_UI_language() end

---Get the current Steam server time, as an UNIX time-stamp.
---@return number time # The current server time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L197)
function steam:get_server_time() end

---Set the position of the Steam overlay.
---@param position steam_overlay_position # The Steam overlay position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L211)
function steam:set_overlay_position(position) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L245)
function steam:set_message() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L269)
function steam:get_message() end

---Check if a given AppID is currently on the system.
---@param ID number # The AppID.
---@return boolean install # The state of the AppID.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L297)
function steam:get_app_install(ID) end

---Check if a given DLC is currently on the system.
---@param ID number # The AppID.
---@return boolean install # The state of the AppID.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L314)
function steam:get_DLC_install(ID) end

---Check if the user has a subscription to the given AppID.
---@param ID number # The AppID.
---@return boolean subscription # Subscription state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L331)
function steam:get_app_subscribe(ID) end

---Check if the user has a VAC ban on record.
---@return boolean ban # The VAC ban state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L345)
function steam:get_VAC_ban() end

---Check if the user has a VAC ban on record.
---@return boolean ban # The VAC ban state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L359)
function steam:get_cyber_cafe() end

---Check if the current AppID has support for low violence.
---@return boolean low_violence # Low violence state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L373)
function steam:get_low_violence() end

---Check if the user has a subscription to the current AppID.
---@return boolean subscription # Subscription state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L387)
function steam:get_subscribe() end

---Get the build ID for the current AppID.
---@return number ID # The build ID.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L401)
function steam:get_app_build_ID() end

---Get the installation path for the given AppID. NOTE: this will work even if the app is not in disk.
---@param ID number # The AppID.
---@return string path # Installation path.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L418)
function steam:get_app_install_path(ID) end

---Get the SteamID of the owner's current AppID.
---@return string ID # The SteamID.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L432)
function steam:get_app_owner() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L443)
function steam:get_game_language_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L454)
function steam:get_game_language() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L465)
function steam:get_beta_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L476)
function steam:get_launch_command_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L491)
function steam:get_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L502)
function steam:activate_overlay() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L514)
function steam:activate_overlay_link() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L526)
function steam:activate_overlay_store() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L547)
function steam:activate_overlay_user() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L563)
function steam:activate_invite_dialog() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L585)
function steam:get_steam_ID() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L596)
function steam:get_level() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L605)
function steam:get_log_on() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L620)
function steam:get_leader_board() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L640)
function steam:get_or_create_leader_board() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L673)
function steam:upload_leader_board_score() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L732)
function steam:get_leader_board_show_kind() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L764)
function steam:get_leader_board_sort_kind() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L795)
function steam:get_leader_board_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L822)
function steam:get_leader_board_entry_count() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L849)
function steam:pull_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L862)
function steam:push_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L875)
function steam:reset_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L895)
function steam:get_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L917)
function steam:set_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L941)
function steam:get_achievement() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L960)
function steam:get_achievement_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L977)
function steam:set_achievement() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1003)
function steam:get_achievement_percent() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1022)
function steam:get_achievement_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1041)
function steam:get_achievement_() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1060)
function steam:get_achievement_hidden() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1079)
function steam:get_achievement_icon() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1108)
function steam:get_session_user() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1121)
function steam:get_session_client_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1134)
function steam:get_session_client_form_factor() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1159)
function steam:get_session_client_resolution() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1179)
function steam:invite_session() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1203)
function steam:set_cloud_app() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1215)
function steam:get_cloud_app() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1226)
function steam:get_cloud_account() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1237)
function steam:get_file_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1257)
function steam:file_delete() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1270)
function steam:file_forget() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1283)
function steam:file_exist() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1296)
function steam:get_file_persist() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1309)
function steam:get_file_time_stamp() end

---Create a new Steam client.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/steam.rs#L1331)
function quiver.steam.new() end

---The video API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/video.rs#L72)
---@class quiver.video
quiver.video = {}

---An unique handle to a video in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/video.rs#L86)
---@class video
video = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/video.rs#L151)
function video:update() end

---Draw a texture.
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/video.rs#L174)
function video:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # TO-DO
---@param box_b box_2 # TO-DO
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/video.rs#L199)
function video:draw_pro(box_a,box_b,point,angle,color) end

---Create a new video resource.
---@param path string # Path to video file.
---@return video video # Video resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/video.rs#L234)
function quiver.video.new(path) end

---The request API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/request.rs#L63)
---@class quiver.request
quiver.request = {}

---Perform a GET request.
---```lua
---local link = "https://raw.githubusercontent.com/sockentrocken/quiver/refs/heads/main/test/asset/sample.txt"
---
----- Get the data from the link. As we know it's not binary, we pass false to the function.
---local response = quiver.request.get(link, false)
---
---assert(response == "Hello, world!")
---
---```
---@param link string # The target URL link.
---@param binary boolean # Receive as binary.
---@return string | data value # The return value.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/request.rs#L93)
function quiver.request.get(link,binary) end

---Perform a POST request.
---```lua
----- POST request with a body. We pass false to get the result as a string.
---local body = quiver.request.post("http://httpbin.org/post", "hello, body", nil, nil, false)
---
----- POST request with a form.
---local form = quiver.request.post("http://httpbin.org/post", nil, {
---    foo = "bar",
---    bar = "baz",
---}, nil, false)
---
----- POST request with a JSON.
---local JSON = quiver.request.post("http://httpbin.org/post", nil, nil, {
---    foo = "bar",
---    bar = "baz",
---}, false)
---
----- Deserialize the JSON result to a Lua table.
---body = quiver.data.deserialize(body)
---form = quiver.data.deserialize(form)
---JSON = quiver.data.deserialize(JSON)
---
----- Assert that the request with a body's data is correct.
---assert(body.data, "hello, body")
---
----- Assert that the request with a form's data is correct.
---assert(form.form.foo, "bar")
---assert(form.form.bar, "baz")
---
----- Assert that the request with a JSON's data is correct.
---assert(JSON.json.foo, "bar")
---assert(JSON.json.bar, "baz")
---
----- POST request with binary data as the body.
---local body = quiver.request.post("http://httpbin.org/post", quiver.data.new({ 255, 0, 255, 0 }), nil, nil, false)
---
----- Deserialize the JSON result to a Lua table.
---body = quiver.data.deserialize(body)
---
----- Get the Base64 string representation of the data.
---body = string.tokenize(body.data, ",")[2]
---
----- Convert the string representation to a byte representation.
---body = quiver.data.to_data(body, 2)
---
----- Decode the data from Base64.
---body = quiver.data.decode(body)
---
----- Get the data as a Lua table.
---body = body:get_buffer()
---
----- Assert that the data sent is the same as the data we got.
---assert(body[1], 255)
---assert(body[2], 0)
---assert(body[3], 255)
---assert(body[4], 0)
---
---```
---@param link string # The target URL link.
---@param data string? # OPTIONAL: The "data" payload.
---@param form table? # OPTIONAL: The "form" payload.
---@param json table? # OPTIONAL: The "json" payload.
---@param binary boolean # Receive as binary.
---@return string | data value # The return value.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/request.rs#L135)
function quiver.request.post(link,data,form,json,binary) end

---The texture API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L70)
---@class quiver.texture
quiver.texture = {}

---An unique handle for a texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L193)
---@class texture
---@field shape_x number # Shape of the texture (X).
---@field shape_y number # Shape of the texture (Y).
texture = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L210)
function texture:to_image() end

---Set the mipmap for a texture.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L221)
function texture:set_mipmap() end

---Set the filter for a texture.
---@param filter texture_filter # Texture filter.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L238)
function texture:set_filter(filter) end

---Set the wrap for a texture.
---@param wrap texture_wrap # Texture wrap.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L255)
function texture:set_wrap(wrap) end

---Draw a texture.
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L275)
function texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # TO-DO
---@param box_b box_2 # TO-DO
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L296)
function texture:draw_pro(box_a,box_b,point,angle,color) end

---Draw a billboard texture.
---@param camera camera_3d # TO-DO
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L327)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L356)
function texture:draw_billboard_pro(camera,source,point,up,scale,origin,angle,color) end

---Create a new texture resource.
---@param path string # Path to texture file.
---@return texture texture # Texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L395)
function quiver.texture.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L418)
function quiver.texture.new_from_memory() end

---An unique handle for a render texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L468)
---@class render_texture
---@field shape_x number # Shape of the texture (X).
---@field shape_y number # Shape of the texture (Y).
render_texture = {}

---Initialize drawing to the render texture.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L488)
function render_texture:begin(call) end

---Draw a texture.
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L513)
function render_texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # TO-DO
---@param box_b box_2 # TO-DO
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L537)
function render_texture:draw_pro(box_a,box_b,point,angle,color) end

---Create a new render texture resource.
---@param shape vector_2 # TO-DO
---@return render_texture render_texture # Render texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/texture.rs#L571)
function quiver.render_texture.new(shape) end

---The ZIP API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/zip.rs#L64)
---@class quiver.zip
quiver.zip = {}

---An unique handle to a ZIP in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/zip.rs#L83)
---@class zip
zip = {}

---Get a file from the ZIP file.
---@param path string # Path to the file.
---@param binary boolean # Read as binary.
---@return string | data file # The file.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/zip.rs#L104)
function zip:get_file(path,binary) end

---Get a list of every file in the ZIP file.
---@return table list # The list of every file.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/zip.rs#L141)
function zip:get_list() end

---Check if the given path is a file.
---@param path string # Path to the file.
---@return boolean value # True if the path is a file, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/zip.rs#L160)
function zip:is_file(path) end

---Check if the given path is a folder.
---@param path string # Path to the folder.
---@return boolean value # True if the path is a folder, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/zip.rs#L180)
function zip:is_path(path) end

---Create a new ZIP resource.
---@param path string # Path to ZIP file.
---@return zip zip # ZIP resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/zip.rs#L203)
function quiver.zip.new(path) end

---The model API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L65)
---@class quiver.model
quiver.model = {}

---An unique handle for a model in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L96)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L114)
function quiver.model.new(path) end

---Bind a texture to the model.
---@param index number # TO-DO
---@param which number # TO-DO
---@param texture texture # Texture to bind to model.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L151)
function model:bind(index,which,texture) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L186)
function model:draw_mesh() end

---Draw the model.
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L219)
function model:draw(point,scale,color) end

---Draw the model (wire-frame).
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L242)
function model:draw_wire(point,scale,color) end

---Draw the model with a transformation.
---@param point vector_3 # TO-DO
---@param angle vector_3 # TO-DO
---@param scale vector_3 # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L266)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L299)
function model:get_box_3() end

---Get the vertex data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Vector3 table.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L324)
function model:mesh_vertex(index) end

---Get the index data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Number table.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L342)
function model:mesh_index(index) end

---Get the triangle count of a specific mesh in the model.
---@param index number # Index of mesh.
---@return number count # Triangle count.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L371)
function model:mesh_triangle_count(index) end

---An unique handle for a model animation in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L387)
---@class model_animation
model_animation = {}

---Create a new ModelAnimation resource.
---@param path string # Path to model file.
---@return model_animation model_animation # ModelAnimation resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L405)
function quiver.model_animation.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L451)
function model_animation:get_bone_() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L469)
function model_animation:get_bone_() end

---Update model with new model animation data.
---@param model model # TO-DO
---@param frame number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/model.rs#L492)
function model_animation:update(model,frame) end

---The drawing API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L81)
---@class quiver.draw
quiver.draw = {}

---Initialize drawing to the screen.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L108)
function quiver.draw.begin(call) end

---Initialize drawing (blend mode) to the screen.
---@param call function # The draw code.
---@param mode function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L131)
function quiver.draw.begin_blend(call,mode) end

---Initialize drawing (scissor mode) to the screen.
---@param call function # The draw code.
---@param view box_2 # The clip test region.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L153)
function quiver.draw.begin_scissor(call,view) end

---Clear the screen with a color.
---@param color color # The color to use for clearing.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L180)
function quiver.draw.clear(color) end

---The 3D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L196)
---@class quiver.draw_3d
quiver.draw_3d = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L230)
function quiver.draw_3d.get_matrix_projection() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L250)
function quiver.draw_3d.get_matrix_model_view() end

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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L283)
function quiver.draw_3d.get_screen_to_world(camera,point,shape) end

---Get a 2D screen-space point for a 3D world-space point.
---@param camera camera_3d # The current camera.
---@param point vector_3 # The world-space point.
---@param shape vector_2 # The size of the view-port.
---@return number point_x # The 2D screen-space point (X).
---@return number point_y # The 2D screen-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L326)
function quiver.draw_3d.get_world_to_screen(camera,point,shape) end

---Initialize the 3D draw mode.
---@param call function # The draw code.
---@param camera camera_3d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L357)
function quiver.draw_3d.begin(call,camera) end

---Draw a grid.
---@param slice number # The slice count of the grid.
---@param space number # The space shift of the grid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L380)
function quiver.draw_3d.draw_grid(slice,space) end

---Draw a cube.
---@param point vector_3 # The point of the cube.
---@param shape vector_3 # The shape of the cube.
---@param color color # The color of the cube.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L398)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a ball.
---@param point vector_3 # The point of the ball.
---@param shape number # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L423)
function quiver.draw_3d.draw_ball(point,shape,color) end

---Draw a 3D box.
---@param shape box_3 # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L443)
function quiver.draw_3d.draw_box_3(shape,color) end

---Draw a ray.
---@param ray ray # The ray.
---@param color color # The color of the ray.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L463)
function quiver.draw_3d.draw_ray(ray,color) end

---Draw a line.
---@param point_a vector_3 # The point A of the line.
---@param point_b vector_3 # The point B of the line.
---@param color color # The color of the line.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L484)
function quiver.draw_3d.draw_line(point_a,point_b,color) end

---Set the current state of backface culling.
---@param state boolean # The new state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L507)
function quiver.draw_3d.set_backface_cull(state) end

---TO-DO
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L528)
function quiver.draw.begin_quad(call) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L555)
function quiver.draw.draw_quad_color() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L570)
function quiver.draw.draw_quad_normal() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L585)
function quiver.draw.draw_quad_coordinate() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L600)
function quiver.draw.draw_quad_vertex() end

---The 2D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L615)
---@class quiver.draw_2d
quiver.draw_2d = {}

---Get a world-space point for a 2D screen-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The screen-space point.
---@return number point_x # The 2D world-space point (X).
---@return number point_y # The 2D world-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L657)
function quiver.draw_2d.get_screen_to_world(camera,point) end

---Get a screen-space point for a 2D world-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The world-space point.
---@return number point_x # The 2D screen-space point (X).
---@return number point_y # The 2D screen-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L686)
function quiver.draw_2d.get_world_to_screen(camera,point) end

---Initialize the 2D draw mode.
---@param call function # The draw code.
---@param camera camera_2d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L711)
function quiver.draw_2d.begin(call,camera) end

---Draw pixel.
---@param point vector_2 # The point of the pixel.
---@param color color # The color of the pixel.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L735)
function quiver.draw_2d.draw_pixel(point,color) end

---Draw a line.
---@param point_a vector_2 # The point A of the line.
---@param point_b vector_2 # The point B of the line.
---@param thick number # The thickness of the line.
---@param color color # The color of the line.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L758)
function quiver.draw_2d.draw_line(point_a,point_b,thick,color) end

---Draw text.
---@param label string # The label of the text.
---@param point vector_2 # The point of the text.
---@param scale number # The angle of the text.
---@param color color # The color of the text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L785)
function quiver.draw_2d.draw_text(label,point,scale,color) end

---Draw a circle.
---@param point vector_2 # TO-DO
---@param radius number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L817)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L845)
function quiver.draw_2d.draw_circle_sector(point,radius,begin_angle,close_angle,segment_count,color) end

---Draw 2D box.
---@param shape box_2 # The shape of the box.
---@param point vector_2 # The point of the box.
---@param angle number # The angle of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L885)
function quiver.draw_2d.draw_box_2(shape,point,angle,color) end

---Draw 2D box with a gradient (X-direction).
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A of the box.
---@param color_b color # The color B of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L911)
function quiver.draw_2d.draw_box_2_gradient_x(shape,color_a,color_b) end

---Draw 2D box with a gradient (Y-direction).
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A of the box.
---@param color_b color # The color B of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L944)
function quiver.draw_2d.draw_box_2_gradient_y(shape,color_a,color_b) end

---Draw 2D box with a 4-point gradient.
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A (T.L.) of the box.
---@param color_b color # The color B (B.L.) of the box.
---@param color_c color # The color C (T.R.) of the box.
---@param color_d color # The color D (B.R.) of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L979)
function quiver.draw_2d.draw_box_2_gradient(shape,color_a,color_b,color_c,color_d) end

---Draw 2D box (out-line).
---@param shape box_2 # The shape of the box.
---@param thick number # The thickness of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L1019)
function quiver.draw_2d.draw_box_2_line(shape,thick,color) end

---Draw 2D box (round).
---@param shape box_2 # The shape of the box.
---@param round number # The roundness of the box.
---@param count number # The segment count of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L1045)
function quiver.draw_2d.draw_box_2_round(shape,round,count,color) end

---Draw 2D box (out-line, round).
---@param shape box_2 # The shape of the box.
---@param round number # The roundness of the box.
---@param count number # The segment count of the box.
---@param thick number # The thickness of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L1072)
function quiver.draw_2d.draw_box_2_line_round(shape,round,count,thick,color) end

---Draw 2D triangle.
---@param point_a vector_2 # The point A of the triangle.
---@param point_b vector_2 # The point B of the triangle.
---@param point_c vector_2 # The point C of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L1098)
function quiver.draw_2d.draw_triangle(point_a,point_b,point_c,color) end

---Draw 2D triangle (out-line).
---@param point_a vector_2 # The point A of the triangle.
---@param point_b vector_2 # The point B of the triangle.
---@param point_c vector_2 # The point C of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/draw.rs#L1126)
function quiver.draw_2d.draw_triangle_line(point_a,point_b,point_c,color) end

---The data API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L64)
---@class quiver.data
quiver.data = {}

---An unique handle for a data buffer in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L103)
---@class data
data = {}

---Get the length of the data buffer.
---@return number length # The length of the data buffer.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L157)
function data:get_length() end

---Get the data buffer.
---@return table buffer # The data buffer.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L169)
function data:get_buffer() end

---Get a slice out of the data buffer, as another data buffer.
---@param index_a number # Index A into the data buffer.
---@param index_b number # Index B into the data buffer.
---@return data slice # The slice, as another data buffer.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L185)
function data:get_slice(index_a,index_b) end

---Compress a given data buffer (DEFLATE).
---```lua
----- Create a table with no data in it.
---local data = {}
---
----- Insert some dummy data in it.
---for x = 1, 63 do
---    table.insert(data, 0)
---end
---
----- Insert some data of significance at the last index.
---table.insert(data, 1)
---
----- Check that the last value is 1.
---assert(data[64] == 1)
---
----- Get a data buffer user-data from Quiver.
---local data = quiver.data.new(data)
---
----- Compress the data.
---local data = quiver.data.compress(data)
---
----- Decompress the data.
---local data = quiver.data.decompress(data)
---
----- Get the data back as a Lua table.
---local data = data:get_buffer()
---
----- Check that the last value is 1.
---assert(data[64] == 1)
---
---```
---@param data data # The data buffer to compress.
---@return data data # The data buffer.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L216)
function quiver.data.compress(data) end

---Decompress a given data buffer (DEFLATE).
---```lua
----- Create a table with no data in it.
---local data = {}
---
----- Insert some dummy data in it.
---for x = 1, 63 do
---    table.insert(data, 0)
---end
---
----- Insert some data of significance at the last index.
---table.insert(data, 1)
---
----- Check that the last value is 1.
---assert(data[64] == 1)
---
----- Get a data buffer user-data from Quiver.
---local data = quiver.data.new(data)
---
----- Compress the data.
---local data = quiver.data.compress(data)
---
----- Decompress the data.
---local data = quiver.data.decompress(data)
---
----- Get the data back as a Lua table.
---local data = data:get_buffer()
---
----- Check that the last value is 1.
---assert(data[64] == 1)
---
---```
---@param data data # The data buffer to decompress.
---@return data data # The data buffer.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L242)
function quiver.data.decompress(data) end

---Encode a given data buffer (Base64).
---@param data data # The data buffer to encode.
---@return data data # The data buffer.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L269)
function quiver.data.encode(data) end

---Decode a given data buffer (Base64).
---@param data data # The data buffer to decode.
---@return data data # The data buffer.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L294)
function quiver.data.decode(data) end

---Hash a given data buffer.
---@param data data # The data buffer to encode.
---@param data hash_kind? # OPTIONAL: The hash method. Default: CRC32.
---@return data data # The hash code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L322)
function quiver.data.hash(data,data) end

---Serialize a given Lua value as another format, in the form of a string.
---@param text any # Lua value to serialize.
---@param kind format_kind? # OPTIONAL: The format to serialize to. Default: JSON.
---@return string value # The value, in string form.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L373)
function quiver.data.serialize(text,kind) end

---Deserialize a given format string as a Lua value.
---@param text string # String to deserialize.
---@param kind format_kind? # OPTIONAL: The format to deserialize from. Default: JSON.
---@return any value # The value, in Lua value form.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L421)
function quiver.data.deserialize(text,kind) end

---Convert a given Lua value to a data buffer.
---@param data any # Lua value to convert to a data buffer.
---@param kind data_kind # The data kind to convert to.
---@return data value # The value, in data buffer form.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L477)
function quiver.data.to_data(data,kind) end

---Convert a given data buffer to a Lua value.
---@param data data # Data buffer to convert to a Lua value.
---@param kind data_kind # The data kind to convert to.
---@return number | string value # The value, in Lua value form.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L508)
function quiver.data.from_data(data,kind) end

---Get a file from the embed file.
---@param path string # Path to the file.
---@param binary boolean # Read as binary.
---@return string | data file # The file.
---
--- ---
---*Available with compile feature: `embed`.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L543)
function quiver.data.get_embed_file(path,binary) end

---Get a list of every file in the embed data.
---@return table list # The list of every file.
---
--- ---
---*Available with compile feature: `embed`.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/data.rs#L572)
function quiver.data.get_embed_list() end

---The socket API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L65)
---@class quiver.socket
quiver.socket = {}

---An unique handle to a TCP (stream) socket in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L83)
---@class socket_TCP_stream
socket_TCP_stream = {}

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L99)
function socket_TCP_stream:get() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L119)
function socket_TCP_stream:set() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L137)
function quiver.socket.new_TCP_stream() end

---An unique handle to a TCP (listen) socket in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L151)
---@class socket_TCP_listen
socket_TCP_listen = {}

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L167)
function socket_TCP:accept() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L186)
function quiver.socket.new_TCP_listen() end

---An unique handle to a UDP socket in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L198)
---@class socket_UDP
socket_UDP = {}

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L214)
function socket_UDP:connect() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L227)
function socket_UDP:get() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L247)
function socket_UDP:set() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L261)
function socket_UDP:get_at() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L281)
function socket_UDP:set_at() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/socket.rs#L302)
function quiver.socket.new_UDP() end

---The Discord API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/discord.rs#L74)
---@class quiver.discord
quiver.discord = {}

---An unique handle to a Discord client.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/discord.rs#L88)
---@class discord
discord = {}

---Set the rich presence.
---@param activity table # The rich presence.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/discord.rs#L145)
function discord:set_rich_presence(activity) end

---Clear the rich presence.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/discord.rs#L255)
function discord:clear_rich_presence() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/discord.rs#L268)
function discord:update() end

---Create a new Discord client.
---@param ID string # Discord App ID.
---@param name string? # OPTIONAL: The application name.
---@param command number? | string? | table? # OPTIONAL: The application command to run on the rich presence state.
---@return string | data file # The file.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/discord.rs#L308)
function quiver.discord.new(ID,name,command) end

---The file API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L64)
---@class quiver.file
quiver.file = {}

---Get the data of a file.
---```lua
----- Write "123" to the file "foo.txt". Since we don't want to write binary, pass false as the third argument.
---quiver.file.set("work/foo.txt", "123", false)
---
----- Read the data back. Again, since we know the file isn't binary, we pass false.
---local data = quiver.file.get("work/foo.txt", false)
---
---assert(data == "123")
---
---```
---@param path string # Path to file.
---@param binary boolean # Read as binary.
---@return string data # File data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L121)
function quiver.file.get_file(path,binary) end

---Set the data of a file.
---```lua
----- Write "123" to the file "foo.txt". Since we don't want to write binary, pass false as the third argument.
---quiver.file.set("work/foo.txt", "123", false)
---
----- Read the data back. Again, since we know the file isn't binary, we pass false.
---local data = quiver.file.get("work/foo.txt", false)
---
---assert(data == "123")
---
---```
---@param path string # Path to file.
---@param data string | data # Data to copy.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L149)
function quiver.file.set_file(path,data) end

---Move a file.
---@param source string # The source path.
---@param target string # The target path.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L179)
function quiver.file.move_file(source,target) end

---Copy a file.
---@param source string # The source path.
---@param target string # The target path.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L199)
function quiver.file.copy_file(source,target) end

---Remove a file.
---@param path string # The path to the file to remove.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L218)
function quiver.file.remove_file(path) end

---Remove a folder.
---@param path string # The path to the folder to remove.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L236)
function quiver.file.remove_path(path) end

---Set the state of the path sand-box.
---@param state boolean # The state of the path sand-box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L254)
function quiver.file.set_path_escape(state) end

---Set the file save call-back.
---@param call function # The call-back. Must accept a file-name and a data parameter, and return a boolean (true on success, false on failure).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L390)
function quiver.file.set_call_save_file(call) end

---Set the file load call-back.
---@param call function # The call-back. Must accept a file-name, and return a data buffer. Return anything else to indicate failure.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L410)
function quiver.file.set_call_load_file(call) end

---Set the file text save call-back.
---@param call function # The call-back. Must accept a file-name and a string parameter, and return a boolean (true on success, false on failure).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L430)
function quiver.file.set_call_save_text(call) end

---Set the file load call-back.
---@param call function # The call-back. Must accept a file-name, and return a string. Return anything else to indicate failure.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L450)
function quiver.file.set_call_load_text(call) end

---Check if a file does exist.
---```lua
----- Write "123" to the file "foo.txt". Since we don't want to write binary, pass false as the third argument.
---quiver.file.set("work/foo.txt", "123", false)
---
---assert(quiver.file.get_file_exist("work/foo.txt"))
---
---```
---@param path string # Path to file.
---@return boolean exist # True if file does exist, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L474)
function quiver.file.get_file_exist(path) end

---Check if a path does exist.
---```lua
----- Write "123" to the file "foo.txt". Since we don't want to write binary, pass false as the third argument.
---quiver.file.set("work/foo.txt", "123", false)
---
---assert(quiver.file.get_file_exist("work/foo.txt"))
---
---```
---@param path string # Path.
---@return boolean exist # True if path does exist, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L494)
function quiver.file.get_path_exist(path) end

---Check if a file's extension is the same as a given one.
---@param path string # Path to file.
---@param extension string # Extension. MUST include dot (.png, .wav, etc.).
---@return boolean check # True if file extension is the same as the given one, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L514)
function quiver.file.get_file_extension_check(path,extension) end

---Get the size of a file.
---@param path string # Path to file.
---@return number size # File size.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L535)
function quiver.file.get_file_size(path) end

---Get the extension of a file.
---@param path string # Path to file.
---@return string extension # File extension.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L554)
function quiver.file.get_file_extension(path) end

---Get the name of a file.
---@param path string # Path to file.
---@param extension boolean # File extension. If true, will return file name with the extension.
---@return string name # File name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L577)
function quiver.file.get_file_name(path,extension) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L598)
function quiver.file.get_absolute_path() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L615)
function quiver.file.get_previous_path() end

---Get the current work path.
---@return string path # Work path.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L635)
function quiver.file.get_work_directory() end

---Get the current application path.
---@return string path # Application path.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L652)
function quiver.file.get_application_directory() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L666)
function quiver.file.create_path() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L689)
function quiver.file.change_path() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L712)
function quiver.file.get_path_file() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L727)
function quiver.file.get_file_name_valid() end

---Scan a path.
---@param path string # Path to scan.
---@param filter string? # OPTIONAL: Extension filter. If filter is 'DIR', will include every directory in the result.
---@param recursive boolean # If true, recursively scan the directory.
---@param absolute boolean # If true, return path relatively.
---@return table list # File list.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L751)
function quiver.file.scan_path(path,filter,recursive,absolute) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L804)
function quiver.file.get_file_drop() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L815)
function quiver.file.get_file_drop_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/file.rs#L842)
function quiver.file.get_file_modification() end

---The general API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L70)
---@class quiver.general
quiver.general = {}

---Get the standard input.
---@return string input # The standard input.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L111)
function quiver.general.standard_input() end

---Load the standard Lua library.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L125)
function quiver.general.load_base() end

---Set the log level.
---@param level number # The log level.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L151)
function quiver.general.set_log_level(level) end

---Open an URL link.
---@param link string # The URL link.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L168)
function quiver.general.open_link(link) end

---Get the current time. Will count up since the initialization of the window.
---@return number time # Current time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L184)
function quiver.general.get_time() end

---Get the time in UNIX time-stamp format.
---@param add number? # OPTIONAL: Add (or subtract) by this amount.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L197)
function quiver.general.get_time_unix(add) end

---Get the current frame time.
---@return number frame_time # Current frame time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L214)
function quiver.general.get_frame_time() end

---Get the current frame rate.
---@return number frame_rate # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L227)
function quiver.general.get_frame_rate() end

---Set the current frame rate.
---@param frame_rate number # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L240)
function quiver.general.set_frame_rate(frame_rate) end

---Get the argument list.
---@return table list # The list of every argument.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L257)
function quiver.general.get_argument() end

---Get the system info.
---@return table info # The system info.
---
--- ---
---*Available with compile feature: `system_info`.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L274)
function quiver.general.get_system() end

---Get the currently in-use memory by the Lua VM.
---@return number memory # The currently in-use memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L292)
function quiver.general.get_memory() end

---Get the current info manifest.
---@return table info # The info manifest.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/general.rs#L307)
function quiver.general.get_info() end

---The collision API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L64)
---@class quiver.collision
quiver.collision = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L104)
function quiver.collision.get_box_2_box_2() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L118)
function quiver.collision.get_circle_circle() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L142)
function quiver.collision.get_circle_box_2() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L165)
function quiver.collision.get_circle_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L190)
function quiver.collision.get_point_box_2() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L204)
function quiver.collision.get_point_circle() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L227)
function quiver.collision.get_point_triangle() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L253)
function quiver.collision.get_point_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L278)
function quiver.collision.get_point_poly() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L299)
function quiver.collision.get_line_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L331)
function quiver.collision.get_box_2_box_2_difference() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L353)
function quiver.collision.get_sphere_sphere() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L377)
function quiver.collision.get_box_3_box_3() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L391)
function quiver.collision.get_box_3_sphere() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L414)
function quiver.collision.get_ray_sphere() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L455)
function quiver.collision.get_ray_box_3() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L496)
function quiver.collision.get_ray_triangle() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/collision.rs#L544)
function quiver.collision.get_ray_quad() end

---The shader API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L64)
---@class quiver.shader
quiver.shader = {}

---An unique handle for a shader in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L81)
---@class shader
shader = {}

---Create a new shader resource.
---@param v_path string # Path to .vs file.
---@param f_path string # Path to .fs file.
---@return shader shader # Shader resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L100)
function quiver.shader.new(v_path,f_path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L139)
function quiver.shader.new_from_memory() end

---TO-DO
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L189)
function shader:begin(call) end

---TO-DO
---@param name string # TO-DO
---@return number location # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L214)
function shader:get_location_name(name) end

---TO-DO
---@param name string # TO-DO
---@return number location # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L231)
function shader:get_location_attribute_name(name) end

---TO-DO
---@param location number # TO-DO
---@return number location # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L248)
function shader:get_location(location) end

---TO-DO
---@param location number # TO-DO
---@param value number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L263)
function shader:set_location(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L282)
function shader:set_shader_integer(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L301)
function shader:set_shader_decimal(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value vector_3 # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L320)
function shader:set_shader_vector_3(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value vector_4 # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/shader.rs#L340)
function shader:set_shader_vector_4(location,value) end

---The image API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L68)
---@class quiver.image
quiver.image = {}

---An unique handle for a image in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L104)
---@class image
---@field shape_x number # Shape of the image (X).
---@field shape_y number # Shape of the image (Y).
image = {}

---Get a texture resource from an image.
---@return texture texture # Texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L127)
function image:to_texture() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L138)
function image:power_of_two() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L156)
function image:crop() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L172)
function image:crop_alpha() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L184)
function image:crop_alpha() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L205)
function image:blur_gaussian() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L217)
function image:kernel_convolution() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L232)
function image:resize() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L253)
function image:extend() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L279)
function image:mipmap() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L291)
function image:dither() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L306)
function image:flip() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L322)
function image:rotate() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L336)
function image:color_tint() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L350)
function image:color_invert() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L362)
function image:color_gray_scale() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L374)
function image:color_contrast() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L386)
function image:color_contrast() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L406)
function image:get_alpha_border() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L418)
function image:get_alpha_border() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L432)
function image:draw_pixel() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L447)
function image:draw_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L472)
function image:draw_circle() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L490)
function image:draw_circle_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L508)
function image:draw_box_2() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L523)
function image:draw_box_2_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L541)
function image:draw_triangle() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L571)
function image:draw_triangle_line() end

---Create a new image resource.
---@param path string # Path to image file.
---@return image image # Image resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L609)
function quiver.image.new(path) end

---Create a new image resource, from memory.
---@param data data # The data buffer.
---@param kind string # The kind of image file (.png, etc.).
---@return image image # Image resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L643)
function quiver.image.new_from_memory(data,kind) end

---Create a new image resource, from the current screen buffer.
---@return image image # Image resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L673)
function quiver.image.new_from_screen() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L695)
function quiver.image.new_color() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L720)
function quiver.image.new_gradient_linear() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L755)
function quiver.image.new_gradient_radial() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L790)
function quiver.image.new_gradient_square() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L825)
function quiver.image.new_check() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L862)
function quiver.image.new_white_noise() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L886)
function quiver.image.new_perlin_noise() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L920)
function quiver.image.new_cellular() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/image.rs#L944)
function quiver.image.new_text() end

---The font API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/font.rs#L64)
---@class quiver.font
quiver.font = {}

---Set the vertical space between each line-break.
---@param space number # Vertical space.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/font.rs#L88)
function quiver.font.set_text_line_space(space) end

---An unique handle to a font in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/font.rs#L101)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/font.rs#L123)
function font:draw(label,point,scale,space,color) end

---Measure the size of a given text on screen, with a given font.
---@param label string # Label of font to measure.
---@param scale number # Scale of font to measure.
---@param space number # Space of font to measure.
---@return number size_x # Size of text (X).
---@return number size_y # Size of text (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/font.rs#L161)
function font:measure_text(label,scale,space) end

---Create a new font resource.
---@param path string # Path to font file.
---@param size number # Size for font.
---@return font font # Font resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/font.rs#L190)
function quiver.font.new(path,size) end

---Create a new font resource, from memory.
---@param data data # The data buffer.
---@param kind string # The kind of font file (.ttf, etc.).
---@param size number # Size for font.
---@return font font # Font resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/font.rs#L221)
function quiver.font.new_from_memory(data,kind,size) end

---Create a new font resource, from the default font.
---@param size number # Size for font.
---@return font font # Font resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/font.rs#L259)
function quiver.font.new_default(size) end

---The automation API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/automation.rs#L64)
---@class quiver.automation
quiver.automation = {}

---An unique handle to an automation event list.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/automation.rs#L80)
---@class automation_event
automation_event = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/automation.rs#L92)
function quiver.automation.new() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/automation.rs#L131)
function automation_event:save() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/automation.rs#L146)
function automation_event:set_active() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/automation.rs#L158)
function automation_event:set_frame() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/automation.rs#L170)
function automation_event:start() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/automation.rs#L182)
function automation_event:stop() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/automation.rs#L194)
function automation_event:play() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/automation.rs#L213)
function automation_event:get_event() end

---The window API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L68)
---@class quiver.window
quiver.window = {}

---Create a new native OS file dialog.
---@param kind file_dialog_kind # The kind of file dialog to display.
---@param title string? # OPTIONAL: Window title.
---@param path string? # OPTIONAL: Default file path.
---@param name string? # OPTIONAL: Default file name.
---@param filter table? # OPTIONAL: Extension filter table. Must be in { extension = { ".txt", ".ini" } format.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L176)
function quiver.window.file_dialog(kind,title,path,name,filter) end

---Create a new native OS text dialog.
---@param kind text_dialog_kind? # OPTIONAL: The kind of text dialog to display.
---@param title string? # OPTIONAL: Window title.
---@param label string? # OPTIONAL: Window label.
---@param button text_dialog_button? # OPTIONAL: Window button layout.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L270)
function quiver.window.text_dialog(kind,title,label,button) end

---Get if the window should close.
---@return boolean close # True if the window should close.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L320)
function quiver.window.get_close() end

---Get the state of the window (full-screen).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L334)
function quiver.window.get_fullscreen() end

---Get the state of the window (hidden).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L348)
function quiver.window.get_hidden() end

---Get the state of the window (minimize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L362)
function quiver.window.get_minimize() end

---Get the state of the window (maximize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L376)
function quiver.window.get_maximize() end

---Get the state of the window (focus).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L390)
function quiver.window.get_focus() end

---Get the state of the window (resize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L404)
function quiver.window.get_resize() end

---Get the state of a window flag.
---@param flag window_flag # Window flag.
---@return boolean state # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L421)
function quiver.window.get_state(flag) end

---Set the state of a window flag.
---@param flag window_flag # Window flag.
---@param state boolean # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L436)
function quiver.window.set_state(flag,state) end

---Set the window to full-screen mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L451)
function quiver.window.set_fullscreen() end

---Set the window to border-less mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L461)
function quiver.window.set_borderless() end

---Minimize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L471)
function quiver.window.set_minimize() end

---Maximize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L481)
function quiver.window.set_maximize() end

---Restore the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L491)
function quiver.window.set_restore() end

---Set the window icon.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L501)
function quiver.window.set_icon() end

---Set the window name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L520)
function quiver.window.set_name() end

---Set the window point.
---@param point vector_2 # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L539)
function quiver.window.set_point(point) end

---Set the window monitor.
---@param index number # Index of monitor to move window to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L558)
function quiver.window.set_screen(index) end

---Set the minimum window shape.
---@param shape vector_2 # Minimum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L575)
function quiver.window.set_shape_min(shape) end

---Set the maximum window shape.
---@param shape vector_2 # Maximum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L594)
function quiver.window.set_shape_max(shape) end

---Set the current window shape.
---@param shape vector_2 # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L613)
function quiver.window.set_shape(shape) end

---Set the window alpha.
---@param alpha number # Alpha of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L632)
function quiver.window.set_alpha(alpha) end

---Focus the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L642)
function quiver.window.set_focus() end

---Get the shape of the window.
---@return number shape_x # Shape of the window (X).
---@return number shape_y # Shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L660)
function quiver.window.get_shape() end

---Get the shape of the current render view.
---@return number shape_x # Shape of the render view (X).
---@return number shape_y # Shape of the render view (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L675)
function quiver.window.get_render_shape() end

---Get the available monitor amount.
---@return number count # Monitor count.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L689)
function quiver.window.get_screen_count() end

---Get the current active monitor, where the window is.
---@return number index # Current active monitor index.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L703)
function quiver.window.get_screen_focus() end

---Get the point of the given monitor.
---@param index number # Index of the monitor.
---@return number point_x # Point of the monitor (X).
---@return number point_y # Point of the monitor (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L721)
function quiver.window.get_screen_point(index) end

---Get the shape of the given monitor.
---@param index number # Index of the monitor.
---@return number shape_x # Shape of the window (X).
---@return number shape_y # Shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L742)
function quiver.window.get_screen_shape(index) end

---Get the physical shape of the given monitor.
---@param index number # Index of the monitor.
---@return number shape_x # Physical shape of the window (X).
---@return number shape_y # Physical shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L760)
function quiver.window.get_screen_shape_physical(index) end

---Get the refresh rate of the given monitor.
---@param index number # Index of the monitor.
---@return number rate # Refresh rate of the monitor.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L782)
function quiver.window.get_screen_rate(index) end

---Get the point of the window.
---@return number point_x # Point of the window (X).
---@return number point_y # Point of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L797)
function quiver.window.get_point() end

---Get the DPI scale of the window.
---@return number scale_x # Scale of the window (X).
---@return number scale_y # Scale of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L816)
function quiver.window.get_scale() end

---Get the name of the given monitor.
---@param index number # Index of the monitor.
---@return string name # Name of the monitor.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L837)
function quiver.window.get_screen_name(index) end

---Get a screen-shot of the current frame.
---@param path string # Path to save the screen-shot to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/window.rs#L857)
function quiver.window.get_screen_shot(path) end

---The music API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L64)
---@class quiver.music
quiver.music = {}

---An unique handle for music in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L81)
---@class music
music = {}

---Create a new music resource.
---@param path string # Path to music file.
---@return music music # Music resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L101)
function quiver.music.new(path) end

---Create a new music resource, from memory.
---@param data data # The data buffer.
---@param kind string # The kind of music file (.png, etc.).
---@return music music # Music resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L134)
function quiver.music.new_from_memory(data,kind) end

---Play the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L173)
function music:play() end

---Check if music is currently playing.
---@return boolean state # State of the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L188)
function music:get_playing() end

---Stop the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L195)
function music:stop() end

---Pause the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L203)
function music:pause() end

---Resume the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L211)
function music:resume() end

---Set volume for the music. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L226)
function music:set_volume(volume) end

---Set pitch for the music.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L241)
function music:set_pitch(pitch) end

---Set pan for the music. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L256)
function music:set_pan(pan) end

---Update the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L264)
function music:update() end

---Set position for the music.
---@param position number # Current position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L279)
function music:set_position(position) end

---Get time length for the music.
---@return number length # Time length.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L294)
function music:get_length() end

---Get time played for the music.
---@return number played # Time played.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/source/rust/base/music.rs#L308)
function music:get_played() end

