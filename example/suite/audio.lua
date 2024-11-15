local sound_volume = 1.0
local sound_pitch  = 1.0
local sound_pan    = 0.5
local music_volume = 1.0
local music_pitch  = 1.0
local music_pan    = 0.5

sound = Sound("example/suite/asset/sound.wav")
music = Music("example/suite/asset/music.wav")

function step_audio()
	local y = 0

	if interface_button(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Play Sound") then
		sound:play()
	end
	y = y + 1

	if interface_button(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Stop Sound") then
		sound:stop()
	end
	y = y + 1

	if interface_button(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Pause Sound") then
		sound:pause()
	end
	y = y + 1

	if interface_button(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Resume Sound") then
		sound:resume()
	end
	y = y + 1

	sound_volume = interface_slider(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "", "Sound Volume", sound_volume, 0.0, 1.0)
	y = y + 1

	sound_pitch = interface_slider(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "", "Sound Pitch", sound_pitch, 0.0, 1.0)
	y = y + 1

	sound_pan = interface_slider(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "", "Sound Pan", sound_pan, 0.0, 1.0)
	y = y + 1

	if interface_button(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Apply") then
		sound:volume(sound_volume)
		sound:pitch(sound_pitch)
		sound:pan(sound_pan)
	end
	y = y + 1

	y = y + 1

	music:update()

	if interface_button(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Play Music") then
		music:play()
	end
	y = y + 1

	if interface_button(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Stop Music") then
		music:stop()
	end
	y = y + 1

	if interface_button(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Pause Music") then
		music:pause()
	end
	y = y + 1

	if interface_button(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Resume Music") then
		music:resume()
	end
	y = y + 1

	music_volume = interface_slider(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "", "Music Volume", music_volume, 0.0, 1.0)
	y = y + 1

	music_pitch = interface_slider(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "", "Music Pitch", music_pitch, 0.0, 2.0)
	y = y + 1

	music_pan = interface_slider(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "", "Music Pan", music_pan, 0.0, 1.0)
	y = y + 1

	if interface_button(box_2:new(vector_2:new(8.0, 32.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Apply") then
		music:volume(music_volume)
		music:pitch(music_pitch)
		music:pan(music_pan)
	end
end