use std::collections::HashMap;

use crate::{defs::{MAP_DATA_FILE, Result}, utils::parser::parse_tile_class};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

impl From<usize> for TileType {
    fn from(value: usize) -> Self {
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

#[derive(Debug, Clone)]
pub struct TileClass {
    id: usize,
    #[allow(dead_code)]
    name: String, 
    mults: Vec<(usize, u64)>
}

#[derive(Debug, Clone)]
pub struct MapData {
    cls: HashMap<TileType, TileClass>
}

impl TileClass {
    pub fn new(id: usize, name: String, mults: Vec<(usize, u64)>) -> Self {
        Self { id, name, mults }
    }

    pub fn get_multiplier(&self, i: usize) -> u64 {
        let mut r = None;
        for (id, mult) in &self.mults {
            if i == *id {
                r = Some(*mult)
            }
        }

        r.unwrap_or(1)
    }
}

impl MapData {
    pub fn new() -> Result<Self> {
        let mut cls = HashMap::new();

        for line in std::fs::read_to_string(MAP_DATA_FILE)?.lines() {
            if !line.starts_with("#") {
                let tmp = parse_tile_class(line)?;
                cls.insert(tmp.id.into(), tmp);
            }
        }

        Ok(Self {
            cls
        })
    }

    pub fn tile(&self, tt: &TileType) -> Option<&TileClass> {
        self.cls.get(tt)
    }
}