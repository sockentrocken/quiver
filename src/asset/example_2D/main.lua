--- Welcome to Quiver!
--- * main.lua -> Main entry-point file.
--- * meta.lua -> Quiver API documentation, for use with LuaLS.
--- * base/ -> Quiver's standard Lua library.

--[[----------------------------------------------------------------]]

require "base/main"

local time = 0.0

function quiver.main()
    --- Main entry-point. Quiver will call this on project initialization.
    while not quiver.window.get_close() do
        time = time + quiver.general.get_frame_time()

        -- Press F1 to reload Quiver.
        if quiver.input.board.get_press(INPUT_BOARD.F1) then
            -- Returning "true" will reload Quiver.
            return true
        end

        -- Initialize drawing.
        quiver.draw.begin(draw)
    end

    -- Returning "false" will exit Quiver.
    return false
end

--[[----------------------------------------------------------------]]

function draw()
    quiver.draw.clear(color:new(255.0, 255.0, 255.0, 255.0))

    local x = math.sin(time)
    local z = math.cos(time)

    -- Begin the 3D draw mode.
    quiver.draw_3d.begin(draw_3d, camera_3d:new(vector_3:new(x * 4.0, 4.0, z * 4.0), vector_3:zero(),
        vector_3:new(0.0, 1.0, 0.0), 90.0, 0.0))

    -- Begin the 2D draw mode.
    quiver.draw_2d.begin(draw_2d, camera_2d:new(vector_2:zero(), vector_2:zero(), 0.0, 1.0))
end

--[[----------------------------------------------------------------]]

function draw_3d()
    -- Draw a grid.
    quiver.draw_3d.draw_grid(64.0, 1.0)

    -- Draw a cube.
    quiver.draw_3d.draw_cube(vector_3:zero(), vector_3:one(), color:new(255.0, 0.0, 0.0, 255.0))
end

--[[----------------------------------------------------------------]]

function draw_2d()
    -- Draw text.
    quiver.draw_2d.draw_text("Hello, world!", vector_2:new(16.0, 16.0), 32.0, color:new(255.0, 0.0, 0.0, 255.0))
end

--[[----------------------------------------------------------------]]

-- Uncomment this to use a custom crash handler.
--[[
function quiver.fail(message)
    --- Fail entry-point. Quiver will call this on a script error, with the script error message as the argument. Note that this function is OPTIONAL, and Quiver will use a default crash handler if missing.
    while not quiver.window.get_close() do
        -- Initialize drawing.
        quiver.draw.begin()

        -- Clear the screen.
        quiver.draw.clear(color:new(1.0, 1.0, 1.0, 1.0))

            -- Begin the 2D draw mode.
            quiver.draw_2d.begin(camera_2d:new(vector_2:zero(), vector_2:zero(), 0.0, 1.0))

                -- Draw text.
                quiver.draw_2d.draw_text(message, vector_2:new(16.0, 16.0), 32.0, color:new(1.0, 0.0, 0.0, 1.0))

            -- Close the 2D draw mode.
            quiver.draw_2d.close()

        -- Finalize drawing.
        quiver.draw.close()
    end
end
]]
