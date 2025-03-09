-- Create a new file-system. It will scan every path in the table to create an asset look-up table.
local i = system:new({
    "game_folder_1", -- Has the file "card.png".
    "game_folder_2", -- Has the file "font.ttf".
    "game_folder_3", -- Has the file "card.png".
})

-- Try locating font.ttf.
assert(i:find("font.ttf"), "game_folder_2/font.ttf")

-- Because "game_folder_3" was last, its data ("card.png") will override the data of "game_folder_1".
assert(i:find("card.png"), "game_folder_3/card.png")
