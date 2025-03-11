--[[
-- Copyright (c) 2025 sockentrocken
--
-- Redistribution and use in source and binary forms, with or without
-- modification, are permitted provided that the following conditions are met:
--
-- 1. Redistributions of source code must retain the above copyright notice,
-- this list of conditions and the following disclaimer.
--
-- 2. Redistributions in binary form must reproduce the above copyright notice,
-- this list of conditions and the following disclaimer in the documentation
-- and/or other materials provided with the distribution.
--
-- Subject to the terms and conditions of this license, each copyright holder
-- and contributor hereby grants to those receiving rights under this license
-- a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
-- (except for failure to satisfy the conditions of this license) patent license
-- to make, have made, use, offer to sell, sell, import, and otherwise transfer
-- this software, where such license applies only to those patent claims, already
-- acquired or hereafter acquired, licensable by such copyright holder or
-- contributor that are necessarily infringed by:
--
-- (a) their Contribution(s) (the licensed copyrights of copyright holders and
-- non-copyrightable additions of contributors, in source or binary form) alone;
-- or
--
-- (b) combination of their Contribution(s) with the work of authorship to which
-- such Contribution(s) was added by such copyright holder or contributor, if,
-- at the time the Contribution is added, such addition causes such combination
-- to be necessarily infringed. The patent license shall not apply to any other
-- combinations which include the Contribution.
--
-- Except as expressly stated above, no rights or licenses from any copyright
-- holder or contributor is granted under this license, whether expressly, by
-- implication, estoppel or otherwise.
--
-- DISCLAIMER
--
-- THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
-- AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
-- IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
-- DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
-- FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
-- DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
-- SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
-- CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
-- OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
-- OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
--]]

---@enum rigid_body_kind
RIGID_BODY_KIND = {
    FIXED                    = 0.0,
    DYNAMIC                  = 1.0,
    KINEMATIC_POSITION_BASED = 2.0,
    KINEMATIC_VELOCITY_BASED = 3.0,
}

---@enum trace_log_level
TRACE_LOG_LEVEL = {
    ALL     = 0,
    TRACE   = 1,
    DEBUG   = 2,
    INFO    = 3,
    WARNING = 4,
    ERROR   = 5,
    FATAL   = 6,
    NONE    = 7
}

---@enum input_device
INPUT_DEVICE    = {
    BOARD = 0,
    MOUSE = 1,
    PAD   = 2,
}

---@enum input_board
INPUT_BOARD     = {
    NULL         = 0,
    APOSTROPHE   = 39,
    COMMA        = 44,
    MINUS        = 45,
    PERIOD       = 46,
    SLASH        = 47,
    ZERO         = 48,
    ONE          = 49,
    TWO          = 50,
    THREE        = 51,
    FOUR         = 52,
    FIVE         = 53,
    SIX          = 54,
    SEVEN        = 55,
    EIGHT        = 56,
    NINE         = 57,
    SEMICOLON    = 59,
    EQUAL        = 61,
    A            = 65,
    B            = 66,
    C            = 67,
    D            = 68,
    E            = 69,
    F            = 70,
    G            = 71,
    H            = 72,
    I            = 73,
    J            = 74,
    K            = 75,
    L            = 76,
    M            = 77,
    N            = 78,
    O            = 79,
    P            = 80,
    Q            = 81,
    R            = 82,
    S            = 83,
    T            = 84,
    U            = 85,
    V            = 86,
    W            = 87,
    X            = 88,
    Y            = 89,
    Z            = 90,
    L_BRACKET    = 91,
    BACKSLASH    = 92,
    R_BRACKET    = 93,
    GRAVE        = 96,
    SPACE        = 32,
    ESCAPE       = 256,
    RETURN       = 257,
    TAB          = 258,
    BACKSPACE    = 259,
    INSERT       = 260,
    DELETE       = 261,
    RIGHT        = 262,
    LEFT         = 263,
    DOWN         = 264,
    UP           = 265,
    PAGE_UP      = 266,
    PAGE_DOWN    = 267,
    HOME         = 268,
    END          = 269,
    CAPS_LOCK    = 280,
    SCROLL_LOCK  = 281,
    NUMBER_LOCK  = 282,
    PRINT_SCREEN = 283,
    PAUSE        = 284,
    F1           = 290,
    F2           = 291,
    F3           = 292,
    F4           = 293,
    F5           = 294,
    F6           = 295,
    F7           = 296,
    F8           = 297,
    F9           = 298,
    F10          = 299,
    F11          = 300,
    F12          = 301,
    L_SHIFT      = 340,
    L_CONTROL    = 341,
    L_ALTERNATE  = 342,
    L_SUPER      = 343,
    R_SHIFT      = 344,
    R_CONTROL    = 345,
    R_ALTERNATE  = 346,
    R_SUPER      = 347,
    KB_MENU      = 348,
    KP_0         = 320,
    KP_1         = 321,
    KP_2         = 322,
    KP_3         = 323,
    KP_4         = 324,
    KP_5         = 325,
    KP_6         = 326,
    KP_7         = 327,
    KP_8         = 328,
    KP_9         = 329,
    KP_DECIMAL   = 330,
    KP_DIVIDE    = 331,
    KP_MULTIPLY  = 332,
    KP_SUBTRACT  = 333,
    KP_ADD       = 334,
    KP_ENTER     = 335,
    KP_EQUAL     = 336,
    BACK         = 4,
    VOLUME_UP    = 24,
    VOLUME_DOWN  = 25,
    [0]          = "Unknown",
    [39]         = "'",
    [44]         = ",",
    [45]         = "-",
    [46]         = ".",
    [47]         = "/",
    [48]         = "0",
    [49]         = "1",
    [50]         = "2",
    [51]         = "3",
    [52]         = "4",
    [53]         = "5",
    [54]         = "6",
    [55]         = "7",
    [56]         = "8",
    [57]         = "9",
    [59]         = ";",
    [61]         = "=",
    [65]         = "A",
    [66]         = "B",
    [67]         = "C",
    [68]         = "D",
    [69]         = "E",
    [70]         = "F",
    [71]         = "G",
    [72]         = "H",
    [73]         = "I",
    [74]         = "J",
    [75]         = "K",
    [76]         = "L",
    [77]         = "M",
    [78]         = "N",
    [79]         = "O",
    [80]         = "P",
    [81]         = "Q",
    [82]         = "R",
    [83]         = "S",
    [84]         = "T",
    [85]         = "U",
    [86]         = "V",
    [87]         = "W",
    [88]         = "X",
    [89]         = "Y",
    [90]         = "Z",
    [91]         = "{",
    [92]         = "\\",
    [93]         = "}",
    [96]         = "`",
    [32]         = "Space",
    [256]        = "Escape",
    [257]        = "Return",
    [258]        = "Tab",
    [259]        = "Backspace",
    [260]        = "Insert",
    [261]        = "Delete",
    [262]        = "Right",
    [263]        = "Left",
    [264]        = "Down",
    [265]        = "Up",
    [266]        = "Page Up",
    [267]        = "Page Down",
    [268]        = "Home",
    [269]        = "End",
    [280]        = "Caps Lock",
    [281]        = "Scroll Lock",
    [282]        = "Number Lock",
    [283]        = "Print Screen",
    [284]        = "Pause",
    [290]        = "F1",
    [291]        = "F2",
    [292]        = "F3",
    [293]        = "F4",
    [294]        = "F5",
    [295]        = "F6",
    [296]        = "F7",
    [297]        = "F8",
    [298]        = "F9",
    [299]        = "F10",
    [300]        = "F11",
    [301]        = "F12",
    [340]        = "L. Shift",
    [341]        = "L. Control",
    [342]        = "L. Alternate",
    [343]        = "L. Super",
    [344]        = "R. Shift",
    [345]        = "R. Control",
    [346]        = "R. Alternate",
    [347]        = "R. Super",
    [348]        = "Menu",
    [320]        = "Pad 0",
    [321]        = "Pad 1",
    [322]        = "Pad 2",
    [323]        = "Pad 3",
    [324]        = "Pad 4",
    [325]        = "Pad 5",
    [326]        = "Pad 6",
    [327]        = "Pad 7",
    [328]        = "Pad 8",
    [329]        = "Pad 9",
    [330]        = "Pad .",
    [331]        = "Pad /",
    [332]        = "Pad *",
    [333]        = "Pad -",
    [334]        = "Pad +",
    [335]        = "Pad Return",
    [336]        = "Pad =",
    [4]          = "Back",
    [24]         = "Volume Up",
    [25]         = "Volume Down",
}

---@enum input_mouse
INPUT_MOUSE     = {
    LEFT    = 0,
    RIGHT   = 1,
    MIDDLE  = 2,
    SIDE    = 3,
    EXTRA   = 4,
    FORWARD = 5,
    BACK    = 6,
    [0]     = "Mouse 0",
    [1]     = "Mouse 1",
    [2]     = "Mouse 2",
    [3]     = "Mouse 3",
    [4]     = "Mouse 4",
    [5]     = "Mouse 5",
    [6]     = "Mouse &",
}

---@enum cursor_mouse
CURSOR_MOUSE    = {
    DEFAULT       = 0,
    ARROW         = 1,
    IBEAM         = 2,
    CROSSHAIR     = 3,
    POINTING_HAND = 4,
    RESIZE_EW     = 5,
    RESIZE_NS     = 6,
    RESIZE_NWSE   = 7,
    RESIZE_NESW   = 8,
    RESIZE_ALL    = 9,
    NOT_ALLOWED   = 10
}

---@enum input_pad
INPUT_PAD       = {
    NULL             = 0,
    LEFT_FACE_UP     = 1,
    LEFT_FACE_RIGHT  = 2,
    LEFT_FACE_DOWN   = 3,
    LEFT_FACE_LEFT   = 4,
    RIGHT_FACE_UP    = 5,
    RIGHT_FACE_RIGHT = 6,
    RIGHT_FACE_DOWN  = 7,
    RIGHT_FACE_LEFT  = 8,
    LEFT_TRIGGER_1   = 9,
    LEFT_TRIGGER_2   = 10,
    RIGHT_TRIGGER_1  = 11,
    RIGHT_TRIGGER_2  = 12,
    MIDDLE_LEFT      = 13,
    MIDDLE           = 14,
    MIDDLE_RIGHT     = 15,
    LEFT_THUMB       = 16,
    RIGHT_THUMB      = 17,
    [0]              = "Unknown",
    [1]              = "L. Up",
    [2]              = "L. Right",
    [3]              = "L. Down",
    [4]              = "L. Left",
    [5]              = "R. Up",
    [6]              = "R. Right",
    [7]              = "R. Down",
    [8]              = "R. Left",
    [9]              = "L. Trigger 1",
    [10]             = "L. Trigger 2",
    [11]             = "R. Trigger 1",
    [12]             = "R. Trigger 2",
    [13]             = "Middle L.",
    [14]             = "Middle",
    [15]             = "Middle R.",
    [16]             = "L. Thumb",
    [17]             = "R. Thumb",
}

---@enum shader_location
SHADER_LOCATION = {
    VERTEX_POSITION    = 0,  -- Shader location: vertex attribute: position
    VERTEX_TEXCOORD01  = 1,  -- Shader location: vertex attribute: texcoord01
    VERTEX_TEXCOORD02  = 2,  -- Shader location: vertex attribute: texcoord02
    VERTEX_NORMAL      = 3,  -- Shader location: vertex attribute: normal
    VERTEX_TANGENT     = 4,  -- Shader location: vertex attribute: tangent
    VERTEX_COLOR       = 5,  -- Shader location: vertex attribute: color
    MATRIX_MVP         = 6,  -- Shader location: matrix uniform: model-view-projection
    MATRIX_VIEW        = 7,  -- Shader location: matrix uniform: view (camera transform)
    MATRIX_PROJECTION  = 8,  -- Shader location: matrix uniform: projection
    MATRIX_MODEL       = 9,  -- Shader location: matrix uniform: model (transform)
    MATRIX_NORMAL      = 10, -- Shader location: matrix uniform: normal
    VECTOR_VIEW        = 11, -- Shader location: vector uniform: view
    COLOR_DIFFUSE      = 12, -- Shader location: vector uniform: diffuse color
    COLOR_SPECULAR     = 13, -- Shader location: vector uniform: specular color
    COLOR_AMBIENT      = 14, -- Shader location: vector uniform: ambient color
    MAP_ALBEDO         = 15, -- Shader location: sampler2d texture: albedo (same as: SHADER_LOC_MAP_DIFFUSE)
    MAP_METALNESS      = 16, -- Shader location: sampler2d texture: metalness (same as: SHADER_LOC_MAP_SPECULAR)
    MAP_NORMAL         = 17, -- Shader location: sampler2d texture: normal
    MAP_ROUGHNESS      = 18, -- Shader location: sampler2d texture: roughness
    MAP_OCCLUSION      = 19, -- Shader location: sampler2d texture: occlusion
    MAP_EMISSION       = 20, -- Shader location: sampler2d texture: emission
    MAP_HEIGHT         = 21, -- Shader location: sampler2d texture: height
    MAP_CUBEMAP        = 22, -- Shader location: samplerCube texture: cubemap
    MAP_IRRADIANCE     = 23, -- Shader location: samplerCube texture: irradiance
    MAP_PREFILTER      = 24, -- Shader location: samplerCube texture: prefilter
    MAP_BRDF           = 25, -- Shader location: sampler2d texture: brdf
    VERTEX_BONEIDS     = 26, -- Shader location: vertex attribute: boneIds
    VERTEX_BONEWEIGHTS = 27, -- Shader location: vertex attribute: boneWeights
    BONE_MATRICES      = 28, -- Shader location: array of matrices uniform: boneMatrices
    VERTEX_INSTANCE_TX = 29  -- Shader location: vertex attribute: instanceTransform
}

---@enum window_flag
WINDOW_FLAG     = {
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
TEXTURE_FILTER  = {
    POINT           = 0, -- No filter, just pixel approximation
    BILINEAR        = 1, -- Linear filtering
    TRILINEAR       = 2, -- Trilinear filtering (linear with mipmaps)
    ANISOTROPIC_4X  = 3, -- Anisotropic filtering 4x
    ANISOTROPIC_8X  = 4, -- Anisotropic filtering 8x
    ANISOTROPIC_16X = 5, -- Anisotropic filtering 16x
}

---@enum texture_wrap
TEXTURE_WRAP    = {
    REPEAT        = 0, -- Repeats texture in tiled mode
    CLAMP         = 1, -- Clamps texture to edge pixel in tiled mode
    MIRROR_REPEAT = 2, -- Mirrors and repeats the texture in tiled mode
    MIRROR_CLAMP  = 3  -- Mirrors and clamps to border the texture in tiled mode
}
