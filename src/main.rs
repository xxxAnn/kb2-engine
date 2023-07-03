#![warn(clippy::all, clippy::pedantic)]

mod server;
mod data;
mod utils;
mod game;
mod prelude;
mod defs;

use defs::{LOCAL_PORT, LOCAL_ADDR};
use prelude::*; 

fn main() {
    let data = Data::new();

    Server::new(LOCAL_ADDR, LOCAL_PORT, Game::new(data))
        .serve()
        .unwrap();
}

