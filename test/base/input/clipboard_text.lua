quiver.input.set_clipboard_text("hello, world!")
local text = quiver.input.get_clipboard_text()
assert(text == "hello, world!")
