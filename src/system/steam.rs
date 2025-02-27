/*
* Copyright (c) 2025 sockentrocken
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use crate::script::*;

//================================================================

use mlua::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.steam", "info": "The Steam API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let steam = lua.create_table()?;

    steam.set("new", lua.create_function(self::Steam::new)?)?;

    table.set("steam", steam)?;

    Ok(())
}

/* class
{ "version": "1.0.0", "name": "steam", "info": "An unique handle to a Steam client." }
*/
struct Steam {
    client: Client,
    single: SingleClient,
}

use std::sync::mpsc;
use steamworks::*;

impl mlua::UserData for Steam {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "steam:update",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("update", |_: &Lua, this, _: ()| {
            this.single.run_callbacks();
            Ok(())
        });

        //================================================================
        // utility.
        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_app_ID",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_app_ID", |_: &Lua, this, _: ()| {
            let utility = this.client.utils();

            Ok(utility.app_id().0)
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_IP_country",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_IP_country", |_: &Lua, this, _: ()| {
            let utility = this.client.utils();

            Ok(utility.ip_country())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_UI_language",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_UI_language", |_: &Lua, this, _: ()| {
            let utility = this.client.utils();

            Ok(utility.ui_language())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_server_time",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_server_time", |_: &Lua, this, _: ()| {
            let utility = this.client.utils();

            Ok(utility.get_server_real_time())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:set_overlay_position",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("set_overlay_position", |_: &Lua, this, kind: i32| {
            let utility = this.client.utils();

            match kind {
                0 => utility.set_overlay_notification_position(NotificationPosition::TopLeft),
                1 => utility.set_overlay_notification_position(NotificationPosition::TopRight),
                2 => utility.set_overlay_notification_position(NotificationPosition::BottomLeft),
                _ => utility.set_overlay_notification_position(NotificationPosition::BottomRight),
            }

            Ok(())
        });

        //================================================================
        // match-making.
        //================================================================

        //================================================================
        // networking.
        //================================================================

        //================================================================
        // app.
        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_app_install",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_app_install", |_: &Lua, this, app_id: u32| {
            let app = this.client.apps();

            Ok(app.is_app_installed(AppId(app_id)))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_DLC_install",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_DLC_install", |_: &Lua, this, app_id: u32| {
            let app = this.client.apps();

            Ok(app.is_dlc_installed(AppId(app_id)))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_app_subscribe",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_app_subscribe", |_: &Lua, this, app_id: u32| {
            let app = this.client.apps();

            Ok(app.is_subscribed_app(AppId(app_id)))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_VAC_ban",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_VAC_ban", |_: &Lua, this, _: ()| {
            let app = this.client.apps();

            Ok(app.is_vac_banned())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_cyber_cafe",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_cyber_cafe", |_: &Lua, this, _: ()| {
            let app = this.client.apps();

            Ok(app.is_cybercafe())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_low_violence",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_low_violence", |_: &Lua, this, _: ()| {
            let app = this.client.apps();

            Ok(app.is_low_violence())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_subscribe",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_subscribe", |_: &Lua, this, _: ()| {
            let app = this.client.apps();

            Ok(app.is_subscribed())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_app_build_ID",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_app_build_ID", |_: &Lua, this, _: ()| {
            let app = this.client.apps();

            Ok(app.app_build_id())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_app_install_directory",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_app_install_directory", |_: &Lua, this, app_id: u32| {
            let app = this.client.apps();

            Ok(app.app_install_dir(AppId(app_id)))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_app_owner",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_app_owner", |_: &Lua, this, _: ()| {
            let app = this.client.apps();

            Ok(app.app_owner().steamid32())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_game_language_list",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_game_language_list", |_: &Lua, this, _: ()| {
            let app = this.client.apps();

            Ok(app.available_game_languages())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_game_language",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_game_language", |_: &Lua, this, _: ()| {
            let app = this.client.apps();

            Ok(app.current_game_language())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_beta_name",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_beta_name", |_: &Lua, this, _: ()| {
            let app = this.client.apps();

            Ok(app.current_beta_name())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_launch_command_line",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_launch_command_line", |_: &Lua, this, _: ()| {
            let app = this.client.apps();

            Ok(app.launch_command_line())
        });

        //================================================================
        // friend.
        //================================================================

        //================================================================
        // input.
        //================================================================

        //================================================================
        // user.
        //================================================================

        //================================================================
        // user statistic data.
        //================================================================

        //================================================================
        // remote play.
        //================================================================

        //================================================================
        // remote storage.
        //================================================================

        //================================================================
        // UGC.
        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_achievement_data",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "get_achievement_data",
            |lua: &Lua, this, (name, key): (String, i32)| {
                let user_stats = this.client.user_stats();
                let achievement = user_stats.achievement(&name);

                match key {
                    0 => lua.to_value(
                        &achievement
                            .get_achievement_display_attribute("name")
                            .unwrap(),
                    ),
                    1 => lua.to_value(
                        &achievement
                            .get_achievement_display_attribute("desc")
                            .unwrap(),
                    ),
                    _ => lua.to_value(
                        &achievement
                            .get_achievement_display_attribute("hidden")
                            .unwrap(),
                    ),
                }
            },
        );
    }
}

impl Steam {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.steam.new",
        "info": "Create a new Steam client."
    }
    */
    fn new(_: &Lua, _: ()) -> mlua::Result<Self> {
        let (client, single) = Client::init_app(AppId(480)).unwrap();

        Ok(Self { client, single })
    }
}
