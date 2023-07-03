mod message;
pub mod game_actions;

use crate::{prelude::{Data, Handler}, defs::ErrorType};

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

    pub fn data(&self) -> &Data {
        &self.data
    }

    fn handle_get_user(&mut self, gm: &GameMessage) -> String {
        match gm.get_numeric_line(1) {
            Ok(l) => self.data_mut().player(l).text(),
            Err(e) => e
        }
    }

    fn handle_get_recipe(&mut self, gm: &GameMessage) -> String {
        match gm.get_numeric_line(1) {
            Ok(l) => {
                match self.data().gamedata().get_recipe_by_id(l) {
                    Some(rs) => format!("get_recipe_\r\n{}", rs.to_string()),
                    None => "Recipe not found".to_owned()
                }
            }
            Err(e) => e
        }
    }

    fn handle_get_recipes(&mut self, _: &GameMessage) -> String {
        let gd = self.data().gamedata();
        format!("get_recipes_\r\n{}", gd.recipes_text())
    }
}

impl Handler for Game {
    fn handle(&mut self, recv: impl Into<String>) -> Result<String, ErrorType> {
        let recv_str: String = recv.into();
        let gm = GameMessage::new(recv_str)?;

        Ok(gm.dispatch().call(&gm, self.data_mut())?.text())
    }
}