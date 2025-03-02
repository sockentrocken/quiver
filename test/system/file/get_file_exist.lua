-- Write "123" to the file "foo.txt". Since we don't want to write binary, pass false as the third argument.
quiver.file.set("work/foo.txt", "123", false)

assert(quiver.file.get_file_exist("work/foo.txt"))
