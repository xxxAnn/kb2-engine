use crate::utils;

pub type Result<T> = std::result::Result<T, utils::Error>;


pub const LOCAL_ADDR: &str = "127.0.0.1";
pub const LOCAL_PORT: u16 = 7878;
pub const DB_PATH: &str = "data/players.db";
pub const MAP_PATH: &str = "data/MAP.KB2";
pub const BASE_QUANTITY: f32 = 10.0;
pub const OBJECT_TABLE_FILE: &str = "gamedata/OBJECT_TABLE.KB2";
pub const MAP_DATA_FILE: &str = "gamedata/MAP_DATA.KB2";
pub const CRAFT_RECIPES_FILE: &str = "gamedata/CRAFT_RECIPES.KB2";
pub const MAP_SIZE: usize = 1000;

pub mod special_item {
    use rand::{thread_rng, Rng};

    pub const ENERGY: usize = 4;
    pub const MONEY: usize = 0;
    pub const X_LOCATION: usize = 5;
    pub const Y_LOCATION: usize = 6;

    pub const LIST: &[usize] = &[ENERGY, MONEY, X_LOCATION, Y_LOCATION];

    const X_SPAWN_RANGE: u64 = 100;
    const Y_SPAWN_RANGE: u64 = 100;

    pub fn get_default(id: usize) -> u64 {
        let mut rng = thread_rng();

        match id {
            MONEY => 100,
            ENERGY => 5,
            X_LOCATION => {
                rng.gen_range(0..=X_SPAWN_RANGE)
            },
            Y_LOCATION => {
                rng.gen_range(0..=Y_SPAWN_RANGE)
            }
        _ => 0
        }
    }
}