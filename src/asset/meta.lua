---The window API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L9)
---@class quiver.window
quiver.window = {}

---Set the window to full-screen mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L45)
function quiver.window.set_fullscreen() end

---Set the window to border-less mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L55)
function quiver.window.set_borderless() end

---Minimize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L65)
function quiver.window.set_minimize() end

---Maximize the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L75)
function quiver.window.set_maximize() end

---Focus the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L85)
function quiver.window.set_focus() end

---Restore the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L95)
function quiver.window.set_restore() end

---Set the window name.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L105)
function quiver.window.set_name() end

---Set the window monitor.
---@param index number # Index of monitor to move window to.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L123)
function quiver.window.set_monitor(index) end

---Set the current window shape.
---@param shape vector_2 # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L139)
function quiver.window.set_shape(shape) end

---Set the minimum window shape.
---@param shape vector_2 # Minimum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L157)
function quiver.window.set_shape_min(shape) end

---Set the maximum window shape.
---@param shape vector_2 # Maximum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L175)
function quiver.window.set_shape_max(shape) end

---Set the window alpha.
---@param alpha number # Alpha of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L193)
function quiver.window.set_alpha(alpha) end

---Set the window point.
---@param point vector_2 # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L209)
function quiver.window.set_point(point) end

---Get the state of the window (full-screen).
---@return state boolean # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L229)
function quiver.window.get_fullscreen() end

---Get the state of the window (minimize).
---@return state boolean # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L242)
function quiver.window.get_minimize() end

---Get the state of the window (maximize).
---@return state boolean # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L255)
function quiver.window.get_maximize() end

---Get the state of the window (focus).
---@return state boolean # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L268)
function quiver.window.get_focus() end

---Get the state of the window (resize).
---@return state boolean # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L281)
function quiver.window.get_resize() end

---Get the state of the window (hidden).
---@return state boolean # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L294)
function quiver.window.get_hidden() end

---Get the shape of the window.
---@return shape vector_2 # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L307)
function quiver.window.get_shape() end

---Get the point of the window.
---@return point vector_2 # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L325)
function quiver.window.get_point() end

---Get the DPI scale of the window.
---@return scale number # Scale of the window.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/window.rs#L342)
function quiver.window.get_scale() end

---The texture API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L12)
---@class quiver.texture
quiver.texture = {}

---An unique handle for a texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L74)
---@class texture
texture = {}

---Create a new texture resource.
---@param path string # Path to texture file.
---@return texture texture # Texture resource.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/texture.rs#L144)
function quiver.texture.new(path) end

---The music API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L12)
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
---@return state boolean # State of the music.
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
---@return length number # Time length.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L188)
function music:get_length() end

---Get time played for the music.
---@return played number # Time played.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/music.rs#L201)
function music:get_played() end

---The font API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/font.rs#L12)
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

---The engine API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L368)
---@class quiver.engine
quiver.engine = {}

---Load the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L374)
function quiver.engine.load() end

---Exit the engine.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L383)
function quiver.engine.exit() end

---Set a key to exit Quiver.
---@param key input_board # Key to exit Quiver with.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L412)
function quiver.engine.set_exit_key(key) end

---Get the current time. Will count up since the initialization of the window.
---@return time number # Current time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L434)
function quiver.engine.get_time() end

---Get the current frame time.
---@return frame_time number # Current frame time.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L447)
function quiver.engine.get_frame_time() end

---Get the current frame rate.
---@return frame_rate number # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L460)
function quiver.engine.get_frame_rate() end

---set the current frame rate.
---@param frame_rate number # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/general.rs#L473)
function quiver.engine.set_frame_rate(frame_rate) end

---The 2D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L20)
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
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L48)
function quiver.draw_2d.begin(camera) end

---Finalize the 2D draw mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L60)
function quiver.draw_2d.close() end

---Draw 2D box.
---@param shape box_2 # The shape of the box.
---@param point vector_2 # The point of the box.
---@param angle number # The angle of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L79)
function quiver.draw_2d.draw_box_2(shape,point,angle,color) end

---Draw text.
---@param label string # The label of the text.
---@param point vector_2 # The point of the text.
---@param scale number # The angle of the text.
---@param color color # The color of the text.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L105)
function quiver.draw_2d.draw_text(label,point,scale,color) end

---The 3D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L132)
---@class quiver.draw_3d
quiver.draw_3d = {}

---Initialize the 3D draw mode. **MUST** call *quiver.draw_3d.close* after 3D drawing is done.
---@param camera camera_3d # The 3D camera to use for drawing.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L157)
function quiver.draw_3d.begin(camera) end

---Finalize the 3D draw mode.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L169)
function quiver.draw_3d.close() end

---Draw a grid.
---@param slice number # The slice count of the grid.
---@param space number # The space shift of the grid.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L186)
function quiver.draw_3d.draw_grid(slice,space) end

---Draw a cube.
---@param point vector_3 # The point of the cube.
---@param shape vector_3 # The shape of the cube.
---@param color color # The color of the cube.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L204)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a ball.
---@param point vector_3 # The point of the ball.
---@param shape number # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L229)
function quiver.draw_3d.draw_cube(point,shape,color) end

---Draw a 3D box.
---@param shape box_3 # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/draw.rs#L249)
function quiver.draw_3d.draw_box_3(shape,color) end

---The input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L187)
---@class quiver.input
quiver.input = {}

---The board input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L199)
---@class quiver.input.board
quiver.input.board = {}

---The mouse input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L215)
---@class quiver.input.mouse
quiver.input.mouse = {}

---The pad input API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L240)
---@class quiver.input.pad
quiver.input.pad = {}

---Set the active state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L270)
function quiver.input.mouse.set_active(state) end

---Set the hidden state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L291)
function quiver.input.mouse.set_hidden(state) end

---Get the hidden state of the mouse.
---@return state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L312)
function quiver.input.mouse.get_hidden() end

---Check if the mouse is currently over the screen.
---@return state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L325)
function quiver.input.mouse.get_screen() end

---Get the current point of the mouse.
---@return point vector_2 # The point of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L338)
function quiver.input.mouse.get_point() end

---Set the current point of the mouse.
---@param point vector_2 # The point of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L354)
function quiver.input.mouse.set_point(point) end

---Get the current delta (i.e. mouse movement) of the mouse.
---@return delta vector_2 # The delta of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L371)
function quiver.input.mouse.get_delta() end

---Set the current shift of the mouse.
---@param shift vector_2 # The shift of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L387)
function quiver.input.mouse.set_shift(shift) end

---Set the current scale of the mouse.
---@param scale vector_2 # The scale of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L404)
function quiver.input.mouse.set_scale(scale) end

---Set the current cursor of the mouse.
---@param cursor cursor_mouse # The cursor of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L421)
function quiver.input.mouse.set_cursor(cursor) end

---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return delta vector_2 # The delta of the mouse wheel.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L443)
function quiver.input.mouse.get_wheel() end

---Get the state of an input (up).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L459)
function quiver.input.mouse.get_up(mouse) end

---Get the state of an input (down).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L476)
function quiver.input.mouse.get_down(mouse) end

---Get the state of an input (press).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L493)
function quiver.input.mouse.get_press(mouse) end

---Get the state of an input (release).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L510)
function quiver.input.mouse.get_release(mouse) end

---Get the state of an input (up).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L529)
function quiver.input.board.get_up(board) end

---Get the state of an input (down).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L546)
function quiver.input.board.get_down(board) end

---Get the state of an input (press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L563)
function quiver.input.board.get_press(board) end

---Get the state of an input (release).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L580)
function quiver.input.board.get_release(board) end

---Get the state of a pad.
---@param index number # The index of the pad to check for.
---@return state boolean # The state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L602)
function quiver.input.pad.get_state(index) end

---Get the name of a pad.
---@param index number # The index of the pad to check for.
---@return name string # The name of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L618)
function quiver.input.pad.get_name(index) end

---Get the last pad button press.
---@return input input_pad # The last pad button press.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L634)
function quiver.input.pad.get_queue() end

---Get the axis count of a pad.
---@param index number # The index of the pad to check for.
---@return axis_count number # The axis count of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L650)
function quiver.input.pad.get_axis_count(index) end

---Get the axis state of a pad.
---@param index number # The index of the pad to check for.
---@param axis number # The axis of the pad to check for.
---@return axis_state number # The axis state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L667)
function quiver.input.pad.get_axis_state(index,axis) end

---Get the state of an input (up).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L680)
function quiver.input.pad.get_up(pad) end

---Get the state of an input (down).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L697)
function quiver.input.pad.get_down(pad) end

---Get the state of an input (press).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L714)
function quiver.input.pad.get_press(pad) end

---Get the state of an input (release).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/input.rs#L731)
function quiver.input.pad.get_release(pad) end

---The sound API.
---
--- ---
---[Source Code Definition](https://github.com/sockentrocken/quiver/tree/main/src/system/sound.rs#L12)
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
---@return state boolean # State of the sound.
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

