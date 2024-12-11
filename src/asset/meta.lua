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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L41)
function quiver.sound.new(path) end

---Play the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L73)
function sound:play() end

---Check if sound is currently playing.
---@return boolean state # State of the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L87)
function sound:get_playing() end

---Stop the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L94)
function sound:stop() end

---Pause the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L102)
function sound:pause() end

---Resume the sound.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L110)
function sound:resume() end

---Set volume for the sound. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L124)
function sound:set_volume(volume) end

---Set pitch for the sound.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L138)
function sound:set_pitch(pitch) end

---Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L152)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L16)
---@class quiver.rapier
quiver.rapier = {}

---An unique handle for a Rapier simulation in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L33)
---@class rapier
rapier = {}

---Create a new Rapier simulation.
---@return rapier rapier # Rapier resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L82)
function quiver.rapier.new() end

---Render every object in the simulation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L110)
function rapier:debug_render() end

---
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L125)
function rapier:step() end

---Create a character controller.
---@return table character_controller # Character controller handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L155)
function rapier:create_character_controller() end

---Move the character controller.
---@param character_controller table # Character controller handle.
---@param collider table # Collider handle.
---@param value vector_3 # Translation.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L170)
function rapier:move_character_controller(character_controller,collider,value) end

---Rigid body info.
---@field kind number # Kind.
---@field position vector_3 # Position.
---@field rotation vector_3 # Rotation.
---@field lin_vel vector_3 # Lin. vel.
---@field ang_vel vector_3 # Ang. vel.
---@field gravity number # Gravity.
---@field can_sleep boolean # Can sleep.
---@field continous boolean # Continous.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L266)
---@class rigid_body_info
rigid_body_info = {}

---Create a rigid body.
---@param kind rigid_body_info # Rigid body kind (fixed, dynamic, velocity-based, position-based).
---@return table body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L290)
function rapier:create_rigid_body(kind) end

---Set a rigid body's linear velocity.
---@param handle table # Rigid body handle.
---@param value vector_3 # Linear velocity.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L324)
function rapier:rigid_body_lin_vel(handle,value) end

---Collider info.
---@field kind table # Kind.
---@field position vector_3 # Position.
---@field rotation vector_3 # Rotation.
---@field density number # Density.
---@field friction number # Friction.
---@field trigger boolean # Trigger.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L358)
---@class collider_info
collider_info = {}

---Get a collider.
---@param handle table # Collider handle.
---@return table collider # Collider.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L380)
function rapier:get_collider(handle) end

---Create a collider.
---@param info collider_info # Collider info.
---@return table collider # Collider handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L398)
function rapier:create_collider(info) end

---Create a collider with a parent.
---@param info table # Collider info.
---@param parent table # Collider rigid body parent.
---@return table collider # Collider handle.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/rapier.rs#L434)
function rapier:create_collider_parent(info,parent) end

---The texture API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L14)
---@class quiver.texture
quiver.texture = {}

---An unique handle for a texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L76)
---@class texture
texture = {}

---Create a new texture resource.
---@param path string # Path to texture file.
---@return texture texture # Texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L146)
function quiver.texture.new(path) end

---The model API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L13)
---@class quiver.model
quiver.model = {}

---An unique handle for a model in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L29)
---@class model
model = {}

---Create a new Model resource.
---@param path string # Path to model file.
---@return Model Model # Model resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L44)
function quiver.model.new(path) end

---Draw the model (wire-frame render).
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/model.rs#L87)
function model:draw_wire() end

---The drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L23)
---@class quiver.draw
quiver.draw = {}

---Initialize drawing to the screen. **MUST** call *quiver.draw.close* after drawing is done.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L39)
function quiver.draw.begin() end

---Finalize drawing to the screen.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L49)
function quiver.draw.close() end

---Clear the screen with a color.
---@param color color # The color to use for clearing.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L65)
function quiver.draw.clear(color) end

---The 2D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L81)
---@class quiver.draw_2d
quiver.draw_2d = {}

---Initialize the 2D draw mode. **MUST** call *quiver.draw_2d.close* after 2D drawing is done.
---```lua
---local camera_2d = Camera2D:new(Vector2:zero(), Vector2:zero(), 0.0, 1.0)
---quiver.draw_2d.begin(camera_2d)
---[...]
---quiver.draw_2d.close()
---
---```
---@param camera camera_2d # The 2D camera to use for drawing.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L109)
function quiver.draw_2d.begin(camera) end

---Finalize the 2D draw mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L121)
function quiver.draw_2d.close() end

---Draw 2D box.
---@param shape box_2 # The shape of the box.
---@param point vector_2 # The point of the box.
---@param angle number # The angle of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L140)
function quiver.draw_2d.draw_box_2(shape,point,angle,color) end

---Draw text.
---@param label string # The label of the text.
---@param point vector_2 # The point of the text.
---@param scale number # The angle of the text.
---@param color color # The color of the text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L166)
function quiver.draw_2d.draw_text(label,point,scale,color) end

---The 3D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L193)
---@class quiver.draw_3d
quiver.draw_3d = {}

---Initialize the 3D draw mode. **MUST** call *quiver.draw_3d.close* after 3D drawing is done.
---@param camera camera_3d # The 3D camera to use for drawing.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L218)
function quiver.draw_3d.begin(camera) end

---Finalize the 3D draw mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L230)
function quiver.draw_3d.close() end

---Draw a grid.
---@param slice number # The slice count of the grid.
---@param space number # The space shift of the grid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L247)
function quiver.draw_3d.draw_grid(slice,space) end

---Draw a cube.
---@param point vector_3 # The point of the cube.
---@param shape vector_3 # The shape of the cube.
---@param color color # The color of the cube.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L265)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a ball.
---@param point vector_3 # The point of the ball.
---@param shape number # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L290)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a 3D box.
---@param shape box_3 # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L310)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L30)
function quiver.general.load() end

---Exit the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L34)
function quiver.general.exit() end

---Set a key to exit Quiver.
---@param key input_board # Key to exit Quiver with.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L44)
function quiver.general.set_exit_key(key) end

---Get the current time. Will count up since the initialization of the window.
---@return number time # Current time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L66)
function quiver.general.get_time() end

---Get the current frame time.
---@return number frame_time # Current frame time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L79)
function quiver.general.get_frame_time() end

---Get the current frame rate.
---@return number frame_rate # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L92)
function quiver.general.get_frame_rate() end

---set the current frame rate.
---@param frame_rate number # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L105)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L14)
---@class quiver.window
quiver.window = {}

---Set the window to full-screen mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L51)
function quiver.window.set_fullscreen() end

---Set the window to border-less mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L61)
function quiver.window.set_borderless() end

---Minimize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L71)
function quiver.window.set_minimize() end

---Maximize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L81)
function quiver.window.set_maximize() end

---Focus the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L91)
function quiver.window.set_focus() end

---Restore the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L101)
function quiver.window.set_restore() end

---Set the window name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L111)
function quiver.window.set_name() end

---Set the window monitor.
---@param index number # Index of monitor to move window to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L129)
function quiver.window.set_monitor(index) end

---Set the current window shape.
---@param shape vector_2 # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L145)
function quiver.window.set_shape(shape) end

---Set the minimum window shape.
---@param shape vector_2 # Minimum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L163)
function quiver.window.set_shape_min(shape) end

---Set the maximum window shape.
---@param shape vector_2 # Maximum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L181)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L215)
function quiver.window.set_point(point) end

---Get the state of the window (full-screen).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L235)
function quiver.window.get_fullscreen() end

---Get the state of the window (minimize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L248)
function quiver.window.get_minimize() end

---Get the state of the window (maximize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L261)
function quiver.window.get_maximize() end

---Get the state of the window (focus).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L274)
function quiver.window.get_focus() end

---Get the state of the window (resize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L287)
function quiver.window.get_resize() end

---Get the state of the window (hidden).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L300)
function quiver.window.get_hidden() end

---Get the shape of the window.
---@return vector_2 shape # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L313)
function quiver.window.get_shape() end

---Get the point of the window.
---@return vector_2 point # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L331)
function quiver.window.get_point() end

---Get the DPI scale of the window.
---@return number scale # Scale of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L348)
function quiver.window.get_scale() end

---Get if the window should close.
---@return boolean close # True if the window should close.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L365)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L41)
function quiver.music.new(path) end

---Play the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L73)
function music:play() end

---Check if music is currently playing.
---@return boolean state # State of the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L87)
function music:get_playing() end

---Stop the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L94)
function music:stop() end

---Pause the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L102)
function music:pause() end

---Resume the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L110)
function music:resume() end

---Set volume for the music. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L124)
function music:set_volume(volume) end

---Set pitch for the music.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L138)
function music:set_pitch(pitch) end

---Set pan for the music. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L152)
function music:set_pan(pan) end

---Update the music.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L160)
function music:update() end

---Set position for the music.
---@param position number # Current position.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L174)
function music:set_position(position) end

---Get time length for the music.
---@return number length # Time length.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L188)
function music:get_length() end

---Get time played for the music.
---@return number played # Time played.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L201)
function music:get_played() end

