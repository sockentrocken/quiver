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

---@class scheduler
---@field routine table
scheduler = {
    __meta = {}
}

---Create a new scheduler.
---@example lua/scheduler.lua
---@return scheduler value # The scheduler.
function scheduler:new()
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "scheduler"
    i.routine = {}

    return i
end

---Insert a new co-routine in the scheduler.
---@param call        function # The function to convert into a co-routine.
---@param name?       string   # OPTIONAL: A name for the co-routine.
---@param over_write? boolean  # OPTIONAL: Whether or not to over-write an already existing co-routine by the same name.
function scheduler:insert(call, name, over_write)
    -- if we want to store this co-routine with a name...
    if name then
        -- do not over-write an existing co-routine in process.
        if self.routine[name] and not over_write then
            return
        end

        -- insert co-routine into the routine list.
        self.routine[name] = coroutine.create(call)
    else
        -- insert co-routine into the routine list.
        table.insert(self.routine, coroutine.create(call))
    end
end

---Resume every co-routine in the scheduler.
function scheduler:resume()
    -- for every co-routine in our routine list...
    for i, routine in pairs(self.routine) do
        local result, error_message = nil, nil

        -- check if the co-routine isn't dead and resume it.
        if not (coroutine.status(routine) == "dead") then
            result, error_message = coroutine.resume(routine)
        end

        -- check if the co-routine is dead.
        if coroutine.status(routine) == "dead" then
            -- throw an error message, if there were any.
            if error_message then
                error(error_message)
            end

            -- if the co-routine is a routine with a name...
            if type(i) == "string" then
                -- remove co-routine.
                self.routine[i] = nil
            else
                -- remove co-routine.
                table.remove(self.routine, i)
            end
        end
    end
end
