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

local POOL_VECTOR_2_AMOUNT  = 1024
local POOL_VECTOR_3_AMOUNT  = 1024
local POOL_VECTOR_4_AMOUNT  = 1024
local POOL_MATRIX_AMOUNT    = 1024
local POOL_CAMERA_2D_AMOUNT = 4
local POOL_CAMERA_3D_AMOUNT = 4
local POOL_COLOR_AMOUNT     = 1024
local POOL_BOX_2_AMOUNT     = 1024
local POOL_BOX_3_AMOUNT     = 1024
local POOL_RAY_AMOUNT       = 256

---@class vector_2
---@field x number
---@field y number
vector_2                    = {
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
            return string.format("{ x : %.2f, y: %.2f }", a.x, a.y)
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

    --[[]]

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

---Set every component for the current vector.
---@param x number # "X" component.
---@param y number # "Y" component.
function vector_2:set(x, y)
    self.x = x
    self.y = y
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

---Get the magnitude of the current vector.
---@return number value # The magnitude.
function vector_2:magnitude()
    return math.sqrt(self.x * self.x + self.y * self.y)
end

---Get the unit vector of the current vector.
---@return vector_2 value # The unit vector.
function vector_2:normalize()
    local length = math.sqrt(self.x * self.x + self.y * self.y)

    if not (length == 0.0) then
        local length = 1.0 / length
        return vector_2:old(self.x * length, self.y * length)
    else
        return self
    end
end

---Get the angle between the current vector, and a given one.
---@param value vector_2 # The vector to calculate the angle to.
---@return number value # The magnitude.
function vector_2:angle(value)
    local result = 0.0;

    local dot = self.x * value.x + self.y * value.y;
    local det = self.x * value.y - self.y * value.x;

    result = math.atan2(det, dot);

    return result;
end

---Scale a vector by the current 2D camera's zoom scale.
---@param camera camera_2d # The current 2D camera.
---@return vector_2 value # The vector.
function vector_2:scale_zoom(camera)
    return self * (1.0 / camera.zoom)
end

vector_2_pool = table_pool:new(vector_2, POOL_VECTOR_2_AMOUNT, "vector_2")

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

    --[[]]

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

---Set every component for the current vector.
---@param x number # "X" component.
---@param y number # "Y" component.
---@param z number # "Z" component.
function vector_3:set(x, y, z)
    self.x = x
    self.y = y
    self.z = z
    return self
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
    return self
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

---Snap the current to a given step.
---@param step number # Step.
---@return vector_3 value # The vector.
function vector_3:snap(step)
    return vector_3:old(
        math.snap(step, self.x),
        math.snap(step, self.y),
        math.snap(step, self.z)
    )
end

---Interpolate the current vector.
---@param value vector_3 # The vector to interpolate to.
---@return vector_3 value # The vector.
function vector_3:interpolate(value, time)
    return vector_3:old(
        math.interpolate(self.x, value.x, time),
        math.interpolate(self.y, value.y, time),
        math.interpolate(self.z, value.z, time)
    )
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

function vector_3:rotate_vector_4(q)
    local result = vector_3:old(0.0, 0.0, 0.0);

    result.x = self.x * (q.x * q.x + q.w * q.w - q.y * q.y - q.z * q.z) + self.y * (2 * q.x * q.y - 2 * q.w * q.z) +
        self.z * (2 * q.x * q.z + 2 * q.w * q.y);
    result.y = self.x * (2 * q.w * q.z + 2 * q.x * q.y) + self.y * (q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z) +
        self.z * (-2 * q.w * q.x + 2 * q.y * q.z);
    result.z = self.x * (-2 * q.w * q.y + 2 * q.x * q.z) + self.y * (2 * q.w * q.x + 2 * q.y * q.z) +
        self.z * (q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z);

    return result;
end

---Get the angle between the current vector, and a given one.
---@param value vector_3 # The vector to calculate the angle to.
---@return number value # The magnitude.
function vector_3:angle(value)
    local result = 0.0;

    local cross = vector_3:old(
        self.y * value.z - self.z * value.y,
        self.z * value.x - self.x * value.z,
        self.x * value.y - self.y * value.x
    );
    local len = math.sqrt(cross.x * cross.x + cross.y * cross.y + cross.z * cross.z);
    local dot = (self.x * value.x + self.y * value.y + self.z * value.z);
    result = math.atan2(len, dot);

    return result;
end

vector_3_pool = table_pool:new(vector_3, POOL_VECTOR_3_AMOUNT, "vector_3")

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

    --[[]]

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

---Set every component for the current vector.
---@param x number # "X" component.
---@param y number # "Y" component.
---@param z number # "Z" component.
---@param w number # "W" component.
function vector_4:set(x, y, z, w)
    self.x = x
    self.y = y
    self.z = z
    self.w = w
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

---Get the unit vector of the current vector.
---@return vector_4 value # The unit vector.
function vector_4:normalize()
    local length = math.sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)

    if not (length == 0.0) then
        local length = 1.0 / length
        return vector_4:old(self.x * length, self.y * length, self.z * length, self.w * length)
    else
        return self
    end
end

function vector_4:from_euler(pitch, yaw, roll)
    local result = vector_4:old(0.0, 0.0, 0.0, 0.0);

    local x0 = math.cos(pitch * 0.5);
    local x1 = math.sin(pitch * 0.5);
    local y0 = math.cos(yaw * 0.5);
    local y1 = math.sin(yaw * 0.5);
    local z0 = math.cos(roll * 0.5);
    local z1 = math.sin(roll * 0.5);

    result.x = x1 * y0 * z0 - x0 * y1 * z1;
    result.y = x0 * y1 * z0 + x1 * y0 * z1;
    result.z = x0 * y0 * z1 - x1 * y1 * z0;
    result.w = x0 * y0 * z0 + x1 * y1 * z1;

    return result;
end

vector_4_pool = table_pool:new(vector_4, POOL_VECTOR_4_AMOUNT)

--[[----------------------------------------------------------------]]

---@class matrix
---@field m0  number
---@field m1  number
---@field m2  number
---@field m3  number
---@field m4  number
---@field m5  number
---@field m6  number
---@field m7  number
---@field m8  number
---@field m9  number
---@field m10 number
---@field m11 number
---@field m12 number
---@field m13 number
---@field m14 number
---@field m15 number
matrix = {
    __meta = {}
}

function matrix:new(m0, m1, m2, m3,
                    m4, m5, m6, m7,
                    m8, m9, m10, m11,
                    m12, m13, m14, m15)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "matrix"
    i.m0 = m0
    i.m1 = m1
    i.m2 = m2
    i.m3 = m3
    i.m4 = m4
    i.m5 = m5
    i.m6 = m6
    i.m7 = m7
    i.m8 = m8
    i.m9 = m9
    i.m10 = m10
    i.m11 = m11
    i.m12 = m12
    i.m13 = m13
    i.m14 = m14
    i.m15 = m15

    return i
end

function matrix:default()
    return matrix:new(
        0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0
    )
end

function matrix:old(m0, m1, m2, m3,
                    m4, m5, m6, m7,
                    m8, m9, m10, m11,
                    m12, m13, m14, m15)
    local i = matrix_pool:get()
    i.m0 = m0
    i.m1 = m1
    i.m2 = m2
    i.m3 = m3
    i.m4 = m4
    i.m5 = m5
    i.m6 = m6
    i.m7 = m7
    i.m8 = m8
    i.m9 = m9
    i.m10 = m10
    i.m11 = m11
    i.m12 = m12
    i.m13 = m13
    i.m14 = m14
    i.m15 = m15
    return i
end

function matrix:set(m0, m1, m2, m3,
                    m4, m5, m6, m7,
                    m8, m9, m10, m11,
                    m12, m13, m14, m15)
    self.m0 = m0
    self.m1 = m1
    self.m2 = m2
    self.m3 = m3
    self.m4 = m4
    self.m5 = m5
    self.m6 = m6
    self.m7 = m7
    self.m8 = m8
    self.m9 = m9
    self.m10 = m10
    self.m11 = m11
    self.m12 = m12
    self.m13 = m13
    self.m14 = m14
    self.m15 = m15
end

matrix_pool = table_pool:new(matrix, POOL_MATRIX_AMOUNT)

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

    --[[]]

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

---Set every component for the current box.
---@param x      number # "X" component.
---@param y      number # "Y" component.
---@param width  number # Width component.
---@param height number # Height component.
function box_2:set(x, y, width, height)
    self.x = x
    self.y = y
    self.width = width
    self.height = height
end

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

    --[[]]

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

function box_3:point(value)
    return box_3:old(self.min + value, self.max + value)
end

function box_3:scale(value)
    return box_3:old(self.min * value, self.max * value)
end

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

    --[[]]

    i.__type    = "ray"
    i.position  = position
    i.direction = direction

    return i
end

function ray:default()
    return ray:new(vector_3:default(), vector_3:default())
end

function ray:old(position, direction)
    local i = ray_pool:get()
    i.position = position
    i.direction = direction
    return i
end

function ray:zero()
    return ray:old(vector_3:zero(), vector_3:zero())
end

function ray:pack(p_x, p_y, p_z, d_x, d_y, d_z)
    self.position:set(p_x, p_y, p_z)
    self.direction:set(d_x, d_y, d_z)
end

ray_pool = table_pool:new(ray, POOL_RAY_AMOUNT)

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
                    math.clamp(0.0, 255.0, math.floor(a * b.r)),
                    math.clamp(0.0, 255.0, math.floor(a * b.g)),
                    math.clamp(0.0, 255.0, math.floor(a * b.b)),
                    b.a
                )
            elseif type(b) == "number" then
                return color:old(
                    math.clamp(0.0, 255.0, math.min(255.0, math.floor(a.r * b))),
                    math.clamp(0.0, 255.0, math.min(255.0, math.floor(a.g * b))),
                    math.clamp(0.0, 255.0, math.min(255.0, math.floor(a.b * b))),
                    a.a
                )
            else
                return color:old(
                    math.clamp(0.0, 255.0, math.floor(a.r * b.r)),
                    math.clamp(0.0, 255.0, math.floor(a.g * b.g)),
                    math.clamp(0.0, 255.0, math.floor(a.b * b.b)),
                    math.clamp(0.0, 255.0, math.floor(a.a * b.a))
                )
            end
        end,
        __tostring = function(a)
            return string.format("{ r : %.2f, g: %.2f, b: %.2f, a: %.2f }", a.r, a.g, a.b, a.a)
        end
    }
}

function color:new(r, g, b, a)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

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

---Set every component for the current color.
---@param r number # "R" component.
---@param g number # "G" component.
---@param b number # "B" component.
---@param a number # "A" component.
function color:set(r, g, b, a)
    self.r = r
    self.g = g
    self.b = b
    self.a = a
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

function color:grey()
    return color:old(127.0, 127.0, 127.0, 255.0)
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

color_pool = table_pool:new(color, POOL_COLOR_AMOUNT)

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

    --[[]]

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

    --[[]]

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

function camera_3d:pack(p_x, p_y, p_z, f_x, f_y, f_z, a_x, a_y, a_z, zoom, kind)
    self.point:set(p_x, p_y, p_z)
    self.focus:set(f_x, f_y, f_z)
    self.angle:set(a_x, a_y, a_z)
    self.zoom = zoom
    self.kind = kind
end

camera_3d_pool = table_pool:new(camera_3d, POOL_CAMERA_3D_AMOUNT)

quiver.collision = {}

---Check if a point and a box are colliding.
---@param  point   vector_2  # Point to check.
---@param  box     box_2     # Box to check.
---@return boolean collision # True if colliding, false otherwise.
function quiver.collision.point_box(point, box)
    return (point.x >= box.x) and (point.x < (box.x + box.width)) and (point.y >= box.y) and
        (point.y < (box.y + box.height))
end

---Check if a box and a box are colliding.
---@param  box_a box_2 # Box A to check.
---@param  box_b box_2 # Box B to check.
---@return boolean collision # True if colliding, false otherwise.
function quiver.collision.box_box(box_a, box_b)
    return (box_a.x < (box_b.x + box_b.width) and (box_a.x + box_a.width) > box_b.x) and
        (box_a.y < (box_b.y + box_b.height) and (box_a.y + box_a.height) > box_b.y)
end

-- TO-DO
function quiver.collision.ray_box(ray, box)
    local collision = {}

    -- Note: If ray.position is inside the box, the distance is negative (as if the ray was reversed)
    -- Reversing ray.direction will give use the correct result
    local insideBox = (ray.position.x > box.min.x) and (ray.position.x < box.max.x) and
        (ray.position.y > box.min.y) and (ray.position.y < box.max.y) and
        (ray.position.z > box.min.z) and (ray.position.z < box.max.z)

    --if (insideBox) then ray.direction = Vector3Negate(ray.direction) end
    if (insideBox) then ray.direction = ray.direction * -1.0 end

    local t            = {}

    t[8]               = 1.0 / ray.direction.x
    t[9]               = 1.0 / ray.direction.y
    t[10]              = 1.0 / ray.direction.z

    t[0]               = (box.min.x - ray.position.x) * t[8]
    t[1]               = (box.max.x - ray.position.x) * t[8]
    t[2]               = (box.min.y - ray.position.y) * t[9]
    t[3]               = (box.max.y - ray.position.y) * t[9]
    t[4]               = (box.min.z - ray.position.z) * t[10]
    t[5]               = (box.max.z - ray.position.z) * t[10]
    t[6]               = math.max(math.max(math.min(t[0], t[1]), math.min(t[2], t[3])), math.min(t[4], t[5]))
    t[7]               = math.min(math.min(math.max(t[0], t[1]), math.max(t[2], t[3])), math.max(t[4], t[5]))

    collision.hit      = not ((t[7] < 0) or (t[6] > t[7]))
    collision.distance = t[6]
    collision.point    = ray.position + (ray.direction * collision.distance)

    -- Get box center point
    collision.normal   = box.min:interpolate(box.max, 0.5)
    -- Get vector center point->hit point
    collision.normal   = collision.point - collision.normal
    -- Scale vector to unit cube
    -- NOTE: We use an additional .01 to fix numerical errors
    collision.normal   = collision.normal * 2.01
    collision.normal   = collision.normal / (box.max - box.min)
    -- The relevant elements of the vector are now slightly larger than 1.0 (or smaller than -1.0)
    -- and the others are somewhere between -1.0 and 1.0 casting to int is exactly our wanted normal!
    collision.normal.x = math.ceil(collision.normal.x)
    collision.normal.y = math.ceil(collision.normal.y)
    collision.normal.z = math.ceil(collision.normal.z)

    collision.normal   = collision.normal:normalize()

    if (insideBox) then
        -- Reset ray.direction
        ray.direction = ray.direction * -1.0
        -- Fix result
        collision.distance = collision.distance * -1.0
        collision.normal = collision.normal * -1.0
    end

    return collision
end

---Reset every table pool index to 1. This should usually be done before beginning to draw, or when running a simulation tick.
function table_pool:clear()
    vector_2_pool:begin()
    vector_3_pool:begin()
    vector_4_pool:begin()
    color_pool:begin()
    box_2_pool:begin()
    box_3_pool:begin()
    ray_pool:begin()
    matrix_pool:begin()
    camera_2d_pool:begin()
    camera_3d_pool:begin()
end
