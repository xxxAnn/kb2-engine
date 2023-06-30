mod db;
mod user;
mod inventory;
mod gamedata;

use db::DBConnection;
use gamedata::GameData;
pub use user::User;


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

    pub fn gamedata(&self) -> &GameData {
        &self.gamedata
    }

    fn add_player(&mut self, id: u64) {
        self.users.push(
            User::new(id, &self.connector)
        )
    }

    pub fn get_player(&mut self, id: u64) -> User {
        let mut found = None;
        
        for user in &self.users {
            if user.id() == id {
                found = Some(user.clone());
                break;
            }
        }

        let usr = match found {
            Some(f) => {
                f
            },
            None => {
                self.add_player(id);
                self.get_player(id)
            }
        };

        usr
    }
}