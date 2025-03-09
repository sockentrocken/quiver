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

use discord_sdk::{
    Snowflake,
    activity::{ActivityActionKind, IntoTimestamp, Secrets},
    overlay::Visibility,
    registration::{Application, BinArg},
    user::UserId,
    wheel::ActivitySpoke,
};
use mlua::prelude::*;
use raylib::prelude::*;
use serde::Deserialize;
use std::num::ParseIntError;

//================================================================

/* class
{ "version": "1.0.0", "name": "quiver.discord", "info": "The discord API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, status_info: &StatusInfo, script_info: Option<&ScriptInfo>) -> mlua::Result<()> {
    let discord = lua.create_table()?;

    discord.set("new", lua.create_async_function(self::Discord::new)?)?;

    table.set("discord", discord)?;

    Ok(())
}

/* class
{ "version": "1.0.0", "name": "discord", "info": "An unique handle to a Discord client." }
*/
struct Discord {
    discord: discord_sdk::Discord,
    user: discord_sdk::user::User,
    wheel: discord_sdk::wheel::Wheel,
    activity: ActivitySpoke,
}

#[derive(Deserialize)]
struct ActivityTable {
    kind: Option<i32>,
    state: Option<String>,
    detail: Option<String>,
    large_image_key: Option<String>,
    small_image_key: Option<String>,
    large_image_text: Option<String>,
    small_image_text: Option<String>,
    instance: Option<bool>,
    party_id: Option<String>,
    party_size_current: Option<u32>,
    party_size_maximum: Option<u32>,
    party_public: Option<bool>,
    secret_join: Option<String>,
    secret_view: Option<String>,
    button_a_name: Option<String>,
    button_a_link: Option<String>,
    button_b_name: Option<String>,
    button_b_link: Option<String>,
    stamp_a: Option<String>,
    stamp_b: Option<String>,
}

impl mlua::UserData for Discord {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("user_ID", |_: &Lua, this| Ok(this.user.id.0.to_string()));
        field.add_field_method_get("user_name", |_: &Lua, this| Ok(this.user.username.clone()));
        field.add_field_method_get("user_discriminator", |_: &Lua, this| {
            if let Some(discriminator) = this.user.discriminator {
                Ok(mlua::Value::Number(discriminator.into()))
            } else {
                Ok(mlua::Value::Nil)
            }
        });
        // TO-DO add get avatar.
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "discord:set_rich_presence",
            "info": "TO-DO"
        }
        */
        method.add_async_method_mut(
            "set_rich_presence",
            |lua: Lua, this, activity: LuaValue| async move {
                let activity: ActivityTable = lua.from_value(activity)?;

                let mut rp = discord_sdk::activity::ActivityBuilder::default();

                if let Some(kind) = activity.kind {
                    match kind {
                        0 => rp = rp.kind(discord_sdk::activity::ActivityKind::Playing),
                        1 => rp = rp.kind(discord_sdk::activity::ActivityKind::Streaming),
                        2 => rp = rp.kind(discord_sdk::activity::ActivityKind::Listening),
                        3 => rp = rp.kind(discord_sdk::activity::ActivityKind::Watching),
                        4 => rp = rp.kind(discord_sdk::activity::ActivityKind::Custom),
                        _ => rp = rp.kind(discord_sdk::activity::ActivityKind::Competing),
                    }
                }

                if let Some(state) = activity.state {
                    rp = rp.state(state);
                }

                if let Some(detail) = activity.detail {
                    rp = rp.details(detail);
                }

                let mut asset = discord_sdk::activity::Assets::default();

                if let Some(large_image_key) = activity.large_image_key {
                    asset = asset.large(large_image_key, activity.large_image_text);
                }

                if let Some(small_image_key) = activity.small_image_key {
                    asset = asset.large(small_image_key, activity.small_image_text);
                }

                if let Some(instance) = activity.instance {
                    rp = rp.instance(instance);
                }

                rp = rp.assets(asset);

                if let Some(party_id) = activity.party_id {
                    if let Some(party_public) = activity.party_public {
                        rp = rp.party(
                            party_id,
                            activity
                                .party_size_current
                                .map(|x| std::num::NonZeroU32::new(x).unwrap()),
                            activity
                                .party_size_maximum
                                .map(|x| std::num::NonZeroU32::new(x).unwrap()),
                            if party_public {
                                discord_sdk::activity::PartyPrivacy::Public
                            } else {
                                discord_sdk::activity::PartyPrivacy::Private
                            },
                        )
                    }
                }

                rp = rp.secrets(Secrets {
                    r#match: None,
                    join: activity.secret_join,
                    spectate: activity.secret_view,
                });

                if let Some(button_a_name) = activity.button_a_name {
                    if let Some(button_a_link) = activity.button_a_link {
                        rp = rp.button(discord_sdk::activity::Button {
                            label: button_a_name,
                            url: button_a_link,
                        });
                    }
                }

                if let Some(button_b_name) = activity.button_b_name {
                    if let Some(button_b_link) = activity.button_b_link {
                        rp = rp.button(discord_sdk::activity::Button {
                            label: button_b_name,
                            url: button_b_link,
                        });
                    }
                }

                rp = rp.timestamps(
                    activity.stamp_a.map(|x| {
                        let x = x.parse::<i64>().unwrap_or_default().into_timestamp();
                        x
                    }),
                    activity.stamp_b.map(|x| {
                        let x = x.parse::<i64>().unwrap_or_default().into_timestamp();
                        x
                    }),
                );

                this.discord.update_activity(rp).await.unwrap();

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "discord:clear_rich_presence",
            "info": "TO-DO"
        }
        */
        method.add_async_method_mut("clear_rich_presence", |_: Lua, this, _: ()| async move {
            this.discord.clear_activity().await.unwrap();
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "discord:update",
            "info": "TO-DO"
        }
        */
        method.add_async_method_mut(
            "update",
            |_: Lua, mut this, function: mlua::Function| async move {
                while let Ok(activity) = this.activity.0.try_recv() {
                    println!("got activity: {activity:?}");

                    match activity {
                        discord_sdk::activity::events::ActivityEvent::Join(event) => {
                            if function.call_async::<()>((0, event.secret)).await.is_err() {}
                        }
                        discord_sdk::activity::events::ActivityEvent::Spectate(event) => {
                            if function.call_async::<()>((1, event.secret)).await.is_err() {}
                        }
                        _ => {}
                    }
                }

                Ok(())
            },
        );
    }
}

impl Discord {
    /* entry
    {
        "version": "1.0.0",
        "name": "quiver.discord.new",
        "info": "Create a new Discord client."
    }
    */
    async fn new(
        _: Lua,
        (id, name, command): (String, Option<String>, Option<LuaValue>),
    ) -> mlua::Result<Self> {
        let (wheel, handler) = discord_sdk::wheel::Wheel::new(Box::new(|error| {
            eprintln!("Discord API error: {error:?}")
        }));

        let id: i64 = id
            .parse()
            .map_err(|x: ParseIntError| mlua::Error::runtime(x.to_string()))?;

        let discord = {
            if let Some(command) = command {
                let command = match command {
                    LuaValue::Integer(value) => {
                        discord_sdk::registration::LaunchCommand::Steam(value as u32)
                    }
                    LuaValue::Number(value) => {
                        discord_sdk::registration::LaunchCommand::Steam(value as u32)
                    }
                    LuaValue::String(value) => discord_sdk::registration::LaunchCommand::Url(
                        discord_sdk::registration::Url::parse(&value.to_string_lossy()).unwrap(),
                    ),
                    LuaValue::Table(value) => {
                        let path: String = value.get("path")?;
                        let argument_list: Vec<String> = value.get("argument_list")?;
                        let argument_list: Vec<BinArg> = argument_list
                            .iter()
                            .map(|x| {
                                if x.is_empty() {
                                    BinArg::Url
                                } else {
                                    BinArg::Arg(x.clone())
                                }
                            })
                            .collect();

                        discord_sdk::registration::LaunchCommand::Bin {
                            path: path.into(),
                            args: argument_list,
                        }
                    }
                    _ => {
                        panic!()
                    }
                };

                discord_sdk::Discord::new(
                    discord_sdk::DiscordApp::Register(Application { id, name, command }),
                    discord_sdk::Subscriptions::ACTIVITY,
                    Box::new(handler),
                )
                .map_err(|x| mlua::Error::runtime(x.to_string()))?
            } else {
                discord_sdk::Discord::new(
                    discord_sdk::DiscordApp::PlainId(id),
                    discord_sdk::Subscriptions::ACTIVITY,
                    Box::new(handler),
                )
                .map_err(|x| mlua::Error::runtime(x.to_string()))?
            }
        };

        let mut user = wheel.user();

        user.0
            .changed()
            .await
            .map_err(|x| mlua::Error::runtime(x.to_string()))?;

        let user = match &*user.0.borrow() {
            discord_sdk::wheel::UserState::Connected(user) => user.clone(),
            discord_sdk::wheel::UserState::Disconnected(error) => {
                { Err(mlua::Error::runtime(error.to_string())) }?
            }
        };

        println!("enter user: {user:?}");

        let activity = wheel.activity();

        Ok(Self {
            discord,
            user,
            wheel,
            activity,
        })
    }
}
