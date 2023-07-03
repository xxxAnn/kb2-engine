pub type ErrorType = String;

pub const LOCAL_ADDR: &str = "127.0.0.1";
pub const LOCAL_PORT: u16 = 7878;
pub const DB_PATH: &str = "data.db";
pub const BASE_QUANTITY: f32 = 10.0;
pub mod SPECIAL_ITEM {
    pub const ENERGY: usize = 4;
    pub const MONEY: usize = 0;
}