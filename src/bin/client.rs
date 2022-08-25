use std::env;

use bevy::{prelude::*, window::close_on_esc};

use zombie_bevy::{
    client::ClientPlugin, graphics::GraphicsPlugin, networking::client::ClientNetworkingPlugin,
    protocol::ProtocolPlugin,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let server_addr = match args.len() {
        2 => args[1].clone(),
        _ => "127.0.0.1".to_string(),
    };
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(ClientNetworkingPlugin::new(server_addr))
        .add_plugin(ProtocolPlugin)
        .add_system(close_on_esc)
        .run();
}
