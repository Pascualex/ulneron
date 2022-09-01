pub mod components;

pub use plugin::ClientGraphicsPlugin;

mod plugin;
mod resources;
mod setup;
mod systems;

use setup::setup;
