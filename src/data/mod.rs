mod db;
mod user;
mod inventory;
mod gamedata;

use db::DBConnection;
use gamedata::GameData;
pub use user::User;
pub use gamedata::Item;


pub struct Data {
    users: Vec<User>,
    gamedata: GameData,
    connector: DBConnection
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Data")
            .field("users", &self.users)
            .field("gamedata", &self.gamedata)
            .finish()
    }
}

impl Data {
    pub fn new() -> Self {
        let connector = DBConnection::new();

        Self {
            users: Vec::new(),
            gamedata: GameData::new(),
            connector
        }
    }

    pub fn update_player_inventory(&self, id: u64, inv_str: impl Into<String>) {
        self.connector.update_player_inventory(id, inv_str);
    }

    pub fn gamedata(&self) -> GameData {
        self.gamedata.clone()
    }

    fn add_player(&mut self, id: u64) {
        self.users.push(
            User::new(id, &self.connector)
        )
    }

    fn get_player_exists(&mut self, id: u64) -> Option<&mut User> {
        let mut found = None;
        
        for user in self.users.iter_mut() {
            if user.id() == id {
                found = Some(user);
                break;
            }
        }

        found
    }

    fn check_if_player_exists(&self, id: u64) -> bool {
        let mut found = None;
        
        for user in self.users.iter() {
            if user.id() == id {
                found = Some(user);
                break;
            }
        }

        found.is_some()
    }

    pub fn get_player(&mut self, id: u64) -> &mut User {
        if self.check_if_player_exists(id) {
            self.get_player_exists(id).unwrap()
        } else {
            self.add_player(id);
            self.get_player(id)
        }
    }        
}