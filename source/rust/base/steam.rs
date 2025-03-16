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
use crate::status::*;

//================================================================

use mlua::prelude::*;
use std::num::ParseIntError;

//================================================================

/* class
{ "version": "1.0.0", "feature": "steam", "name": "quiver.steam", "info": "The Steam API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
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
    utility: Utils<ClientManager>,
    app: Apps<ClientManager>,
    friend: Friends<ClientManager>,
    user: User<ClientManager>,
    user_statistic: UserStats<ClientManager>,
    remote_play: RemotePlay<ClientManager>,
    remote_storage: RemoteStorage<ClientManager>,
    network_message: NetworkingMessages<ClientManager>,
}

unsafe impl Send for Steam {}

use networking_messages::NetworkingMessages;
use networking_types::{NetworkingIdentity, SendFlags};
use steamworks::*;

impl mlua::UserData for Steam {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        // TO-DO add every other call-back.

        field.add_field_method_set(
            "call_back_overlay_change",
            |_, this, function: mlua::Function| {
                this.client.register_callback(
                    move |call: GameOverlayActivated| {
                        if function.call::<()>(call.active).is_err() {}
                    },
                );
                Ok(())
            },
        );

        field.add_field_method_set(
            "call_back_persona_change",
            |_, this, function: mlua::Function| {
                this.client
                    .register_callback(move |call: PersonaStateChange| {
                        if function
                            .call::<()>((call.steam_id.raw(), call.flags.bits()))
                            .is_err()
                        {}
                    });
                Ok(())
            },
        );
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "steam:update",
            "info": "Update the Steam state."
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
            "info": "Get the current AppID.",
            "result": [
                { "name": "ID", "info": "The current AppID.", "kind": "number" }
            ]
        }
        */
        method.add_method_mut("get_app_ID", |_: &Lua, this, _: ()| {
            Ok(this.utility.app_id().0)
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_IP_country",
            "info": "Get the current country code from the user's IP.",
            "result": [
                { "name": "code", "info": "The current country code.", "kind": "string" }
            ]
        }
        */
        method.add_method_mut("get_IP_country", |_: &Lua, this, _: ()| {
            Ok(this.utility.ip_country())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_UI_language",
            "info": "Get the current language for the Steam client.",
            "result": [
                { "name": "language", "info": "The current language.", "kind": "string" }
            ]
        }
        */
        method.add_method_mut("get_UI_language", |_: &Lua, this, _: ()| {
            Ok(this.utility.ui_language())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_server_time",
            "info": "Get the current Steam server time, as an UNIX time-stamp.",
            "result": [
                { "name": "time", "info": "The current server time.", "kind": "number" }
            ]
        }
        */
        method.add_method_mut("get_server_time", |_: &Lua, this, _: ()| {
            Ok(this.utility.get_server_real_time())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:set_overlay_position",
            "info": "Set the position of the Steam overlay.",
            "member": [
                { "name": "position", "info": "The Steam overlay position.", "kind": "steam_overlay_position" }
            ]
        }
        */
        method.add_method_mut("set_overlay_position", |_: &Lua, this, position: i32| {
            match position {
                0 => this
                    .utility
                    .set_overlay_notification_position(NotificationPosition::TopLeft),
                1 => this
                    .utility
                    .set_overlay_notification_position(NotificationPosition::TopRight),
                2 => this
                    .utility
                    .set_overlay_notification_position(NotificationPosition::BottomLeft),
                _ => this
                    .utility
                    .set_overlay_notification_position(NotificationPosition::BottomRight),
            }

            Ok(())
        });

        //================================================================
        // match-making.
        //================================================================

        //================================================================
        // networking. (NetworkingMessages)
        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:set_message",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("set_message", |_: &Lua, this, _: ()| {
            for friend in this.friend.get_friends(FriendFlags::IMMEDIATE) {
                let identity = NetworkingIdentity::new_steam_id(friend.id());

                let data = [0, 1, 2, 3, 4, 5, 6, 7];

                let _ = this.network_message.send_message_to_user(
                    identity,
                    SendFlags::UNRELIABLE_NO_DELAY,
                    &data,
                    0,
                );
            }

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_message",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_message", |_: &Lua, this, _: ()| {
            for message in this.network_message.receive_messages_on_channel(0, 100) {
                let peer = message.identity_peer();
                let data = message.data();

                println!("{peer:?} : {data:?}");
            }

            Ok(())
        });

        //================================================================
        // app.
        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_app_install",
            "info": "Check if a given AppID is currently on the system.",
            "member": [
                { "name": "ID", "info": "The AppID.", "kind": "number" }
            ],
            "result": [
                { "name": "install", "info": "The state of the AppID.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut("get_app_install", |_: &Lua, this, app_id: u32| {
            Ok(this.app.is_app_installed(AppId(app_id)))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_DLC_install",
            "info": "Check if a given DLC is currently on the system.",
            "member": [
                { "name": "ID", "info": "The AppID.", "kind": "number" }
            ],
            "result": [
                { "name": "install", "info": "The state of the AppID.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut("get_DLC_install", |_: &Lua, this, app_id: u32| {
            Ok(this.app.is_dlc_installed(AppId(app_id)))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_app_subscribe",
            "info": "Check if the user has a subscription to the given AppID.",
            "member": [
                { "name": "ID", "info": "The AppID.", "kind": "number" }
            ],
            "result": [
                { "name": "subscription", "info": "Subscription state.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut("get_app_subscribe", |_: &Lua, this, app_id: u32| {
            Ok(this.app.is_subscribed_app(AppId(app_id)))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_VAC_ban",
            "info": "Check if the user has a VAC ban on record.",
            "result": [
                { "name": "ban", "info": "The VAC ban state.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut("get_VAC_ban", |_: &Lua, this, _: ()| {
            Ok(this.app.is_vac_banned())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_cyber_cafe",
            "info": "Check if the user has a VAC ban on record.",
            "result": [
                { "name": "ban", "info": "The VAC ban state.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut("get_cyber_cafe", |_: &Lua, this, _: ()| {
            Ok(this.app.is_cybercafe())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_low_violence",
            "info": "Check if the current AppID has support for low violence.",
            "result": [
                { "name": "low_violence", "info": "Low violence state.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut("get_low_violence", |_: &Lua, this, _: ()| {
            Ok(this.app.is_low_violence())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_subscribe",
            "info": "Check if the user has a subscription to the current AppID.",
            "result": [
                { "name": "subscription", "info": "Subscription state.", "kind": "boolean" }
            ]
        }
        */
        method.add_method_mut("get_subscribe", |_: &Lua, this, _: ()| {
            Ok(this.app.is_subscribed())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_app_build_ID",
            "info": "Get the build ID for the current AppID.",
            "result": [
                { "name": "ID", "info": "The build ID.", "kind": "number" }
            ]
        }
        */
        method.add_method_mut("get_app_build_ID", |_: &Lua, this, _: ()| {
            Ok(this.app.app_build_id())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_app_install_path",
            "info": "Get the installation path for the given AppID. NOTE: this will work even if the app is not in disk.",
            "member": [
                { "name": "ID", "info": "The AppID.", "kind": "number" }
            ],
            "result": [
                { "name": "path", "info": "Installation path.", "kind": "string" }
            ]
        }
        */
        method.add_method_mut("get_app_install_path", |_: &Lua, this, app_id: u32| {
            Ok(this.app.app_install_dir(AppId(app_id)))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_app_owner",
            "info": "Get the SteamID of the owner's current AppID.",
            "result": [
                { "name": "ID", "info": "The SteamID.", "kind": "string" }
            ]
        }
        */
        method.add_method_mut("get_app_owner", |_: &Lua, this, _: ()| {
            Ok(this.app.app_owner().raw().to_string())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_game_language_list",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_game_language_list", |_: &Lua, this, _: ()| {
            Ok(this.app.available_game_languages())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_game_language",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_game_language", |_: &Lua, this, _: ()| {
            Ok(this.app.current_game_language())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_beta_name",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_beta_name", |_: &Lua, this, _: ()| {
            Ok(this.app.current_beta_name())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_launch_command_line",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_launch_command_line", |_: &Lua, this, _: ()| {
            Ok(this.app.launch_command_line())
        });

        //================================================================
        // friend.
        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_name",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_name", |_: &Lua, this, _: ()| Ok(this.friend.name()));

        // get_friends, get_coplay_friends, get_friend, request_user_rmation

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:activate_overlay",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("activate_overlay", |_: &Lua, this, dialog: String| {
            this.friend.activate_game_overlay(&dialog);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:activate_overlay_link",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("activate_overlay_link", |_: &Lua, this, link: String| {
            this.friend.activate_game_overlay_to_web_page(&link);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:activate_overlay_store",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "activate_overlay_store",
            |_: &Lua, this, (id, kind): (u32, i32)| {
                let flag = match kind {
                    1 => OverlayToStoreFlag::AddToCart,
                    2 => OverlayToStoreFlag::AddToCartAndShow,
                    _ => OverlayToStoreFlag::None,
                };

                this.friend.activate_game_overlay_to_store(AppId(id), flag);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:activate_overlay_user",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "activate_overlay_user",
            |_: &Lua, this, (dialog, id): (String, u64)| {
                this.friend
                    .activate_game_overlay_to_user(&dialog, SteamId::from_raw(id));
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:activate_invite_dialog",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("activate_invite_dialog", |_: &Lua, this, id: u64| {
            this.friend.activate_invite_dialog(LobbyId::from_raw(id));
            Ok(())
        });

        // set_rich_presence, clear_rich_presence, get_user_restrictions

        //================================================================
        // input.
        //================================================================

        //================================================================
        // user.
        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_steam_ID",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_steam_ID", |_: &Lua, this, _: ()| {
            Ok(this.user.steam_id().raw().to_string())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_level",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_level", |_: &Lua, this, _: ()| Ok(this.user.level()));

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_log_on",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_log_on", |_: &Lua, this, _: ()| {
            Ok(this.user.logged_on())
        });

        //================================================================
        // user statistic data.
        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_leader_board",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "get_leader_board",
            |_: &Lua, this, (name, function): (String, mlua::Function)| {
                this.user_statistic.find_leaderboard(&name, move |call| {
                    if let Ok(Some(call)) = call {
                        if function.call::<()>(call.raw().to_string()).is_err() {}
                    }
                });

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_or_create_leader_board",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "get_or_create_leader_board",
            |_: &Lua, this, (name, sort_ascend, show_kind, function): (String, bool, i32, mlua::Function)| {
                let sort_kind = {
                    if sort_ascend {
                        LeaderboardSortMethod::Ascending
                    } else {
                        LeaderboardSortMethod::Descending
                    }
                };
                let show_kind = match show_kind {
                    0 => LeaderboardDisplayType::Numeric,
                    1 => LeaderboardDisplayType::TimeSeconds,
                    _ => LeaderboardDisplayType::TimeMilliSeconds,
                };

                this.user_statistic.find_or_create_leaderboard(&name, sort_kind, show_kind, move |call| {
                    if let Ok(Some(call)) = call {
                        if function.call::<()>(call.raw().to_string()).is_err() {}
                    }
                });

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:upload_leader_board_score",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "upload_leader_board_score",
            |_: &Lua,
             this,
             (name, keep_best, score, additional, function): (
                String,
                bool,
                i32,
                Vec<i32>,
                mlua::Function,
            )| {
                let c = this.client.clone();
                let method = {
                    if keep_best {
                        UploadScoreMethod::KeepBest
                    } else {
                        UploadScoreMethod::ForceUpdate
                    }
                };

                this.user_statistic.find_leaderboard(&name, move |call| {
                    if let Ok(Some(call)) = call {
                        let u_s = c.user_stats();

                        u_s.upload_leaderboard_score(
                            &call,
                            method,
                            score,
                            &additional,
                            move |call| {
                                if let Ok(Some(call)) = call {
                                    if function
                                        .call::<()>((
                                            call.score,
                                            call.was_changed,
                                            call.global_rank_new,
                                            call.global_rank_previous,
                                        ))
                                        .is_err()
                                    {}
                                }
                            },
                        );
                    }
                });

                Ok(())
            },
        );

        // TO-DO download_leaderboard_entries

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_leader_board_show_kind",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "get_leader_board_show_kind",
            |_: &Lua, this, (name, function): (String, mlua::Function)| {
                let c = this.client.clone();

                this.user_statistic.find_leaderboard(&name, move |call| {
                    let u_s = c.user_stats();

                    if let Ok(Some(call)) = call {
                        if let Some(call) = u_s.get_leaderboard_display_type(&call) {
                            let kind = match call {
                                LeaderboardDisplayType::Numeric => 0,
                                LeaderboardDisplayType::TimeSeconds => 1,
                                LeaderboardDisplayType::TimeMilliSeconds => 2,
                            };

                            if function.call::<()>(kind).is_err() {};
                        }
                    }
                });

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_leader_board_sort_kind",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "get_leader_board_sort_kind",
            |_: &Lua, this, (name, function): (String, mlua::Function)| {
                let c = this.client.clone();

                this.user_statistic.find_leaderboard(&name, move |call| {
                    let u_s = c.user_stats();

                    if let Ok(Some(call)) = call {
                        if let Some(call) = u_s.get_leaderboard_sort_method(&call) {
                            let kind = match call {
                                LeaderboardSortMethod::Ascending => true,
                                LeaderboardSortMethod::Descending => false,
                            };

                            if function.call::<()>(kind).is_err() {};
                        }
                    }
                });

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_leader_board_name",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "get_leader_board_name",
            |_: &Lua, this, (name, function): (String, mlua::Function)| {
                let c = this.client.clone();

                this.user_statistic.find_leaderboard(&name, move |call| {
                    let u_s = c.user_stats();

                    if let Ok(Some(call)) = call {
                        if function
                            .call::<()>(u_s.get_leaderboard_name(&call))
                            .is_err()
                        {};
                    }
                });

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_leader_board_entry_count",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "get_leader_board_entry_count",
            |_: &Lua, this, (name, function): (String, mlua::Function)| {
                let c = this.client.clone();

                this.user_statistic.find_leaderboard(&name, move |call| {
                    let u_s = c.user_stats();

                    if let Ok(Some(call)) = call {
                        if function
                            .call::<()>(u_s.get_leaderboard_entry_count(&call))
                            .is_err()
                        {};
                    }
                });

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:pull_user_statistic",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("pull_user_statistic", |_: &Lua, this, _: ()| {
            this.user_statistic.request_current_stats();

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:push_user_statistic",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("push_user_statistic", |_: &Lua, this, _: ()| {
            if this.user_statistic.store_stats().is_ok() {};

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:reset_user_statistic",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "reset_user_statistic",
            |_: &Lua, this, reset_achievement: bool| {
                if this
                    .user_statistic
                    .reset_all_stats(reset_achievement)
                    .is_ok()
                {};

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_user_statistic",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "get_user_statistic",
            |_: &Lua, this, (name, kind): (String, bool)| {
                if kind {
                    if let Ok(result) = this.user_statistic.get_stat_i32(&name) {
                        return Ok(mlua::Value::Integer(result as i64));
                    }
                } else if let Ok(result) = this.user_statistic.get_stat_f32(&name) {
                    return Ok(mlua::Value::Number(result as f64));
                }

                Ok(mlua::Value::Nil)
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:set_user_statistic",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "set_user_statistic",
            |_: &Lua, this, (name, kind, value): (String, bool, f64)| {
                if kind {
                    if let Ok(result) = this.user_statistic.set_stat_i32(&name, value as i32) {
                        return Ok(result);
                    }
                } else if let Ok(result) = this.user_statistic.set_stat_f32(&name, value as f32) {
                    return Ok(result);
                }

                Err(mlua::Error::runtime(
                    "Steam::set_user_statistic(): Error setting user statistic.",
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_achievement",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_achievement", |_: &Lua, this, name: String| {
            let achievement = this.user_statistic.achievement(&name);

            if let Ok(result) = achievement.get() {
                Ok(result)
            } else {
                Err(mlua::Error::runtime(
                    "Steam::get_achievement(): Error getting achievement.",
                ))
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_achievement_list",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_achievement_list", |_: &Lua, this, _: ()| {
            if let Some(result) = this.user_statistic.get_achievement_names() {
                Ok(result)
            } else {
                Err(mlua::Error::runtime(
                    "Steam::get_achievement_list(): Error getting achievement list.",
                ))
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:set_achievement",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "set_achievement",
            |_: &Lua, this, (name, value): (String, bool)| {
                let achievement = this.user_statistic.achievement(&name);

                if value {
                    if let Ok(result) = achievement.set() {
                        return Ok(result);
                    }
                } else if let Ok(result) = achievement.clear() {
                    return Ok(result);
                }

                Err(mlua::Error::runtime(
                    "Steam::set_achievement(): Error setting achievement.",
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_achievement_percent",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_achievement_percent", |_: &Lua, this, name: String| {
            let achievement = this.user_statistic.achievement(&name);

            if let Ok(result) = achievement.get_achievement_achieved_percent() {
                Ok(result)
            } else {
                Err(mlua::Error::runtime(
                    "Steam::get_achievement_percent(): Error getting achievement.",
                ))
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_achievement_name",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_achievement_name", |_: &Lua, this, name: String| {
            let achievement = this.user_statistic.achievement(&name);

            if let Ok(result) = achievement.get_achievement_display_attribute("name") {
                Ok(result.to_string())
            } else {
                Err(mlua::Error::runtime(
                    "Steam::get_achievement_name(): Error getting achievement.",
                ))
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_achievement_",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_achievement_", |_: &Lua, this, name: String| {
            let achievement = this.user_statistic.achievement(&name);

            if let Ok(result) = achievement.get_achievement_display_attribute("desc") {
                Ok(result.to_string())
            } else {
                Err(mlua::Error::runtime(
                    "Steam::get_achievement_(): Error getting achievement.",
                ))
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_achievement_hidden",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_achievement_hidden", |_: &Lua, this, name: String| {
            let achievement = this.user_statistic.achievement(&name);

            if let Ok(result) = achievement.get_achievement_display_attribute("hidden") {
                Ok(result == "1")
            } else {
                Err(mlua::Error::runtime(
                    "Steam::get_achievement_hidden(): Error getting achievement.",
                ))
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_achievement_icon",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_achievement_icon", |lua: &Lua, this, name: String| {
            let achievement = this.user_statistic.achievement(&name);

            if let Some(result) = achievement.get_achievement_icon() {
                let data = crate::base::data::Data::new(lua, result)?;
                let data = lua.create_userdata(data)?;

                Ok(mlua::Value::UserData(data))
            } else {
                Err(mlua::Error::runtime(
                    "Steam::get_achievement_percent(): Error getting achievement.",
                ))
            }
        });

        //================================================================
        // remote play.
        //================================================================

        // TO-DO get session list? might not make sense given how the API is set up around getting a session first
        // and operating upon it.

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_session_user",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_session_user", |_: &Lua, this, id: u32| {
            let session = this.remote_play.session(RemotePlaySessionId::from_raw(id));

            Ok(session.user().raw())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_session_client_name",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_session_client_name", |_: &Lua, this, id: u32| {
            let session = this.remote_play.session(RemotePlaySessionId::from_raw(id));

            Ok(session.client_name())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_session_client_form_factor",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "get_session_client_form_factor",
            |_: &Lua, this, id: u32| {
                let session = this.remote_play.session(RemotePlaySessionId::from_raw(id));

                if let Some(form_factor) = session.client_form_factor() {
                    match form_factor {
                        SteamDeviceFormFactor::Phone => Ok(mlua::Value::Integer(0)),
                        SteamDeviceFormFactor::Tablet => Ok(mlua::Value::Integer(1)),
                        SteamDeviceFormFactor::Computer => Ok(mlua::Value::Integer(2)),
                        SteamDeviceFormFactor::TV => Ok(mlua::Value::Integer(3)),
                    }
                } else {
                    Ok(mlua::Value::Nil)
                }
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_session_client_resolution",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_session_client_resolution", |_: &Lua, this, id: u32| {
            let session = this.remote_play.session(RemotePlaySessionId::from_raw(id));

            if let Some(resolution) = session.client_resolution() {
                Ok((
                    mlua::Value::Integer(resolution.0 as i64),
                    mlua::Value::Integer(resolution.1 as i64),
                ))
            } else {
                Ok((mlua::Value::Nil, mlua::Value::Nil))
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:invite_session",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "invite_session",
            |_: &Lua, this, (id, steam_id): (u32, String)| {
                let session = this.remote_play.session(RemotePlaySessionId::from_raw(id));

                let steam_id = steam_id
                    .parse()
                    .map_err(|x: ParseIntError| mlua::Error::runtime(x.to_string()))?;

                Ok(session.invite(SteamId::from_raw(steam_id)))
            },
        );

        //================================================================
        // remote storage.
        //================================================================

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:set_cloud_app",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("set_cloud_app", |_: &Lua, this, state: bool| {
            this.remote_storage.set_cloud_enabled_for_app(state);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_cloud_app",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_cloud_app", |_: &Lua, this, _: ()| {
            Ok(this.remote_storage.is_cloud_enabled_for_app())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_cloud_account",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_cloud_account", |_: &Lua, this, _: ()| {
            Ok(this.remote_storage.is_cloud_enabled_for_account())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_file_list",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_file_list", |lua: &Lua, this, _: ()| {
            let table = lua.create_table()?;

            for file in this.remote_storage.files() {
                let t_file = lua.create_table()?;
                t_file.set("name", file.name)?;
                t_file.set("size", file.size)?;
                table.push(t_file)?;
            }

            lua.to_value(&table)
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:file_delete",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("file_delete", |_: &Lua, this, path: String| {
            let file = this.remote_storage.file(&path);

            Ok(file.delete())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:file_forget",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("file_forget", |_: &Lua, this, path: String| {
            let file = this.remote_storage.file(&path);

            Ok(file.forget())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:file_exist",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_file_exist", |_: &Lua, this, path: String| {
            let file = this.remote_storage.file(&path);

            Ok(file.exists())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_file_persist",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_file_persist", |_: &Lua, this, path: String| {
            let file = this.remote_storage.file(&path);

            Ok(file.is_persisted())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "steam:get_file_time_stamp",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_file_time_stamp", |_: &Lua, this, path: String| {
            let file = this.remote_storage.file(&path);

            Ok(file.timestamp())
        });

        // TO-DO add write, read.

        //================================================================
        // UGC.
        //================================================================
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
    fn new(_: &Lua, id: u32) -> mlua::Result<Self> {
        let (client, single) =
            Client::init_app(AppId(id)).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        let utility = client.utils();
        let app = client.apps();
        let friend = client.friends();
        let user = client.user();
        let user_statistic = client.user_stats();
        let remote_play = client.remote_play();
        let remote_storage = client.remote_storage();
        let network_message = client.networking_messages();

        // Even though NetworkingMessages appears as ad-hoc API, it's internally session based. We must accept any incoming
        // messages before communicating with the peer.
        network_message.session_request_callback(move |req| {
            println!("Accepting session request from {:?}", req.remote());
            req.accept();
        });

        // Install a callback to debug print failed peer connections
        network_message.session_failed_callback(|info| {
            eprintln!("Session failed: {info:#?}");
        });

        Ok(Self {
            client,
            single,
            utility,
            app,
            friend,
            user,
            user_statistic,
            remote_play,
            remote_storage,
            network_message,
        })
    }
}
