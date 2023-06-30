use super::{db::DBConnection, Data, inventory::Inventory};

pub struct User {
    id: u64,
    inventory: Inventory,
    money: u64,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("inventory", &self.inventory)
            .field("money", &self.money)
            .finish()
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), inventory: self.inventory.clone(), money: self.money.clone() }
    }
}

impl User {
    pub fn new(id: u64, connector: &DBConnection) -> Self {
        let inventory = connector.get_player_inventory(id);

        let money = inventory.balance();

        Self {
            id,
            inventory,
            money,
        }
    }    

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn add_item(&mut self, item_id: usize, quantity: u64, data: &Data) {
        self.inventory.add_item(item_id, quantity);
        self.save(data);
    }

    pub fn save(&mut self, data: &Data) {
        let inv_str = self.inventory.dump();
        
        data.update_player_inventory(self.id, inv_str);
    }
}
