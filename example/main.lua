require "example/base"
require "example/suite/video"
require "example/suite/audio"
require "example/suite/input"
require "example/suite/debug"
require "example/suite/interface"
require "example/suite/steam"
require "example/suite/discord"
require "example/suite/request"
require "example/suite/parry"
require "example/suite/rapier"

---@enum example_suite
EXAMPLE_SUITE = {
    NONE = 0,
    VIDEO = 1,
    AUDIO = 2,
    INPUT = 3,
    DEBUG = 4,
    INTERFACE = 5,
    STEAM = 6,
    DISCORD = 7,
    REQUEST = 8,
    PARRY = 9,
    RAPIER = 10,
}

current_suite = EXAMPLE_SUITE.NONE

function main()
    --discord = Discord()
    --steam = Steam()
end

function step()
    if get_board_press(INPUT_BOARD.KEY_F1) then
        local debug_state = get_debug_state()
        set_debug_state(not debug_state)
    end

    if get_board_press(INPUT_BOARD.KEY_F2) then
        engine_load()
    end

    local x = math.sin(get_time())
    local z = math.cos(get_time())

    begin_mode_3d(camera_3d:new(vector_3:new(x * 4.0, 4.0, z * 4.0), vector_3:zero(), vector_3:new(0.0, 1.0, 0.0), 90.0))

        draw_grid(8.0, 1.0)

        draw_cube(vector_3:zero(), vector_3:one(), color:new(0.0, 0.0, 1.0, 1.0))

    close_mode_3d()

    local shape = get_window_shape()
    local scale = math.ceil(shape.y / 384.0)

    begin_mode_2d(camera_2d:new(vector_2:zero(), vector_2:zero(), 0.0, scale))

    set_mouse_scale(vector_2:new(1.0 / scale, 1.0 / scale))

    if current_suite == EXAMPLE_SUITE.NONE then
        local y = 0

        draw_text("Welcome to Quiver! Select an example suite below.", vector_2:new(8.0, 8.0), 8.0, color:new(1.0, 0.0, 0.0, 1.0))

        if interface_button(box_2:new(vector_2:new(8.0, 20.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Video Suite")     then current_suite = EXAMPLE_SUITE.VIDEO     end y = y + 1
        if interface_button(box_2:new(vector_2:new(8.0, 20.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Audio Suite")     then current_suite = EXAMPLE_SUITE.AUDIO     end y = y + 1
        if interface_button(box_2:new(vector_2:new(8.0, 20.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Input Suite")     then current_suite = EXAMPLE_SUITE.INPUT     end y = y + 1
        if interface_button(box_2:new(vector_2:new(8.0, 20.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Debug Suite")     then current_suite = EXAMPLE_SUITE.DEBUG     end y = y + 1
        if interface_button(box_2:new(vector_2:new(8.0, 20.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Interface Suite") then current_suite = EXAMPLE_SUITE.INTERFACE end y = y + 1
        if interface_button(box_2:new(vector_2:new(8.0, 20.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Steam Suite")     then current_suite = EXAMPLE_SUITE.STEAM end y = y + 1
        if interface_button(box_2:new(vector_2:new(8.0, 20.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Discord Suite")   then current_suite = EXAMPLE_SUITE.DISCORD end y = y + 1
        if interface_button(box_2:new(vector_2:new(8.0, 20.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Request Suite")   then current_suite = EXAMPLE_SUITE.INTERFACE end y = y + 1
        if interface_button(box_2:new(vector_2:new(8.0, 20.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Parry Suite")     then current_suite = EXAMPLE_SUITE.INTERFACE end y = y + 1
        if interface_button(box_2:new(vector_2:new(8.0, 20.0 + 18.0 * y), vector_2:new(96.0, 16.0)), "Rapier Suite")    then current_suite = EXAMPLE_SUITE.INTERFACE end y = y + 1

        draw_text("Asset data from FreeDoom and LibreQuake.", vector_2:new(8.0, (shape.y / scale) - 28.0), 8.0, color:new(1.0, 0.0, 0.0, 1.0))
        draw_text("Press [ESCAPE] to exit.", vector_2:new(8.0, (shape.y / scale) - 16.0), 8.0, color:new(1.0, 0.0, 0.0, 1.0))
    elseif current_suite == EXAMPLE_SUITE.VIDEO then
        step_video()
    elseif current_suite == EXAMPLE_SUITE.AUDIO then
        step_audio()
    elseif current_suite == EXAMPLE_SUITE.INPUT then
        step_input()
    elseif current_suite == EXAMPLE_SUITE.DEBUG then
        step_debug()
    elseif current_suite == EXAMPLE_SUITE.INTERFACE then
        step_interface()
    elseif current_suite == EXAMPLE_SUITE.STEAM then
        step_steam()
    elseif current_suite == EXAMPLE_SUITE.DISCORD then
        --step_discord()
    elseif current_suite == EXAMPLE_SUITE.REQUEST then
        --step_request()
    elseif current_suite == EXAMPLE_SUITE.PARRY then
        --step_parry()
    elseif current_suite == EXAMPLE_SUITE.RAPIER then
        --step_rapier()
    end

    if not (current_suite == EXAMPLE_SUITE.NONE) then
        if interface_button(box_2:new(vector_2:new(8.0, 8.0), vector_2:new(96.0, 16.0)), "Back") then current_suite = EXAMPLE_SUITE.NONE end
    end

    close_mode_2d()
end

function exit()
end