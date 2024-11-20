use mlua::prelude::*;

pub use discord_sdk as ds;
pub use tokio;

/// Application identifier for "Andy's Test App" used in the Discord SDK's
/// examples.
pub const APP_ID: ds::AppId = 310270644849737729;

pub struct Client {
    pub discord: ds::Discord,
    pub user: ds::user::User,
    pub wheel: ds::wheel::Wheel,
}

pub async fn make_client(subs: ds::Subscriptions) -> Client {
    let (wheel, handler) = ds::wheel::Wheel::new(Box::new(|err| {
        println!("encountered an error");
    }));

    let mut user = wheel.user();

    let discord = ds::Discord::new(ds::DiscordApp::PlainId(APP_ID), subs, Box::new(handler))
        .expect("unable to create discord client");

    println!("waiting for handshake...");
    user.0.changed().await.unwrap();

    let user = match &*user.0.borrow() {
        ds::wheel::UserState::Connected(user) => user.clone(),
        ds::wheel::UserState::Disconnected(err) => panic!("failed to connect to Discord: {}", err),
    };

    println!("connected to Discord, local user is {:#?}", user);

    Client {
        discord,
        user,
        wheel,
    }
}

use std::time::SystemTime;

pub struct Discord {
    pub client: Client
}

impl mlua::UserData for Discord {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method("user_name", |_: &Lua, this, _: ()| {
            Ok(this.client.user.username.clone())
        });
    }
}

impl Discord {
    pub async fn new(_: Lua, _: ()) -> mlua::Result<Self> {
        let client = make_client(ds::Subscriptions::ACTIVITY).await;

        let mut activity_events = client.wheel.activity();

        tokio::task::spawn(async move {
            while let Ok(ae) = activity_events.0.recv().await {
                println!("received activity event");
            }
        });

        let rp = ds::activity::ActivityBuilder::default()
            .details("Fruit Tarts".to_owned())
            .state("Pop Snacks".to_owned())
            .assets(
                ds::activity::Assets::default()
                    .large("the".to_owned(), Some("u mage".to_owned()))
                    .small("the".to_owned(), Some("i mage".to_owned())),
            )
            .button(ds::activity::Button {
                label: "discord-sdk by EmbarkStudios".to_owned(),
                url: "https://github.com/EmbarkStudios/discord-sdk".to_owned(),
            })
            .start_timestamp(SystemTime::now());

        println!(
            "updated activity: {:?}",
            client.discord.update_activity(rp).await
        );

        /*
        let mut r = String::new();
        let _ = std::io::stdin().read_line(&mut r);

        println!(
            "cleared activity: {:?}",
            client.discord.clear_activity().await
        );
        */

        //client.discord.disconnect().await;

        Ok(Self {
            client
        })
    }
}

#[rustfmt::skip]
pub fn set_global(lua: &Lua, global: &mlua::Table) -> mlua::Result<()> {
    global.set("Discord", lua.create_async_function(self::Discord::new)?)?;

    Ok(())
}
