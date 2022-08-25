pub mod client;
pub mod graphics;
pub mod networking;
pub mod protocol;
pub mod server;

pub const TIME_STEP: f32 = 1.0 / 30.0;
pub const BUFFER_SIZE: usize = 10 * 1024;
