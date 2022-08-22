use bevy::{prelude::*, window::close_on_esc};

use zombie_bevy::{
    client::ClientPlugin, networking::client::ClientNetworkingPlugin, protocol::ProtocolPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(ClientNetworkingPlugin)
        .add_plugin(ProtocolPlugin)
        .add_system(close_on_esc)
        .run();
}
