texture_1 = Texture("example/suite/asset/texture_1.png")
texture_2 = Texture("example/suite/asset/texture_2.png")

function main_video()
end

function step_video()
	texture_1:draw(vector_2:new(8.0, 32.0), 0.0, 1.0, color:white())
end