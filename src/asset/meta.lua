---@meta

---@class (exact) vector
---@field x number
---@field y number
---@field z number
---@field w number
---@field X number
---@field Y number
---@field Z number
---@field W number
---@field zero vector Constant vector with every component set to 0.
---@field one  vector Constant vector with every component set to 1.
---@field create    function
---@field magnitude function
---@field normalize function
---@field cross     function
---@field dot       function
---@field angle     function
---@field floor     function
---@field ceil      function
---@field abs       function
---@field sign      function
---@field clamp     function
---@field min       function
---@field max       function
vector = {}

---Create a new vector.
---@param x number The X component of the vector.
---@param y number The Y component of the vector.
---@param z number The Z component of the vector.
---@param w number The W component of the vector.
---@return vector # The new vector.
function vector.create(x, y, z, w) end

---Calculate the magnitude of a vector.
---@param vector vector The vector to calculate the magnitude of.
---@return number # The magnitude of the vector.
function vector.magnitude(vector) end

---Normalize a vector.
---@param vector vector The vector to normalize.
---@return vector # The unit vector version of the vector.
function vector.normalize(vector) end

---Calculate the cross product between *vector_1* and *vector_2*.
---@param vector_1 vector Vector 1.
---@param vector_2 vector Vector 2.
---@return vector # The cross product.
function vector.cross(vector_1, vector_2) end

---Calculate the dot product between *vector_1* and *vector_2*.
---@param vector_1 vector Vector 1.
---@param vector_2 vector Vector 2.
---@return number # The dot product.
function vector.dot(vector_1, vector_2) end

---Calculate the angle between *vector_1* and *vector_2*.
---@param vector_1 vector Vector 1.
---@param vector_2 vector Vector 2.
---@param axis? vector Axis vector. If set, will determine the sign of the angle.
---@return number # The radian angle.
function vector.angle(vector_1, vector_2, axis) end

---Round a vector down.
---@param vector vector The vector to round down.
---@return vector # The vector, with *floor* set on every component.
function vector.floor(vector) end

---Round a vector up.
---@param vector vector The vector to round up.
---@return vector # The vector, with *ceil* set on every component.
function vector.ceil(vector) end

---Calculate the absolute value of a vector.
---@param vector vector The vector to calculate the absolute value of.
---@return vector # The vector, with *abs* set on every component.
function vector.abs(vector) end

--- Calculate the sign of every component of a vector.
---@param vector vector The vector to calculate the sign of.
---@return vector # The vector, with *sign* set on every component.
function vector.sign(vector) end

---Apply a minimum/maximum value to every component of a vector.
---@param vector vector The vector to clamp to a minimum/maximum value of.
---@param min vector Minimum value vector.
---@param max vector Maximum value vector.
---@return vector # The vector, with *clamp* set on every component.
function vector.clamp(vector, min, max) end

---Apply a maximum value to every component of a vector.
---@param vector vector The vector to clamp to a maximum value of.
---@param max vector Maximum value vector.
---@return vector # The vector, with *max* set on every component.
function vector.max(vector, max) end

---Apply a minimum value to every component of a vector.
---@param vector vector The vector to clamp to a maximum value of.
---@param min vector Minimum value vector.
---@return vector # The vector, with *min* set on every component.
function vector.min(vector, min) end

---@class (exact) file
local file = {}

---An unique file handle for a file in memory.
---@param path string Path to file.
---@return file # The user-data object.
function File(path) end

---Load the engine.
function engine_load() end

---Exit the engine.
function engine_exit() end

---Get the current state of the debug window.
function get_debug() end

---Set the current state of the debug window.
---@param value boolean New state.
function set_debug(value) end

---Get the current state of the debug logger.
function get_logger() end

---Set the current state of the debug logger.
---@param value boolean New state.
function set_logger(value) end

---Wipe the debug logger text.
function wipe_logger() end

---Show the debug logger text.
---@param value boolean New state.
function show_logger(value) end

---Push a new string to the debug logger.
---@param label string Label for line to print.
---@param color? vector Color for line to print.
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

---@class (exact) model
local model = {}

---An unique handle for a model in memory.
---@param path string Path to file.
---@return model # The user-data object.
function Model(path) end

---@class (exact) texture
local texture = {}

---An unique handle for a texture in memory.
---@param path string Path to file.
---@return texture # The user-data object.
function Texture(path) end

---Draw the texture.
---@param point The point of the texture.
---@param angle The point of the texture.
---@param scale The point of the texture.
---@param color The point of the texture.
function texture:draw(point, angle, scale, color) end

---Draw the texture (pro variant).
---@param rec_a The "source" rectangle of the texture.
---@param rec_b The "target" rectangle of the texture.
---@param angle The point of the texture.
---@param scale The point of the texture.
---@param color The point of the texture.
function texture:draw(point, angle, scale, color) end

---@class (exact) render_texture
local render_texture = {}

---An unique handle for a render texture in memory.
---@param path string Path to file.
---@return render_texture # The user-data object.
function RenderTexture(path) end

---@class (exact) image
local image = {}

---An unique handle for an image in memory.
---@param path string Path to file.
---@return image # The user-data object.
function Image(path) end

---@class (exact) font
local font = {}

---An unique handle for a font in memory.
---@param path string Path to file.
---@return font # The user-data object.
function Font(path) end

---@class (exact) shader
local shader = {}

---An unique handle for a shader in memory.
---@param path string Path to file.
---@return shader # The user-data object.
function Shader(path) end

---@class (exact) steam
local steam = {}

---A handle into the Steam API.
---@return steam # The user-data object.
function Steam() end

---Clear the screen with a color.
---@param color vector color to clear the screen with.
function clear_screen(color) end

---Get the size of the screen.
---@return vector # Screen size.
function get_window_shape() end

---Get the state of the window (minimized).
---@return boolean # True if minimized, false otherwise.
function get_window_minimized() end

---Get the state of the window (maximized).
---@return boolean # True if maximized, false otherwise.
function get_window_maximized() end

---Get the state of the window (focused).
---@return boolean # True if focused, false otherwise.
function get_window_focused() end

---Get the state of the window (resized).
---@return boolean # True if resized, false otherwise.
function get_window_resized() end

---Initialize the 3D draw mode. **MUST** call *close_mode_3d* after 3D drawing is done.
---@param point vector The point of the camera.
---@param focus vector The focus of the camera.
---@param up vector The direction pointing "up" of the camera.
---@param zoom number The zoom of the camera.
function begin_mode_3d(point, focus, up, zoom) end

---Finalize the 3D draw mode.
function close_mode_3d() end

---*3D mode operation.* Draw a grid.
---@param slice number The slice count of the grid.
---@param space number The space shift of the grid.
function draw_grid(slice, space) end

---*3D mode operation.* Draw a cube.
---@param point vector The point of the cube.
---@param shape vector The shape of the cube.
---@param color vector The color of the cube.
function draw_cube(point, shape, color) end

--- foo

--- foo

---Initialize the 2D draw mode. **MUST** call *close_mode_2d* after 2D drawing is done.
---@param point vector The point of the camera.
---@param focus vector The focus of the camera.
---@param angle number The angle of the camera.
---@param zoom number The zoom of the camera.
function begin_mode_2d(point, focus, angle, zoom) end

---Finalize the 2D draw mode.
function close_mode_2d() end

---*2D mode operation.* Get the world-space point for a screen-space point.
---@param point vector Screen-space point to convert from.
---@param c_point vector The point of the camera.
---@param c_focus vector The focus of the camera.
---@param c_angle number The angle of the camera.
---@param c_zoom number The zoom of the camera.
---@return vector # World-space point.
function get_screen_to_world(point, c_point, c_focus, c_angle, c_zoom) end

---*2D mode operation.* Get the screen-space point for a world-space point.
---@param point vector World-space point to convert from.
---@param c_point vector The point of the camera.
---@param c_focus vector The focus of the camera.
---@param c_angle number The angle of the camera.
---@param c_zoom number The zoom of the camera.
---@return vector # Screen-space point.
function get_world_to_screen(point, c_point, c_focus, c_angle, c_zoom) end

---*2D mode operation.* Draw a rectangle.
---@param shape vector Shape for the rectangle.
---@param point vector Point for the rectangle.
---@param angle vector Angle for the rectangle.
---@param color vector Color for the rectangle.
function draw_rectangle(shape, point, angle, color) end

---*2D mode operation.* Draw text.
---@param text string Text to draw.
---@param point vector Point for the text.
---@param scale number Scale for the text.
---@param color vector Color for the text.
function draw_text(text, point, scale, color) end

---@class (exact) sound
local sound = {}

---An unique handle for sound in memory.
---@param path string Path to file.
---@return sound # The user-data object.
function Sound(path) end

--- foo

---@class (exact) music
local music = {}

---An unique handle for music in memory.
---@param path string Path to file.
---@return music # The user-data object.
function Music(path) end

---Play music.
function music:play() end

---Update music stream.
function music:update() end

---Set the interface alpha.
---@param value number The alpha of the interface.
function set_interface_alpha(value) end

---Draw an interface button.
---@param shape vector The shape of the button. X/Y for position, Z/W for width/height.
---@param label string The label of the button.
---@return boolean # True on button click.
function interface_button(shape, label) end

---Draw an interface toggle.
---@param shape vector  The shape of the slider. X/Y for position, Z/W for width/height.
---@param label string  The label of the slider.
---@param value boolean The value of the slider.
---@return boolean # The new value of *value*, if any.
function interface_toggle(shape, label, value) end

---Draw an interface check box.
---@param shape vector  The shape of the check box. X/Y for position, Z/W for width/height.
---@param label string  The label of the check box.
---@param value boolean The value of the check box.
---@return boolean # The new value of *value*, if any.
function interface_check_box(shape, label, value) end

---Draw an interface spinner.
---@param shape vector  The shape of the spinner. X/Y for position, Z/W for width/height.
---@param label string  The label of the spinner.
---@param value number  The value of the spinner.
---@param min   number  The minimum value of the spinner.
---@param max   number  The maximum value of the spinner.
---@param edit  boolean The edit mode value of the spinner.
---@return number # The new value of *value*, if any.
function interface_spinner(shape, label, value, min, max, edit) end

---Draw an interface combo box.
---@param shape vector The shape of the combo box. X/Y for position, Z/W for width/height.
---@param label string The label of the combo box.
---@param value number The value of the combo box.
---@return number # The new value of *value*, if any.
function interface_combo_box(shape, label, value) end

---Draw an interface slider.
---@param shape   vector The shape of the slider. X/Y for position, Z/W for width/height.
---@param label_a string The label of the slider.
---@param label_b string The label of the slider.
---@param value   number The value of the slider.
---@param min     number The minimum value of the slider.
---@param max     number The maximum value of the slider.
---@return number # The new value of *value*, if any.
function interface_slider(shape, label_a, label_b, value, min, max) end

---Draw an interface slider bar.
---@param shape   vector The shape of the slider bar. X/Y for position, Z/W for width/height.
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
---@return vector # The point of the mouse.
function get_mouse_point() end

---Get the current delta (i.e. mouse movement) of the mouse.
---@return vector # The delta of the mouse.
function get_mouse_delta() end

---Set the current point of the mouse.
---@param value vector New point.
function set_mouse_point(value) end

---Set the current shift of the mouse.
---@param value vector New shift.
function set_mouse_shift(value) end

---Set the current scale of the mouse.
---@param value vector New scale.
function set_mouse_scale(value) end

---Set the current cursor of the mouse.
---@param value cursor_mouse New cursor.
function set_mouse_cursor(value) end

---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return vector # The delta of the mouse wheel.
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

