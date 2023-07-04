use std::collections::HashMap;

#[derive(Clone, Debug, Hash)]
pub enum TileType {
    Grassland,
    Mountain,
    Hill
}

impl From<u64> for TileType {
    fn from(value: u64) -> Self {
        match value {
            1 => Self::Hill,
            2 => Self::Mountain,
            _ => Self::Grassland,            
        }
    }
}

impl From<TileType> for u64 {
    fn from(value: TileType) -> Self {
        match value {
            TileType::Grassland => 0,
            TileType::Hill => 1,
            TileType::Mountain => 2,
        }
    }
}

pub struct TileClass {
    id: usize,
    name: String, 
    mults: Vec<(usize, u64)>
}

pub struct MapData {
    cls: HashMap<TileType, TileClass>
}