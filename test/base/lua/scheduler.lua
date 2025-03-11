-- Create a scheduler.
local i = scheduler:new()

-- Insert a new function by a given name. Name is optional.
i:insert(function()
    local time = quiver.general.get_time()

    print("instance " .. time)

    local data = quiver.request.get("http://httpbin.org/ip")

    print("instance " .. time .. " done.")
end, "routine_1")

-- Resume scheduler. This would typically be done in your main game loop.
i:resume()

-- Check if the co-routine is still alive.
if i.routine["routine_1"] then
    quiver.draw_2d.draw_text("Running co-routine...", vector_2:old(8.0, 128.0), 32.0, color:white())
end
