---@meta

---The main Quiver API.
---@class quiver
quiver = {}

---The window API.
---@class quiver.window
quiver.window = {}

---Set the window to full-screen mode.
function quiver.window.set_fullscreen() end

---Set the window to border-less mode.
function quiver.window.set_borderless() end

---Minimize the window.
function quiver.window.set_minimize() end

---Maximize the window.
function quiver.window.set_maximize() end

---Focus the window.
function quiver.window.set_focus() end

---Restore the window.
function quiver.window.set_restore() end

---Set the window name.
function quiver.window.set_name() end

---Set the window monitor.
---@param index number Index of monitor to move window to.
function quiver.window.set_monitor(index) end

---Set the current window shape.
---@param shape vector_2 Shape of the window.
function quiver.window.set_shape(shape) end

---Set the minimum window shape.
---@param shape vector_2 Minimum shape of the window.
function quiver.window.set_shape_min(shape) end

---Set the maximum window shape.
---@param shape vector_2 Maximum shape of the window.
function quiver.window.set_shape_max(shape) end

---Set the window alpha.
---@param alpha number Alpha of the window.
function quiver.window.set_alpha(alpha) end

---Set the window point.
---@param point vector_2 Point of the window.
function quiver.window.set_point(point) end

---Get the state of the window (full-screen).
---@return boolean state # State of the window.
function quiver.window.get_fullscreen() end

---Get the state of the window (minimize).
---@return boolean state # State of the window.
function quiver.window.get_minimize() end

---Get the state of the window (maximize).
---@return boolean state # State of the window.
function quiver.window.get_maximize() end

---Get the state of the window (focus).
---@return boolean state # State of the window.
function quiver.window.get_focus() end

---Get the state of the window (resize).
---@return boolean state # State of the window.
function quiver.window.get_resize() end

---Get the state of the window (hidden).
---@return boolean state # State of the window.
function quiver.window.get_hidden() end

---Get the shape of the window.
---@return vector_2 shape # Shape of the window.
function quiver.window.get_shape() end

---Get the point of the window.
---@return vector_2 point # Point of the window.
function quiver.window.get_point() end

---Get the DPI scale of the window.
---@return number scale # Scale of the window.
function quiver.window.get_scale() end

---The texture API.
---@class quiver.texture
quiver.texture = {}

---An unique handle for a texture in memory.
---@class texture
---@field shape vector_2 # Shape of the texture.
texture = {}

---Create a new texture resource.
---@param path string Path to texture file.
---@return texture texture # Texture resource.
function quiver.texture.new(path) end

---The 3D drawing API.
---@class quiver.draw_3d
quiver.draw_3d = {}

---Initialize the 3D draw mode. **MUST** call *quiver.draw_3d.close* after 3D drawing is done.
---@param camera camera_3d The 3D camera to use for drawing.
function quiver.draw_3d.begin(camera) end

---Finalize the 3D draw mode.
function quiver.draw_3d.close() end

---Draw a grid.
---@param slice number The slice count of the grid.
---@param space number The space shift of the grid.
function quiver.draw_3d.draw_grid(slice, space) end

---Draw a cube.
---@param point vector_3 The point of the cube.
---@param shape vector_3 The shape of the cube.
---@param color color The color of the cube.
function quiver.draw_3d.draw_cube(point, shape, color) end

---Draw a ball.
---@param point vector_3 The point of the ball.
---@param shape number The shape of the ball.
---@param color color The color of the ball.
function quiver.draw_3d.draw_cube(point, shape, color) end

---Draw a 3D box.
---@param shape box_3 The shape of the ball.
---@param color color The color of the ball.
function quiver.draw_3d.draw_box_3(shape, color) end

---The music API.
---@class quiver.music
quiver.music = {}

---An unique handle for music in memory.
---@class music
music = {}

---Create a new music resource.
---@param path string Path to music file.
---@return music music # Music resource.
function quiver.music.new(path) end

---Play the music.
function music:play() end

---Check if music is currently playing.
---@return boolean state # State of the music.
function music:get_playing() end

---Stop the music.
function music:stop() end

---Pause the music.
function music:pause() end

---Resume the music.
function music:resume() end

---Set volume for the music. (range: 0.0 - 1.0)
---@param volume number Current volume.
function music:set_volume(volume) end

---Set pitch for the music.
---@param pitch number Current pitch.
function music:set_pitch(pitch) end

---Set pan for the music. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number Current pan.
function music:set_pan(pan) end

---Update the music.
function music:update() end

---Set position for the music.
---@param position number Current position.
function music:set_position(position) end

---Get time length for the music.
---@return number length # Time length.
function music:get_length() end

---Get time played for the music.
---@return number played # Time played.
function music:get_played() end

---The font API.
---@class quiver.font
quiver.font = {}

---An unique handle to a font in memory.
---@class font
font = {}

---Draw a font.
---@param label string Label of font to draw.
---@param point vector_2 Point of font to draw.
---@param scale number Scale of font to draw.
---@param space number Space of font to draw.
---@param color color Color of font to draw.
function font.draw(label, point, scale, space, color) end

---Create a new font resource.
---@param path string Path to font file.
---@return font font # Font resource.
function quiver.font.new(path) end

---The engine API.
---@class quiver.engine
quiver.engine = {}

---Load the engine.
function quiver.engine.load() end

---Exit the engine.
function quiver.engine.exit() end

---The debug window API.
---@class quiver.debug
quiver.debug = {}

---Get the current state of the debug window.
---@return boolean state # Current state.
function quiver.debug.get_state() end

---Set the current state of the debug window.
---@param state boolean Current state.
function quiver.debug.set_state(state) end

---The debug logger API.
---@class quiver.logger
quiver.logger = {}

---Get the current state of the debug logger.
---@return boolean state # Current state.
function quiver.logger.get_state() end

---Set the current state of the debug logger.
---@param state boolean Current state.
function quiver.logger.set_state(state) end

---Wipe the debug logger text.
function quiver.logger.wipe() end

---Show the debug logger text.
---@param state boolean Current state.
function quiver.logger.show(state) end

---Push a new string to the debug logger.
---@param label string Label for line to print.
---@param color? color Color for line to print.
function quiver.logger.push(label, color) end

---The debug parser API.
---@class quiver.parser
quiver.parser = {}

---Push a new method to the debug parser.
---@param name string Name for method to push.
---@param info string Info for method to push.
---@param call function Function call-back for method to push.
function quiver.parser.push(name, info, call) end

---Set a key to exit Quiver.
---@param key input_board Key to exit Quiver with.
function quiver.engine.set_exit_key(key) end

---Get the current time. Will count up since the initialization of the window.
---@return number time # Current time.
function quiver.engine.get_time() end

---Get the current frame time.
---@return number frame_time # Current frame time.
function quiver.engine.get_frame_time() end

---Get the current frame rate.
---@return number frame_rate # Current frame rate.
function quiver.engine.get_frame_rate() end

---set the current frame rate.
---@param frame_rate number Current frame rate.
function quiver.engine.set_frame_rate(frame_rate) end

---The input API.
---@class quiver.input
quiver.input = {}

---The board input API.
---@class quiver.input.board
quiver.input.board = {}

---The mouse input API.
---@class quiver.input.mouse
quiver.input.mouse = {}

---The pad input API.
---@class quiver.input.pad
quiver.input.pad = {}

---Set the active state of the mouse.
---@param state boolean Current state.
function quiver.input.mouse.set_active(state) end

---Set the hidden state of the mouse.
---@param state boolean Current state.
function quiver.input.mouse.set_hidden(state) end

---Get the hidden state of the mouse.
---@return boolean state # Current state.
function quiver.input.mouse.get_hidden() end

---Check if the mouse is currently over the screen.
---@return boolean state # Current state.
function quiver.input.mouse.get_screen() end

---Get the current point of the mouse.
---@return vector_2 point # The point of the mouse.
function quiver.input.mouse.get_point() end

---Set the current point of the mouse.
---@param point vector_2 The point of the mouse.
function quiver.input.mouse.set_point(point) end

---Get the current delta (i.e. mouse movement) of the mouse.
---@return vector_2 delta # The delta of the mouse.
function quiver.input.mouse.get_delta() end

---Set the current shift of the mouse.
---@param shift vector_2 The shift of the mouse.
function quiver.input.mouse.set_shift(shift) end

---Set the current scale of the mouse.
---@param scale vector_2 The scale of the mouse.
function quiver.input.mouse.set_scale(scale) end

---Set the current cursor of the mouse.
---@param cursor cursor_mouse The cursor of the mouse.
function quiver.input.mouse.set_cursor(cursor) end

---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return vector_2 delta # The delta of the mouse wheel.
function quiver.input.mouse.get_wheel() end

---Get the state of an input (up).
---@param mouse input_mouse The mouse button to check for.
function quiver.input.mouse.get_up(mouse) end

---Get the state of an input (down).
---@param mouse input_mouse The mouse button to check for.
function quiver.input.mouse.get_down(mouse) end

---Get the state of an input (press).
---@param mouse input_mouse The mouse button to check for.
function quiver.input.mouse.get_press(mouse) end

---Get the state of an input (release).
---@param mouse input_mouse The mouse button to check for.
function quiver.input.mouse.get_release(mouse) end

---Get the state of an input (up).
---@param board input_board The board button to check for.
function quiver.input.board.get_up(board) end

---Get the state of an input (down).
---@param board input_board The board button to check for.
function quiver.input.board.get_down(board) end

---Get the state of an input (press).
---@param board input_board The board button to check for.
function quiver.input.board.get_press(board) end

---Get the state of an input (release).
---@param board input_board The board button to check for.
function quiver.input.board.get_release(board) end

---Get the state of a pad.
---@param index number The index of the pad to check for.
---@return boolean state # The state of the pad.
function quiver.input.pad.get_state(index) end

---Get the name of a pad.
---@param index number The index of the pad to check for.
---@return string name # The name of the pad.
function quiver.input.pad.get_name(index) end

---Get the last pad button press.
---@return input_pad input # The last pad button press.
function quiver.input.pad.get_queue() end

---Get the axis count of a pad.
---@param index number The index of the pad to check for.
---@return number axis_count # The axis count of the pad.
function quiver.input.pad.get_axis_count(index) end

---Get the axis state of a pad.
---@param index number The index of the pad to check for.
---@param axis number The axis of the pad to check for.
---@return number axis_state # The axis state of the pad.
function quiver.input.pad.get_axis_state(index, axis) end

---Get the state of an input (up).
---@param pad input_pad The pad button to check for.
function quiver.input.pad.get_up(pad) end

---Get the state of an input (down).
---@param pad input_pad The pad button to check for.
function quiver.input.pad.get_down(pad) end

---Get the state of an input (press).
---@param pad input_pad The pad button to check for.
function quiver.input.pad.get_press(pad) end

---Get the state of an input (release).
---@param pad input_pad The pad button to check for.
function quiver.input.pad.get_release(pad) end

---The sound API.
---@class quiver.sound
quiver.sound = {}

---An unique handle for sound in memory.
---@class sound
sound = {}

---Create a new sound resource.
---@param path string Path to sound file.
---@return sound sound # Sound resource.
function quiver.sound.new(path) end

---Play the sound.
function sound:play() end

---Check if sound is currently playing.
---@return boolean state # State of the sound.
function sound:get_playing() end

---Stop the sound.
function sound:stop() end

---Pause the sound.
function sound:pause() end

---Resume the sound.
function sound:resume() end

---Set volume for the sound. (range: 0.0 - 1.0)
---@param volume number Current volume.
function sound:set_volume(volume) end

---Set pitch for the sound.
---@param pitch number Current pitch.
function sound:set_pitch(pitch) end

---Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number Current pan.
function sound:set_pan(pan) end

---The 2D drawing API.
---@class quiver.draw_2d
quiver.draw_2d = {}

---Initialize the 2D draw mode. **MUST** call *quiver.draw_2d.close* after 2D drawing is done.
---@param camera camera_2d The 2D camera to use for drawing.
function quiver.draw_2d.begin(camera) end

---Finalize the 2D draw mode.
function quiver.draw_2d.close() end

---Draw 2D box.
---@param shape box_2 The shape of the box.
---@param point vector_2 The point of the box.
---@param angle number The angle of the box.
---@param color color The color of the box.
function quiver.draw_2d.draw_box_2(shape, point, angle, color) end

---Draw text.
---@param label string The label of the text.
---@param point vector_2 The point of the text.
---@param scale number The angle of the text.
---@param color color The color of the text.
function quiver.draw_2d.draw_text(label, point, scale, color) end

