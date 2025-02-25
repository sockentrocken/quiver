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

---@enum FILE_KIND
FILE_KIND = {
	DISK  = 0,
	PACK  = 1,
	EMBED = 2,
}

---@class file_entry
file_entry = {
	__meta = {}
}

function file_entry:new(path, kind)
	local i = {}
	setmetatable(i, self.__meta)
	getmetatable(i).__index = self

	i.path = path
	i.kind = kind

	return i
end

---@class system
---@field search      table
---@field locate      table
---@field memory_list table
---@field memory_data table
system = {
	__meta = {}
}

---Create a new virtual file-system. For serialization, you may want to only serialize "search", "locate", and "memory_list", which only contain serializable data.
---```lua
---local i = system:new({
---    "game_folder_1", -- image.png, sound.wav, model.obj
---    "game_folder_2", -- image.png
---    "game_folder_3"  -- sound.wav
---})
---
----- Scan "g_f_1", "g_f_2", "g_f_3" to update the asset look-up table.
---i:scan()
---
---i:find("image.png") -- "game_folder_2/image.png"
---i:find("sound.wav") -- "game_folder_3/sound.wav"
---i:find("model.obj") -- "game_folder_1/model.obj"
---```
---@return system value # The virtual file-system.
function system:new(search)
	local i = {}
	setmetatable(i, self.__meta)
	getmetatable(i).__index = self

	--[[]]

	i.__type      = "system"
	i.locate      = {}
	i.memory_list = {
		texture         = {},
		model           = {},
		model_animation = {},
		sound           = {},
		music           = {},
		shader          = {},
		font            = {}
	}
	i.memory_data = {
		texture         = {},
		model           = {},
		model_animation = {},
		sound           = {},
		music           = {},
		shader          = {},
		font            = {}
	}

	i:scan(search)


	return i
end

---Scan every directory in the asset's search table, to update the asset look-up table.
function system:scan(search)
	local embed_list = quiver.data.get_embed_list()

	-- for each search path in the search table...
	for _, search_path in ipairs(search) do
		-- check if the given path is a folder or a file.
		if quiver.file.get_path_exist(search_path) then
			-- scan the path recursively.
			local list = quiver.file.scan_path(search_path, nil, true, true)

			for _, search_file in ipairs(list) do
				self.locate[search_file] = file_entry:new(search_path .. "/" .. search_file, FILE_KIND.DISK)
			end
		else
			if quiver.file.get_file_exist(search_path) then
				local pack = quiver.zip.new(search_path)
				local list = pack:get_list()

				for _, search_file in ipairs(list) do
					print(search_file)

					self.locate[search_file] = file_entry:new(pack, FILE_KIND.PACK)
				end
			else
				for _, search_file in ipairs(embed_list) do
					local token = string.tokenize(search_file, "/")

					if token[2] then
						if token[1] == search_path then
							self.locate[token[2]] = file_entry:new(search_file, FILE_KIND.EMBED)
						end
					end
				end
			end
		end
	end

	-- TO-DO this might conflict with quiver's own embed file loader for require...?
	-- file-system loader
	-- embed loader
	-- lua loader, which will check for disk, .ZIP and embed, again.
	-- this should *probably* take the highest priority of all...?
	table.insert(package.loaders, function(path)
		local asset = self.locate[path .. ".lua"]

		if asset then
			if asset.kind == FILE_KIND.DISK then
				asset = quiver.file.get(asset.path)
			elseif asset.kind == FILE_KIND.PACK then
				asset = asset.path:get_file(path .. ".lua", false, true)
			elseif asset.kind == FILE_KIND.EMBED then
				asset = quiver.data.get_embed_file(asset.path)
			end

			print("returning...")
			return loadstring(asset, path)
		else
			return string.format("\n\tno file '\"%s\"' in system user-data", path)
		end
	end)
end

function system:list(search)
	local result = {}

	for path, _ in pairs(self.locate) do
		if string.start_with(path, search) then
			table.insert(result, path)
		end
	end

	return result
end

---Find an asset by name, to get the full path of the asset.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return string full_path # The "full" path to the asset.
function system:find(faux_path)
	return self.locate[faux_path]
end

---Re-load every asset in memory.
function system:load()
	for path, _ in pairs(self.memory_data.texture) do
		self:set_texture(path, true)
	end
	for path, _ in pairs(self.memory_data.model) do
		self:set_model(path, true)
	end
end

local function file_system_set_asset(self, memory_data, memory_list, call_new, call_new_memory, force, faux_path, ...)
	-- if asset was already in memory...
	if memory_data[faux_path] then
		if force then
			-- remove from the book-keeping memory table.
			table.remove_object(memory_list, faux_path)

			-- remove from the data-keeping memory table.
			memory_data[faux_path] = nil

			collectgarbage("collect")
		else
			return memory_data[faux_path]
		end
	end

	-- locate the asset.
	local asset = self.locate[faux_path]

	if asset.kind == FILE_KIND.DISK then
		print("Loading from disk...")

		-- create the asset.
		asset = call_new(asset.path, ...)
	elseif asset.kind == FILE_KIND.PACK then
		print("Loading from pack...")

		local data = asset.path:get_file(faux_path, true)

		-- create the asset.
		asset = call_new_memory(data, ...)
	elseif asset.kind == FILE_KIND.EMBED then
		print("Loading from embed...")

		local data = quiver.data.get_embed_file(asset.path, true)

		-- create the asset.
		asset = call_new_memory(data, ...)
	end

	-- insert into the book-keeping memory table.
	table.insert(memory_list, faux_path)

	-- insert into the data-keeping memory table.
	memory_data[faux_path] = asset

	return asset
end

---Get a Lua source file from the file-system table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return string asset # The asset.
function system:get_source(faux_path)
	return string.sub(self.locate[faux_path], 0.0, -5.0)
end

---Get a texture asset from the file-system model resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return texture asset # The asset.
function system:get_texture(faux_path)
	return self.memory_data.texture[faux_path]
end

---Set a texture asset into the file-system model resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return texture asset # The asset.
function system:set_texture(faux_path, force, ...)
	return file_system_set_asset(self, self.memory_data.texture, self.memory_list.texture, quiver.texture.new,
		quiver.texture.new_from_memory, force, faux_path, ...)
end

---Get a model asset from the file-system model resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return model asset # The asset.
function system:get_model(faux_path)
	return self.memory_data.model[faux_path]
end

---Set a model asset into the file-system model resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return model asset # The asset.
function system:set_model(faux_path, force, ...)
	return file_system_set_asset(self, self.memory_data.model, self.memory_list.model, quiver.model.new, nil, force,
		faux_path, ...)
end

---Get a model animation asset from the file-system model animation resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return model_animation asset # The asset.
function system:get_model_animation(faux_path)
	return self.memory_data.model_animation[faux_path]
end

---Set a model animation asset into the file-system model animation resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return model_animation asset # The asset.
function system:set_model_animation(faux_path, force, ...)
	return file_system_set_asset(self, self.memory_data.model_animation, self.memory_list.model_animation,
		quiver.model_animation.new, nil, force, faux_path, ...)
end

---Get a sound asset from the file-system sound resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return sound asset # The asset.
function system:get_sound(faux_path)
	return self.memory_data.sound[faux_path]
end

---Set a sound asset into the file-system sound resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return sound asset # The asset.
function system:set_sound(faux_path, force, ...)
	return file_system_set_asset(self, self.memory_data.sound, self.memory_list.sound, quiver.sound.new, quiver.sound
		.new_from_memory, force, faux_path, ...)
end

---Get a music asset from the file-system music resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return music asset # The asset.
function system:get_music(faux_path)
	return self.memory_data.music[faux_path]
end

---Set a music asset into the file-system music resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return music asset # The asset.
function system:set_music(faux_path, force, ...)
	return file_system_set_asset(self, self.memory_data.music, self.memory_list.music, quiver.music.new, quiver.music
		.new_from_memory, force, faux_path, ...)
end

---Get a model asset from the file-system model resource table.
---@param  faux_name string # The "faux" name to the asset.
---@return shader asset # The asset.
function system:get_shader(faux_name)
	return self.memory_data.shader[faux_name]
end

---Set a shader asset into the file-system model resource table.
---@param  faux_name    string # The "faux" name to the asset. It will be the key for storing the asset.
---@param  faux_path_vs string # The "faux" path to the ".vs" asset, not taking into consideration the search path in which it was found.
---@param  faux_path_fs string # The "faux" path to the ".fs" asset, not taking into consideration the search path in which it was found.
---@return shader asset # The asset.
function system:set_shader(faux_name, faux_path_vs, faux_path_fs, force)
	-- NOTE: storing a shader is slightly different from every other asset as it will
	-- normally take in more than one path (.vs and .fs). for that reason, a specific
	-- implementation just for the shader asset has to be made.

	-- if asset was already in memory...
	if self.memory_data.shader[faux_name] then
		if force then
			-- remove from the book-keeping memory table.
			table.remove_object(self.memory_list.shader, faux_name)

			-- remove from the data-keeping memory table.
			self.memory_data.shader[faux_name] = nil

			collectgarbage("collect")
		else
			return self.memory_data.shader[faux_name]
		end
	end

	-- locate the asset.
	local asset_vs = self.locate[faux_path_vs]
	local asset_fs = self.locate[faux_path_fs]

	if asset_vs.kind == FILE_KIND.DISK then
		asset_vs = quiver.file.get(asset_vs.path)
	elseif asset_vs.kind == FILE_KIND.PACK then
		asset_vs = asset_vs.path:get_file(faux_path_vs)
	elseif asset_vs.kind == FILE_KIND.EMBED then
		asset_vs = quiver.data.get_embed_file(asset_vs.path)
	end

	if asset_fs.kind == FILE_KIND.DISK then
		asset_fs = quiver.file.get(asset_fs.path)
	elseif asset_fs.kind == FILE_KIND.PACK then
		asset_fs = asset_fs.path:get_file(faux_path_vs)
	elseif asset_fs.kind == FILE_KIND.EMBED then
		asset_fs = quiver.data.get_embed_file(asset_fs.path)
	end

	-- create the asset.
	asset = quiver.shader.new_from_memory(asset_vs, asset_fs)

	-- insert into the book-keeping memory table.
	table.insert(self.memory_list.shader, faux_name)

	-- insert into the data-keeping memory table.
	self.memory_data.shader[faux_name] = asset

	return asset
end

---Get a model asset from the file-system model resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return font asset # The asset.
function system:get_font(faux_path)
	return self.memory_data.font[faux_path]
end

---Set a font asset into the file-system model resource table.
---@param  faux_path string # The "faux" path to the asset, not taking into consideration the search path in which it was found.
---@return font asset # The asset.
function system:set_font(faux_path, force, ...)
	return file_system_set_asset(self, self.memory_data.font, self.memory_list.font, quiver.font.new,
		quiver.font.new_from_memory,
		force, faux_path,
		...)
end
