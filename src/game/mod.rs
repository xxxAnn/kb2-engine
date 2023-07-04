mod message;
pub mod game_actions;

use crate::{prelude::{Data, Handler}, defs::{Kb2Result}};

pub use game_actions::Summary;

use message::GameMessage;

pub struct Game {
    data: Data
}

impl Game {
    pub fn new(data: Data) -> Self {
        Self {
            data
        }
    }

    pub fn data_mut(&mut self) -> &mut Data {
        &mut self.data
    }

    #[allow(dead_code)]
    pub fn data(&self) -> &Data {
        &self.data
    }
}

impl Handler for Game {
    fn handle(&mut self, recv: impl Into<String>) -> Kb2Result<String> {
        let recv_str: String = recv.into();
        let gm = GameMessage::new(&recv_str)?;

        Ok(gm.dispatch().call(&gm, self.data_mut())?.text())
    }
}