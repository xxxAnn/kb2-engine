mod db;
mod user;
mod inventory;
mod gamedata;
mod map;
mod dump;

use gamedata::GameData;
pub use user::User;
pub use gamedata::{Item, Recipe, TileType, MapData, TileClass};
pub use map::Map;
pub use dump::Dump;


use crate::{utils::parser::parse_map, defs::{MAP_PATH, Kb2Result}};


pub struct Data {
    users: Vec<User>,
    gamedata: GameData,
    map: Map
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
    pub fn new() -> Kb2Result<Self> {
        Ok(Self {
            users: Vec::new(),
            gamedata: GameData::new()?,
            map: Map::new(
                parse_map(
                    std::fs::read_to_string(MAP_PATH)?
                )?
            )
        })
    }

    pub fn update_map(&mut self, mut map: Map) -> Kb2Result<()> {
        map.save()?;
        self.map = map;
        Ok(())
    }

    pub fn map(&self) -> Map {
        self.map.clone()
    }
    pub fn gamedata(&self) -> GameData {
        self.gamedata.clone()
    }

    fn add_player(&mut self, id: u64) -> Kb2Result<()> {
        self.users.push(
            User::new(id)?
        );

        Ok(())
    }

    fn get_player_mut(&mut self, id: u64) -> Option<&mut User> {
        let mut found = None;
        
        for user in &mut self.users {
            if user.id() == id {
                found = Some(user);
                break;
            }
        }

        found
    }

    fn get_player(&self, id: u64) -> Option<&User> {
        let mut found = None;
        
        for user in &self.users {
            if user.id() == id {
                found = Some(user);
                break;
            }
        }

        found
    }

    fn check_if_player_exists(&self, id: u64) -> bool {
        let mut found = None;
        
        for user in &self.users {
            if user.id() == id {
                found = Some(user);
                break;
            }
        }

        found.is_some()
    }

    #[allow(dead_code)]
    pub fn player(&mut self, id: u64) -> Option<&User> {
        if self.check_if_player_exists(id) {
            self.get_player(id) 
        } else {
            self.add_player(id).ok()?;
            self.player(id)
        }
    }

    pub fn player_mut(&mut self, id: u64) -> Option<&mut User> {
        if self.check_if_player_exists(id) {
            self.get_player_mut(id)
        } else {
            self.add_player(id).ok()?;
            self.player_mut(id)
        }
    }        
}