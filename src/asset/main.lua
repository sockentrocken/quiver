--- Welcome to Quiver!
--- * main.lua -> Main entry-point file.
--- * meta.lua -> Quiver API documentation, for use with LuaLS.
--- * base.lua -> Quiver's standard Lua library.

--[[----------------------------------------------------------------]]--

require "{path}/base"

--- Main entry-point. Quiver will call this function first initializing the Lua virtual machine.
function main()
end

--- Step entry-point. Quiver will call this function every single frame.
function step()
    local x = math.sin(quiver.engine.get_time())
    local z = math.cos(quiver.engine.get_time())

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
        quiver.draw_2d.draw_text("Hello, world!.", vector_2:new(16.0, 16.0), 32.0, color:new(1.0, 0.0, 0.0, 1.0))

    -- Close the 2D draw mode.
    quiver.draw_2d.close()
end

--- Exit entry-point. Quiver will call this function when the engine is set to close (quiver.engine.closure()), or when the Lua virtual machine is reset (quiver.engine.restart()).
function exit()
end