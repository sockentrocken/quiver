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

---@class path_node
---@field position vector_2|vector_3
---@field parent path_node
---@field f_cost number
---@field g_cost number
---@field h_cost number
path_node = {
    __meta = {}
}

---Create a new path node.
---@param position vector_3|vector_2 # The position of the node.
---@return value path_node # The node.
function path_node:new(position)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    i.__type = "path_node"
    i.position = position
    i.parent = nil
    i.g_cost = 0.0
    i.h_cost = 0.0
    i.f_cost = 0.0

    return i
end

---Get a path from point A to point B, given a list of every point node.
---@param point_a    path_node # Point A.
---@param point_b    path_node # Point B.
---@param node_list  table     # A list of every point node.
---@param node_find  function  # A function call-back with every nearby node. Function must be of the type `call_back(node_a, node_b)` and return a boolean, true for valid nearby node, false otherwise.
---@return table|nil value # A path from point A to point B.
function path_node:find(point_a, point_b, node_list, node_find)
    -- initialize the open and lock list.
    local open_list = { point_a }
    local lock_list = {}

    -- initialize the g, h, and f-cost of point A.
    point_a.g_cost = 0.0
    point_a.h_cost = (point_a.position - point_b.position):magnitude()
    point_a.f_cost = point_a.g_cost + point_a.h_cost

    -- while the open list isn't empty...
    while #open_list > 0.0 do
        local active_find = 1
        local pick_node = open_list[1]
        local active_distance = math.huge

        -- for every node in the open list...
        for i, node in ipairs(open_list) do
            -- if the f-cost of the current node is lower than the current lowest...
            if active_distance > node.f_cost then
                -- set active node and distance.
                active_find = i
                pick_node = node
                active_distance = node.f_cost
            end
        end

        -- if we are at point B...
        if pick_node == point_b then
            local path = {}
            local find = pick_node

            -- while the traversal node is not nil...
            while find do
                -- unroll path.
                table.insert(path, find)

                -- go up the parent tree.
                find = find.parent
            end

            -- return path.
            return path
        end

        -- remove the active node from the open list and move it to the lock list.
        table.remove(open_list, active_find)
        table.insert(lock_list, pick_node)

        local near = {}

        -- for every node in the node list...
        for _, node in ipairs(node_list) do
            -- if the current node is not the active node and the current node is a valid node...
            if node ~= pick_node and node_find(pick_node, node) then
                -- add the node as a near node.
                table.insert(near, node)
            end
        end

        -- for every node in the near list...
        for _, near_node in ipairs(near) do
            if not table.in_set(lock_list, near_node) then
                -- calculate g, h-cost.
                local g_cost = (near_node.position - pick_node.position):magnitude() + pick_node.g_cost
                local h_cost = (near_node.position - point_b.position):magnitude()

                if not table.in_set(open_list, near_node) or g_cost < near_node.g_cost then
                    -- link near node, add g, h, f-cost.
                    near_node.parent = pick_node
                    near_node.g_cost = g_cost
                    near_node.h_cost = h_cost
                    near_node.f_cost = near_node.g_cost + near_node.h_cost

                    -- if near node isn't in the open list...
                    if not table.in_set(open_list, near_node) then
                        -- add to open list.
                        table.insert(open_list, near_node)
                    end
                end
            end
        end
    end

    -- no valid path found, return nil.
    return nil
end
