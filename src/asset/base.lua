--[[
* MIT License
*
* Copyright (c) 2024 sockentrocken
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
]]

---A table pool, for initializing a memory arena of a certain kind for borrowing later.
---@class table_pool
---@field index number
---@field count number
---@field kind  table
table_pool = {}

---Create a new table pool.
---@param kind table  # The kind of table this table pool will initialize a memory arena for. MUST have a "default" function.
---@param size number # The size of the table.
function table_pool:new(kind, size)
    local i = {}
    setmetatable(i, {
        __index = self
    })

    -- initialize the table pool from 1 to {size} with the default instance of the {kind}.
    for x = 1, size + 1 do
        i[x] = kind:default()
    end

    -- set index, count, and kind.
    i.index = 1
    i.count = size
    i.kind = kind

    return i
end

---Clear the table pool index.
function table_pool:clear()
    self.index = 1
end

---Borrow a table from the table pool. WILL allocate a new table if every table in the pool is already in use.
function table_pool:get()
    -- increase the index by 1.
    self.index = self.index + 1

    -- index overflow!
    if self.index > self.count then
        -- create a new table.
        self[self.index] = self.kind:default()
        -- update our known table pool size.
        self.count = self.index
    end

    -- borrow table.
    return self[self.index - 1]
end

--[[----------------------------------------------------------------]]

---Print every key/value pair in a table.
---@param value table # Table to print.
function table_print(value)
    if not (value == nil) then
        for k, v in pairs(value) do
            print(tostring(k) .. ":" .. tostring(v))
        end
    end
end

---Check the sanity of a number, which will check for NaN and Infinite.
---@param value number # Number to check.
---@return boolean sanity # True if number is not sane, false otherwise.
function number_sanity(value)
    return not (value == value) or value == math.huge
end

---Check the sign of a number.
---@param value number # Number to check.
---@return number sign # 1.0 if number is positive OR equal to 0.0, -1.0 otherwise.
function number_sign(value)
    return value >= 0 and 1.0 or -1.0
end

---Get the percentage of a value in a range.
---@param min number # Minimum value.
---@param max number # Maximum value.
---@param value number # Input value.
---@return number percentage # Percentage.
function percentage_from_value(min, max, value)
    return (value - min) / (max - min)
end

---Get the value of a percentage in a range.
---@param min number # Minimum value.
---@param max number # Maximum value.
---@param value number # Input percentage.
---@return number value # Value.
function value_from_percentage(min, max, value)
    return value * (max - min) + min
end

---Get a random variation of a given value, which can either be positive or negative.
---@param value number # Number to randomize.
---@return number value # A value between [-number, number].
function number_random(value)
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
function number_interpolate(a, b, time)
    return (1.0 - time) * a + time * b
end

---Clamp a value in a range.
---@param min   number # Minimum value.
---@param max   number # Maximum value.
---@param value number # Value to clamp.
---@return number value # The value, within the min/max range.
function number_clamp(min, max, value)
    if value < min then return min end
    if value > max then return max end
    return value
end

---Return the "X", "Y", "Z" vector from an Euler angle.
---@param angle vector_3
---@return vector_3 d_x # "X" direction.
---@return vector_3 d_y # "Y" direction.
---@return vector_3 d_z # "Z" direction.
function direction_from_euler(angle)
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
    x = 0.0,
    y = 0.0,
}

---Create a new vector (2 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@return vector_2 value # The vector.
function vector_2:new(x, y)
    local i = {}
    setmetatable(i, {
        __index = self,
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
    })
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
    x = 0.0,
    y = 0.0,
    z = 0.0,
}

---Create a new vector (3 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@param z number # "Z" component.
---@return vector_3 value # The vector.
function vector_3:new(x, y, z)
    local i = {}
    setmetatable(i, {
        __index = self,
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
    })
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
    x = 0.0,
    y = 0.0,
    z = 0.0,
    w = 0.0,
}

---Create a new vector (4 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@param z number # "Z" component.
---@param w number # "W" component.
---@return vector_4 value # The vector.
function vector_4:new(x, y, z, w)
    local i = {}
    setmetatable(i, {
        __index = self,
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
    })
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
    local i = vector_3_pool:get()
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

--[[----------------------------------------------------------------]]

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

function camera_3d:default()
    return camera_3d:new(vector_3:new(0.0, 0.0, 0.0), vector_3:new(0.0, 0.0, 0.0), vector_3:new(0.0, 0.0, 0.0), 0.0)
end

function camera_3d:old(point, focus, angle, zoom)
    local i = camera_3d_pool:get()
    i.point = point
    i.focus = focus
    i.angle = angle
    i.zoom = zoom
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
    x = 0.0,
    y = 0.0,
    width = 0.0,
    height = 0.0,
}

function box_2:new(x, y, width, height)
    local i = {}
    setmetatable(i, {
        __index = self
    })
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

--[[----------------------------------------------------------------]]

---@class ray
---@field position  vector_3
---@field direction vector_3
ray = {
    position  = vector_3:zero(),
    direction = vector_3:zero(),
}

function ray:new(position, direction)
    local i = {}
    setmetatable(i, {
        __index = self
    })
    i.position  = position
    i.direction = direction
    return i
end

--[[----------------------------------------------------------------]]

function table_pool:begin()
    vector_2_pool:begin()
    vector_3_pool:begin()
    vector_4_pool:begin()
    color_pool:begin()
    box_2_pool:begin()
    camera_3d_pool:begin()
end

--[[----------------------------------------------------------------]]

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
    UNDECORATED              = 0x00000008, -- Set to disable window decoration (frame and buttons)
    HIDDEN                   = 0x00000080, -- Set to hide window
    MINIMIZED                = 0x00000200, -- Set to minimize window (iconify)
    MAXIMIZED                = 0x00000400, -- Set to maximize window (expanded to monitor)
    UNFOCUSED                = 0x00000800, -- Set to window non focused
    TOPMOST                  = 0x00001000, -- Set to window always on top
    ALWAYS_RUN               = 0x00000100, -- Set to allow windows running while minimized
    TRANSPARENT              = 0x00000010, -- Set to allow transparent framebuffer
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
