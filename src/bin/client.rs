use std::env;

use bevy::{prelude::*, window::close_on_esc};

use ulneron::{
    client::game::ClientGamePlugin,
    client::{
        controller::ClientControllerPlugin, graphics::ClientGraphicsPlugin,
        lobby::ClientLobbyPlugin, networking::ClientNetworkingPlugin, ui::ClientUiPlugin,
    },
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let server_address = match args.len() {
        2 => args[1].clone(),
        _ => "127.0.0.1".to_string(),
    };
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientLobbyPlugin)
        .add_plugin(ClientGamePlugin)
        .add_plugin(ClientGraphicsPlugin)
        .add_plugin(ClientNetworkingPlugin::new(server_address))
        .add_plugin(ClientUiPlugin)
        .add_plugin(ClientControllerPlugin)
        .add_system(close_on_esc)
        .run();
}
