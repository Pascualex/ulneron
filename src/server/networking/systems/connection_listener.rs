use std::net::{TcpListener, TcpStream};

use bevy::prelude::*;

pub fn connection_listener(listener: Res<TcpListener>, mut streams: ResMut<Vec<TcpStream>>) {
    while let Ok((stream, _)) = listener.accept() {
        stream.set_nonblocking(true).unwrap();
        streams.push(stream);
    }
}
