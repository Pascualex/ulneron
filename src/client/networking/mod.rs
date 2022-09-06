pub mod resources;

pub use message::UpstreamMessage;
pub use plugin::ClientNetworkingPlugin;

mod message;
mod plugin;
mod systems;
