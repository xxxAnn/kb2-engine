use rand::{thread_rng, Rng};

use crate::defs::{MAP_PATH, Kb2Result};

use super::gamedata::TileType;

pub const MAX_SIZE: usize = 1000;

#[derive(Clone)]
pub struct Map {
    m: Vec<Vec<TileType>>
}

impl ToString for TileType {
    fn to_string(&self) -> String {
        let num: u64 = self.clone().into();
        num.to_string()
    }
}

impl Map {
    pub fn new(m: Vec<Vec<TileType>>) -> Self {
        Self { 
            m
        }
    }

    pub fn dump(&self) -> String {
        Iterator::collect::<Vec<String>>(self.m.iter().map(|v| Iterator::collect::<Vec<String>>(v.iter().map(ToString::to_string)).join(","))).join("\r\n")
    }

    fn generate_x_line(&self, x: usize) -> Vec<TileType> {
        let mut rng = thread_rng();

        (0..=x).into_iter().map(|_| TileType::from(rng.gen_range(0..=2))).collect()
    }

    fn extend_x_line(&mut self, x: usize, y: usize) {
        let to_add = x - self.m[y].len();
        let mut line = self.generate_x_line(to_add);
        self.m[y].append(&mut line);
    }

    fn generate_y_to(&mut self, x: usize, y: usize) {
        for _ in (self.m.len()-1)..=y {
            self.m.push(self.generate_x_line(x));
        }
    }

    fn save(&self) -> Kb2Result<()> {
        std::fs::write(MAP_PATH, self.dump())?;
        Ok(())
    }

    pub fn get_tile(&mut self, px: usize, py: usize) -> &TileType {

        let x = px % MAX_SIZE;
        let y = py % MAX_SIZE;

        let mut changes = false;
        if y >= self.m.len() {
            changes = true;
            self.generate_y_to(x, y);
        }

        // should exist
        if x >= self.m[y].len() {
            changes = true;
            self.extend_x_line(x, y);
        }

        if changes {
            self.save().unwrap();
        }

        &self.m[y][x]
    }
}