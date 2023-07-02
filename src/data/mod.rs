mod db;
mod user;
mod inventory;
mod gamedata;

use gamedata::GameData;
pub use user::User;
pub use gamedata::Item;


pub struct Data {
    users: Vec<User>,
    gamedata: GameData,
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
        Self {
            users: Vec::new(),
            gamedata: GameData::new(),
        }
    }

    pub fn gamedata(&self) -> GameData {
        self.gamedata.clone()
    }

    fn add_player(&mut self, id: u64) {
        self.users.push(
            User::new(id)
        )
    }

    fn get_player_mut(&mut self, id: u64) -> Option<&mut User> {
        let mut found = None;
        
        for user in self.users.iter_mut() {
            if user.id() == id {
                found = Some(user);
                break;
            }
        }

        found
    }

    fn get_player(&self, id: u64) -> Option<&User> {
        let mut found = None;
        
        for user in self.users.iter() {
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

    pub fn player(&mut self, id: u64) -> &User {
        if self.check_if_player_exists(id) {
            self.get_player(id).unwrap()
        } else {
            self.add_player(id);
            self.player(id)
        }
    }

    pub fn player_mut(&mut self, id: u64) -> &mut User {
        if self.check_if_player_exists(id) {
            self.get_player_mut(id).unwrap()
        } else {
            self.add_player(id);
            self.player_mut(id)
        }
    }        
}