-- Create a table with no data in it.
local data = {}

-- Insert some dummy data in it.
for x = 1, 63 do
	table.insert(data, 0)
end

-- Insert some data of significance at the last index.
table.insert(data, 1)

-- Check that the last value is 1.
assert(data[64] == 1)

-- Get a data buffer user-data from Quiver.
local data = quiver.data.new(data)

-- Compress the data.
local data = quiver.data.compress(data)

-- Decompress the data.
local data = quiver.data.decompress(data)

-- Get the data back as a Lua table.
local data = data:get_buffer()

-- Check that the last value is 1.
assert(data[64] == 1)
