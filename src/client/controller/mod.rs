pub mod data;
pub mod events;
pub mod resources;

pub use plugin::ClientControllerPlugin;

mod plugin;
mod setup;
mod systems;

use setup::setup;
