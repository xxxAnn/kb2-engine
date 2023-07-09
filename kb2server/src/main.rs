mod game;
mod server;

use server::Server;
use game::Game;

use kb2lib::prelude::*;
use kb2lib::defs::*;

fn main() -> Result<()> {
    let data = Data::new()?;

    Server::new("0.0.0.0", 80u16, Game::new(data))
        .serve()?;

    Ok(())
}
