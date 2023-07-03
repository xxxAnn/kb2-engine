#![warn(clippy::all, clippy::pedantic)]

mod server;
mod data;
mod utils;
mod game;
mod prelude;
mod defs;

use prelude::*; 

const LOCAL_ADDR: &str = "127.0.0.1";
const LOCAL_PORT: u16 = 7878;

fn main() {
    let data = Data::new();

    Server::new(LOCAL_ADDR, LOCAL_PORT, Game::new(data))
        .serve()
        .unwrap();
}

