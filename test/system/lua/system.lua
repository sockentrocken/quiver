-- Create a new file-system.
local i = system:new({
	"game_folder_1", -- image.png, sound.wav, model.obj
	"game_folder_2", -- image.png
	"game_folder_3" -- sound.wav
})

-- Scan "g_f_1", "g_f_2", "g_f_3" to update the asset look-up table.
i:scan()

i:find("image.png") -- "game_folder_2/image.png"
i:find("sound.wav") -- "game_folder_3/sound.wav"
i:find("model.obj") -- "game_folder_1/model.obj"
