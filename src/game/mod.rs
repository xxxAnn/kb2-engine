mod game_modules;
mod message;

use game_modules::{Exploit, Summarize, ExploitSummary};

use crate::prelude::{Data, Handler};

pub use game_modules::Summary;

use self::message::{GameMessage, Dispatcher};

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

    pub fn exploit(&mut self, userid: u64) -> ExploitSummary {
        Exploit::new(self.data_mut(), userid).call()
    }

    fn handle_exploit(&mut self, gm: &GameMessage) -> String {
        match gm.get_numeric_line(1) {
            Ok(l) => self.exploit(l).text(),
            Err(e) => e
        }
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

    fn handle_get_recipes(&mut self, gm: &GameMessage) -> String {
        let gd = self.data().gamedata();
        format!("get_recipes_\r\n{}", gd.recipes_text())
    }
}

impl Handler for Game {
    fn handle(&mut self, recv: impl Into<String>) -> String {
        let recv_str: String = recv.into();
        let msg = GameMessage::new(recv_str);

        match msg {
            Ok(gm) => {
                match gm.dispatch() {
                    Dispatcher::Exploit => self.handle_exploit(&gm),
                    Dispatcher::GetUser => self.handle_get_user(&gm),
                    Dispatcher::GetRecipes => self.handle_get_recipes(&gm),
                    Dispatcher::GetRecipe => self.handle_get_recipe(&gm),
                    Dispatcher::Unknown => "Invalid action code".to_owned()
                }
            }
            Err(e) => e
        }
    }
}