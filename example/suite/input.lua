function step_input()
	local y = 0
	local color_r = color:new(1.0, 0.0, 0.0, 1.0)
	local color_g = color:new(0.0, 1.0, 0.0, 1.0)

	-- Return true if key is up, false otherwise.
	local board_up      = get_board_up(INPUT_BOARD.KEY_SPACE)
	-- Return true if key is down, false otherwise.
	local board_down    = get_board_down(INPUT_BOARD.KEY_SPACE)
	-- Return true if key was first pressed, false otherwise.
	local board_press   = get_board_press(INPUT_BOARD.KEY_SPACE)
	-- Return true if key was first released, false otherwise.
	local board_release = get_board_release(INPUT_BOARD.KEY_SPACE)

	-- Return true if key is up, false otherwise.
	local mouse_up      = get_mouse_up(INPUT_MOUSE.MOUSE_BUTTON_LEFT)
	-- Return true if key is down, false otherwise.
	local mouse_down    = get_mouse_down(INPUT_MOUSE.MOUSE_BUTTON_LEFT)
	-- Return true if key was first pressed, false otherwise.
	local mouse_press   = get_mouse_press(INPUT_MOUSE.MOUSE_BUTTON_LEFT)
	-- Return true if key was first released, false otherwise.
	local mouse_release = get_mouse_release(INPUT_MOUSE.MOUSE_BUTTON_LEFT)


	draw_text("[SPACE] key is up.",       vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, board_up      and color_g or color_r) y = y + 1
	draw_text("[SPACE] key is down.",     vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, board_down    and color_g or color_r) y = y + 1
	draw_text("[SPACE] key is pressed.",  vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, board_press   and color_g or color_r) y = y + 1
	draw_text("[SPACE] key is released.", vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, board_release and color_g or color_r) y = y + 1

	y = y + 1

	draw_text("[LEFT MOUSE BUTTON] mouse button is up.",       vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, mouse_up      and color_g or color_r) y = y + 1
	draw_text("[LEFT MOUSE BUTTON] mouse button is down.",     vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, mouse_down    and color_g or color_r) y = y + 1
	draw_text("[LEFT MOUSE BUTTON] mouse button is pressed.",  vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, mouse_press   and color_g or color_r) y = y + 1
	draw_text("[LEFT MOUSE BUTTON] mouse button is released.", vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, mouse_release and color_g or color_r) y = y + 1

	y = y + 1

	-- Check if a pad is available.
	if get_pad_state(0) then
		local pad_name    = get_pad_name(0)
		-- Return true if key is up, false otherwise.
		local pad_up      = get_pad_up(0, INPUT_PAD.GAMEPAD_BUTTON_RIGHT_FACE_DOWN)
		-- Return true if key is down, false otherwise.
		local pad_down    = get_pad_down(0, INPUT_PAD.GAMEPAD_BUTTON_RIGHT_FACE_DOWN)
		-- Return true if key was first pressed, false otherwise.
		local pad_press   = get_pad_press(0, INPUT_PAD.GAMEPAD_BUTTON_RIGHT_FACE_DOWN)
		-- Return true if key was first released, false otherwise.
		local pad_release = get_pad_release(0, INPUT_PAD.GAMEPAD_BUTTON_RIGHT_FACE_DOWN)

		draw_text("Pad name: "..pad_name,       		   vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, color_r) y = y + 1

		draw_text("[PAD (X)/(A)] pad button is up.",       vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, pad_up      and color_g or color_r) y = y + 1
		draw_text("[PAD (X)/(A)] pad button is down.",     vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, pad_down    and color_g or color_r) y = y + 1
		draw_text("[PAD (X)/(A)] pad button is pressed.",  vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, pad_press   and color_g or color_r) y = y + 1
		draw_text("[PAD (X)/(A)] pad button is released.", vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, pad_release and color_g or color_r) y = y + 1

		for x = 0, get_pad_axis_count(0) do
			local state = get_pad_axis_state(0, x)

			draw_text(string.format("Axis %d: %.2f", x, state), vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, color_r) y = y + 1
		end
	else
		draw_text("Pad not available.", vector_2:new(8.0, 32.0 + 10.0 * y), 8.0, color_r) y = y + 1
	end
end