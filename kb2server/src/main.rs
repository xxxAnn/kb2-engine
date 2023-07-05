mod game;
mod server;

use server::Server;
use game::Game;

use kb2lib::prelude::*;
use kb2lib::defs::*;

fn main() -> Result<()> {
    let data = Data::new()?;

    Server::new(LOCAL_ADDR, LOCAL_PORT, Game::new(data))
        .serve()?;

    Ok(())
}
