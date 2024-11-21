--[[
-- Welcome to Quiver!
* info.lua -> info entry-point file.
* main.lua -> main entry-point file. You can change the name of this file in info.lua.
* meta.lua -> Quiver API documentation, for use with LuaLS.
* base.lua -> Quiver's standard Lua library.
]]--

require "{path}/base"

--- Main entry-point. Quiver will call this function first initializing the Lua virtual machine.
function main()
    push_logger("Main entry-point.")

    -- Push every "standard" console method. These will be accessible from the console by typing their name.
    push_parser("engine_load", "Load the engine.", engine_load)
    push_parser("engine_exit", "Exit the engine.", engine_exit)
    push_parser("wipe_logger", "Wipe the debug logger text.", wipe_logger)
end

--- Step entry-point. Quiver will call this function every single frame.
function step()
    -- Toggle the debug window on F1 press.
    if get_board_press(INPUT_BOARD.KEY_F1) then
        -- Get debug window state.
        local debug_state = get_debug_state()

        -- Set state.
        set_debug_state(not debug_state)
    end

    local x = math.sin(get_time())
    local z = math.cos(get_time())

    -- Begin the 3D draw mode.
    begin_mode_3d(camera_3d:new(vector_3:new(x * 4.0, 4.0, z * 4.0), vector_3:zero(), vector_3:new(0.0, 1.0, 0.0), 90.0))

        -- Draw a grid.
        draw_grid(64.0, 1.0)

        -- Draw a cube.
        draw_cube(vector_3:zero(), vector_3:one(), color:new(1.0, 0.0, 0.0, 1.0))

    -- Close the 3D draw mode.
    close_mode_3d()

    -- Begin the 2D draw mode.
    begin_mode_2d(camera_2d:new(vector_2:zero(), vector_2:zero(), 0.0, 1.0))

        -- Draw text.
        draw_text("Press [F1] to toggle the debug window.", vector_2:new(16.0, 16.0), 32.0, color:new(1.0, 0.0, 0.0, 1.0))

    -- Close the 2D draw mode.
    close_mode_2d()
end

--- Exit entry-point. Quiver will call this function when the engine is set to close (engine_exit()), or when the Lua virtual machine is reset (engine_load()).
function exit()
    push_logger("Exit entry-point.")
end