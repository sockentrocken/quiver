---@meta

---@class model
local model = {}

---An unique handle for a model in memory.
---@param path string Path to file.
---@return model # The user-data object.
function Model(path) end

---@class texture
---@field size vector_2 # Size of the texture.
local texture = {}

---Draw the texture.
---@param point vector_2 The point of the texture.
---@param angle number   The angle of the texture.
---@param scale number   The scale of the texture.
---@param color color    The color of the texture.
function texture:draw(point, angle, scale, color) end

---Draw the texture (pro variant).
---@param box_a box_2    The source rectangle of the texture.
---@param box_b box_2    The target rectangle of the texture.
---@param point vector_2 The point of the texture.
---@param angle number   The angle of the texture.
---@param color color    The color of the texture.
function texture:draw_pro(box_a, box_b, point, angle, color) end

---An unique handle for a texture in memory.
---@param path string Path to file.
---@return texture # The user-data object.
function Texture(path) end

---@class render_texture
---@field size vector_2 # Size of the texture.
local render_texture = {}

---Draw the texture.
---@param point vector_2 The point of the texture.
---@param angle number   The angle of the texture.
---@param scale number   The scale of the texture.
---@param color color    The color of the texture.
function render_texture:draw(point, angle, scale, color) end

---Draw the texture (pro variant).
---@param box_a box_2    The source rectangle of the texture.
---@param box_b box_2    The target rectangle of the texture.
---@param point vector_2 The point of the texture.
---@param angle number   The angle of the texture.
---@param color color    The color of the texture.
function render_texture:draw_pro(box_a, box_b, point, angle, color) end

---An unique handle for a render texture in memory.
---@param path string Path to file.
---@return render_texture # The user-data object.
function RenderTexture(path) end

---@class image
---@field size vector_2 # Size of the texture.
local image = {}

---An unique handle for an image in memory.
---@param path string Path to file.
---@return image # The user-data object.
function Image(path) end

---@class font
local font = {}

---Draw a font.
---@param text  string   Text of font to draw.
---@param point vector_2 Point of font to draw.
---@param scale number   Scale of font to draw.
---@param space number   Space of font to draw.
---@param color color    Color of font to draw.
function font:draw(text, point, scale, space, color) end

---An unique handle for a font in memory.
---@param path string Path to file.
---@return font # The user-data object.
function Font(path) end

---@class shader
local shader = {}

---Initialize the shader draw mode. **MUST** call *close* after shader drawing is done.
function shader:begin() end

---Finalize the shader draw mode.
function shader:close() end

---An unique handle for a shader in memory.
---@param v_path? string Path to shader .vs file.
---@param f_path? string Path to shader .fs file.
---@return shader # The user-data object.
function Shader(v_path, f_path) end

---Clear the screen with a color.
---@param color color Color to clear the screen with.
function set_screen_color(color) end

---Get the size of the screen.
---@return vector_2 # Screen size.
function get_window_shape() end

---Get the state of the window (minimized).
---@return boolean # True if minimized, false otherwise.
function get_window_minimize() end

---Get the state of the window (maximized).
---@return boolean # True if maximized, false otherwise.
function get_window_maximize() end

---Get the state of the window (focused).
---@return boolean # True if focused, false otherwise.
function get_window_focus() end

---Get the state of the window (resized).
---@return boolean # True if resized, false otherwise.
function get_window_resize() end

---Initialize the 3D draw mode. **MUST** call *close_mode_3d* after 3D drawing is done.
---@param camera camera_3d The 3D camera to use for drawing.
function begin_mode_3d(camera) end

---Finalize the 3D draw mode.
function close_mode_3d() end

---Draw a grid.
---@param slice number The slice count of the grid.
---@param space number The space shift of the grid.
function draw_grid(slice, space) end

---Draw a cube.
---@param point vector_3 The point of the cube.
---@param shape vector_3 The shape of the cube.
---@param color color    The color of the cube.
function draw_cube(point, shape, color) end

---Draw a ball.
---@param point vector_3 The point of the ball.
---@param shape number   The shape of the ball.
---@param color color    The color of the ball.
function draw_ball(point, shape, color) end

---Draw a 3D box.
---@param box_3 box_3 Box to draw.
---@param color color The color of the box to draw.
function draw_ball(point, shape, color) end

---Initialize the 2D draw mode. **MUST** call *close_mode_2d* after 2D drawing is done.
---@param camera camera_2d The 2D camera to use for drawing.
function begin_mode_2d(camera) end

---Finalize the 2D draw mode.
function close_mode_2d() end

---Get the world-space point for a screen-space point.
---@param point  vector_2  Screen-space point to convert from.
---@param camera camera_2d Camera to convert from.
---@return vector_2 # World-space point.
function get_screen_to_world(point, camera) end

---Get the screen-space point for a world-space point.
---@param point  vector_2  World-space point to convert from.
---@param camera camera_2d Camera to convert from.
---@return vector_2 # Screen-space point.
function get_world_to_screen(point, camera) end

---Draw a 2D box.
---@param shape box_2    Box to draw.
---@param point vector_2 The point of the box.
---@param angle number   The angle of the box.
---@param color color    The color of the box.
function draw_box_2(shape, point, angle, color) end

---Draw text.
---@param text  string   Text to draw.
---@param point vector_2 Point for the text.
---@param scale number   Scale for the text.
---@param color color    Color for the text.
function draw_text(text, point, scale, color) end

---@class file
local file = {}

---An unique file handle for a file in memory.
---@param path string Path to file.
---@return file # The user-data object.
function File(path) end

---@class watcher_info
local watcher_info = {}

---@class file_watcher
local file_watcher = {}

---Poll for a notification in the file watcher's directory.
---@return watcher_info | nil # Will return a non-nil value on event.
function file_watcher:update() end

---An unique handle for a file watcher in memory.
---@param path string Path to file/directory.
---@return file_watcher # The user-data object.
function FileWatcher(path) end

---Load the engine.
function engine_load() end

---Exit the engine.
function engine_exit() end

---Get the current state of the debug window.
function get_debug_state() end

---Set the current state of the debug window.
---@param value boolean New state.
function set_debug_state(value) end

---Get the current state of the debug logger.
function get_logger_state() end

---Set the current state of the debug logger.
---@param value boolean New state.
function set_logger_state(value) end

---Wipe the debug logger text.
function wipe_logger() end

---Show the debug logger text.
---@param value boolean New state.
function show_logger(value) end

---Push a new string to the debug logger.
---@param label  string Label for line to print.
---@param color? color  Color for line to print.
function push_logger(label, color) end

---Push a new method to the debug parser.
---@param name string Name for method to push.
---@param info string Info for method to push.
---@param call function Function call-back for method to push.
function push_parser(name, info, call) end

---Set a key to exit Quiver.
---@param key input_board Key to exit Quiver with.
function set_exit_key(key) end

---Get the current time. Will count up since the initialization of the window.
---@return number # Current time.
function get_time() end

---Get the current frame time.
---@return number # Current frame time.
function get_frame_time() end

---Get the current frame rate.
---@return number # Current frame rate.
function get_frame_rate() end

---Set the current frame rate.
---@param value number Value to set the frame rate to.
function set_frame_rate(value) end

---Convert a table to a JSON string.
---@param value table Table to convert to a JSON string.
---@return string # JSON conversion of table.
function table_to_json(value) end

---Convert a JSON string to a table.
---@param value string JSON string to convert to a table.
---@return table # Table conversion of a JSON string.
function json_to_table(value) end

---Check if a file does exist in disk.
---@param path string Path for file to check.
---@return boolean # True if file does exist, false otherwise.
function get_file_exist(path) end

---@class steam
local steam = {}

---Play sound.
function sound:play() end

---An unique handle for the Steam API. Creating more than one handle will result in an error.
---@param app? number App ID. If nil, it will resort to the default (480) SpaceWar example.
---@return steam # The user-data object.
function Steam(app) end

---Set the interface alpha.
---@param value number The alpha of the interface.
function set_interface_alpha(value) end

---Draw an interface button.
---@param shape box_2  The shape of the button.
---@param label string The label of the button.
---@return boolean # True on button click.
function interface_button(shape, label) end

---Draw an interface toggle.
---@param shape box_2   The shape of the slider.
---@param label string  The label of the slider.
---@param value boolean The value of the slider.
---@return boolean # The new value of *value*, if any.
function interface_toggle(shape, label, value) end

---Draw an interface check box.
---@param shape box_2   The shape of the check box.
---@param label string  The label of the check box.
---@param value boolean The value of the check box.
---@return boolean # The new value of *value*, if any.
function interface_check_box(shape, label, value) end

---Draw an interface combo box.
---@param shape box_2  The shape of the combo box.
---@param label string The label of the combo box.
---@param value number The value of the combo box.
---@return number # The new value of *value*, if any.
function interface_combo_box(shape, label, value) end

---Draw an interface spinner.
---@param shape box_2   The shape of the spinner.
---@param label string  The label of the spinner.
---@param value number  The value of the spinner.
---@param min   number  The minimum value of the spinner.
---@param max   number  The maximum value of the spinner.
---@param edit  boolean The edit mode value of the spinner.
---@return number # The new value of *value*, if any.
function interface_spinner(shape, label, value, min, max, edit) end

---Draw an interface slider.
---@param shape   box_2  The shape of the slider.
---@param label_a string The label of the slider.
---@param label_b string The label of the slider.
---@param value   number The value of the slider.
---@param min     number The minimum value of the slider.
---@param max     number The maximum value of the slider.
---@return number # The new value of *value*, if any.
function interface_slider(shape, label_a, label_b, value, min, max) end

---Draw an interface slider bar.
---@param shape   box_2  The shape of the slider bar.
---@param label_a string The label of the slider bar.
---@param label_b string The label of the slider bar.
---@param value   number The value of the slider bar.
---@param min     number The minimum value of the slider bar.
---@param max     number The maximum value of the slider bar.
---@return number # The new value of *value*, if any.
function interface_slider_bar(shape, label_a, label_b, value, min, max) end

---Set the active state of the mouse.
---@param value boolean New state. Active if true, inactive if false.
function set_mouse_active(value) end

---Set the hidden state of the mouse.
---@param value boolean New state. Hide if true, show if false.
function set_mouse_hidden(value) end

---Check if the mouse is currently hidden.
---@return boolean # The hidden state of the mouse.
function get_mouse_hidden() end

---Check if the mouse is currently over the screen.
---@return boolean # The screen state of the mouse.
function get_mouse_screen() end

---Get the current point of the mouse.
---@return vector_2 # The point of the mouse.
function get_mouse_point() end

---Get the current delta (i.e. mouse movement) of the mouse.
---@return vector_2 # The delta of the mouse.
function get_mouse_delta() end

---Set the current point of the mouse.
---@param value vector_2 New point.
function set_mouse_point(value) end

---Set the current shift of the mouse.
---@param value vector_2 New shift.
function set_mouse_shift(value) end

---Set the current scale of the mouse.
---@param value vector_2 New scale.
function set_mouse_scale(value) end

---Set the current cursor of the mouse.
---@param value cursor_mouse New cursor.
function set_mouse_cursor(value) end

---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return vector_2 # The delta of the mouse wheel.
function get_mouse_wheel() end

---Get the state of an input (up).
---@param value input_mouse The input to check for.
function get_mouse_up(value) end

---Get the state of an input (down).
---@param value input_mouse The input to check for.
function get_mouse_down(value) end

---Get the state of an input (press).
---@param value input_mouse The input to check for.
function get_mouse_press(value) end

---Get the state of an input (release).
---@param value input_mouse The input to check for.
function get_mouse_release(value) end

---Get the state of an input (up).
---@param value input_board The input to check for.
function get_board_up(value) end

---Get the state of an input (down).
---@param value input_board The input to check for.
function get_board_down(value) end

---Get the state of an input (press).
---@param value input_board The input to check for.
function get_board_press(value) end

---Get the state of an input (release).
---@param value input_board The input to check for.
function get_board_release(value) end

---Get the state of a pad.
---@param index integer The index of the pad to check for.
---@return boolean # True if pad is available, false otherwise.
function get_pad_state(index) end

---Get the name of a pad.
---@param index integer The index of the pad to check for.
---@return string # The name of the pad.
function get_pad_name(index) end

---Get the last pad button pressed.
---@return input_pad # The last pad button pressed.
function get_pad_queue() end

---Get the axis count of a pad.
---@param index integer The index of the pad to check for.
---@return number # The axis count of the pad.
function get_pad_axis_count(index) end

---Get the axis state of a pad.
---@param index integer The index of the pad to check for.
---@param axis  integer The axis of the pad to check for.
---@return number # The axis state of the pad.
function get_pad_axis_state(index, axis) end

---Get the state of an input (up).
---@param index integer The index of the pad to check for.
---@param value input_pad The input to check for.
function get_pad_up(index, value) end

---Get the state of an input (down).
---@param index integer The index of the pad to check for.
---@param value input_pad The input to check for.
function get_pad_down(index, value) end

---Get the state of an input (press).
---@param index integer The index of the pad to check for.
---@param value input_pad The input to check for.
function get_pad_press(index, value) end

---Get the state of an input (release).
---@param index integer The index of the pad to check for.
---@param value input_pad The input to check for.
function get_pad_release(index, value) end

---@class sound
local sound = {}

---Play sound.
function sound:play() end

---Get if sound is playing.
function sound:get_playing() end

---Stop sound.
function sound:stop() end

---Pause sound.
function sound:pause() end

---Resume sound.
function sound:resume() end

---Set volume for sound. (range: 0.0 - 1.0)
---@param value number The volume for the sound.
function sound:volume(value) end

---Set pitch for sound.
---@param value number The pitch for the sound.
function sound:pitch(value) end

---Set pan for sound. (range: 0.0 - 1.0; 0.5 is center)
---@param value number The pan for the sound.
function sound:pan(value) end

---An unique handle for sound in memory.
---@param path string Path to file.
---@return sound # The user-data object.
function Sound(path) end

---@class music
local music = {}

---Play music.
function music:play() end

---Get if music is playing.
function music:get_playing() end

---Stop music.
function music:stop() end

---Pause music.
function music:pause() end

---Resume music.
function music:resume() end

---Set volume for music. (range: 0.0 - 1.0)
---@param value number The volume for the music.
function music:volume(value) end

---Set pitch for music.
---@param value number The pitch for the music.
function music:pitch(value) end

---Set pan for music. (range: 0.0 - 1.0; 0.5 is center)
---@param value number The pan for the music.
function music:pan(value) end

---Update music stream.
function music:update() end

---Set position for music.
---@param value number The position for the music.
function music:seek() end

---Get time length of music.
---@return number # The length of the music.
function music:length() end

---Get time played of music.
---@return number # The time played of the music.
function music:played() end

---An unique handle for music in memory.
---@param path string Path to file.
---@return music # The user-data object.
function Music(path) end

