use std::net::TcpListener;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Endpoint {
    pub listener: TcpListener,
}

impl Endpoint {
    pub fn new(port: u32) -> Self {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(addr).unwrap();
        listener.set_nonblocking(true).unwrap();
        Self { listener }
    }
}
