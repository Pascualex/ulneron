pub mod resources;

pub use plugin::ClientControllerPlugin;

mod plugin;
mod setup;
mod systems;

use setup::setup;
