function step_steam()
	local name = steam:user_name()

	draw_text(name, vector_2:new(8.0, 32.0), 8.0, color:new(1.0, 0.0, 0.0, 1.0))
end