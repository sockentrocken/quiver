-- BSD Zero Clause License
--
-- Copyright (c) 2025 sockentrocken
--
-- Permission to use, copy, modify, and/or distribute this software for any
-- purpose with or without fee is hereby granted.
--
-- THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
-- REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
-- AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
-- INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
-- LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
-- OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
-- PERFORMANCE OF THIS SOFTWARE.

bit = require("bit")

---@enum rigid_body_kind
RIGID_BODY_KIND = {
    DYNAMIC = "Dynamic",
    FIXED = "Fixed",
    KINEMATIC_POSITION_BASED = "KinematicPositionBased",
    KINEMATIC_VELOCITY_BASED = "KinematicVelocityBased",
}

---@enum input_board
INPUT_BOARD = {
    KEY_NULL = 0,
    KEY_APOSTROPHE = 39,
    KEY_COMMA = 44,
    KEY_MINUS = 45,
    KEY_PERIOD = 46,
    KEY_SLASH = 47,
    KEY_ZERO = 48,
    KEY_ONE = 49,
    KEY_TWO = 50,
    KEY_THREE = 51,
    KEY_FOUR = 52,
    KEY_FIVE = 53,
    KEY_SIX = 54,
    KEY_SEVEN = 55,
    KEY_EIGHT = 56,
    KEY_NINE = 57,
    KEY_SEMICOLON = 59,
    KEY_EQUAL = 61,
    KEY_A = 65,
    KEY_B = 66,
    KEY_C = 67,
    KEY_D = 68,
    KEY_E = 69,
    KEY_F = 70,
    KEY_G = 71,
    KEY_H = 72,
    KEY_I = 73,
    KEY_J = 74,
    KEY_K = 75,
    KEY_L = 76,
    KEY_M = 77,
    KEY_N = 78,
    KEY_O = 79,
    KEY_P = 80,
    KEY_Q = 81,
    KEY_R = 82,
    KEY_S = 83,
    KEY_T = 84,
    KEY_U = 85,
    KEY_V = 86,
    KEY_W = 87,
    KEY_X = 88,
    KEY_Y = 89,
    KEY_Z = 90,
    KEY_LEFT_BRACKET = 91,
    KEY_BACKSLASH = 92,
    KEY_RIGHT_BRACKET = 93,
    KEY_GRAVE = 96,
    KEY_SPACE = 32,
    KEY_ESCAPE = 256,
    KEY_ENTER = 257,
    KEY_TAB = 258,
    KEY_BACKSPACE = 259,
    KEY_INSERT = 260,
    KEY_DELETE = 261,
    KEY_RIGHT = 262,
    KEY_LEFT = 263,
    KEY_DOWN = 264,
    KEY_UP = 265,
    KEY_PAGE_UP = 266,
    KEY_PAGE_DOWN = 267,
    KEY_HOME = 268,
    KEY_END = 269,
    KEY_CAPS_LOCK = 280,
    KEY_SCROLL_LOCK = 281,
    KEY_NUM_LOCK = 282,
    KEY_PRINT_SCREEN = 283,
    KEY_PAUSE = 284,
    KEY_F1 = 290,
    KEY_F2 = 291,
    KEY_F3 = 292,
    KEY_F4 = 293,
    KEY_F5 = 294,
    KEY_F6 = 295,
    KEY_F7 = 296,
    KEY_F8 = 297,
    KEY_F9 = 298,
    KEY_F10 = 299,
    KEY_F11 = 300,
    KEY_F12 = 301,
    KEY_LEFT_SHIFT = 340,
    KEY_LEFT_CONTROL = 341,
    KEY_LEFT_ALT = 342,
    KEY_LEFT_SUPER = 343,
    KEY_RIGHT_SHIFT = 344,
    KEY_RIGHT_CONTROL = 345,
    KEY_RIGHT_ALT = 346,
    KEY_RIGHT_SUPER = 347,
    KEY_KB_MENU = 348,
    KEY_KP_0 = 320,
    KEY_KP_1 = 321,
    KEY_KP_2 = 322,
    KEY_KP_3 = 323,
    KEY_KP_4 = 324,
    KEY_KP_5 = 325,
    KEY_KP_6 = 326,
    KEY_KP_7 = 327,
    KEY_KP_8 = 328,
    KEY_KP_9 = 329,
    KEY_KP_DECIMAL = 330,
    KEY_KP_DIVIDE = 331,
    KEY_KP_MULTIPLY = 332,
    KEY_KP_SUBTRACT = 333,
    KEY_KP_ADD = 334,
    KEY_KP_ENTER = 335,
    KEY_KP_EQUAL = 336,
    KEY_BACK = 4,
    KEY_VOLUME_UP = 24,
    KEY_VOLUME_DOWN = 25,
}

---@enum input_mouse
INPUT_MOUSE = {
    MOUSE_BUTTON_LEFT = 0,
    MOUSE_BUTTON_RIGHT = 1,
    MOUSE_BUTTON_MIDDLE = 2,
    MOUSE_BUTTON_SIDE = 3,
    MOUSE_BUTTON_EXTRA = 4,
    MOUSE_BUTTON_FORWARD = 5,
    MOUSE_BUTTON_BACK = 6,
}

---@enum cursor_mouse
CURSOR_MOUSE = {
    MOUSE_CURSOR_DEFAULT       = 0,
    MOUSE_CURSOR_ARROW         = 1,
    MOUSE_CURSOR_IBEAM         = 2,
    MOUSE_CURSOR_CROSSHAIR     = 3,
    MOUSE_CURSOR_POINTING_HAND = 4,
    MOUSE_CURSOR_RESIZE_EW     = 5,
    MOUSE_CURSOR_RESIZE_NS     = 6,
    MOUSE_CURSOR_RESIZE_NWSE   = 7,
    MOUSE_CURSOR_RESIZE_NESW   = 8,
    MOUSE_CURSOR_RESIZE_ALL    = 9,
    MOUSE_CURSOR_NOT_ALLOWED   = 10
}

---@enum input_pad
INPUT_PAD = {
    GAMEPAD_BUTTON_UNKNOWN = 0,
    GAMEPAD_BUTTON_LEFT_FACE_UP = 1,
    GAMEPAD_BUTTON_LEFT_FACE_RIGHT = 2,
    GAMEPAD_BUTTON_LEFT_FACE_DOWN = 3,
    GAMEPAD_BUTTON_LEFT_FACE_LEFT = 4,
    GAMEPAD_BUTTON_RIGHT_FACE_UP = 5,
    GAMEPAD_BUTTON_RIGHT_FACE_RIGHT = 6,
    GAMEPAD_BUTTON_RIGHT_FACE_DOWN = 7,
    GAMEPAD_BUTTON_RIGHT_FACE_LEFT = 8,
    GAMEPAD_BUTTON_LEFT_TRIGGER_1 = 9,
    GAMEPAD_BUTTON_LEFT_TRIGGER_2 = 10,
    GAMEPAD_BUTTON_RIGHT_TRIGGER_1 = 11,
    GAMEPAD_BUTTON_RIGHT_TRIGGER_2 = 12,
    GAMEPAD_BUTTON_MIDDLE_LEFT = 13,
    GAMEPAD_BUTTON_MIDDLE = 14,
    GAMEPAD_BUTTON_MIDDLE_RIGHT = 15,
    GAMEPAD_BUTTON_LEFT_THUMB = 16,
    GAMEPAD_BUTTON_RIGHT_THUMB = 17,
}

---@enum shader_location
SHADER_LOCATION = {
    VERTEX_POSITION = 0,     -- Shader location: vertex attribute: position
    VERTEX_TEXCOORD01 = 1,   -- Shader location: vertex attribute: texcoord01
    VERTEX_TEXCOORD02 = 2,   -- Shader location: vertex attribute: texcoord02
    VERTEX_NORMAL = 3,       -- Shader location: vertex attribute: normal
    VERTEX_TANGENT = 4,      -- Shader location: vertex attribute: tangent
    VERTEX_COLOR = 5,        -- Shader location: vertex attribute: color
    MATRIX_MVP = 6,          -- Shader location: matrix uniform: model-view-projection
    MATRIX_VIEW = 7,         -- Shader location: matrix uniform: view (camera transform)
    MATRIX_PROJECTION = 8,   -- Shader location: matrix uniform: projection
    MATRIX_MODEL = 9,        -- Shader location: matrix uniform: model (transform)
    MATRIX_NORMAL = 10,      -- Shader location: matrix uniform: normal
    VECTOR_VIEW = 11,        -- Shader location: vector uniform: view
    COLOR_DIFFUSE = 12,      -- Shader location: vector uniform: diffuse color
    COLOR_SPECULAR = 13,     -- Shader location: vector uniform: specular color
    COLOR_AMBIENT = 14,      -- Shader location: vector uniform: ambient color
    MAP_ALBEDO = 15,         -- Shader location: sampler2d texture: albedo (same as: SHADER_LOC_MAP_DIFFUSE)
    MAP_METALNESS = 16,      -- Shader location: sampler2d texture: metalness (same as: SHADER_LOC_MAP_SPECULAR)
    MAP_NORMAL = 17,         -- Shader location: sampler2d texture: normal
    MAP_ROUGHNESS = 18,      -- Shader location: sampler2d texture: roughness
    MAP_OCCLUSION = 19,      -- Shader location: sampler2d texture: occlusion
    MAP_EMISSION = 20,       -- Shader location: sampler2d texture: emission
    MAP_HEIGHT = 21,         -- Shader location: sampler2d texture: height
    MAP_CUBEMAP = 22,        -- Shader location: samplerCube texture: cubemap
    MAP_IRRADIANCE = 23,     -- Shader location: samplerCube texture: irradiance
    MAP_PREFILTER = 24,      -- Shader location: samplerCube texture: prefilter
    MAP_BRDF = 25,           -- Shader location: sampler2d texture: brdf
    VERTEX_BONEIDS = 26,     -- Shader location: vertex attribute: boneIds
    VERTEX_BONEWEIGHTS = 27, -- Shader location: vertex attribute: boneWeights
    BONE_MATRICES = 28,      -- Shader location: array of matrices uniform: boneMatrices
    VERTEX_INSTANCE_TX = 29  -- Shader location: vertex attribute: instanceTransform
}

---@enum window_flag
WINDOW_FLAG = {
    VSYNC_HINT               = 0x00000040, -- Set to try enabling V-Sync on GPU
    FULLSCREEN_MODE          = 0x00000002, -- Set to run program in fullscreen
    RESIZABLE                = 0x00000004, -- Set to allow resizable window
    UNDECORATED              = 0x00000008, -- Set to disable window decoration (window and buttons)
    HIDDEN                   = 0x00000080, -- Set to hide window
    MINIMIZED                = 0x00000200, -- Set to minimize window (iconify)
    MAXIMIZED                = 0x00000400, -- Set to maximize window (expanded to monitor)
    UNFOCUSED                = 0x00000800, -- Set to window non focused
    TOPMOST                  = 0x00001000, -- Set to window always on top
    ALWAYS_RUN               = 0x00000100, -- Set to allow windows running while minimized
    TRANSPARENT              = 0x00000010, -- Set to allow transparent windowbuffer
    HIGHDPI                  = 0x00002000, -- Set to support HighDPI
    MOUSE_PASSTHROUGH        = 0x00004000, -- Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
    BORDERLESS_WINDOWED_MODE = 0x00008000, -- Set to run program in borderless windowed mode
    MSAA_4X_HINT             = 0x00000020, -- Set to try enabling MSAA 4X
    INTERLACED_HINT          = 0x00010000  -- Set to try enabling interlaced video format (for V3D)
}

---@enum texture_filter
TEXTURE_FILTER = {
    POINT = 0,           -- No filter, just pixel approximation
    BILINEAR = 1,        -- Linear filtering
    TRILINEAR = 2,       -- Trilinear filtering (linear with mipmaps)
    ANISOTROPIC_4X = 3,  -- Anisotropic filtering 4x
    ANISOTROPIC_8X = 4,  -- Anisotropic filtering 8x
    ANISOTROPIC_16X = 5, -- Anisotropic filtering 16x
}

---@enum texture_wrap
TEXTURE_WRAP = {
    REPEAT = 0,        -- Repeats texture in tiled mode
    CLAMP = 1,         -- Clamps texture to edge pixel in tiled mode
    MIRROR_REPEAT = 2, -- Mirrors and repeats the texture in tiled mode
    MIRROR_CLAMP = 3   -- Mirrors and clamps to border the texture in tiled mode
}

--[[----------------------------------------------------------------]]

collision = {}

---Check if a point and a box are colliding.
---@param  point   vector_2  # Point to check.
---@param  box     box_2     # Box to check.
---@return boolean collision # True if colliding, false otherwise.
function collision.point_box(point, box)
    return (point.x >= box.x) and (point.x < (box.x + box.width)) and (point.y >= box.y) and
        (point.y < (box.y + box.height))
end

---Check if a box and a box are colliding.
---@param  box_a box_2 # Box A to check.
---@param  box_b box_2 # Box B to check.
---@return boolean collision # True if colliding, false otherwise.
function collision.box_box(box_a, box_b)
    return (box_a.x < (box_b.x + box_b.width) and (box_a.x + box_a.width) > box_b.x) and
        (box_a.y < (box_b.y + box_b.height) and (box_a.y + box_a.height) > box_b.y)
end

--[[----------------------------------------------------------------]]

---A table pool, for initializing a memory arena of a certain kind for borrowing later.
---@class table_pool
---@field index number
---@field count number
---@field kind  table
table_pool = {
    __meta = {}
}

---Create a new table pool.
---@param kind table  # The kind of table this table pool will initialize a memory arena for. MUST have a "default" function.
---@param size number # The size of the table.
function table_pool:new(kind, size)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    -- initialize the table pool from 1 to {size} with the default instance of the {kind}.
    for x = 1, size + 1 do
        i[x] = kind:default()
    end

    i.__type = "table_pool"
    i.index = 1
    i.count = size
    i.kind = kind

    return i
end

---Clear the table pool index.
function table_pool:begin()
    self.index = 1
end

---Borrow a table from the table pool. WILL allocate a new table if every table in the pool is already in use.
function table_pool:get()
    -- increase the index by 1.
    self.index = self.index + 1

    -- index overflow!
    if self.index > self.count then
        error("index overflow")
        -- create a new table.
        self[self.index] = self.kind:default()
        -- update our known table pool size.
        self.count = self.index
    end

    -- borrow table.
    return self[self.index - 1]
end

--[[----------------------------------------------------------------]]

---Check if a string does start with another string.
---@param text string # Main text.
---@param find string # Text to check against with main text.
function string.start_with(text, find)
    return string.sub(text, 1, string.len(find)) == find
end

---Tokenize a string.
---@param text string # Text to tokenize.
---@param find string # Pattern to tokenize with.
function string.tokenize(text, find)
    local i = {}

    for token in text:gmatch(find) do
        table.insert(i, token)
    end

    return i
end

---Print every key/value pair in a table.
---@param value table # Table to print.
function table.print(value)
    for k, v in pairs(value) do
        print(tostring(k) .. ":" .. tostring(v))

        if type(v) == "table" then
            table.print(v)
        end
    end
end

---Remove an object from an array table by value.
---@param value  table # Table to remove the value from.
---@param object any   # Value to remove.
function table.remove_object(value, object)
    for k, v in ipairs(value) do
        if v == object then
            table.remove(value, k)
            return
        end
    end
end

---Recursively restore every table within a table's meta table.
---@param value table # Table to restore.
function table.restore_meta(value)
    -- for each key/value pair in the table...
    for k, v in pairs(value) do
        -- if the current value is a table...
        if type(v) == "table" then
            -- if the current table has a .__type field...
            if v.__type then
                -- locate the "class" table.
                local meta = _G[v.__type]

                -- if the class table does exist...
                if meta then
                    -- restore the current table's meta-table to be that of the class table.
                    setmetatable(v, meta.__meta)
                else
                    error(string.format(
                        "table.restore_meta(): Found \"__type\" for table, but could not find \"%s\" class table.",
                        v.__type))
                end
            end

            -- recursively iterate table.
            table.restore_meta(v)
        end
    end

    -- check the given value as well.
    if type(value) == "table" then
        -- if the current table has a .__type field...
        if value.__type then
            -- locate the "class" table.
            local meta = _G[value.__type]

            -- if the class table does exist...
            if meta then
                -- restore the current table's meta-table to be that of the class table.
                setmetatable(value, meta.__meta)
            else
                error(string.format(
                    "table.restore_meta(): Found \"__type\" for table, but could not find \"%s\" class table.",
                    value.__type))
            end
        end
    end
end

---Check the sanity of a number, which will check for NaN and Infinite.
---@param value number # Number to check.
---@return boolean sanity # True if number is not sane, false otherwise.
function math.sanity(value)
    return not (value == value) or value == math.huge
end

---Check the sign of a number.
---@param value number # Number to check.
---@return number sign # 1.0 if number is positive OR equal to 0.0, -1.0 otherwise.
function math.sign(value)
    return value >= 0 and 1.0 or -1.0
end

---Get the percentage of a value in a range.
---@param min number # Minimum value.
---@param max number # Maximum value.
---@param value number # Input value.
---@return number percentage # Percentage.
function math.percentage_from_value(min, max, value)
    return (value - min) / (max - min)
end

---Get the value of a percentage in a range.
---@param min number # Minimum value.
---@param max number # Maximum value.
---@param value number # Input percentage.
---@return number value # Value.
function math.value_from_percentage(min, max, value)
    return value * (max - min) + min
end

---Snap a value to a given step.
---@param step number # Step.
---@param value number # Input value.
---@return number value # Value.
function math.snap(snap, value)
    return math.floor(value / snap) * snap
end

---Get a random variation of a given value, which can either be positive or negative.
---@param value number # Number to randomize.
---@return number value # A value between [-number, number].
function math.random_sign(value)
    local random = math.random()
    if random > 0.5 then
        return value * percentage_from_value(0.5, 1.0, random)
    else
        return value * percentage_from_value(0.0, 0.5, random) * -1.0
    end
end

---Linear interpolation.
---@param a    number # Point "A".
---@param b    number # Point "B".
---@param time number # Time into the interpolation.
---@return number interpolation # The interpolation.
function math.interpolate(a, b, time)
    return (1.0 - time) * a + time * b
end

---Clamp a value in a range.
---@param min   number # Minimum value.
---@param max   number # Maximum value.
---@param value number # Value to clamp.
---@return number value # The value, within the min/max range.
function math.clamp(min, max, value)
    if value < min then return min end
    if value > max then return max end
    return value
end

---Roll-over a value: if value is lower than the minimum, roll-over to the maximum, and viceversa.
---@param min   number # Minimum value.
---@param max   number # Maximum value.
---@param value number # Value to roll-over.
---@return number value # The value, within the min/max roll-over range.
function math.roll_over(min, max, value)
    if value < min then return max end
    if value > max then return min end
    return value
end

---Return the "X", "Y", "Z" vector from an Euler angle.
---@param angle vector_3
---@return vector_3 d_x # "X" direction.
---@return vector_3 d_y # "Y" direction.
---@return vector_3 d_z # "Z" direction.
function math.direction_from_euler(angle)
    local d_x = vector_3:zero()
    local d_y = vector_3:zero()
    local d_z = vector_3:zero()

    -- Convert to radian.
    local angle = vector_2:old(angle.x * (math.pi / 180.0), angle.y * (math.pi / 180.0))

    -- "X" vector.
    d_x.x = math.cos(angle.y) * math.sin(angle.x)
    d_x.y = math.sin(angle.y) * -1.0
    d_x.z = math.cos(angle.y) * math.cos(angle.x)

    -- "Y" vector.
    d_y.x = math.sin(angle.y) * math.sin(angle.x)
    d_y.y = math.cos(angle.y)
    d_y.z = math.sin(angle.y) * math.cos(angle.x)

    -- "Z" vector.
    d_z.x = math.cos(angle.x)
    d_z.y = 0.0
    d_z.z = math.sin(angle.x) * -1.0

    return d_x, d_y, d_z
end

--[[----------------------------------------------------------------]]

---@class vector_2
---@field x number
---@field y number
vector_2 = {
    __meta = {
        __add = function(a, b) return vector_2:old(a.x + b.x, a.y + b.y) end,
        __sub = function(a, b) return vector_2:old(a.x - b.x, a.y - b.y) end,
        __mul = function(a, b)
            if type(a) == "number" then
                return vector_2:old(a * b.x, a * b.y)
            elseif type(b) == "number" then
                return vector_2:old(a.x * b, a.y * b)
            else
                return vector_2:old(a.x * b.x, a.y * b.y)
            end
        end,
        __div = function(a, b) return vector_2:old(a.x / b.x, a.y / b.y) end,
        __tostring = function(a)
            return "{ x:" .. tostring(a.x) .. " y:" .. tostring(a.y) .. " }" .. tostring(a.z) .. " }"
        end
    }
}

---Create a new vector (2 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@return vector_2 value # The vector.
function vector_2:new(x, y)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "vector_2"
    i.x = x
    i.y = y

    return i
end

function vector_2:default()
    return vector_2:new(0.0, 0.0)
end

---Borrow an old vector from the vector pool. (2 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@return vector_2 value # The vector.
function vector_2:old(x, y)
    local i = vector_2_pool:get()
    i.x = x
    i.y = y
    return i
end

---Create a new, GC-vector from an old table-pool vector.
---@return vector_2 value # The new vector.
function vector_2:old_to_new()
    return vector_2:new(self.x, self.y)
end

---Borrow an old, table-pool vector from a new GC-vector.
---@return vector_2 value # The old vector.
function vector_2:new_to_old()
    return vector_2:old(self.x, self.y)
end

---Copy the data of a given vector into the current vector.
---@param value vector_2 # The vector to copy from.
function vector_2:copy(value)
    self.x = value.x
    self.y = value.y
end

---Get the "X" vector.
---@return vector_2 value # The vector.
function vector_2:x()
    return vector_2:old(1.0, 0.0)
end

---Get the "Y" vector.
---@return vector_2 value # The vector.
function vector_2:y()
    return vector_2:old(0.0, 1.0)
end

---Get a vector, with every component set to "1".
---@return vector_2 value # The vector.
function vector_2:one()
    return vector_2:old(1.0, 1.0)
end

---Get a vector, with every component set to "0".
---@return vector_2 value # The vector.
function vector_2:zero()
    return vector_2:old(0.0, 0.0)
end

local POOL_VECTOR_2_AMOUNT = 1024

vector_2_pool = table_pool:new(vector_2, POOL_VECTOR_2_AMOUNT)

--[[----------------------------------------------------------------]]

---@class vector_3
---@field x number
---@field y number
---@field z number
vector_3 = {
    __meta = {
        __add = function(a, b) return vector_3:old(a.x + b.x, a.y + b.y, a.z + b.z) end,
        __sub = function(a, b) return vector_3:old(a.x - b.x, a.y - b.y, a.z - b.z) end,
        __mul = function(a, b)
            if type(a) == "number" then
                return vector_3:old(a * b.x, a * b.y, a * b.z)
            elseif type(b) == "number" then
                return vector_3:old(a.x * b, a.y * b, a.z * b)
            else
                return vector_3:old(a.x * b.x, a.y * b.y, a.z * b.z)
            end
        end,
        __div = function(a, b) return vector_3:old(a.x / b.x, a.y / b.y, a.z / b.z) end,
        __tostring = function(a)
            return string.format("{ x : %.2f, y: %.2f, z: %.2f }", a.x, a.y, a.z)
        end
    }
}

---Create a new vector (3 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@param z number # "Z" component.
---@return vector_3 value # The vector.
function vector_3:new(x, y, z)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "vector_3"
    i.x = x
    i.y = y
    i.z = z

    return i
end

function vector_3:default()
    return vector_3:new(0.0, 0.0, 0.0)
end

---Borrow an old vector from the vector pool. (3 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@param z number # "Z" component.
---@return vector_3 value # The vector.
function vector_3:old(x, y, z)
    local i = vector_3_pool:get()
    i.x = x
    i.y = y
    i.z = z
    return i
end

---Create a new, GC-vector from an old table-pool vector.
---@return vector_3 value # The new vector.
function vector_3:old_to_new()
    return vector_3:new(self.x, self.y, self.z)
end

---Borrow an old, table-pool vector from a new GC-vector.
---@return vector_3 value # The old vector.
function vector_3:new_to_old()
    return vector_3:old(self.x, self.y, self.z)
end

---Copy the data of a given vector into the current vector.
---@param value vector_3 # The vector to copy from.
function vector_3:copy(value)
    self.x = value.x
    self.y = value.y
    self.z = value.z
end

---Get the "X" vector.
---@return vector_3 value # The vector.
function vector_3:x()
    return vector_3:old(1.0, 0.0, 0.0)
end

---Get the "Y" vector.
---@return vector_3 value # The vector.
function vector_3:y()
    return vector_3:old(0.0, 1.0, 0.0)
end

---Get the "Z" vector.
---@return vector_3 value # The vector.
function vector_3:z()
    return vector_3:old(0.0, 0.0, 1.0)
end

---Get a vector, with every component set to "1".
---@return vector_3 value # The vector.
function vector_3:one()
    return vector_3:old(1.0, 1.0, 1.0)
end

---Get a vector, with every component set to "0".
---@return vector_3 value # The vector.
function vector_3:zero()
    return vector_3:old(0.0, 0.0, 0.0)
end

---Get the dot product between the current vector, and another one.
---@param value vector_3 # Vector to perform the dot product with.
---@return number value # The dot product.
function vector_3:dot(value)
    return (self.x * value.x + self.y * value.y + self.z * value.z)
end

---Get the cross product between the current vector, and another one.
---@param value vector_3 # Vector to perform the cross product with.
---@return vector_3 value # The cross product.
function vector_3:cross(value)
    return vector_3:old(self.y * value.z - self.z * value.y, self.z * value.x - self.x * value.z,
        self.x * value.y - self.y * value.x)
end

---Get the magnitude of the current vector.
---@return number value # The magnitude.
function vector_3:magnitude()
    return math.sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
end

---Get the unit vector of the current vector.
---@return vector_3 value # The unit vector.
function vector_3:normalize()
    local length = math.sqrt(self.x * self.x + self.y * self.y + self.z * self.z)

    if not (length == 0.0) then
        local length = 1.0 / length
        return vector_3:old(self.x * length, self.y * length, self.z * length)
    else
        return self
    end
end

---Rotate the current vector by an axis and an angle.
---@param axis  vector_3 # The axis.
---@param angle number # The angle.
---@return vector_3 value # The vector.
function vector_3:rotate_axis_angle(axis, angle)
    local axis = axis:normalize()

    angle      = angle / 2.0
    local a    = math.sin(angle)
    local b    = axis.x * a
    local c    = axis.y * a
    local d    = axis.z * a
    a          = math.cos(angle)
    local w    = vector_3:old(b, c, d)

    local wv   = w:cross(self)

    local wwv  = w:cross(wv)

    wv         = wv * a * 2.0

    wwv        = wwv * 2.0

    return vector_3:old(self.x + wv.x + wwv.x, self.y + wv.y + wwv.y, self.z + wv.z + wwv.z)
end

local POOL_VECTOR_3_AMOUNT = 1024

vector_3_pool = table_pool:new(vector_3, POOL_VECTOR_3_AMOUNT)

--[[----------------------------------------------------------------]]

---@class vector_4
---@field x number
---@field y number
---@field z number
---@field w number
vector_4 = {
    __meta = {
        __add = function(a, b) return vector_4:old(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w) end,
        __sub = function(a, b) return vector_4:old(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w) end,
        __mul = function(a, b)
            if type(a) == "number" then
                return vector_4:old(a * b.x, a * b.y, a * b.z, a * b.w)
            elseif type(b) == "number" then
                return vector_4:old(a.x * b, a.y * b, a.z * b, a.w * b)
            else
                return vector_4:old(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w)
            end
        end,
        __div = function(a, b) return vector_4:old(a.x / b.x, a.y / b.y, a.z / b.z, a.w / b.w) end,
        __tostring = function(a)
            return "{ x:" ..
                tostring(a.x) .. " y:" .. tostring(a.y) .. " z:" .. tostring(a.z) .. " w:" .. tostring(a.w) .. " }"
        end
    }
}

---Create a new vector (4 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@param z number # "Z" component.
---@param w number # "W" component.
---@return vector_4 value # The vector.
function vector_4:new(x, y, z, w)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "vector_4"
    i.x = x
    i.y = y
    i.z = z
    i.w = w

    return i
end

function vector_4:default()
    return vector_4:new(0.0, 0.0, 0.0, 0.0)
end

---Borrow an old vector from the vector pool. (3 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@param z number # "Z" component.
---@return vector_4 value # The vector.
function vector_4:old(x, y, z, w)
    local i = vector_4_pool:get()
    i.x = x
    i.y = y
    i.z = z
    i.w = w
    return i
end

---Create a new, GC-vector from an old table-pool vector.
---@return vector_4 value # The new vector.
function vector_4:old_to_new()
    return vector_4:new(self.x, self.y, self.z, self.w)
end

---Borrow an old, table-pool vector from a new GC-vector.
---@return vector_4 value # The old vector.
function vector_4:new_to_old()
    return vector_4:old(self.x, self.y, self.z, self.w)
end

---Copy the data of a given vector into the current vector.
---@param value vector_4 # The vector to copy from.
function vector_4:copy(value)
    self.x = value.x
    self.y = value.y
    self.z = value.z
    self.w = value.w
end

---Get the "X" vector.
---@return vector_4 value # The vector.
function vector_4:x()
    return vector_4:old(1.0, 0.0, 0.0, 0.0)
end

---Get the "Y" vector.
---@return vector_4 value # The vector.
function vector_4:y()
    return vector_4:old(0.0, 1.0, 0.0, 0.0)
end

---Get the "Z" vector.
---@return vector_4 value # The vector.
function vector_4:z()
    return vector_4:old(0.0, 0.0, 1.0, 0.0)
end

---Get the "W" vector.
---@return vector_4 value # The vector.
function vector_4:w()
    return vector_4:old(0.0, 0.0, 0.0, 1.0)
end

---Get a vector, with every component set to "1".
---@return vector_4 value # The vector.
function vector_4:one()
    return vector_4:old(1.0, 1.0, 1.0, 1.0)
end

---Get a vector, with every component set to "0".
---@return vector_4 value # The vector.
function vector_4:zero()
    return vector_4:old(0.0, 0.0, 0.0, 0.0)
end

local POOL_VECTOR_4_AMOUNT = 1024

vector_4_pool = table_pool:new(vector_4, POOL_VECTOR_4_AMOUNT)

--[[----------------------------------------------------------------]]

---@class camera_2d
---@field shift vector_2
---@field focus vector_2
---@field angle number
---@field zoom  number
camera_2d = {
    __meta = {}
}

function camera_2d:new(shift, focus, angle, zoom)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "camera_2d"
    i.shift = shift
    i.focus = focus
    i.angle = angle
    i.zoom = zoom

    return i
end

function camera_2d:default()
    return camera_2d:new(vector_2:new(0.0, 0.0), vector_2:new(0.0, 0.0), 0.0, 0.0)
end

function camera_2d:old(shift, focus, angle, zoom)
    local i = camera_2d_pool:get()
    i.shift = shift
    i.focus = focus
    i.angle = angle
    i.zoom = zoom
    return i
end

local POOL_CAMERA_2D_AMOUNT = 4

camera_2d_pool = table_pool:new(camera_2d, POOL_CAMERA_2D_AMOUNT)

--[[----------------------------------------------------------------]]

---@enum camera_3d_kind
CAMERA_3D_KIND = {
    PERSPECTIVE = 0,
    ORTHOGRAPHIC = 1,
}

---@class camera_3d
---@field point vector_3
---@field focus vector_3
---@field angle vector_3
---@field zoom  number
---@field kind  camera_3d_kind
camera_3d = {
    __meta = {}
}

function camera_3d:new(point, focus, angle, zoom, kind)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "camera_3d"
    i.point = point
    i.focus = focus
    i.angle = angle
    i.zoom = zoom
    i.kind = kind

    return i
end

function camera_3d:default()
    return camera_3d:new(vector_3:new(0.0, 0.0, 0.0), vector_3:new(0.0, 0.0, 0.0), vector_3:new(0.0, 0.0, 0.0), 0.0,
        CAMERA_3D_KIND.PERSPECTIVE)
end

function camera_3d:old(point, focus, angle, zoom, kind)
    local i = camera_3d_pool:get()
    i.point = point
    i.focus = focus
    i.angle = angle
    i.zoom = zoom
    i.kind = kind
    return i
end

local POOL_CAMERA_3D_AMOUNT = 4

camera_3d_pool = table_pool:new(camera_3d, POOL_CAMERA_3D_AMOUNT)

--[[----------------------------------------------------------------]]

---@class color
---@field r number
---@field g number
---@field b number
---@field a number
color = {
    __meta = {
        __mul = function(a, b)
            if type(a) == "number" then
                return color:old(
                    math.floor(a * b.r),
                    math.floor(a * b.g),
                    math.floor(a * b.b),
                    b.a
                )
            elseif type(b) == "number" then
                return color:old(
                    math.floor(a.r * b),
                    math.floor(a.g * b),
                    math.floor(a.b * b),
                    a.a
                )
            else
                return color:old(
                    math.floor(a.r * b.r),
                    math.floor(a.g * b.g),
                    math.floor(a.b * b.b),
                    a.a * b.a
                )
            end
        end
    }
}

function color:new(r, g, b, a)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "color"
    i.r = r
    i.g = g
    i.b = b
    i.a = a

    return i
end

function color:default()
    return color:new(0.0, 0.0, 0.0, 0.0)
end

---Borrow an old color from the color pool.
---@param r number # "R" component.
---@param g number # "G" component.
---@param b number # "B" component.
---@param a number # "A" component.
---@return color value # The color.
function color:old(r, g, b, a)
    local i = color_pool:get()
    i.r = r
    i.g = g
    i.b = b
    i.a = a
    return i
end

---Create a new, GC-vector from an old table-pool vector.
---@return color value # The new vector.
function color:old_to_new()
    return color:new(self.r, self.g, self.b, self.a)
end

---Borrow an old, table-pool vector from a new GC-vector.
---@return color value # The old vector.
function color:new_to_old()
    return color:old(self.r, self.g, self.b, self.a)
end

---Copy the data of a given vector into the current vector.
---@param value color # The vector to copy from.
function color:copy(value)
    self.r = value.r
    self.g = value.g
    self.b = value.b
    self.a = value.a
end

function color:white()
    return color:old(255.0, 255.0, 255.0, 255.0)
end

function color:black()
    return color:old(0.0, 0.0, 0.0, 255.0)
end

function color:red()
    return color:old(255.0, 0.0, 0.0, 255.0)
end

function color:green()
    return color:old(0.0, 255.0, 0.0, 255.0)
end

function color:blue()
    return color:old(0.0, 0.0, 255.0, 255.0)
end

local POOL_COLOR_AMOUNT = 1024

color_pool = table_pool:new(color, POOL_COLOR_AMOUNT)

--[[----------------------------------------------------------------]]

---@class box_2
---@field x      number
---@field y      number
---@field width  number
---@field height number
box_2 = {
    __meta = {}
}

function box_2:new(x, y, width, height)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "box_2"
    i.x = x
    i.y = y
    i.width = width
    i.height = height

    return i
end

function box_2:default()
    return box_2:new(0.0, 0.0, 0.0, 0.0)
end

function box_2:old(x, y, width, height)
    local i = box_2_pool:get()
    i.x = x
    i.y = y
    i.width = width
    i.height = height
    return i
end

local POOL_BOX_2_AMOUNT = 1024

box_2_pool = table_pool:new(box_2, POOL_BOX_2_AMOUNT)

--[[----------------------------------------------------------------]]

---@class box_3
---@field min vector_3
---@field max vector_3
box_3 = {
    __meta = {}
}

function box_3:new(min, max)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "box_3"
    i.min = min
    i.max = max

    return i
end

function box_3:default()
    return box_3:new(vector_3:default(), vector_3:default())
end

function box_3:old(min, max)
    local i = box_3_pool:get()
    i.min = min
    i.max = max
    return i
end

local POOL_BOX_3_AMOUNT = 1024

box_3_pool = table_pool:new(box_3, POOL_BOX_3_AMOUNT)

--[[----------------------------------------------------------------]]

---@class ray
---@field position  vector_3
---@field direction vector_3
ray = {
    __meta = {}
}

function ray:new(position, direction)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type    = "ray"
    i.position  = position
    i.direction = direction

    return i
end

--[[----------------------------------------------------------------]]

local BORDER_COLOR_A_MAIN = color:new(76.0, 88.0, 68.0, 255.0)
local BORDER_COLOR_A_SIDE = color:new(62.0, 70.0, 55.0, 255.0)
local BORDER_COLOR_B = color:new(124.0, 133.0, 116.0, 255.0)
local BORDER_COLOR_C = color:new(37.0, 48.0, 31.0, 255.0)

function draw_box_2_border(box, invert, color)
    local thick = 2.0
    local thick_half = thick * 0.5

    local color_a = invert and BORDER_COLOR_A_SIDE or BORDER_COLOR_A_MAIN
    local color_b = invert and BORDER_COLOR_C or BORDER_COLOR_B
    local color_c = invert and BORDER_COLOR_B or BORDER_COLOR_C

    if color then
        color_a = invert and color * 0.25 or color
        color_b = invert and color * 0.50 or color * 0.75
        color_c = invert and color * 0.75 or color * 0.50
    end

    quiver.draw_2d.draw_box_2(box, vector_2:zero(), 0.0, color_a)

    quiver.draw_2d.draw_line(
        vector_2:old(box.x + thick_half, box.y),
        vector_2:old(box.x + thick_half, box.y + box.height),
        thick, color_b)
    quiver.draw_2d.draw_line(
        vector_2:old(box.x, box.y + thick_half),
        vector_2:old(box.x + box.width, box.y + thick_half),
        thick, color_b)

    quiver.draw_2d.draw_line(
        vector_2:old(box.x, box.y + box.height - thick_half),
        vector_2:old(box.x + box.width, box.y + box.height - thick_half),
        thick, color_c)
    quiver.draw_2d.draw_line(
        vector_2:old(box.x + box.width - thick_half, box.y),
        vector_2:old(box.x + box.width - thick_half, box.y + box.height),
        thick, color_c)
end

--[[----------------------------------------------------------------]]

local DOT_COLOR = color:new(255.0, 255.0, 255.0, 255.0)
local DOT_SPACE = 8.0

function draw_box_2_dot(box)
    for i = 0, math.floor((box.width - DOT_SPACE) / DOT_SPACE) do
        local point_a = vector_2:old(box.x + (i * DOT_SPACE), box.y)
        local point_b = vector_2:old(box.x + (i * DOT_SPACE) + DOT_SPACE / 2.0, box.y)

        quiver.draw_2d.draw_line(point_a, point_b, 2.0, DOT_COLOR)

        local point_a = vector_2:old(box.x + (i * DOT_SPACE), box.y + box.height)
        local point_b = vector_2:old(box.x + (i * DOT_SPACE) + DOT_SPACE / 2.0, box.y + box.height)

        quiver.draw_2d.draw_line(point_a, point_b, 2.0, DOT_COLOR)
    end

    for i = 0, math.floor((box.height - DOT_SPACE) / DOT_SPACE) do
        local point_a = vector_2:old(box.x, box.y + (i * DOT_SPACE))
        local point_b = vector_2:old(box.x, box.y + (i * DOT_SPACE) + DOT_SPACE / 2.0)

        quiver.draw_2d.draw_line(point_a, point_b, 2.0, DOT_COLOR)

        local point_a = vector_2:old(box.x + box.width, box.y + (i * DOT_SPACE))
        local point_b = vector_2:old(box.x + box.width, box.y + (i * DOT_SPACE) + DOT_SPACE / 2.0)

        quiver.draw_2d.draw_line(point_a, point_b, 2.0, DOT_COLOR)
    end
end

--[[----------------------------------------------------------------]]

---@class file_system
---@field search      table
---@field locate      table
---@field memory_list table
---@field memory_data table
file_system = {
    __meta = {}
}

---Create a new virtual file-system. For serialization, you may want to only serialize "search", "locate", and "memory_list", which only contain serializable data.
---```lua
---local i = file_system:new({
---    "game_folder_1", -- image.png, sound.wav, model.obj
---    "game_folder_2", -- image.png
---    "game_folder_3"  -- sound.wav
---})
---
----- Scan "g_f_1", "g_f_2", "g_f_3" to update the asset look-up table.
---i:scan()
---
---i:find("image.png") -- "game_folder_2/image.png"
---i:find("sound.wav") -- "game_folder_3/sound.wav"
---i:find("model.obj") -- "game_folder_1/model.obj"
---```
---@return file_system value # The virtual file-system.
function file_system:new(search)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "file_system"
    i.locate = {}
    i.memory_list = {
        texture = {}
    }
    i.memory_data = {
        texture = {}
    }

    i:scan(search)

    return i
end

---Scan every directory in the asset's search table, to update the asset look-up table.
function file_system:scan(search)
    -- get the info path (i.e. path: "main_folder").
    local _, path = quiver.general.get_info()

    -- for each search path in the search table...
    for _, search_path in ipairs(search) do
        -- scan the path recursively.
        local list = quiver.file.scan_path(search_path, nil, true)
        -- make the full path (main_folder/game_folder_1).
        local wipe = path .. "/" .. search_path

        for _, search_file in ipairs(list) do
            -- strip "main_folder/game_folder_1/video/image.png" to "video/image.png".
            local entry = string.sub(search_file, #wipe + 2, -1)
            local value = string.sub(search_file, #path + 2, -1)

            -- set entry. (i.e. "video/image.png" = "main_folder/game_folder_1/video/image.png").
            self.locate[entry] = value
        end
    end
end

---Find an asset by name, to get the full path of the asset.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return string full_path # The "full" path to the asset.
function file_system:find(faux_path)
    return self.locate[faux_path]
end

---Re-load every asset in memory.
function file_system:load()
    for path, _ in pairs(self.memory_data.texture) do
        self:set_texture(path)
    end
end

---Set a texture asset into the file-system model resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return texture   asset  # The asset.
function file_system:set_texture(faux_path)
    -- if asset was already in memory...
    if self.memory_data.texture[faux_path] then
        -- remove from the book-keeping memory table.
        table.remove_object(self.memory_list.texture, faux_path)

        -- remove from the data-keeping memory table.
        self.memory_data.texture[faux_path] = nil

        collectgarbage("collect")
    end

    -- locate the asset.
    local asset = self.locate[faux_path]

    -- create the asset.
    asset = quiver.texture.new(asset)

    -- insert into the book-keeping memory table.
    table.insert(self.memory_list.texture, faux_path)

    -- insert into the data-keeping memory table.
    self.memory_data.texture[faux_path] = asset

    return asset
end

---Get a texture asset from the file-system model resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return texture   asset  # The asset.
function file_system:get_texture(faux_path)
    return self.memory_data.texture[faux_path]
end

--[[----------------------------------------------------------------]]

---@enum action_device
ACTION_DEVICE = {
    BOARD = 1,
    MOUSE = 2,
    PAD = 3
}

---@class action_button
---@field device action_device
---@field button number
action_button = {
    __meta = {}
}

---Create a new action button.
---@param device action_device #
function action_button:new(device, button)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "action_button"
    i.device = device
    i.button = button

    return i
end

function action_button:up()
    if self.device == ACTION_DEVICE.BOARD then return quiver.input.board.get_up(self.button) end
    if self.device == ACTION_DEVICE.MOUSE then return quiver.input.mouse.get_up(self.button) end
    if self.device == ACTION_DEVICE.PAD then return quiver.input.pad.get_up(0.0, self.button) end

    error(string.format("action_button::up(): Unknown device \"%f\".", self.device))
end

function action_button:down()
    if self.device == ACTION_DEVICE.BOARD then return quiver.input.board.get_down(self.button) end
    if self.device == ACTION_DEVICE.MOUSE then return quiver.input.mouse.get_down(self.button) end
    if self.device == ACTION_DEVICE.PAD then return quiver.input.pad.get_down(0.0, self.button) end

    error(string.format("action_button::down(): Unknown device \"%f\".", self.device))
end

function action_button:press()
    if self.device == ACTION_DEVICE.BOARD then return quiver.input.board.get_press(self.button) end
    if self.device == ACTION_DEVICE.MOUSE then return quiver.input.mouse.get_press(self.button) end
    if self.device == ACTION_DEVICE.PAD then return quiver.input.pad.get_press(0.0, self.button) end

    error(string.format("action_button::press(): Unknown device \"%f\".", self.device))
end

function action_button:release()
    if self.device == ACTION_DEVICE.BOARD then return quiver.input.board.get_release(self.button) end
    if self.device == ACTION_DEVICE.MOUSE then return quiver.input.mouse.get_release(self.button) end
    if self.device == ACTION_DEVICE.PAD then return quiver.input.pad.get_release(0.0, self.button) end

    error(string.format("action_button::release(): Unknown device \"%f\".", self.device))
end

---@class action
---@field list table
action = {
    __meta = {}
}

---Create a new action.
---@param  button_list table # A table array of every action button to be bound to this action.
---@return action      value # The action.
function action:new(button_list)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "action"
    i.list = button_list

    return i
end

function action:up()
    -- iterate over every button in our button list.
    for i, button in ipairs(self.list) do
        if button:up() then
            return true, i
        end
    end

    return false, nil
end

function action:down()
    -- iterate over every button in our button list.
    for i, button in ipairs(self.list) do
        if button:down() then
            return true, i
        end
    end

    return false, nil
end

function action:press()
    -- iterate over every button in our button list.
    for i, button in ipairs(self.list) do
        if button:press() then
            return true, i
        end
    end

    return false, nil
end

function action:release()
    -- iterate over every button in our button list.
    for i, button in ipairs(self.list) do
        if button:release() then
            return true, i
        end
    end

    return false, nil
end

--[[----------------------------------------------------------------]]

---@enum gizmo_flag
GIZMO_FLAG = {
    IGNORE_BOARD = 0x00000001,
    IGNORE_MOUSE = 0x00000010,
}

---@enum window_device
WINDOW_DEVICE = {
    BOARD = 0,
    MOUSE = 1,
    PAD = 2,
}

local WINDOW_ACTION_ABOVE = action:new(
    {
        action_button:new(ACTION_DEVICE.BOARD, INPUT_BOARD.KEY_W),
        action_button:new(ACTION_DEVICE.PAD, INPUT_PAD.GAMEPAD_BUTTON_LEFT_FACE_UP),
    }
)
local WINDOW_ACTION_BELOW = action:new(
    {
        action_button:new(ACTION_DEVICE.BOARD, INPUT_BOARD.KEY_S),
        action_button:new(ACTION_DEVICE.PAD, INPUT_PAD.GAMEPAD_BUTTON_LEFT_FACE_DOWN),
    }
)
local WINDOW_ACTION_FOCUS = action:new(
    {
        action_button:new(ACTION_DEVICE.BOARD, INPUT_BOARD.KEY_SPACE),
        action_button:new(ACTION_DEVICE.MOUSE, INPUT_MOUSE.MOUSE_BUTTON_LEFT),
        action_button:new(ACTION_DEVICE.PAD, INPUT_PAD.GAMEPAD_BUTTON_RIGHT_FACE_DOWN),
    }
)

local WINDOW_SHIFT_A = vector_2:new(6.0, 4.0)
local WINDOW_SHIFT_B = vector_2:new(8.0, 6.0)
local WINDOW_DOT = vector_2:new(4.0, 4.0)

---@class window
---@field index  number
---@field count  number
---@field focus  number | nil
---@field device window_device
window = {
    __meta = {}
}

function window:new()
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "window"
    i.index = 0.0
    i.count = 0.0
    i.focus = nil
    i.glyph = nil
    i.which = 0.0
    i.device = WINDOW_DEVICE.MOUSE

    return i
end

---Begin the window.
function window:begin()
    self.count = 0.0
    self.glyph = nil
end

---Close the window.
---@param pass_logger? logger # OPTIONAL: Logger. If its state is set on, the window will not enable/disable the mouse.
function window:close(pass_logger)
    local above = WINDOW_ACTION_ABOVE:press()
    local below = WINDOW_ACTION_BELOW:press()

    if above then
        self.index =
            self.index - 1.0
        self.which = -1.0
    end

    if below then
        self.index =
            self.index + 1.0
        self.which = 1.0
    end

    local check = quiver.input.board.get_key_code_queue()

    if check > 0.0 then
        if not quiver.input.mouse.get_hidden() then
            quiver.input.mouse.set_active(false)
        end

        self.device = WINDOW_DEVICE.BOARD
    end

    local check = quiver.input.mouse.get_press(INPUT_MOUSE.MOUSE_BUTTON_LEFT)

    if check then
        if quiver.input.mouse.get_hidden() then
            quiver.input.mouse.set_active(true)
        end

        self.device = WINDOW_DEVICE.MOUSE
    end

    local check = quiver.input.pad.get_queue()

    if check > 0.0 then
        if not quiver.input.mouse.get_hidden() then
            quiver.input.mouse.set_active(false)
        end

        self.device = WINDOW_DEVICE.PAD
    end

    if self.glyph then
        self:glyph()
    end

    -- roll over the value in case it is not hovering over any valid gizmo.
    self.index = math.roll_over(0.0, self.count - 1.0, self.index)
end

function window:glyph_select()
    local x, y = quiver.window.get_shape()
    local point = vector_2:old(8.0, y - 40.0)
    local label = "[W/S] Navigation | [SPACE] Select"

    draw_box_2_border(box_2:old(point.x, point.y, x - 16.0, 32.0), false)

    if self.device == WINDOW_DEVICE.MOUSE then
        label = "[MOUSE1] Select"
    elseif self.device == WINDOW_DEVICE.PAD then
        label = "[D-PAD UP/D-PAD DOWN] Navigation | [X] Select"
    end

    LOGGER_FONT:draw(label, point + WINDOW_SHIFT_A, LOGGER_FONT_SCALE, LOGGER_FONT_SPACE, color:white())
end

function window:glyph_adjust()
    local x, y = quiver.window.get_shape()
    local point = vector_2:old(8.0, y - 40.0)
    local label = "[W/S] Navigation | [A/D] Adjust"

    draw_box_2_border(box_2:old(point.x, point.y, x - 16.0, 32.0), false)

    if self.device == WINDOW_DEVICE.PAD then
        label = "[D-PAD UP/D-PAD DOWN] Navigation | [D-PAD LEFT/D-PAD RIGHT] Adjust"
    end

    LOGGER_FONT:draw(label, point + WINDOW_SHIFT_A, LOGGER_FONT_SCALE, LOGGER_FONT_SPACE, color:white())
end

---[For Internal Use] Draw a border.
---@param shape box_2    # The shape of the border.
---@param hover boolean  # Mouse focus. Whether or not the mouse cursor is over this gizmo.
---@param index boolean  # Board focus. Whether or not the board cursor is over this gizmo.
---@param focus boolean  # Gizmo focus. Whether or not the window focus is on this gizmo.
---@param text? string   # OPTIONAL: Text to draw.
---@param move? vector_2 # OPTIONAL: Text off-set.
---@param glyph? function # OPTIONAL: Glyph call-back. The window will draw this on window:close().
function window:border(shape, hover, index, focus, text, move, glyph)
    local shift = focus and vector_2:old(shape.x + WINDOW_SHIFT_B.x, shape.y + WINDOW_SHIFT_B.y) or
        vector_2:old(shape.x + WINDOW_SHIFT_A.x, shape.y + WINDOW_SHIFT_A.y)

    if move then
        shift = shift + move
    end

    draw_box_2_border(shape, focus)

    if not self.focus then
        if index or hover then
            draw_box_2_dot(shape:old(shape.x + WINDOW_DOT.x, shape.y + WINDOW_DOT.y, shape.width - WINDOW_DOT.x * 2.0,
                shape.height - WINDOW_DOT.y * 2.0))
        end
    else
        if index or focus then
            draw_box_2_dot(shape:old(shape.x + WINDOW_DOT.x, shape.y + WINDOW_DOT.y, shape.width - WINDOW_DOT.x * 2.0,
                shape.height - WINDOW_DOT.y * 2.0))
        end
    end

    if text then
        LOGGER_FONT:draw(text, shift + vector_2:old(1.0, 1.0), LOGGER_FONT_SCALE, LOGGER_FONT_SPACE,
            color:old(127.0, 127.0, 127.0, 255.0))
        LOGGER_FONT:draw(text, shift, LOGGER_FONT_SCALE, LOGGER_FONT_SPACE, color:old(255.0, 255.0, 255.0, 255.0))
    end

    if hover or index or focus then
        if glyph then
            self.glyph = glyph
        else
            self.glyph = self.glyph_select
        end
    end
end

function window:state(shape, flag, action)
    local mouse_x, mouse_y = quiver.input.mouse.get_point()
    local mouse = vector_2:old(mouse_x, mouse_y)
    local hover = self.device == WINDOW_DEVICE.MOUSE and collision.point_box(mouse, shape)
    local index = (self.device == WINDOW_DEVICE.BOARD or self.device == WINDOW_DEVICE.PAD) and self.index == self.count
    local click = false
    local which = nil

    if flag then
        if not (bit.band(flag, GIZMO_FLAG.IGNORE_BOARD) == 0) then
            if index then
                index = false
                self.index = self.index + self.which
            end
        end

        if not (bit.band(flag, GIZMO_FLAG.IGNORE_MOUSE) == 0) then
            if hover then
                hover = false
            end
        end
    end

    if hover or index then
        local hover_click = WINDOW_ACTION_FOCUS:press()

        if action then
            hover_click = action:press()
        end

        if hover_click then
            self.focus = self.count
        end
    end

    local focus = self.focus == self.count

    if focus then
        local focus_click = WINDOW_ACTION_FOCUS:release()

        if action then
            focus_click, which = action:release()
        end

        if focus_click then
            click = true
            self.focus = nil
        end
    end

    self.count = self.count + 1

    return hover, index, focus, click, which
end

function window:button(shape, text, flag)
    local hover, index, focus, click = self:state(shape, flag)

    self:border(shape, hover, index, focus, text)

    return click
end

function window:toggle(shape, text, value, flag)
    local hover, index, focus, click = self:state(shape, flag)

    self:border(shape, hover, index, focus, text, vector_2:old(shape.width, 0.0))

    if click then
        value = not value
    end

    if value then
        draw_box_2_border(box_2:old(shape.x + 6.0, shape.y + 6.0, shape.width - 12.0, shape.height - 12.0), false,
            color:white())
    end

    return value, click
end

function window:slider(shape, text, value, min, max, step, flag)
    local slider_action = action:new(
        {
            action_button:new(ACTION_DEVICE.BOARD, INPUT_BOARD.KEY_A),
            action_button:new(ACTION_DEVICE.BOARD, INPUT_BOARD.KEY_D),
            action_button:new(ACTION_DEVICE.MOUSE, INPUT_MOUSE.MOUSE_BUTTON_LEFT),
            action_button:new(ACTION_DEVICE.PAD, INPUT_PAD.GAMEPAD_BUTTON_LEFT_FACE_LEFT),
            action_button:new(ACTION_DEVICE.PAD, INPUT_PAD.GAMEPAD_BUTTON_LEFT_FACE_RIGHT),
        }
    )
    local hover, index, focus, click, which = self:state(shape, flag, slider_action)

    self:border(shape, hover, index, focus, text, vector_2:old(shape.width, 0.0), self.glyph_adjust)

    if self.device == WINDOW_DEVICE.MOUSE then
        if focus then
            local mouse_x = quiver.input.mouse.get_point()
            local result = math.percentage_from_value(shape.x + 6.0, shape.x + 6.0 + shape.width - 12.0, mouse_x)
            result = math.clamp(0.0, 1.0, result)
            result = math.value_from_percentage(min, max, result)
            result = math.snap(step, result)
            value = result
        end
    end

    if which then
        which = slider_action.list[which]

        if which.button == INPUT_BOARD.KEY_A or which == INPUT_PAD.GAMEPAD_BUTTON_LEFT_FACE_LEFT then
            value = value - step
        end

        if which.button == INPUT_BOARD.KEY_D or which == INPUT_PAD.GAMEPAD_BUTTON_LEFT_FACE_RIGHT then
            value = value + step
        end

        value = math.clamp(min, max, value)
    end

    local percentage = math.percentage_from_value(min, max, value)

    if percentage > 0.0 then
        draw_box_2_border(
            box_2:old(shape.x + 6.0, shape.y + 6.0, (shape.width - 12.0) * percentage, shape.height - 12.0),
            false,
            color:white())
    end

    return value, click
end

function window:switch(shape, text, value, pool, flag)
    local value_a = nil
    local value_b = nil
    local label = "N/A"

    for pool_value, pool_label in pairs(pool) do
        if value == pool_value then
            label = pool_label

            if pool[pool_value - 1] then
                value_a = pool_value - 1
            end

            if pool[pool_value + 1] then
                value_b = pool_value + 1
            end
        end
    end

    -- draw l-side button.
    local shape_a = box_2:old(shape.x, shape.y, 32.0, shape.height)

    local hover, index, focus, click = self:state(shape_a, flag + GIZMO_FLAG.IGNORE_BOARD)

    self:border(shape_a, hover, index, focus)

    quiver.draw_2d.draw_triangle(
        vector_2:old(shape_a.x + shape_a.width - 8.0, shape_a.y + 8.0),
        vector_2:old(shape_a.x + 16.0 - (shape_a.width - 16.0) * 0.5,
            shape_a.y + 8.0 + (shape_a.height - 16.0) * 0.5),
        vector_2:old(shape_a.x + shape_a.width - 8.0, shape_a.y + 8.0 + shape_a.height - 16.0),
        color:white())

    if click then
        if value_a then
            value = value_a
        end
    end

    -- draw center text.
    local switch_action = action:new(
        {
            action_button:new(ACTION_DEVICE.BOARD, INPUT_BOARD.KEY_A),
            action_button:new(ACTION_DEVICE.BOARD, INPUT_BOARD.KEY_D),
        }
    )

    local point = vector_2:old(shape.x + 32.0 + 4.0, shape.y)

    local shape_a = box_2:old(point.x, point.y, shape.width, shape.height)

    local hover, index, focus, click, which = self:state(shape_a, flag + GIZMO_FLAG.IGNORE_MOUSE, switch_action)

    if which then
        which = switch_action.list[which]

        if which.button == INPUT_BOARD.KEY_A then
            if value_a then
                value = value_a
            end
        else
            if value_b then
                value = value_b
            end
        end
    end

    self:border(shape_a, hover, index, focus, nil, nil, self.glyph_adjust)

    LOGGER_FONT:draw(label, point + vector_2:old(8.0, 4.0), LOGGER_FONT_SCALE, LOGGER_FONT_SPACE, color:white())

    -- draw r-side button.
    local shape_a = box_2:old(shape.x + 32.0 + 8.0 + shape.width, shape.y, 32.0, shape.height)

    local hover, index, focus, click = self:state(shape_a, flag + GIZMO_FLAG.IGNORE_BOARD)

    self:border(shape_a, hover, index, focus, text, vector_2:old(shape_a.width + 0.0, 0.0))

    quiver.draw_2d.draw_triangle(
        vector_2:old(shape_a.x + 8.0, shape_a.y + 8.0),
        vector_2:old(shape_a.x + 8.0, shape_a.y + 8.0 + shape_a.height - 16.0),
        vector_2:old(shape_a.x + 8.0 + shape_a.width - 16.0, shape_a.y + 8.0 + (shape_a.height - 16.0) * 0.5),
        color:white())

    if click then
        if value_b then
            value = value_b
        end
    end

    return value
end

function window:button(shape, text, flag)
    local hover, index, focus, click = self:state(shape, flag)

    self:border(shape, hover, index, focus, text)

    return click
end

--[[----------------------------------------------------------------]]

local LOGGER_LINE_COLOR_HISTORY = color:new(127.0, 127.0, 127.0, 255.0)
local LOGGER_LINE_COLOR_MESSAGE = color:new(255.0, 255.0, 255.0, 255.0)
local LOGGER_LINE_COLOR_FAILURE = color:new(255.0, 0.0, 0.0, 255.0)
local LOGGER_LINE_COUNT = 4.0
local LOGGER_LINE_DELAY = 4.0
local LOGGER_LINE_LABEL_TIME = false

---@class logger_line
logger_line = {
    __meta = {}
}

function logger_line:new(label, color)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "logger_line"
    i.label = label
    i.color = color
    i.time = quiver.general.get_time()

    return i
end

--[[----------------------------------------------------------------]]

---@class logger_command
logger_command = {
    __meta = {}
}

function logger_command:new(info, call)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type = "logger_command"
    i.info = info
    i.call = call

    return i
end

--[[----------------------------------------------------------------]]

LOGGER_FONT_SCALE        = 24.0
LOGGER_FONT_SPACE        = 1.0
--LOGGER_FONT              = quiver.font.new_default()
LOGGER_FONT              = quiver.font.new("font.ttf", LOGGER_FONT_SCALE)
LOGGER_COLOR_MAIN        = color:new(0.0, 0.0, 0.0, 127.0)
LOGGER_COLOR_SIDE        = color:new(0.0, 0.0, 0.0, 127.0)
LOGGER_COLOR__type       = color:new(0.0, 0.0, 0.0, 127.0)
LOGGER_SHAPE             = vector_2:new(1.0, 0.5)
LOGGER_LINE_CAP          = 64.0
LOGGER_KEY_TOGGLE        = INPUT_BOARD.KEY_F2
LOGGER_KEY_DELETE        = INPUT_BOARD.KEY_BACKSPACE
LOGGER_KEY_SUGGEST       = INPUT_BOARD.KEY_TAB
LOGGER_KEY_HISTORY_ABOVE = INPUT_BOARD.KEY_UP
LOGGER_KEY_HISTORY_BELOW = INPUT_BOARD.KEY_DOWN
LOGGER_KEY_PRINT         = INPUT_BOARD.KEY_ENTER

---@class logger
---@field worker  string
---@field buffer  table
---@field suggest table
---@field history table
---@field command table
---@field active  boolean
logger                   = {
    __meta = {}
}

---Create a new logger.
---@return logger value # The logger.
function logger:new()
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[----------------------------------------------------------------]]

    i.__type  = "logger"
    i.worker  = ""
    i.buffer  = {}
    i.suggest = {}
    i.history = {}
    i.command = {}
    i.active  = false
    i.scroll  = 0.0

    --[[----------------------------------------------------------------]]

    i.command["find"] = logger_command:new("Find every logger command.", function(self, token)
        local find = token[2]

        if find then
            for name, command in pairs(self.command) do
                if string.start_with(name, find) then
                    self:print(name .. ": " .. command.info)
                end
            end
        else
            self:print("Usage: find {name: string}.", LOGGER_LINE_COLOR_FAILURE)
        end
    end
    )

    i.command["wipe"] = logger_command:new("Wipe every logger line.", function(self)
        self.buffer = {}
    end
    )

    return i
end

---Clear and build the suggest buffer.
local function logger_suggest_build(self)
    -- clear the buffer.
    self.suggest = {}

    -- for each command in the command list...
    for name, _ in pairs(self.command) do
        -- if the command name does start with the worker buffer string...
        if string.start_with(name, self.worker) then
            -- add to suggest buffer.
            table.insert(self.suggest, name)
        end
    end
end

---Input handling: handle deletion.
local function logger_handle_delete(self)
    -- pop the last character of the working buffer.
    self.worker = string.sub(self.worker, 0, #self.worker - 1)

    -- re-build the suggestion buffer.
    logger_suggest_build(self)
end

---Input handling: handle suggest navigation.
local function logger_handle_suggest(self)
    -- get the length of the suggest buffer.
    local count = #self.suggest

    -- if the suggest buffer is bigger than 0.0...
    if count > 0.0 then
        local empty = true

        -- for each line in the suggest buffer...
        for i, name in pairs(self.suggest) do
            -- the current working buffer is the same as this command's name.
            if self.worker == name then
                -- we can index one command above...
                if i + 1 <= count then
                    -- set the working buffer text to the command above, don't do anything else.
                    self.worker = self.suggest[i + 1]
                    empty = false
                end

                break
            end
        end

        -- no text equal to the worker working buffer found or we are indexing into nil.
        if empty then
            -- set worker string.
            self.worker = self.suggest[1]
        end
    end
end

---Input handling: handle history navigation.
local function logger_handle_history(self, direction)
    -- get the length of the history buffer.
    local count = #self.history

    -- if the history buffer is bigger than 0.0...
    if count > 0.0 then
        local empty = true
        local index = 1.0

        -- for each line in the history buffer...
        for i, name in pairs(self.history) do
            if self.worker == name then
                -- get the direction of the history scroll.
                local which = direction and -1.0 or 1.0

                -- if {i} + {which} is within the correct index range...
                if i + which >= 1.0 and i + which <= count then
                    -- set index, don't do anything else.
                    index = i + which
                    empty = false
                end

                break
            end
        end

        -- no text equal to the working buffer found or we are indexing into nil.
        if empty then
            if direction then
                -- going up, roll over to {count}.
                index = count
            else
                -- going down, roll over to {1.0}.
                index = 1.0
            end
        end

        -- set working buffer.
        self.worker = self.history[index]
    end
end

---Input handling: handle printing a line to the logger from the working buffer.
local function logger_handle_print(self)
    -- if the working buffer isn't empty...
    if not (self.buffer == "") then
        -- tokenize the string. use the first match as the command name, everything else as an argument.
        local token = self.worker:tokenize("%S+")

        -- find the command by name.
        local command = self.command[token[1]]

        -- print the working buffer.
        self:print(self.worker, LOGGER_LINE_COLOR_HISTORY)

        -- if there is a valid command...
        if command then
            -- call it, pass the tokenization table.
            command.call(self, token)
        else
            -- print error message.
            self:print("Unknown command.", LOGGER_LINE_COLOR_FAILURE)
        end

        -- insert as part of the history table, clear working buffer, clear suggestion list.
        table.insert(self.history, self.worker)
        self.worker  = ""
        self.suggest = {}
    end
end

---Input handling: handle the press of a key.
local function logger_handle_press(self)
    -- get latest unicode key.
    local uni = quiver.input.board.get_uni_code_queue()

    -- while the queue isn't empty...
    while not (uni == 0) do
        -- attach a character to the end of the working buffer string, re-build suggest buffer.
        self.worker = self.worker .. string.char(uni)
        logger_suggest_build(self)

        uni = quiver.input.board.get_uni_code_queue()
    end
end

---Draw the main logger layout. Only drawn when logger is set.
---@param self    logger # The logger.
---@param window? window # OPTIONAL: Frame for rendering every possible command suggestion.
local function logger_draw_main(self, window)
    local count = #self.buffer

    -- get mouse wheel movement, scroll logger buffer.
    local _, y = quiver.input.mouse.get_wheel()
    self.scroll = math.max(0.0, self.scroll + y)

    -- get window shape, calculate each box's shape.
    local x, y = quiver.window.get_shape()
    x = x * LOGGER_SHAPE.x
    y = y * LOGGER_SHAPE.y
    local box_main = box_2:old(0.0, 0.0, x, y)
    local box_side = box_2:old(8.0, 8.0, x - 16.0, y - 28.0 - LOGGER_FONT_SCALE)
    local box__type = box_2:old(8.0, y - 12.0 - LOGGER_FONT_SCALE, x - 16.0, LOGGER_FONT_SCALE + 4.0)

    -- input handling block.
    if quiver.input.board.get_press(LOGGER_KEY_DELETE) or quiver.input.board.get_press_repeat(LOGGER_KEY_DELETE) then
        logger_handle_delete(self)
    elseif quiver.input.board.get_press(LOGGER_KEY_SUGGEST) or quiver.input.board.get_press_repeat(LOGGER_KEY_SUGGEST) then
        logger_handle_suggest(self)
    elseif quiver.input.board.get_press(LOGGER_KEY_HISTORY_ABOVE) or quiver.input.board.get_press_repeat(LOGGER_KEY_HISTORY_ABOVE) then
        logger_handle_history(self, 1.0)
    elseif quiver.input.board.get_press(LOGGER_KEY_HISTORY_BELOW) or quiver.input.board.get_press_repeat(LOGGER_KEY_HISTORY_BELOW) then
        logger_handle_history(self, 0.0)
    elseif quiver.input.board.get_press(LOGGER_KEY_PRINT) then
        logger_handle_print(self)
    else
        logger_handle_press(self)
    end

    -- draw box.
    draw_box_2_border(box_main, false)
    draw_box_2_border(box_side, true)
    draw_box_2_border(box__type, true)

    -- draw every line in the logger buffer inside of a GL scissor test.
    quiver.draw.begin_scissor(function()
        for i, line in pairs(self.buffer) do
            i = (count - i) - self.scroll

            local text_point = vector_2:old(12.0,
                (box_side.y + box_side.height - LOGGER_FONT_SCALE - 4.0) - (i * LOGGER_FONT_SCALE))
            local text_shape = box_2:old(text_point.x, text_point.y, 16.0, LOGGER_FONT_SCALE)

            if collision.box_box(text_shape, box_side) then
                LOGGER_FONT:draw(line.label, text_point, LOGGER_FONT_SCALE,
                    LOGGER_FONT_SPACE, line.color)
            end
        end
    end, box_side)

    -- draw working buffer.
    LOGGER_FONT:draw(self.worker, vector_2:old(12.0, box__type.y), LOGGER_FONT_SCALE, LOGGER_FONT_SPACE, color:white())

    -- if window is not nil...
    if window then
        -- if the working buffer isn't empty...
        if not (self.worker == "") then
            -- for each suggestion in the suggest buffer...
            for i, name in pairs(self.suggest) do
                -- start from zero.
                i = i - 1

                -- measure text.
                local size_x, size_y = LOGGER_FONT:measure_text(name, LOGGER_FONT_SCALE, LOGGER_FONT_SPACE)
                size_x = size_x + WINDOW_SHIFT_A.x * 2.0
                size_y = size_y + WINDOW_SHIFT_A.y * 2.0

                -- draw button, if button is set off, replace working buffer with the suggestion instead.
                if window:button(box_2:old(box__type.x, box__type.y + box__type.height + (i * size_y), size_x, size_y), name, GIZMO_FLAG.IGNORE_BOARD) then
                    self.worker = name
                    self.suggest = {}
                end
            end
        end
    end
end

---Draw a small portion of the most recently sent content in the logger buffer. Only drawn when logger is not set.
local function logger_draw_side(self)
    -- get the length of the buffer worker.
    local count = #self.buffer

    -- draw the latest logger buffer, iterating through the buffer in reverse.
    for i = 1, LOGGER_LINE_COUNT do
        local line = self.buffer[count + 1 - i]

        -- line isn't nil...
        if line then
            -- line is within the time threshold...
            if quiver.general.get_time() < line.time + LOGGER_LINE_DELAY then
                -- start from 0.
                i = i - 1

                local text_point_a = vector_2:old(13.0, 13.0 + (i * LOGGER_FONT_SCALE))
                local text_point_b = vector_2:old(12.0, 12.0 + (i * LOGGER_FONT_SCALE))
                local label = line.label

                -- line with time-stamp is set, add time-stamp to beginning.
                if LOGGER_LINE_LABEL_TIME then
                    label = string.format("(%.2f) %s", line.time, line.label)
                end

                -- draw back-drop.
                LOGGER_FONT:draw(label, text_point_a, LOGGER_FONT_SCALE, LOGGER_FONT_SPACE, line.color * 0.5)
                -- draw line.
                LOGGER_FONT:draw(label, text_point_b, LOGGER_FONT_SCALE, LOGGER_FONT_SPACE, line.color)
            end
        end
    end
end

---Draw the logger.
---@param window? window # OPTIONAL: Frame for rendering every possible command suggestion.
function logger:draw(window)
    -- toggle key was hit, toggle active state.
    if quiver.input.board.get_press(LOGGER_KEY_TOGGLE) then
        self.active = not self.active
    end

    if self.active then
        -- logger active state is set, draw main layout.
        logger_draw_main(self, window)
    else
        -- logger active state is not set, draw side layout.
        logger_draw_side(self)
    end
end

---Print a new line to the logger.
---@param line_label  string # Line label.
---@param line_color? color  # OPTIONAL: Line color.
function logger:print(line_label, line_color)
    -- if line color is nil, use default.
    if not line_color then
        line_color = LOGGER_LINE_COLOR_MESSAGE
    end

    table.insert(self.buffer, logger_line:new(tostring(line_label), line_color))

    if #self.buffer > LOGGER_LINE_CAP then
        table.remove(self.buffer, 1)
    end
end

--[[----------------------------------------------------------------]]

---Reset every table pool index to 1. This should usually be done before beginning to draw, or when running a simulation tick.
function table_pool:clear()
    vector_2_pool:begin()
    vector_3_pool:begin()
    vector_4_pool:begin()
    color_pool:begin()
    box_2_pool:begin()
    box_3_pool:begin()
    camera_2d_pool:begin()
    camera_3d_pool:begin()
end
