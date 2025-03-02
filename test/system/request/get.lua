local link = "https://raw.githubusercontent.com/sockentrocken/quiver/refs/heads/main/test/asset/sample.txt"

-- Get the data from the link. As we know it's not binary, we pass false to the function.
local response = quiver.request.get(link, false)

assert(response == "Hello, world!")
