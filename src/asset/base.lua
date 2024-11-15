---@class vector_2
---@field x number
---@field y number
vector_2 = {
    _type = "vector_2",
    x = 0.0,
    y = 0.0,
}

function vector_2:new(x, y)
    local i = {}
    setmetatable(i, {
        __index = self,
        __add = function(a, b) return vector_2:new(a.x + b.x, a.y + b.y) end,
        __sub = function(a, b) return vector_2:new(a.x - b.x, a.y - b.y) end,
        __mul = function(a, b) return vector_2:new(a.x * b.x, a.y * b.y) end,
        __div = function(a, b) return vector_2:new(a.x / b.x, a.y / b.y) end,
        __tostring = function(a) return "{ x:"..tostring(a.x).." y:"..tostring(a.y).." }"..tostring(a.z).." }" end
    })
    i.x = x
    i.y = y
    return i
end

function vector_2:x()
    return vector_2:new(1.0, 0.0)
end

function vector_2:y()
    return vector_2:new(0.0, 1.0)
end

function vector_2:one()
    return vector_2:new(1.0, 1.0)
end

function vector_2:zero()
    return vector_2:new(0.0, 0.0)
end

---@class vector_3
---@field x number
---@field y number
---@field z number
vector_3 = {
    _type = "vector_3",
    x = 0.0,
    y = 0.0,
    z = 0.0,
}

function vector_3:new(x, y, z)
    local i = {}
    setmetatable(i, {
        __index = self,
        __add = function(a, b) return vector_3:new(a.x + b.x, a.y + b.y, a.z + b.z) end,
        __sub = function(a, b) return vector_3:new(a.x - b.x, a.y - b.y, a.z - b.z) end,
        __mul = function(a, b) return vector_3:new(a.x * b.x, a.y * b.y, a.z * b.z) end,
        __div = function(a, b) return vector_3:new(a.x / b.x, a.y / b.y, a.z / b.z) end,
        __tostring = function(a) return "{ x:"..tostring(a.x).." y:"..tostring(a.y).." z:"..tostring(a.z).." }" end
    })
    i.x = x
    i.y = y
    i.z = z
    return i
end

function vector_3:x()
    return vector_3:new(1.0, 0.0, 0.0)
end

function vector_3:y()
    return vector_3:new(0.0, 1.0, 0.0)
end

function vector_3:z()
    return vector_3:new(0.0, 0.0, 1.0)
end

function vector_3:one()
    return vector_3:new(1.0, 1.0, 1.0)
end

function vector_3:zero()
    return vector_3:new(0.0, 0.0, 0.0)
end

---@class camera_2d
---@field shift vector_2
---@field focus vector_2
---@field angle number
---@field zoom  number
camera_2d = {
    _type = "camera_2d",
    shift = vector_2:zero(),
    focus = vector_2:zero(),
    angle = 0.0,
    zoom  = 0.0,
}

function camera_2d:new(shift, focus, angle, zoom)
    local i = {}
    setmetatable(i, {
        __index = self
    })
    i.shift = shift
    i.focus = focus
    i.angle = angle
    i.zoom = zoom
    return i
end

---@class camera_3d
---@field point vector_3
---@field focus vector_3
---@field angle vector_3
---@field zoom  number
camera_3d = {
    _type = "camera_3d",
    point = vector_3:zero(),
    focus = vector_3:zero(),
    angle = vector_3:zero(),
    zoom  = 0.0,
}

function camera_3d:new(point, focus, angle, zoom)
    local i = {}
    setmetatable(i, {
        __index = self
    })
    i.point = point
    i.focus = focus
    i.angle = angle
    i.zoom = zoom
    return i
end

---@class color
---@field r number
---@field g number
---@field b number
---@field a number
color = {
    r = 0.0,
    g = 0.0,
    b = 0.0,
    a = 0.0,
}

function color:new(r, g, b, a)
    local i = {}
    setmetatable(i, {
        __index = self
    })
    i.r = r
    i.g = g
    i.b = b
    i.a = a
    return i
end

function color:white()
    return color:new(1.0, 1.0, 1.0, 1.0)
end

function color:black()
    return color:new(0.0, 0.0, 0.0, 1.0)
end

---@class box_2
---@field min vector_2
---@field max vector_2
box_2 = {
    _type = "box_2",
    min = vector_2:zero(),
    max = vector_2:zero(),
}

function box_2:new(min, max)
    local i = {}
    setmetatable(i, {
        __index = self
    })
    i.min = min
    i.max = max
    return i
end

---@class box_3
---@field min vector_3
---@field max vector_3
box_3 = {
    _type = "box_3",
    min = vector_3:zero(),
    max = vector_3:zero(),
}

function box_3:new(min, max)
    local i = {}
    setmetatable(i, {
        __index = self
    })
    i.min = min
    i.max = max
    return i
end

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

