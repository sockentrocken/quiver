value_1 = false
value_2 = false
value_3 = 0
value_4 = 0
value_5 = 0
value_6 = 0

function step_interface()
	local y = 0

	interface_button(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Button")
	y = y + 1
	value_1 = interface_toggle(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Toggle", value_1)
	y = y + 1
	value_2 = interface_check_box(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(16.0, 16.0)), "Check Box", value_2)
	y = y + 1
	value_3 = interface_combo_box(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Combo 1; Combo 2", value_3)
	y = y + 1
	value_4 = interface_spinner(box_2:new(vector_2:new(48.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Spinner", value_4, -8.0, 8.0, false)
	y = y + 1
	value_5 = interface_slider(box_2:new(vector_2:new(48.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Slider L", "Slider R", value_5, -8.0, 8.0)
	y = y + 1
	value_6 = interface_slider_bar(box_2:new(vector_2:new(48.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Slider L", "Slider R", value_6, -8.0, 8.0)
	y = y + 1
end