use crate::utils;

pub type ErrorType = utils::Kb2Error;
pub type Kb2Result<T> = Result<T, ErrorType>;

pub const LOCAL_ADDR: &str = "127.0.0.1";
pub const LOCAL_PORT: u16 = 7878;
pub const DB_PATH: &str = "data.db";
pub const BASE_QUANTITY: f32 = 10.0;
pub const OBJECT_TABLE_FILE: &str = "gamedata/OBJECT_TABLE.KB2";
pub const CRAFT_RECIPES_FILE: &str = "gamedata/CRAFT_RECIPES.KB2";

pub mod special_item {
    pub const ENERGY: usize = 4;
    pub const MONEY: usize = 0;
}