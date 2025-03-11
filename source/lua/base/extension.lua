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

    for token in text:gmatch("([^" .. find .. "]+)") do
        table.insert(i, token)
    end

    return i
end

--[[----------------------------------------------------------------]]

---Get the length of the hash-side of a table.
---@param value table # Table to calculate length from.
---@return number length # The length of the hash-side of the table.
function table.hash_length(value)
    local i = 0.0

    for _, _ in pairs(value) do i = i + 1.0 end

    return i
end

---Deep copy a table.
---@param value table # Table to copy.
---@return table value # The table.
function table.copy(value, work)
    if not work then
        work = {}
    end

    for k, v in pairs(value) do
        if type(v) == "table" then
            work[k] = table.copy(v)
        else
            work[k] = v
        end
    end

    return work
end

---Print every key/value pair in a table.
---@param value table # Table to print.
function table.print(value, depth)
    if not depth then
        depth = 1.0
    end

    print("{")

    for k, v in pairs(value) do
        local k = type(k) == "number" and string.format("[%d]", k) or k
        local i = ""

        for x = 1, depth do
            i = i .. "  "
        end

        print(i .. tostring(k) .. " = " .. tostring(v))

        if type(v) == "table" then
            table.print(v, depth + 1.0)
        end
    end

    print("}")
end

---Check if an object is within a table.
---@param value  table # Table to check the value in.
---@param object any   # Value to check.
---@return boolean check # True if value is in table, false otherwise.
function table.in_set(value, object)
    for k, v in ipairs(value) do
        if v == object then
            return true
        end
    end

    return false
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
                    getmetatable(v).__index = meta
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
                if meta.__meta then
                    setmetatable(value, meta.__meta)
                    getmetatable(value).__index = meta
                end
            else
                error(string.format(
                    "table.restore_meta(): Found \"__type\" for table, but could not find \"%s\" class table.",
                    value.__type))
            end
        end
    end
end

--[[----------------------------------------------------------------]]

---Check the sanity of a number, which will check for NaN and Infinite.
---@param value number # Number to check.
---@return boolean sanity # True if number is not sane, false otherwise.
function math.sanity(value)
    return not (value == value) or value == math.huge
end

---Check the sign of a number.
---@param value number # Number to check.
---@return number sign # 1.0 if number is positive, or -1.0 if number is negative. 0.0 otherwise.
function math.sign(value)
    if value == 0.0 then return 0.0 end
    if value > 0.0 then return 1.00 end
    if value < 0.0 then return -1.0 end
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
---@param step  number # Step.
---@param value number # Input value.
---@return number value # Value.
function math.snap(step, value)
    return math.floor(value / step) * step
end

---Get a random variation of a given value, which can either be positive or negative.
---@param value number # Number to randomize.
---@return number value # A value between [-number, number].
function math.random_sign(value)
    local random = math.random()
    if random > 0.5 then
        return value * math.percentage_from_value(0.5, 1.0, random)
    else
        return value * math.percentage_from_value(0.0, 0.5, random) * -1.0
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
    local angle = vector_3:old(
        angle.z * (math.pi / 180.0) * -1.0,
        angle.y * (math.pi / 180.0) * -1.0,
        angle.x * (math.pi / 180.0) * -1.0
    )

    local sr, sp, sy, cr, cp, cy;

    sy = math.sin(angle.y);
    cy = math.cos(angle.y);

    sp = math.sin(angle.x);
    cp = math.cos(angle.x);

    sr = math.sin(angle.z);
    cr = math.cos(angle.z);

    d_x.x = cp * cy;
    d_x.y = -sp;
    d_x.z = cp * sy;

    d_z.x = (-1 * sr * sp * cy + -1 * cr * -sy);
    d_z.y = -1 * sr * cp;
    d_z.z = (-1 * sr * sp * sy + -1 * cr * cy);

    d_y.x = (cr * sp * cy + -sr * -sy);
    d_y.y = cr * cp;
    d_y.z = (cr * sp * sy + -sr * cy);

    return d_x, d_y, d_z
end

function math.degree_to_radian(value)
    return value * (math.pi / 180.0)
end

function math.radian_to_degree(value)
    return value * (180.0 / math.pi)
end

--[[----------------------------------------------------------------]]

-- all code from https://easings.net/.

ease = {}

---Ease in sine. (https://easings.net/#easeInSine)
---@param value number
---@return number value # Result.
function ease.in_sine(value)
    return 1 - math.cos((value * math.pi) / 2)
end

---Ease out sine. (https://easings.net/#easeOutSine)
---@param value number
---@return number value # Result.
function ease.out_sine(value)
    return math.sin((value * math.pi) / 2)
end

---Ease in-out sine. (https://easings.net/#easeInOutSine)
---@param value number
---@return number value # Result.
function ease.in_out_sine(value)
    return -(math.cos(math.pi * value) - 1) / 2
end

---Ease in quad. (https://easings.net/#easeInQuad)
---@param value number
---@return number value # Result.
function ease.in_quad(value)
    return value * value
end

---Ease out quad. (https://easings.net/#easeOutQuad)
---@param value number
---@return number value # Result.
function ease.out_quad(value)
    return 1 - (1 - value) * (1 - value)
end

---Ease in-out quad. (https://easings.net/#easeInOutQuad)
---@param value number
---@return number value # Result.
function ease.in_out_quad(value)
    return value < 0.5 and 2 * value * value or 1 - math.pow(-2 * value + 2, 2) / 2
end

---Ease in cubic. (https://easings.net/#easeInCubic)
---@param value number
---@return number value # Result.
function ease.in_cubic(value)
    return value * value * value
end

---Ease out cubic. (https://easings.net/#easeOutCubic)
---@param value number
---@return number value # Result.
function ease.out_cubic(value)
    return 1 - math.pow(1 - value, 3)
end

---Ease in-out cubic. (https://easings.net/#easeInOutCubic)
---@param value number
---@return number value # Result.
function ease.in_out_cubic(value)
    return value < 0.5 and 4 * value * value * value or 1 - math.pow(-2 * value + 2, 3) / 2;
end
