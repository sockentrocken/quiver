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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L65)
---@class quiver.sound
quiver.sound = {}

---An unique handle for sound in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L86)
---@class sound
sound = {}

---Create a new sound resource.
---@param path string # Path to sound file.
---@param alias number # OPTIONAL: The total sound alias count to load for the sound.
---@return sound sound # Sound resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L105)
function quiver.sound.new(path,alias) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L138)
function quiver.sound.new_from_memory() end

---Create a sound alias.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L205)
function sound:create_alias() end

---Remove a sound alias.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L215)
function sound:remove_alias() end

---Clear every sound alias.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L229)
function sound:remove_alias() end

---Play the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L242)
function sound:play() end

---Check if sound is currently playing.
---@return boolean state # State of the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L266)
function sound:get_playing() end

---Stop the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L283)
function sound:stop() end

---Pause the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L300)
function sound:pause() end

---Resume the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L317)
function sound:resume() end

---Set volume for the sound. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L343)
function sound:set_volume(volume) end

---Set pitch for the sound.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L372)
function sound:set_pitch(pitch) end

---Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L401)
function sound:set_pan(pan) end

---The input API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L64)
---@class quiver.input
quiver.input = {}

---The board input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L82)
---@class quiver.input.board
quiver.input.board = {}

---The mouse input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L100)
---@class quiver.input.mouse
quiver.input.mouse = {}

---The pad input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L126)
---@class quiver.input.pad
quiver.input.pad = {}

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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L169)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L189)
function quiver.input.board.get_clipboard_text() end

---Get the last unicode glyph in the queue.
---@return number key_code # Key-code. If 0, queue is empty.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L209)
function quiver.input.board.get_key_code_queue() end

---Get the last unicode glyph in the queue.
---@return number uni_code # Uni-code. If 0, queue is empty.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L223)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L241)
function quiver.input.board.get_name(board) end

---Get the state of an input (up).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L266)
function quiver.input.board.get_up(board) end

---Get the state of an input (down).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L280)
function quiver.input.board.get_down(board) end

---Get the state of an input (press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L294)
function quiver.input.board.get_press(board) end

---Get the state of an input (repeat-press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L308)
function quiver.input.board.get_press_repeat(board) end

---Get the state of an input (release).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L322)
function quiver.input.board.get_release(board) end

---Set the active state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L338)
function quiver.input.mouse.set_active(state) end

---Set the hidden state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L360)
function quiver.input.mouse.set_hidden(state) end

---Get the hidden state of the mouse.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L382)
function quiver.input.mouse.get_hidden() end

---Check if the mouse is currently over the screen.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L396)
function quiver.input.mouse.get_screen() end

---Get the current point of the mouse.
---@return number point_x # The point of the mouse (X).
---@return number point_y # The point of the mouse (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L411)
function quiver.input.mouse.get_point() end

---Set the current point of the mouse.
---@param point vector_2 # The point of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L428)
function quiver.input.mouse.set_point(point) end

---Get the current delta (i.e. mouse movement) of the mouse.
---@return number delta_x # The delta of the mouse (X).
---@return number delta_y # The delta of the mouse (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L447)
function quiver.input.mouse.get_delta() end

---Set the current shift of the mouse.
---@param shift vector_2 # The shift of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L464)
function quiver.input.mouse.set_shift(shift) end

---Set the current scale of the mouse.
---@param scale vector_2 # The scale of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L482)
function quiver.input.mouse.set_scale(scale) end

---Set the current cursor of the mouse.
---@param cursor cursor_mouse # The cursor of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L500)
function quiver.input.mouse.set_cursor(cursor) end

---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return number delta_x # The delta of the mouse wheel (X).
---@return number delta_y # The delta of the mouse wheel (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L518)
function quiver.input.mouse.get_wheel() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L532)
function quiver.input.mouse.get_mouse_queue() end

---Get the state of an input (up).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L556)
function quiver.input.mouse.get_up(mouse) end

---Get the state of an input (down).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L570)
function quiver.input.mouse.get_down(mouse) end

---Get the state of an input (press).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L584)
function quiver.input.mouse.get_press(mouse) end

---Get the state of an input (release).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L598)
function quiver.input.mouse.get_release(mouse) end

---Get the state of a pad.
---@param index number # The index of the pad to check for.
---@return boolean state # The state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L617)
function quiver.input.pad.get_state(index) end

---Get the name of a pad.
---@param index number # The index of the pad to check for.
---@return string name # The name of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L634)
function quiver.input.pad.get_name(index) end

---Get the state of an input (press).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L654)
function quiver.input.pad.get_press(pad) end

---Get the state of an input (down).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L668)
function quiver.input.pad.get_down(pad) end

---Get the state of an input (release).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L682)
function quiver.input.pad.get_release(pad) end

---Get the state of an input (up).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L696)
function quiver.input.pad.get_up(pad) end

---Get the last pad button press.
---@return input_pad input # The last pad button press.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L710)
function quiver.input.pad.get_queue() end

---Get the axis count of a pad.
---@param index number # The index of the pad to check for.
---@return number axis_count # The axis count of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L727)
function quiver.input.pad.get_axis_count(index) end

---Get the axis state of a pad.
---@param index number # The index of the pad to check for.
---@param axis number # The axis of the pad to check for.
---@return number axis_state # The axis state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L745)
function quiver.input.pad.get_axis_state(index,axis) end

---Set the rumble of a pad.
---@param index number # The index of the pad to rumble.
---@param motor_a number # The intensity of the L. rumble motor.
---@param motor_b number # The intensity of the R. rumble motor.
---@param duration number # The duration of the rumble.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L762)
function quiver.input.pad.set_rumble(index,motor_a,motor_b,duration) end

---The Rapier API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L71)
---@class quiver.rapier
quiver.rapier = {}

---An unique handle for a Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L85)
---@class rapier
rapier = {}

---Create a new Rapier simulation.
---@return rapier rapier # Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L113)
function quiver.rapier.new() end

---Cast a ray.
---@param ray ray # Ray to cast.
---@param length number # Ray length.
---@param solid boolean # TO-DO
---@param exclude_rigid table # TO-DO
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L161)
function rapier:cast_ray(ray,length,solid,exclude_rigid) end

---Cast a ray, and also get the normal information..
---@param ray ray # Ray to cast.
---@param length number # Ray length.
---@param solid boolean # TO-DO
---@param exclude_rigid table # TO-DO
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L219)
function rapier:cast_ray_normal(ray,length,solid,exclude_rigid) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L273)
function rapier:test_intersect_cuboid_cuboid() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L317)
function rapier:test_intersect_cuboid() end

---Get the shape of a collider (cuboid).
---@param collider table # Collider handle.
---@return number half_shape_x # Half-shape of the cuboid. (X).
---@return number half_shape_y # Half-shape of the cuboid. (Y).
---@return number half_shape_z # Half-shape of the cuboid. (Z).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L388)
function rapier:get_collider_shape_cuboid(collider) end

---Set the shape of a collider (cuboid).
---@param collider table # Collider handle.
---@param half_shape vector_3 # Half-shape of cuboid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L424)
function rapier:set_collider_shape_cuboid(collider,half_shape) end

---Get the parent of a collider.
---@param collider table # Collider handle.
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L465)
function rapier:get_collider_parent(collider) end

---Get the position of a collider.
---@param collider table # Collider handle.
---@return number position_x # Collider position (X).
---@return number position_y # Collider position (Y).
---@return number position_z # Collider position (Z).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L496)
function rapier:get_collider_position(collider) end

---Set the position of a collider.
---@param collider table # Collider handle.
---@param position vector_3 # Collider position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L523)
function rapier:set_collider_position(collider,position) end

---Set the rotation of a collider.
---@param collider table # Collider handle.
---@param rotation vector_3 # Collider rotation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L551)
function rapier:set_collider_rotation(collider,rotation) end

---Set the sensor state of a collider.
---@param collider table # Collider handle.
---@param sensor boolean # Collider sensor state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L580)
function rapier:set_collider_sensor(collider,sensor) end

---Remove a collider.
---@param collider table # Collider handle.
---@param wake_parent boolean # Whether or not to wake up the rigid body parent this collider is bound to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L609)
function rapier:collider_remove(collider,wake_parent) end

---Remove a rigid body.
---@param rigid_body table # Rigid body handle.
---@param remove_collider boolean # Whether or not to remove every collider this rigid body is bound to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L636)
function rapier:rigid_body_remove(rigid_body,remove_collider) end

---Create a character controller.
---@return table character_controller # Character controller.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L664)
function rapier:character_controller() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L675)
function rapier:set_character_controller_up_vector() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L692)
function rapier:set_character_controller_slope() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L709)
function rapier:set_character_auto_step() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L756)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L792)
function rapier:character_controller_move(step,character,collider,translation) end

---Create a rigid body.
---@param kind rigid_body_kind # Rigid body kind.
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L841)
function rapier:rigid_body(kind) end

---Get the user data of a rigid_body.
---@param rigid_body userdata # Rigid body handle.
---@return number user_data # Rigid body user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L865)
function rapier:get_rigid_body_user_data(rigid_body) end

---Set the user data of a rigid_body.
---@param rigid_body userdata # Rigid body handle.
---@param user_data number # Rigid body user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L891)
function rapier:set_rigid_body_user_data(rigid_body,user_data) end

---Set the position of a rigid_body.
---@param rigid_body userdata # rigid_body handle.
---@param position vector_3 # rigid_body position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L918)
function rapier:set_rigid_body_position(rigid_body,position) end

---Set the rotation of a rigid_body.
---@param rigid_body table # rigid_body handle.
---@param rotation vector_3 # rigid_body rotation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L947)
function rapier:set_rigid_body_rotation(rigid_body,rotation) end

---Get the user data of a collider.
---@param collider userdata # Collider handle.
---@return number user_data # Collider user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L982)
function rapier:get_collider_user_data(collider) end

---Set the user data of a collider.
---@param collider userdata # Collider handle.
---@param user_data number # Collider user data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1005)
function rapier:set_collider_user_data(collider,user_data) end

---Create a collider builder (cuboid).
---@param half_shape vector_3 # Half-shape of cuboid.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1036)
function rapier:collider_builder_cuboid(half_shape) end

---Create a collider builder (tri-mesh).
---@param point_table table # The point array table.
---@param index_table table # The index array table.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1063)
function rapier:collider_builder_tri_mesh(point_table,index_table) end

---Create a collider builder (convex hull).
---@param vector_table table # A vector_3 vertex array table.
---@return table collider_builer # Collider builder.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1111)
function rapier:collider_builder_convex_hull(vector_table) end

---Step the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1136)
function rapier:step() end

---Render the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L1174)
function rapier:debug_render() end

---The Steam API.
---
--- ---
---*Available with compile feature: `steam`.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L64)
---@class quiver.steam
quiver.steam = {}

---An unique handle to a Steam client.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L78)
---@class steam
steam = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L136)
function steam:update() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L152)
function steam:get_app_ID() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L163)
function steam:get_IP_country() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L174)
function steam:get_UI_language() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L185)
function steam:get_server_time() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L196)
function steam:set_overlay_position() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L230)
function steam:set_message() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L254)
function steam:get_message() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L276)
function steam:get_app_install() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L287)
function steam:get_DLC_install() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L298)
function steam:get_app_subscribe() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L309)
function steam:get_VAC_ban() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L320)
function steam:get_cyber_cafe() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L331)
function steam:get_low_violence() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L342)
function steam:get_subscribe() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L353)
function steam:get_app_build_ID() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L364)
function steam:get_app_install_directory() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L375)
function steam:get_app_owner() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L386)
function steam:get_game_language_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L397)
function steam:get_game_language() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L408)
function steam:get_beta_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L419)
function steam:get_launch_command_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L434)
function steam:get_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L445)
function steam:activate_overlay() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L457)
function steam:activate_overlay_link() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L469)
function steam:activate_overlay_store() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L490)
function steam:activate_overlay_user() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L506)
function steam:activate_invite_dialog() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L528)
function steam:get_steam_ID() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L539)
function steam:get_level() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L548)
function steam:get_log_on() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L563)
function steam:get_leader_board() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L583)
function steam:get_or_create_leader_board() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L616)
function steam:upload_leader_board_score() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L675)
function steam:get_leader_board_show_kind() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L707)
function steam:get_leader_board_sort_kind() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L738)
function steam:get_leader_board_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L765)
function steam:get_leader_board_entry_count() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L792)
function steam:pull_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L805)
function steam:push_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L818)
function steam:reset_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L838)
function steam:get_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L860)
function steam:set_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L884)
function steam:get_achievement() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L903)
function steam:get_achievement_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L920)
function steam:set_achievement() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L946)
function steam:get_achievement_percent() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L965)
function steam:get_achievement_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L984)
function steam:get_achievement_() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1003)
function steam:get_achievement_hidden() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1022)
function steam:get_achievement_icon() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1051)
function steam:get_session_user() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1064)
function steam:get_session_client_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1077)
function steam:get_session_client_form_factor() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1102)
function steam:get_session_client_resolution() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1122)
function steam:invite_session() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1146)
function steam:set_cloud_app() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1158)
function steam:get_cloud_app() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1169)
function steam:get_cloud_account() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1180)
function steam:get_file_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1200)
function steam:file_delete() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1213)
function steam:file_forget() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1226)
function steam:file_exist() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1239)
function steam:get_file_persist() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1252)
function steam:get_file_time_stamp() end

---Create a new Steam client.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/steam.rs#L1274)
function quiver.steam.new() end

---The video API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/video.rs#L72)
---@class quiver.video
quiver.video = {}

---An unique handle to a video in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/video.rs#L90)
---@class video
video = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/video.rs#L155)
function video:update() end

---Draw a texture.
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/video.rs#L178)
function video:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # TO-DO
---@param box_b box_2 # TO-DO
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/video.rs#L203)
function video:draw_pro(box_a,box_b,point,angle,color) end

---Create a new video resource.
---@param path string # Path to video file.
---@return video video # Video resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/video.rs#L238)
function quiver.video.new(path) end

---The request API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/request.rs#L62)
---@class quiver.request
quiver.request = {}

---TO-DO
---```lua
---local link = "https://raw.githubusercontent.com/sockentrocken/quiver/refs/heads/main/test/asset/sample.txt"
---
----- Get the data from the link. As we know it's not binary, we pass false to the function.
---local response = quiver.request.get(link, false)
---
---assert(response == "Hello, world!")
---
---```
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/request.rs#L84)
function quiver.request.get() end

---TO-DO
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
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/request.rs#L115)
function quiver.request.post() end

---The texture API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L71)
---@class quiver.texture
quiver.texture = {}

---An unique handle for a texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L198)
---@class texture
---@field shape_x number # Shape of the texture (X).
---@field shape_y number # Shape of the texture (Y).
texture = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L215)
function texture:to_image() end

---Set the mipmap for a texture.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L226)
function texture:set_mipmap() end

---Set the filter for a texture.
---@param filter texture_filter # Texture filter.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L243)
function texture:set_filter(filter) end

---Set the wrap for a texture.
---@param wrap texture_wrap # Texture wrap.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L260)
function texture:set_wrap(wrap) end

---Draw a texture.
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L280)
function texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # TO-DO
---@param box_b box_2 # TO-DO
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L301)
function texture:draw_pro(box_a,box_b,point,angle,color) end

---Draw a billboard texture.
---@param camera camera_3d # TO-DO
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L332)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L361)
function texture:draw_billboard_pro(camera,source,point,up,scale,origin,angle,color) end

---Create a new texture resource.
---@param path string # Path to texture file.
---@return texture texture # Texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L400)
function quiver.texture.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L424)
function quiver.texture.new_from_memory() end

---An unique handle for a render texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L474)
---@class render_texture
---@field shape_x number # Shape of the texture (X).
---@field shape_y number # Shape of the texture (Y).
render_texture = {}

---Initialize drawing to the render texture.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L494)
function render_texture:begin(call) end

---Draw a texture.
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L519)
function render_texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # TO-DO
---@param box_b box_2 # TO-DO
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L543)
function render_texture:draw_pro(box_a,box_b,point,angle,color) end

---Create a new render texture resource.
---@param shape vector_2 # TO-DO
---@return render_texture render_texture # Render texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L577)
function quiver.render_texture.new(shape) end

---The ZIP API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L64)
---@class quiver.zip
quiver.zip = {}

---An unique handle to a ZIP in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L78)
---@class zip
zip = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L91)
function zip:get_file() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L129)
function zip:get_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L142)
function zip:is_file() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L156)
function zip:is_path() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L170)
function zip:is_link() end

---Create a new ZIP resource.
---@param path string # Path to ZIP file.
---@return zip zip # ZIP resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/zip.rs#L193)
function quiver.zip.new(path) end

---The model API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L67)
---@class quiver.model
quiver.model = {}

---An unique handle for a model in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L105)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L128)
function quiver.model.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L166)
function model:insert_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L181)
function model:remove_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L193)
function model:clear_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L206)
function model:set_transform_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L228)
function model:set_transform_list_batch() end

---Bind a texture to the model.
---@param index number # TO-DO
---@param which number # TO-DO
---@param texture texture # Texture to bind to model.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L256)
function model:bind(index,which,texture) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L291)
function model:draw_mesh() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L319)
function model:draw_mesh_instance() end

---Draw the model.
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L345)
function model:draw(point,scale,color) end

---Draw the model (wire-frame).
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L368)
function model:draw_wire(point,scale,color) end

---Draw the model with a transformation.
---@param point vector_3 # TO-DO
---@param angle vector_3 # TO-DO
---@param scale vector_3 # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L392)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L425)
function model:get_box_3() end

---Get the vertex data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Vector3 table.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L450)
function model:mesh_vertex(index) end

---Get the index data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Number table.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L468)
function model:mesh_index(index) end

---Get the triangle count of a specific mesh in the model.
---@param index number # Index of mesh.
---@return number count # Triangle count.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L497)
function model:mesh_triangle_count(index) end

---An unique handle for a model animation in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L513)
---@class model_animation
model_animation = {}

---Create a new ModelAnimation resource.
---@param path string # Path to model file.
---@return model_animation model_animation # ModelAnimation resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L531)
function quiver.model_animation.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L578)
function model_animation:get_bone_() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L596)
function model_animation:get_bone_() end

---Update model with new model animation data.
---@param model model # TO-DO
---@param frame number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L619)
function model_animation:update(model,frame) end

---The drawing API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L80)
---@class quiver.draw
quiver.draw = {}

---Initialize drawing to the screen.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L107)
function quiver.draw.begin(call) end

---Initialize drawing (blend mode) to the screen.
---@param call function # The draw code.
---@param mode function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L130)
function quiver.draw.begin_blend(call,mode) end

---Initialize drawing (scissor mode) to the screen.
---@param call function # The draw code.
---@param view box_2 # The clip test region.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L152)
function quiver.draw.begin_scissor(call,view) end

---Clear the screen with a color.
---@param color color # The color to use for clearing.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L179)
function quiver.draw.clear(color) end

---The 3D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L195)
---@class quiver.draw_3d
quiver.draw_3d = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L230)
function quiver.draw_3d.get_matrix_projection() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L250)
function quiver.draw_3d.get_matrix_model_view() end

---Update the 3D camera (pro).
---@param camera camera_3d # The camera to update.
---@param position vector_3 # TO-DO
---@param rotation vector_3 # TO-DO
---@param zoom number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L276)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L325)
function quiver.draw_3d.get_screen_to_world(camera,point,shape) end

---Get a 2D screen-space point for a 3D world-space point.
---@param camera camera_3d # The current camera.
---@param point vector_3 # The world-space point.
---@param shape vector_2 # The size of the view-port.
---@return number point_x # The 2D screen-space point (X).
---@return number point_y # The 2D screen-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L368)
function quiver.draw_3d.get_world_to_screen(camera,point,shape) end

---Initialize the 3D draw mode.
---@param call function # The draw code.
---@param camera camera_3d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L399)
function quiver.draw_3d.begin(call,camera) end

---Draw a grid.
---@param slice number # The slice count of the grid.
---@param space number # The space shift of the grid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L422)
function quiver.draw_3d.draw_grid(slice,space) end

---Draw a cube.
---@param point vector_3 # The point of the cube.
---@param shape vector_3 # The shape of the cube.
---@param color color # The color of the cube.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L440)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a ball.
---@param point vector_3 # The point of the ball.
---@param shape number # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L465)
function quiver.draw_3d.draw_ball(point,shape,color) end

---Draw a 3D box.
---@param shape box_3 # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L485)
function quiver.draw_3d.draw_box_3(shape,color) end

---Draw a ray.
---@param ray ray # The ray.
---@param color color # The color of the ray.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L505)
function quiver.draw_3d.draw_ray(ray,color) end

---Draw a line.
---@param point_a vector_3 # The point A of the line.
---@param point_b vector_3 # The point B of the line.
---@param color color # The color of the line.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L526)
function quiver.draw_3d.draw_line(point_a,point_b,color) end

---Set the current state of backface culling.
---@param state boolean # The new state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L549)
function quiver.draw_3d.set_backface_cull(state) end

---TO-DO
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L570)
function quiver.draw.begin_quad(call) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L597)
function quiver.draw.draw_quad_color() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L612)
function quiver.draw.draw_quad_normal() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L627)
function quiver.draw.draw_quad_coordinate() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L642)
function quiver.draw.draw_quad_vertex() end

---The 2D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L657)
---@class quiver.draw_2d
quiver.draw_2d = {}

---Get a world-space point for a 2D screen-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The screen-space point.
---@return number point_x # The 2D world-space point (X).
---@return number point_y # The 2D world-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L699)
function quiver.draw_2d.get_screen_to_world(camera,point) end

---Get a screen-space point for a 2D world-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The world-space point.
---@return number point_x # The 2D screen-space point (X).
---@return number point_y # The 2D screen-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L728)
function quiver.draw_2d.get_world_to_screen(camera,point) end

---Initialize the 2D draw mode.
---@param call function # The draw code.
---@param camera camera_2d # The 2D camera.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L753)
function quiver.draw_2d.begin(call,camera) end

---Draw pixel.
---@param point vector_2 # The point of the pixel.
---@param color color # The color of the pixel.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L777)
function quiver.draw_2d.draw_pixel(point,color) end

---Draw a line.
---@param point_a vector_2 # The point A of the line.
---@param point_b vector_2 # The point B of the line.
---@param thick number # The thickness of the line.
---@param color color # The color of the line.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L800)
function quiver.draw_2d.draw_line(point_a,point_b,thick,color) end

---Draw text.
---@param label string # The label of the text.
---@param point vector_2 # The point of the text.
---@param scale number # The angle of the text.
---@param color color # The color of the text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L827)
function quiver.draw_2d.draw_text(label,point,scale,color) end

---Draw a circle.
---@param point vector_2 # TO-DO
---@param radius number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L859)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L887)
function quiver.draw_2d.draw_circle_sector(point,radius,begin_angle,close_angle,segment_count,color) end

---Draw 2D box.
---@param shape box_2 # The shape of the box.
---@param point vector_2 # The point of the box.
---@param angle number # The angle of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L927)
function quiver.draw_2d.draw_box_2(shape,point,angle,color) end

---Draw 2D box with a gradient (X-direction).
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A of the box.
---@param color_b color # The color B of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L953)
function quiver.draw_2d.draw_box_2_gradient_x(shape,color_a,color_b) end

---Draw 2D box with a gradient (Y-direction).
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A of the box.
---@param color_b color # The color B of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L986)
function quiver.draw_2d.draw_box_2_gradient_y(shape,color_a,color_b) end

---Draw 2D box with a 4-point gradient.
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A (T.L.) of the box.
---@param color_b color # The color B (B.L.) of the box.
---@param color_c color # The color C (T.R.) of the box.
---@param color_d color # The color D (B.R.) of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1021)
function quiver.draw_2d.draw_box_2_gradient(shape,color_a,color_b,color_c,color_d) end

---Draw 2D box (out-line).
---@param shape box_2 # The shape of the box.
---@param thick number # The thickness of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1061)
function quiver.draw_2d.draw_box_2_line(shape,thick,color) end

---Draw 2D box (round).
---@param shape box_2 # The shape of the box.
---@param round number # The roundness of the box.
---@param count number # The segment count of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1087)
function quiver.draw_2d.draw_box_2_round(shape,round,count,color) end

---Draw 2D box (out-line, round).
---@param shape box_2 # The shape of the box.
---@param round number # The roundness of the box.
---@param count number # The segment count of the box.
---@param thick number # The thickness of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1114)
function quiver.draw_2d.draw_box_2_line_round(shape,round,count,thick,color) end

---Draw 2D triangle.
---@param point_a vector_2 # The point A of the triangle.
---@param point_b vector_2 # The point B of the triangle.
---@param point_c vector_2 # The point C of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1140)
function quiver.draw_2d.draw_triangle(point_a,point_b,point_c,color) end

---Draw 2D triangle (out-line).
---@param point_a vector_2 # The point A of the triangle.
---@param point_b vector_2 # The point B of the triangle.
---@param point_c vector_2 # The point C of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L1168)
function quiver.draw_2d.draw_triangle_line(point_a,point_b,point_c,color) end

---The data API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L63)
---@class quiver.data
quiver.data = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L146)
function data:foo() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L155)
function data:get_buffer() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L164)
function data:get_slice() end

---TO-DO
---```lua
----- Create a table with no data in it.
---local data = {}
---
----- Insert some dummy data in it.
---for x = 1, 63 do
---	table.insert(data, 0)
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
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L191)
function quiver.data.compress() end

---TO-DO
---```lua
----- Create a table with no data in it.
---local data = {}
---
----- Insert some dummy data in it.
---for x = 1, 63 do
---	table.insert(data, 0)
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
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L211)
function quiver.data.decompress() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L232)
function quiver.data.encode() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L251)
function quiver.data.decode() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L279)
function quiver.data.hash() end

---Serialize a given Lua value as another format, in the form of a string.
---@param text any # Lua value to serialize.
---@param kind number # Format.
---@return string value # The value, in string form.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L341)
function quiver.data.serialize(text,kind) end

---Deserialize a given format string as a Lua value.
---@param text string # String to deserialize.
---@param kind number # Format.
---@return any value # The value, in Lua value form.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L389)
function quiver.data.deserialize(text,kind) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L438)
function quiver.data.to_data() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L462)
function quiver.data.from_data() end

---TO-DO
---
--- ---
---*Available with compile feature: `embed`.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L490)
function quiver.data.get_embed_file() end

---TO-DO
---
--- ---
---*Available with compile feature: `embed`.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/data.rs#L510)
function quiver.data.get_embed_list() end

---The discord API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/discord.rs#L74)
---@class quiver.discord
quiver.discord = {}

---An unique handle to a Discord client.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/discord.rs#L88)
---@class discord
discord = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/discord.rs#L141)
function discord:set_rich_presence() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/discord.rs#L250)
function discord:clear_rich_presence() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/discord.rs#L262)
function discord:update() end

---Create a new Discord client.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/discord.rs#L293)
function quiver.discord.new() end

---The file API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L64)
---@class quiver.file
quiver.file = {}

---Get the data of a file, in string format.
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
---@return string data # File data.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L117)
function quiver.file.get(path) end

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
---@param data string # Data to copy.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L145)
function quiver.file.set(path,data) end

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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L174)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L194)
function quiver.file.get_path_exist(path) end

---Check if a file's extension is the same as a given one.
---@param path string # Path to file.
---@param extension string # Extension. MUST include dot (.png, .wav, etc.).
---@return boolean check # True if file extension is the same as the given one, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L214)
function quiver.file.get_file_extension_check(path,extension) end

---Get the size of a file.
---@param path string # Path to file.
---@return number size # File size.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L235)
function quiver.file.get_file_size(path) end

---Get the extension of a file.
---@param path string # Path to file.
---@return string extension # File extension.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L254)
function quiver.file.get_file_extension(path) end

---Get the name of a file.
---@param path string # Path to file.
---@param extension boolean # File extension. If true, will return file name with the extension.
---@return string name # File name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L277)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L318)
function quiver.file.get_application_directory() end

---Scan a path.
---@param path string # Path to scan.
---@param filter string # OPTIONAL: Extension filter. If filter is 'DIR', will include every directory in the result.
---@param recursive boolean # If true, recursively scan the directory.
---@param absolute boolean # If true, return path relatively.
---@return table list # File list.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L341)
function quiver.file.scan_path(path,filter,recursive,absolute) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L394)
function quiver.file.get_path_escape() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L405)
function quiver.file.set_path_escape() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L416)
function quiver.file.move() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L432)
function quiver.file.copy() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L448)
function quiver.file.remove_file() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/file.rs#L463)
function quiver.file.remove_path() end

---The general API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L69)
---@class quiver.general
quiver.general = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L111)
function quiver.general.standard_input() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L127)
function quiver.general.load_base() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L149)
function quiver.general.set_log_level() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L163)
function quiver.general.open_link() end

---Load the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L173)
function quiver.general.load() end

---Exit the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L177)
function quiver.general.exit() end

---Set a key to exit Quiver.
---@param key input_board # Key to exit Quiver with.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L187)
function quiver.general.set_exit_key(key) end

---Get the current time. Will count up since the initialization of the window.
---@return number time # Current time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L203)
function quiver.general.get_time() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L213)
function quiver.general.get_time_unix() end

---Get the current frame time.
---@return number frame_time # Current frame time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L230)
function quiver.general.get_frame_time() end

---Get the current frame rate.
---@return number frame_rate # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L243)
function quiver.general.get_frame_rate() end

---Set the current frame rate.
---@param frame_rate number # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L256)
function quiver.general.set_frame_rate(frame_rate) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L270)
function quiver.general.get_argument() end

---TO-DO
---
--- ---
---*Available with compile feature: `system_info`.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L284)
function quiver.general.get_system() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L299)
function quiver.general.get_memory() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L310)
function quiver.general.get_info() end

---The shader API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L65)
---@class quiver.shader
quiver.shader = {}

---An unique handle for a shader in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L86)
---@class shader
shader = {}

---Create a new shader resource.
---@param v_path string # Path to .vs file.
---@param f_path string # Path to .fs file.
---@return shader shader # Shader resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L105)
function quiver.shader.new(v_path,f_path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L146)
function quiver.shader.new_from_memory() end

---TO-DO
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L196)
function shader:begin(call) end

---TO-DO
---@param name string # TO-DO
---@return number location # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L221)
function shader:get_location_name(name) end

---TO-DO
---@param name string # TO-DO
---@return number location # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L238)
function shader:get_location_attribute_name(name) end

---TO-DO
---@param location number # TO-DO
---@return number location # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L255)
function shader:get_location(location) end

---TO-DO
---@param location number # TO-DO
---@param value number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L270)
function shader:set_location(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L289)
function shader:set_shader_integer(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L308)
function shader:set_shader_decimal(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value vector_3 # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L327)
function shader:set_shader_vector_3(location,value) end

---TO-DO
---@param location number # TO-DO
---@param value vector_4 # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/shader.rs#L347)
function shader:set_shader_vector_4(location,value) end

---The image API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L68)
---@class quiver.image
quiver.image = {}

---An unique handle for a image in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L91)
---@class image
---@field shape_x number # Shape of the image (X).
---@field shape_y number # Shape of the image (Y).
image = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L109)
function image:to_texture() end

---Create a new image resource.
---@param path string # Path to image file.
---@return image image # Image resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L129)
function quiver.image.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/image.rs#L155)
function quiver.image.new_from_memory() end

---The font API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L65)
---@class quiver.font
quiver.font = {}

---Set the vertical space between each line-break.
---@param space number # Vertical space.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L93)
function quiver.font.set_text_line_space(space) end

---An unique handle to a font in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L106)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L128)
function font:draw(label,point,scale,space,color) end

---Measure the size of a given text on screen, with a given font.
---@param label string # Label of font to measure.
---@param scale number # Scale of font to measure.
---@param space number # Space of font to measure.
---@return number size_x # Size of text (X).
---@return number size_y # Size of text (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L166)
function font:measure_text(label,scale,space) end

---Create a new font resource.
---@param path string # Path to font file.
---@param size number # Size for font.
---@return font font # Font resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L195)
function quiver.font.new(path,size) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L219)
function quiver.font.new_from_memory() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L251)
function quiver.font.new_default() end

---The window API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L68)
---@class quiver.window
quiver.window = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L163)
function quiver.window.file_dialog() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L250)
function quiver.window.text_dialog() end

---Get if the window should close.
---@return boolean close # True if the window should close.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L300)
function quiver.window.get_close() end

---Get the state of the window (full-screen).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L314)
function quiver.window.get_fullscreen() end

---Get the state of the window (hidden).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L328)
function quiver.window.get_hidden() end

---Get the state of the window (minimize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L342)
function quiver.window.get_minimize() end

---Get the state of the window (maximize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L356)
function quiver.window.get_maximize() end

---Get the state of the window (focus).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L370)
function quiver.window.get_focus() end

---Get the state of the window (resize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L384)
function quiver.window.get_resize() end

---Get the state of a window flag.
---@param flag window_flag # Window flag.
---@return boolean state # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L401)
function quiver.window.get_state(flag) end

---Set the state of a window flag.
---@param flag window_flag # Window flag.
---@param state boolean # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L416)
function quiver.window.set_state(flag,state) end

---Set the window to full-screen mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L431)
function quiver.window.set_fullscreen() end

---Set the window to border-less mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L441)
function quiver.window.set_borderless() end

---Minimize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L451)
function quiver.window.set_minimize() end

---Maximize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L461)
function quiver.window.set_maximize() end

---Restore the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L471)
function quiver.window.set_restore() end

---Set the window icon.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L481)
function quiver.window.set_icon() end

---Set the window name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L500)
function quiver.window.set_name() end

---Set the window point.
---@param point vector_2 # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L519)
function quiver.window.set_point(point) end

---Set the window monitor.
---@param index number # Index of monitor to move window to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L538)
function quiver.window.set_screen(index) end

---Set the minimum window shape.
---@param shape vector_2 # Minimum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L555)
function quiver.window.set_shape_min(shape) end

---Set the maximum window shape.
---@param shape vector_2 # Maximum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L574)
function quiver.window.set_shape_max(shape) end

---Set the current window shape.
---@param shape vector_2 # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L593)
function quiver.window.set_shape(shape) end

---Set the window alpha.
---@param alpha number # Alpha of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L612)
function quiver.window.set_alpha(alpha) end

---Focus the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L622)
function quiver.window.set_focus() end

---Get the shape of the window.
---@return number shape_x # Shape of the window (X).
---@return number shape_y # Shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L640)
function quiver.window.get_shape() end

---Get the shape of the current render view.
---@return number shape_x # Shape of the render view (X).
---@return number shape_y # Shape of the render view (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L655)
function quiver.window.get_render_shape() end

---Get the available monitor amount.
---@return number count # Monitor count.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L669)
function quiver.window.get_screen_count() end

---Get the current active monitor, where the window is.
---@return number index # Current active monitor index.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L683)
function quiver.window.get_screen_focus() end

---Get the point of the given monitor.
---@param index number # Index of the monitor.
---@return number point_x # Point of the monitor (X).
---@return number point_y # Point of the monitor (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L701)
function quiver.window.get_screen_point(index) end

---Get the shape of the given monitor.
---@param index number # Index of the monitor.
---@return number shape_x # Shape of the window (X).
---@return number shape_y # Shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L722)
function quiver.window.get_screen_shape(index) end

---Get the physical shape of the given monitor.
---@param index number # Index of the monitor.
---@return number shape_x # Physical shape of the window (X).
---@return number shape_y # Physical shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L740)
function quiver.window.get_screen_shape_physical(index) end

---Get the refresh rate of the given monitor.
---@param index number # Index of the monitor.
---@return number rate # Refresh rate of the monitor.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L762)
function quiver.window.get_screen_rate(index) end

---Get the point of the window.
---@return number point_x # Point of the window (X).
---@return number point_y # Point of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L777)
function quiver.window.get_point() end

---Get the DPI scale of the window.
---@return number scale_x # Scale of the window (X).
---@return number scale_y # Scale of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L796)
function quiver.window.get_scale() end

---Get the name of the given monitor.
---@param index number # Index of the monitor.
---@return string name # Name of the monitor.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L817)
function quiver.window.get_screen_name(index) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L834)
function quiver.window.get_screen_shot() end

---The music API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L65)
---@class quiver.music
quiver.music = {}

---An unique handle for music in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L86)
---@class music
music = {}

---Create a new music resource.
---@param path string # Path to music file.
---@return music music # Music resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L105)
function quiver.music.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L131)
function quiver.music.new_from_memory() end

---Play the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L170)
function music:play() end

---Check if music is currently playing.
---@return boolean state # State of the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L185)
function music:get_playing() end

---Stop the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L192)
function music:stop() end

---Pause the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L200)
function music:pause() end

---Resume the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L208)
function music:resume() end

---Set volume for the music. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L223)
function music:set_volume(volume) end

---Set pitch for the music.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L238)
function music:set_pitch(pitch) end

---Set pan for the music. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L253)
function music:set_pan(pan) end

---Update the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L261)
function music:update() end

---Set position for the music.
---@param position number # Current position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L276)
function music:set_position(position) end

---Get time length for the music.
---@return number length # Time length.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L291)
function music:get_length() end

---Get time played for the music.
---@return number played # Time played.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L305)
function music:get_played() end

