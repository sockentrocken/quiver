-- Write "123" to the file "foo.txt". Since we don't want to write binary, pass false as the third argument.
quiver.file.set("work/foo.txt", "123", false)

-- Read the data back. Again, since we know the file isn't binary, we pass false.
local data = quiver.file.get("work/foo.txt", false)

assert(data == "123")
