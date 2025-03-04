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

require "bit"

local WINDOW_COLOR_PRIMARY_MAIN = color:new(255, 87, 34, 255)
local WINDOW_COLOR_PRIMARY_SIDE = color:new(255, 152, 0, 255)
local WINDOW_COLOR_TEXT_WHITE   = color:new(255, 255, 255, 255)
local WINDOW_COLOR_TEXT_BLACK   = color:new(33, 33, 33, 255)
local WINDOW_CARD_ROUND_SHAPE   = 0.25
local WINDOW_CARD_ROUND_COUNT   = 4
local WINDOW_ACTION_ABOVE       = action:new(
	{
		action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.W),
		action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.LEFT_FACE_UP),
	}
)
local WINDOW_ACTION_BELOW       = action:new(
	{
		action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.S),
		action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.LEFT_FACE_DOWN),
	}
)
local WINDOW_ACTION_FOCUS       = action:new(
	{
		action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.SPACE),
		action_button:new(INPUT_DEVICE.MOUSE, INPUT_MOUSE.LEFT),
		action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.RIGHT_FACE_DOWN),
	}
)
local WINDOW_ACTION_ALTERNATE   = action:new(
	{
		action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.SPACE),
		action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.TAB),
		action_button:new(INPUT_DEVICE.MOUSE, INPUT_MOUSE.LEFT),
		action_button:new(INPUT_DEVICE.MOUSE, INPUT_MOUSE.RIGHT),
		action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.RIGHT_FACE_DOWN),
		action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.RIGHT_FACE_UP),
	}
)
local WINDOW_ACTION_LATERAL     = action:new(
	{
		action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.A),
		action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.D),
		action_button:new(INPUT_DEVICE.MOUSE, INPUT_MOUSE.LEFT),
		action_button:new(INPUT_DEVICE.MOUSE, INPUT_MOUSE.RIGHT),
		action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.LEFT_FACE_LEFT),
		action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.LEFT_FACE_RIGHT),
	}
)
local WINDOW_SHIFT_A            = vector_2:new(6.0, 4.0)
local WINDOW_SHIFT_B            = vector_2:new(8.0, 6.0)
local WINDOW_DOT                = vector_2:new(4.0, 4.0)

---@class gizmo
---@field hover 	  number
---@field sound_hover boolean
---@field sound_focus boolean
gizmo                           = {}

---Create a new gizmo.
---@return gizmo value # The gizmo.
function gizmo:new()
	local i = {}
	setmetatable(i, {
		__index = self
	})

	--[[]]

	i.__type       = "gizmo"
	i.hover        = 0.0
	i.focus        = 0.0
	i.scroll_value = 0.0
	i.scroll_frame = 0.0

	return i
end

---Calculate the point of a gizmo with animation.
---@param lobby lobby # The lobby.
---@param shape box_2 # The shape.
---@return box_2 shape # The shape, with animation.
function gizmo:move(window, shape)
	-- move shape horizontally.
	--shape.x = shape.x + (ease.in_out_quad(self.hover) * 8.0) - 16.0 + math.out_quad(math.min(1.0, window.time * 4.0)) * 16.0

	shape.y = shape.y - (ease.in_out_quad(self.hover) * 4.0) + (ease.in_out_quad(self.focus) * 4.0)

	return shape
end

---Calculate the color of a gizmo with animation.
---@param lobby lobby # The lobby.
---@param color color # The color.
---@return color color # The color, with animation.
function gizmo:fade(window, color)
	-- fade in/out from hover.
	color = color * (ease.in_out_quad(self.hover) * 0.25 + 0.75)

	-- fade in/out from time.
	--color.a = math.floor(math.out_quad(math.min(1.0, window.time * 4.0)) * 255.0)

	return color
end

---@enum gizmo_flag
GIZMO_FLAG = {
	IGNORE_BOARD   = 0x00000001,
	IGNORE_MOUSE   = 0x00000010,
	CLICK_ON_PRESS = 0x00000100,
}

---@class window
---@field index  number
---@field count  number
---@field focus  number | nil
---@field device input_device
window     = {
	__meta = {}
}

---Draw a glyph.
---@param self        window # The window.
---@param board_label string # Board label.
---@param mouse_label string # Mouse label.
---@param pad_label   string # Pad label.
local function window_glyph(self, board_label, mouse_label, pad_label)
	local x, y = quiver.window.get_render_shape()
	local point = vector_2:old(8.0, y - 40.0)
	local label = board_label

	-- draw border.
	quiver.draw_2d.draw_box_2_border(box_2:old(point.x, point.y, x - 16.0, 32.0), false)

	-- if active device is the mouse...
	if self.device == INPUT_DEVICE.MOUSE then
		-- use mouse label.
		label = mouse_label
		-- if active device is the pad...
	elseif self.device == INPUT_DEVICE.PAD then
		-- use pad label.
		label = pad_label
	end

	-- draw label.
	LOGGER_FONT:draw(label, point + WINDOW_SHIFT_A, LOGGER_FONT_SCALE, LOGGER_FONT_SPACE, color:white())
end

local function window_gizmo(self, label, hover, index, focus, click)
	local delta = quiver.general.get_frame_time()

	label = label .. self.count

	if not self.data[label] then
		self.data[label] = gizmo:new()
	end

	local data = self.data[label]

	data.hover = math.clamp(0.0, 1.0,
		data.hover + ((hover or index or focus) and delta * 8.0 or delta * -8.0))

	data.focus = math.clamp(0.0, 1.0,
		data.focus + (focus and delta * 8.0 or delta * -8.0))

	return data
end

---Draw a border.
---@param shape  box_2    # The shape of the border.
---@param hover  boolean  # Mouse focus. Whether or not the mouse cursor is over this gizmo.
---@param index  boolean  # Board focus. Whether or not the board cursor is over this gizmo.
---@param focus  boolean  # Gizmo focus. Whether or not the window focus is on this gizmo.
---@param label? string   # OPTIONAL: Text to draw.
---@param move?  vector_2 # OPTIONAL: Text off-set.
local function window_border(self, shape, hover, index, focus, label, move)
	local gizmo = window_gizmo(self, label, hover, index, focus, click)
	local shape = gizmo:move(self, shape)
	local color = gizmo:fade(self, WINDOW_COLOR_PRIMARY_MAIN)
	local shift = vector_2:old(shape.x + WINDOW_SHIFT_A.x, shape.y + WINDOW_SHIFT_A.y)

	-- if move isn't nil...
	if move then
		-- apply text off-set.
		shift = shift + move
	end

	-- draw border.
	--quiver.draw_2d.draw_box_2_border(shape, focus)
	quiver.draw_2d.draw_box_2_gradient_y(
		box_2:old(shape.x, shape.y + shape.height - (shape.height * WINDOW_CARD_ROUND_SHAPE * 0.35), shape.width,
			(shape.height * WINDOW_CARD_ROUND_SHAPE * 0.35) * 4.0),
		color:old(0, 0, 0, 99),
		color:old(0, 0, 0, 0))
	quiver.draw_2d.draw_box_2_round(shape, WINDOW_CARD_ROUND_SHAPE, WINDOW_CARD_ROUND_COUNT, color)

	-- if we are not the focus gizmo...
	if not self.focus then
		-- if we have board/pad hover OR mouse hover...
		if index or hover then
			quiver.draw_2d.draw_box_2_dot(shape:old(shape.x + WINDOW_DOT.x, shape.y + WINDOW_DOT.y,
				shape.width - WINDOW_DOT.x * 2.0,
				shape.height - WINDOW_DOT.y * 2.0))
		end
	else
		-- if we have board/pad hover OR we are the focus gizmo...
		if index or focus then
			quiver.draw_2d.draw_box_2_dot(shape:old(shape.x + WINDOW_DOT.x, shape.y + WINDOW_DOT.y,
				shape.width - WINDOW_DOT.x * 2.0,
				shape.height - WINDOW_DOT.y * 2.0))
		end
	end

	-- if label isn't nil...
	if label then
		local color_a = gizmo:fade(self, color:old(127.0, 127.0, 127.0, 255.0))
		local color_b = gizmo:fade(self, color:old(255.0, 255.0, 255.0, 255.0))

		-- draw text, draw with back-drop.
		LOGGER_FONT:draw(label, shift + vector_2:old(1.0, 1.0), LOGGER_FONT_SCALE, LOGGER_FONT_SPACE, color_a)
		LOGGER_FONT:draw(label, shift, LOGGER_FONT_SCALE, LOGGER_FONT_SPACE, color_b)
	end
end

---Get the state of a gizmo.
---@param self   window     # The window.
---@param shape  box_2      # The shape of the gizmo.
---@param flag?  gizmo_flag # OPTIONAL: The flag of the gizmo.
---@param input? action     # OPTIONAL: The input of the gizmo. Will override the default focus action for the gizmo.
local function window_state(self, shape, flag, input)
	-- get the mouse position.
	local mouse_x, mouse_y = quiver.input.mouse.get_point()
	local mouse = vector_2:old(mouse_x, mouse_y)

	local check = true

	-- if there is a view-port shape set...
	if self.shape then
		-- check if the gizmo is within it.
		check = quiver.collision.box_box(shape, self.shape) and quiver.collision.point_box(mouse, self.shape)
	end

	-- mouse interaction check.
	local hover = self.device == INPUT_DEVICE.MOUSE and quiver.collision.point_box(mouse, shape) and check
	-- board interaction check.
	local index = self.device ~= INPUT_DEVICE.MOUSE and self.index == self.count
	-- whether or not this gizmo has been set off.
	local click = false
	local which = nil

	-- if flag isn't nil...
	if flag then
		-- gizmo flag set to ignore board/pad input.
		if bit.band(flag, GIZMO_FLAG.IGNORE_BOARD) ~= 0 then
			-- if board/pad is interacting with us...
			if index then
				-- set to false, and scroll away from us, using the last input direction.
				index = false
				self.index = self.index + self.which
			end
		end

		-- gizmo flag set to ignore mouse input.
		if bit.band(flag, GIZMO_FLAG.IGNORE_MOUSE) ~= 0 then
			-- if mouse is interacting with us...
			if hover then
				-- set to false.
				hover = false
			end
		end
	end

	-- if we have any form of interaction with the gizmo...
	if hover or index then
		-- check if the focus button has been set off.
		local hover_click = WINDOW_ACTION_FOCUS:press(self.device)

		-- if input over-ride isn't nil...
		if input then
			-- over-ride the default focus button with the given one.
			hover_click, which = input:press(self.device)
		end

		if hover_click then
			if flag and bit.band(flag, GIZMO_FLAG.CLICK_ON_PRESS) ~= 0 then
				click = true
			else
				-- focus button was set off, set us as the focus gizmo.
				self.focus = self.count
			end
		end
	end

	-- check if we are the focus gizmo.
	local focus = self.focus == self.count

	-- if we are the focus gizmo...
	if focus then
		-- check if the focus button has been set up.
		local focus_click = WINDOW_ACTION_FOCUS:release(self.device)

		-- if input over-ride isn't nil...
		if input then
			-- over-ride the default focus button with the given one.
			focus_click, which = input:release(self.device)
		end

		-- focus button was set up, set off click event, release focus gizmo.
		if focus_click then
			click = true
			self.focus = nil
		end
	end

	-- increase gizmo count.
	self.count = self.count + 1
	self.frame = shape.y + shape.height

	if index then
		self.gizmo = shape
	end

	return hover, index, focus, click, which
end

---Create a new window.
---@return window value # The window.
function window:new()
	local i = {}
	setmetatable(i, self.__meta)
	getmetatable(i).__index = self

	--[[]]

	i.__type = "window"
	i.index = 0.0
	i.count = 0.0
	i.point = 0.0
	i.which = 0.0
	i.shift = false
	i.device = INPUT_DEVICE.MOUSE
	i.data = {}

	return i
end

---Begin the window.
function window:begin()
	self.count = 0.0
end

---Close the window.
---@param lock boolean # If true, will lock user input.
function window:close(lock)
	local above = WINDOW_ACTION_ABOVE:press(self.device)
	local below = WINDOW_ACTION_BELOW:press(self.device)

	-- roll over the value in case it is not hovering over any valid gizmo.
	self.index = math.roll_over(0.0, self.count - 1.0, self.index)

	-- if temporarily locking navigation input or currently focusing a gizmo...
	if self.shift or self.focus or lock then
		-- remove navigation lock.
		self.shift = false

		-- disregard any input.
		return
	end

	-- scroll above.
	if above then
		self.index = self.index - 1.0
		self.which = -1.0
	end

	-- scroll below.
	if below then
		self.index = self.index + 1.0
		self.which = 1.0
	end

	-- get the latest board press.
	local board_check = quiver.input.board.get_key_code_queue() > 0.0
	local mouse_check = quiver.input.mouse.get_press(INPUT_MOUSE.LEFT)
	local pad_check = quiver.input.pad.get_queue() > 0.0

	-- a board or pad button was set off...
	if board_check or pad_check then
		-- set the active device as either board, or pad.
		self:set_device(board_check and INPUT_DEVICE.BOARD or INPUT_DEVICE.PAD)
	end

	-- a mouse button was set off...
	if mouse_check then
		self:set_device(INPUT_DEVICE.MOUSE)
	end
end

function window:set_device(device)
	if device == INPUT_DEVICE.BOARD or device == INPUT_DEVICE.PAD then
		if not quiver.input.mouse.get_hidden() then
			-- if mouse wasn't hidden, disable.
			quiver.input.mouse.set_active(false)
		end
	else
		if quiver.input.mouse.get_hidden() then
			-- if mouse was hidden, enable.
			quiver.input.mouse.set_active(true)
		end

		-- reset index.
		self.index = 0.0
	end

	-- set the active device.
	self.device = device
end

local function window_check_draw(self, shape)
	if self.shape then
		return quiver.collision.box_box(self.shape, shape)
	end

	return true
end

---Draw a text gizmo.
---@param point  box_2  # The point of the gizmo.
---@param label  string # The label of the gizmo.
---@param font   string # The font of the gizmo.
---@param scale  number # The text scale of the gizmo.
---@param space  number # The text space of the gizmo.
---@param color  number # The text color of the gizmo.
---@return boolean click # True on click, false otherwise.
function window:text(point, label, font, scale, space, color, call_back, call_data)
	-- scroll.
	point.y = point.y + self.point

	if window_check_draw(self, box_2:old(point.x, point.y, 1.0, scale)) then
		font:draw(label, point, scale, space, color)
	end
end

---Draw a button gizmo.
---@param shape box_2      # The shape of the gizmo.
---@param label string     # The label of the gizmo.
---@param flag? gizmo_flag # OPTIONAL: The flag of the gizmo.
---@return boolean click # True on click, false otherwise.
function window:button(shape, label, flag, call_back, call_data)
	-- scroll.
	shape.y = shape.y + self.point

	-- get the state of this gizmo.
	local hover, index, focus, click = window_state(self, shape, flag)

	if window_check_draw(self, shape) then
		if call_back then
			call_back(call_data, self, shape, hover, index, focus, click, label)
		else
			-- draw a border.
			window_border(self, shape, hover, index, focus, label)
		end
	end

	-- return true on click.
	return click
end

---Draw a toggle gizmo.
---@param shape box_2      # The shape of the gizmo.
---@param label string     # The label of the gizmo.
---@param value number     # The value of the gizmo.
---@param flag? gizmo_flag # OPTIONAL: The flag of the gizmo.
---@return number  value # The value.
---@return boolean click # True on click, false otherwise.
function window:toggle(shape, label, value, flag, call_back, call_data)
	-- scroll.
	shape.y = shape.y + self.point

	-- get the state of this gizmo.
	local hover, index, focus, click = window_state(self, shape, flag)

	-- toggle value on click.
	if click then
		value = not value
	end

	if window_check_draw(self, shape) then
		if call_back then
			call_back(call_data, self, shape, hover, index, focus, label, value)
		else
			-- draw a border, with a text off-set.
			window_border(self, shape, hover, index, focus, label, vector_2:old(shape.width, 0.0))

			-- if value is set on, draw a small box to indicate so.
			if value then
				quiver.draw_2d.draw_box_2_round(
					box_2:old(shape.x + 6.0, shape.y + 6.0, shape.width - 12.0, shape.height - 12.0),
					WINDOW_CARD_ROUND_SHAPE, WINDOW_CARD_ROUND_COUNT, WINDOW_COLOR_PRIMARY_SIDE)
			end
		end
	end

	-- return value, and click.
	return value, click
end

---Draw a slider gizmo.
---@param shape box_2      # The shape of the gizmo.
---@param label string     # The label of the gizmo.
---@param value number     # The value of the gizmo.
---@param min   number     # The minimum value of the gizmo.
---@param max   number     # The minimum value of the gizmo.
---@param step  number     # The step size of the gizmo.
---@param flag? gizmo_flag # OPTIONAL: The flag of the gizmo.
---@return number  value # The value.
---@return boolean click # True on click, false otherwise.
function window:slider(shape, label, value, min, max, step, flag, call_back, call_data)
	-- scroll.
	shape.y = shape.y + self.point

	-- click on press flag is incompatible with this gizmo, remove if present.
	if flag then
		flag = bit.band(flag, bit.bnot(GIZMO_FLAG.CLICK_ON_PRESS))
	end

	-- get the state of this gizmo.
	local hover, index, focus, click, which = window_state(self, shape, flag, WINDOW_ACTION_LATERAL)

	-- special preference for the mouse.
	if self.device == INPUT_DEVICE.MOUSE then
		-- if gizmo is in focus...
		if focus then
			-- get mouse position (X).
			local mouse_x = quiver.input.mouse.get_point()

			-- calculate value.
			local result = math.percentage_from_value(shape.x + 6.0, shape.x + 6.0 + shape.width - 12.0, mouse_x)
			result = math.clamp(0.0, 1.0, result)
			result = math.value_from_percentage(min, max, result)
			result = math.snap(step, result)
			value = result
		end
	else
		-- if there has been input at all...
		if which then
			-- get the actual button.
			which = WINDOW_ACTION_LATERAL.list[which]

			if which.button == INPUT_BOARD.A or which == INPUT_PAD.LEFT_FACE_LEFT then
				-- decrease value.
				value = value - step
			else
				-- increase value.
				value = value + step
			end

			-- clamp.
			value = math.clamp(min, max, value)
		end
	end

	if window_check_draw(self, shape) then
		-- get the percentage of the value within the minimum/maximum range.
		local percentage = math.clamp(0.0, 1.0, math.percentage_from_value(min, max, value))

		if call_back then
			call_back(call_data, self, shape, hover, index, focus, label, value, percentage)
		else
			-- draw a border, with a text off-set.
			window_border(self, shape, hover, index, focus, label, vector_2:old(shape.width, 0.0),
				function() window_glyph(self, "board", "mouse", "pad") end)

			-- if percentage is above 0.0...
			if percentage > 0.0 then
				quiver.draw_2d.draw_box_2_round(
					box_2:old(shape.x + 6.0, shape.y + 6.0, (shape.width - 12.0) * percentage, shape.height - 12.0),
					WINDOW_CARD_ROUND_SHAPE, WINDOW_CARD_ROUND_COUNT, WINDOW_COLOR_PRIMARY_SIDE)
			end

			-- measure text.
			local measure = LOGGER_FONT:measure_text(value, LOGGER_FONT_SCALE, LOGGER_FONT_SPACE)

			-- draw value.
			LOGGER_FONT:draw(value, vector_2:old(shape.x + (shape.width * 0.5) - (measure * 0.5), shape.y + 4.0),
				LOGGER_FONT_SCALE,
				LOGGER_FONT_SPACE,
				color:white())
		end
	end

	-- return value, and click.
	return value, click
end

---Draw a switch gizmo.
---@param shape box_2      # The shape of the gizmo.
---@param label string     # The label of the gizmo.
---@param value number     # The value of the gizmo.
---@param pool  table      # The value pool of the gizmo.
---@param flag? gizmo_flag # OPTIONAL: The flag of the gizmo.
---@return number  value # The value.
---@return boolean click # True on click, false otherwise.
function window:switch(shape, label, value, pool, flag, call_back, call_data)
	-- scroll.
	shape.y = shape.y + self.point

	-- get the state of this gizmo.
	local hover, index, focus, click, which = window_state(self, shape, flag, WINDOW_ACTION_LATERAL)

	local value_a = nil
	local value_b = nil
	local value_label = "N/A"

	value_label = pool[value]

	-- if there's an entry below us...
	if pool[value - 1] then
		value_a = value - 1
	end

	-- if there's an entry below us...
	if pool[value + 1] then
		value_b = value + 1
	end

	-- if there has been input at all...
	if which then
		-- get the actual button.
		which = WINDOW_ACTION_LATERAL.list[which]

		if which.button == INPUT_BOARD.A or which.button == INPUT_MOUSE.LEFT or which.button == INPUT_PAD.LEFT_FACE_LEFT then
			-- if below value is valid...
			if value_a then
				-- decrease value.
				value = value_a
			end
		else
			-- if above value is valid...
			if value_b then
				-- increase value.
				value = value_b
			end
		end
	end

	if window_check_draw(self, shape) then
		if call_back then
			call_back(call_data, self, shape, hover, index, focus, label, value_label)
		else
			-- draw a border, with a text off-set.
			window_border(self, shape, hover, index, focus, label, vector_2:old(shape.width, 0.0))

			-- measure text.
			local measure = LOGGER_FONT:measure_text(value_label, LOGGER_FONT_SCALE, LOGGER_FONT_SPACE)

			-- draw value.
			LOGGER_FONT:draw(value_label, vector_2:old(shape.x + (shape.width * 0.5) - (measure * 0.5), shape.y + 4.0),
				LOGGER_FONT_SCALE,
				LOGGER_FONT_SPACE,
				color:white())
		end
	end

	-- return value, and click.
	return value, click
end

---Draw an action gizmo.
---@param shape   box_2      # The shape of the gizmo.
---@param label   string     # The label of the gizmo.
---@param value   action     # The value of the gizmo.
---@param clamp?  number     # OPTIONAL: The maximum button count for the action. If nil, do not clamp.
---@param flag?   gizmo_flag # The flag of the gizmo.
---@return boolean click # True on click, false otherwise.
function window:action(shape, label, value, clamp, flag, call_back, call_data)
	-- scroll.
	shape.y = shape.y + self.point

	local pick = false

	-- if pick gizmo is not nil...
	if self.pick then
		-- check if we are the pick gizmo.
		pick = self.pick == self.count
	end

	-- if we are the pick gizmo...
	if pick then
		-- get every button press in the queue.
		local board_queue = quiver.input.board.get_key_code_queue()
		local mouse_queue = quiver.input.mouse.get_queue()

		-- if a button was set off...
		if board_queue > 0.0 or mouse_queue then
			if clamp then
				if #value.list >= clamp then
					-- remove every button for this action.
					value.list = {}
				end
			end

			-- if button came from the board, attach board action.
			if board_queue > 0.0 then
				value:attach(action_button:new(INPUT_DEVICE.BOARD, board_queue))
			end

			-- if button came from the mouse, attach mouse action.
			if mouse_queue then
				value:attach(action_button:new(INPUT_DEVICE.MOUSE, mouse_queue))
			end

			-- remove us from the focus gizmo, lock navigation, and remove us from the pick gizmo.
			self.focus = nil
			self.shift = true
			self.pick = nil
		end
	end

	local action = pick and action:new({}) or WINDOW_ACTION_ALTERNATE

	-- get the state of this gizmo.
	local hover, index, focus, click, which = window_state(self, shape, flag, action)

	-- if there has been input at all...
	if which then
		-- get the actual button.
		which = WINDOW_ACTION_ALTERNATE.list[which]

		if which.button == INPUT_BOARD.SPACE or which.button == INPUT_MOUSE.LEFT or which.button == INPUT_PAD.LEFT_FACE_DOWN then
			-- make us the focus/pick gizmo
			self.focus = self.count - 1.0
			self.pick = self.count - 1.0
		else
			-- remove every button for this action.
			value.list = {}
		end
	end

	if window_check_draw(self, shape) then
		if call_back then
			call_back(call_data, self, shape, hover, index, focus, label, value)
		else
			-- draw a border.
			window_border(self, shape, hover, index, focus, label, vector_2:old(shape.width, 0.0))

			local label = #value.list > 0.0 and "" or "N/A"

			-- for every button in the action's list...
			for i, button in ipairs(value.list) do
				-- concatenate the button's name.
				label = label .. (i > 1.0 and "/" or "")
					.. button:name()
			end

			-- measure text.
			local measure = LOGGER_FONT:measure_text(label, LOGGER_FONT_SCALE, LOGGER_FONT_SPACE)

			-- draw value.
			LOGGER_FONT:draw(label, vector_2:old(shape.x + (shape.width * 0.5) - (measure * 0.5), shape.y + 4.0),
				LOGGER_FONT_SCALE,
				LOGGER_FONT_SPACE,
				color:white())
		end
	end

	return click
end

---Draw an action gizmo.
---@param shape   box_2      # The shape of the gizmo.
---@param label   string     # The label of the gizmo.
---@param value   action     # The value of the gizmo.
---@param flag?   gizmo_flag # The flag of the gizmo.
---@return boolean click # True on click, false otherwise.
function window:entry(shape, label, value, flag)
	-- scroll.
	shape.y = shape.y + self.point

	local pick = false

	-- if pick gizmo is not nil...
	if self.pick then
		-- check if we are the pick gizmo.
		pick = self.pick == self.count
	end

	-- if we are the pick gizmo...
	if pick then
		-- get every button press in the queue.
		local board_queue = quiver.input.board.get_uni_code_queue()

		-- if a button was set off...
		if board_queue > 0.0 then
			value = value .. string.char(board_queue)
		end

		if quiver.input.board.get_press(INPUT_BOARD.RETURN) then
			-- remove us from the focus gizmo, lock navigation, and remove us from the pick gizmo.
			self.focus = nil
			self.shift = true
			self.pick = nil
		elseif quiver.input.board.get_press(INPUT_BOARD.BACKSPACE) then
			-- pop the last character of the working buffer.
			value = string.sub(value, 0, #value - 1)
		end
	end

	local action = pick and action:new({}) or WINDOW_ACTION_FOCUS

	-- get the state of this gizmo.
	local hover, index, focus, click, which = window_state(self, shape, flag, action)

	-- if there has been input at all...
	if which then
		self.focus = self.count - 1.0
		self.pick = self.count - 1.0
	end

	-- draw a border.
	window_border(self, shape, hover, index, focus, label, vector_2:old(shape.width, 0.0))

	-- draw value.
	LOGGER_FONT:draw(value, vector_2:old(shape.x + 4.0, shape.y + 4.0),
		LOGGER_FONT_SCALE,
		LOGGER_FONT_SPACE,
		color:white())

	return value
end

---Draw a scroll gizmo.
---@param shape box_2    # The shape of the gizmo.
---@param value number   # The value of the gizmo.
---@param frame number   # The frame of the gizmo.
---@param call  function # The draw function.
---@return number value
---@return number frame
function window:scroll(shape, call, call_back, call_data)
	if call_back then
		call_back(call_data, self, shape)
		--else
		--	quiver.draw_2d.draw_box_2_border(shape, true)
	end

	local gizmo = window_gizmo(self, window.count)

	local view_size = math.min(0.0, shape.height - gizmo.scroll_frame)
	self.point = view_size * gizmo.scroll_value
	self.shape = shape
	self.frame = 0.0

	local begin = self.count

	quiver.draw.begin_scissor(call, shape)

	local close = self.count

	gizmo.scroll_frame = (self.frame - shape.y) - self.point

	self.point = 0.0
	self.shape = nil
	self.frame = 0.0

	if self.gizmo then
		if self.index >= begin and self.index <= close then
			if self.gizmo.y < shape.y then
				local subtract = shape.y - self.gizmo.y

				gizmo.scroll_value = math.clamp(0.0, 1.0, gizmo.scroll_value + (subtract / view_size))
			end

			if self.gizmo.y + self.gizmo.height > shape.y + shape.height then
				local subtract = (self.gizmo.y + self.gizmo.height) - (shape.y + shape.height)

				gizmo.scroll_value = math.clamp(0.0, 1.0, gizmo.scroll_value - (subtract / view_size))
			end
		else
			if self.index < begin then gizmo.scroll_value = 0.0 end
			if self.index > close then gizmo.scroll_value = 1.0 end
		end
	end

	self.gizmo = nil

	local mouse = vector_2:old(quiver.input.mouse.get_point())
	local delta = vector_2:old(quiver.input.mouse.get_wheel())

	if quiver.collision.point_box(mouse, shape) then
		gizmo.scroll_value = math.clamp(0.0, 1.0, gizmo.scroll_value - delta.y * 0.05)
	end
end
