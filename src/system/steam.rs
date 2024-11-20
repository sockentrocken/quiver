use mlua::prelude::*;

use steamworks::AppId;
use steamworks::Client;
use steamworks::FriendFlags;
use steamworks::GameOverlayActivated;
use steamworks::PersonaStateChange;
use steamworks::SingleClient;

/*
fn main() {
    let (client, single) = Client::init().unwrap();

    let _cb = client.register_callback(|p: PersonaStateChange| {
        println!("Got callback: {:?}", p);
    });

    let utils = client.utils();
    println!("Utils:");
    println!("AppId: {:?}", utils.app_id());
    println!("UI Language: {}", utils.ui_language());

    let apps = client.apps();
    println!("Apps");
    println!("IsInstalled(480): {}", apps.is_app_installed(AppId(480)));
    println!("InstallDir(480): {}", apps.app_install_dir(AppId(480)));
    println!("BuildId: {}", apps.app_build_id());
    println!("AppOwner: {:?}", apps.app_owner());
    println!("Langs: {:?}", apps.available_game_languages());
    println!("Lang: {}", apps.current_game_language());
    println!("Beta: {:?}", apps.current_beta_name());

    let friends = client.friends();
    println!("Friends");
    let list = friends.get_friends(FriendFlags::IMMEDIATE);
    println!("{:?}", list);
    for f in &list {
        println!("Friend: {:?} - {}({:?})", f.id(), f.name(), f.state());
        friends.request_user_information(f.id(), true);
    }

    for _ in 0..50 {
        single.run_callbacks();
        ::std::thread::sleep(::std::time::Duration::from_millis(100));
    }
}
*/

/* meta
---@class steam
local steam = {}
*/
pub struct Steam {
    client: Client,
    single: SingleClient,
}

impl mlua::UserData for Steam {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* meta
        ---Play sound.
        function sound:play() end
        */
        method.add_method("play", |_, this, ()| unsafe { Ok(()) });
    }
}

impl Steam {
    /* meta
    ---An unique handle for the Steam API. Creating more than one handle will result in an error.
    ---@param app? number App ID. If nil, it will resort to the default (480) SpaceWar example.
    ---@return steam # The user-data object.
    function Steam(app) end
    */
    pub fn new(_: &Lua, app: Option<u32>) -> mlua::Result<Self> {
        if let Some(app) = app {
            let (client, single) =
                Client::init_app(app).map_err(|e| mlua::Error::runtime(e.to_string()))?;

            let _cb = client.register_callback(|p: GameOverlayActivated| {
                println!("Got callback: {:?}", p);
            });

            Ok(Self { client, single })
        } else {
            let (client, single) =
                Client::init().map_err(|e| mlua::Error::runtime(e.to_string()))?;

            Ok(Self { client, single })
        }
    }
}

#[rustfmt::skip]
pub fn set_global(lua: &Lua, global: &mlua::Table) -> mlua::Result<()> {
    global.set("Steam", lua.create_function(self::Steam::new)?)?;

    Ok(())
}
