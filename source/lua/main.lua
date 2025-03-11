--- Welcome to Quiver!
--- * main.lua -> Main entry-point file.
--- * meta.lua -> Quiver API documentation, for use with LuaLS.
--- * base/ -> Quiver's standard Lua library.

--[[----------------------------------------------------------------]]

-- Note that you can also call quiver.general.load_base() to load the standard Lua library.
require "base/main"

local time = 0.0

-- Info entry point. Quiver will call this on window initialization. Note that this function is OPTIONAL, and Quiver will use a default info manifest if missing.
function quiver.info()
    -- Every entry in this table is completely optional.
    return {
        -- Window name.
        name       = "Quiver",
        -- Window icon. If nil, will use Quiver's logo. If empty (not-nil, empty string), will not set any icon. If not empty, will load an icon with that path.
        icon       = nil,
        -- Window size.
        size       = { 1024, 768 },
        -- Window frame-rate.
        rate       = 60,
        -- Window mode.
        head       = true,
        -- Vertical sync.
        sync       = false,
        -- Full-screen.
        full       = false,
        -- Border-less.
        no_border  = false,
        -- Window decoration.
        no_decor   = false,
        -- Window focus.
        no_focus   = false,
        -- Window resizability.
        resizable  = false,
        -- Initiate window as hidden.
        hidden     = false,
        -- Initiate window as minimized.
        minimize   = false,
        -- Initiate window as maximized.
        maximize   = false,
        -- Always show window on top.
        always_top = false,
        -- Always run window, even when minimized.
        always_run = false,
        -- Allow window transparency.
        alpha      = false,
        -- Allow high DPI window.
        scale      = false,
        -- Allow MSAA.
        msaa       = false,
        -- Allow mouse pass-through.
        mouse_pass = false,
        -- Allow video interlace for V3D.
        interlace  = false,
    }
end

--[[----------------------------------------------------------------]]

-- Main entry-point. Quiver will call this on project initialization.
function quiver.main()
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
--- Fail entry-point. Quiver will call this on a script error, with the script error message as the argument. Note that this function is OPTIONAL, and Quiver will use a default crash handler if missing.
function quiver.fail(message)
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
