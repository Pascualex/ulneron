pub mod components;
pub mod resources;

pub use plugin::ClientPlugin;
pub use setup::setup;

mod plugin;
mod setup;
mod systems;
