use sqlite::Connection;
use crate::defs::{DB_PATH, Kb2Result};

use super::inventory::Inventory;

pub struct DBConnection {
    conn: Connection
}

impl DBConnection {
    pub fn new() -> Kb2Result<Self> {
        let conn = sqlite::open(DB_PATH)?;

        Ok(Self {
            conn
        })
    }

    fn _create_user(&self, userid: u64) -> Kb2Result<()> {
        let query = format!("
            INSERT INTO userdata (userid, inventory) VALUES ({userid}, '0:100')"
        );
        Ok(self.conn.execute(query)?)
        
    }

    fn _get_inventory_str(&self, userid: u64) -> Option<String> {
        let nquery = format!("
            SELECT inventory FROM userdata WHERE userid = {userid}
        ");
    
        let mut inv_str = Option::None;
    
        self.conn.iterate(nquery, |pairs| {
            for &(_, value) in pairs.iter() {
                if let Some(v) = value {
                    inv_str = Some(v.to_string())
                } 
            }
            true
        }).ok()?;

        inv_str
    }

    pub fn update_player_inventory(&self, id: u64, inv_str: impl Into<String>) -> Kb2Result<()>{
        let inv: String = inv_str.into();

        
        let query = format!("
            UPDATE userdata SET inventory = '{inv}' WHERE userid = {id}"
        );

        Ok(self.conn.execute(query)?)
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