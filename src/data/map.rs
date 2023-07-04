use rand::{thread_rng, Rng};

use crate::defs::{MAP_PATH, Kb2Result, MAP_SIZE};

use super::gamedata::TileType;
#[derive(Clone)]
pub struct Map {
    m: Vec<Vec<TileType>>,
    change: bool
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
            m,
            change: false
        }
    }

    pub fn dump(&self) -> String {
        Iterator::collect::<Vec<String>>(self.m.iter().map(|v| Iterator::collect::<Vec<String>>(v.iter().map(ToString::to_string)).join(","))).join("\r\n")
    }

    fn generate_x_line(&self, x: usize) -> Vec<TileType> {
        let mut rng = thread_rng();

        (0..=x).into_iter().map(|_| TileType::from(rng.gen_range(0..=2u64))).collect()
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

    pub fn save(&mut self) -> Kb2Result<()> {
        if self.change {
            std::fs::write(MAP_PATH, self.dump())?;
            self.change = false;
        }
        Ok(())
    }

    pub fn get_tile(&mut self, px: usize, py: usize) -> &TileType {

        let x = px % MAP_SIZE;
        let y = py % MAP_SIZE;

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

        self.change = changes;

        &self.m[y][x]
    }
}