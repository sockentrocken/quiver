--- Welcome to Quiver!
--- * main.lua -> Main entry-point file.
--- * base.lua -> Quiver's standard Lua library.
--- * meta.lua -> Quiver API documentation, for use with LuaLS.

--[[----------------------------------------------------------------]]--

require "base"

function quiver.main()
    --- Main entry-point. Quiver will call this on module initialization.
end

function quiver.step()
    --- Step entry-point. Quiver will call this every frame.
    local x = math.sin(quiver.general.get_time())
    local z = math.cos(quiver.general.get_time())

    -- Begin the 3D draw mode.
    quiver.draw_3d.begin(camera_3d:new(vector_3:new(x * 4.0, 4.0, z * 4.0), vector_3:zero(), vector_3:new(0.0, 1.0, 0.0), 90.0))

        -- Draw a grid.
        quiver.draw_3d.draw_grid(64.0, 1.0)

        -- Draw a cube.
        quiver.draw_3d.draw_cube(vector_3:zero(), vector_3:one(), color:new(1.0, 0.0, 0.0, 1.0))

    -- Close the 3D draw mode.
    quiver.draw_3d.close()

    -- Begin the 2D draw mode.
    quiver.draw_2d.begin(camera_2d:new(vector_2:zero(), vector_2:zero(), 0.0, 1.0))

        -- Draw text.
        quiver.draw_2d.draw_text("Hello, world!", vector_2:new(16.0, 16.0), 32.0, color:new(1.0, 0.0, 0.0, 1.0))

    -- Close the 2D draw mode.
    quiver.draw_2d.close()
end

function quiver.exit()
    --- Exit entry-point. Quiver will call this on module de-initialization.
end