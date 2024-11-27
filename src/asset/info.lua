return {
    system = {
        -- Enable the model system.
        model = {model},
        -- Enable the texture system.
        texture = {texture},
        -- Enable the image system.
        image = {image},
        -- Enable the sound system.
        sound = {sound},
        -- Enable the music system.
        music = {music},
        -- Enable the font system.
        font = {font},
        -- Enable the shader system.
        shader = {shader},
    },
    window = {
        -- Set full-screen mode.
        fullscreen = {fullscreen},
        -- Set border-less mode.
        borderless = {borderless},
        -- Allow vertical-sync.
        sync = {sync},
        -- Allow MSAA.
        msaa = {msaa},
        -- Allow resizing of window.
        resize = {resize},
        -- Appear hidden.
        hidden = {hidden},
        -- Appear minimized.
        minimize = {minimize},
        -- Appear maximized.
        maximize = {maximize},
        -- Appear without decoration.
        no_decor = {no_decor},
        -- Appear without focus.
        no_focus = {no_focus},
        -- Always render on front.
        on_front = {on_front},
        -- Allow rendering the window even when hidden.
        run_hidden = {run_hidden},
        -- Allow mouse pass-through.
        mouse_pass = {mouse_pass},
        -- Window transparency.
        draw_alpha = {draw_alpha},
        -- High DPI scale.
        high_scale = {high_scale},
        -- Window name.
        name = "{window_name}",
        -- Frame-rate.
        rate = {window_rate},
        -- OPTIONAL: A table of every icon to use for the window.
        --icon = {},
        -- Window size.
        shape = {1024, 768},
        -- Window transparency.
        alpha = 1.0
    }
}