use sqlite::Connection;
use crate::defs::DB_PATH;

use super::inventory::Inventory;

pub struct DBConnection {
    conn: Connection
}

impl DBConnection {
    pub fn new() -> Self {
        let conn = sqlite::open(DB_PATH).unwrap();

        Self {
            conn
        }
    }

    fn _create_user(&self, userid: u64) {
        let query = format!("
            INSERT INTO userdata (userid, inventory) VALUES ({userid}, '0:100')"
        );
        self.conn.execute(query).unwrap();
    }

    fn _get_inventory_str(&self, userid: u64) -> Option<String> {
        let nquery = format!("
            SELECT inventory FROM userdata WHERE userid = {userid}
        ");
    
        let mut inv_str = Option::None;
    
        self.conn.iterate(nquery, |pairs| {
            for &(_, value) in pairs.iter() {
                inv_str = Some(value.unwrap().to_string());
            }
            true
        })
        .unwrap();

        inv_str
    }

    pub fn update_player_inventory(&self, id: u64, inv_str: impl Into<String>) {
        let inv: String = inv_str.into();

        
        let query = format!("
            UPDATE userdata SET inventory = '{inv}' WHERE userid = {id}"
        );
        self.conn.execute(query).unwrap();
    }

    pub fn get_player_inventory(&self, userid: u64) -> Inventory {   
        if let Some(tx) = self._get_inventory_str(userid) {
            Inventory::new(tx)
        } else {
            self._create_user(userid);
    
            Inventory::new("0:100")
        }
    }
}