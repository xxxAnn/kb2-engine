use std::str::FromStr;

use crate::{prelude::Item, defs::Result, data::{TileType, TileClass}};

fn parse_mults(s: impl Into<String>) -> Vec<(usize, u64)> {
    let s_s: String = s.into();

    s_s.split("&").into_iter().map(|s| parse_item(s)).flatten().collect() // the unwrap can't panic
}

pub fn parse_tile_class(s: impl Into<String>) -> Result<TileClass> {
    let s_s: String = s.into();

    let mut splts = s_s.split(",");

    let id: usize = splts.next()
        .ok_or("Empty Tile Class Data.")?
        .parse()
        .or(Err("Couldn't parse Tile Class ID."))?;

    let name: String = splts.next()
        .ok_or("Tile Class has no name.")?
        .to_owned();

    let mults: Vec<(usize, u64)> = parse_mults(
        splts.next()
            .ok_or("Tile Class has no mults")?
        );

    Ok(TileClass::new(id, name, mults))
}

pub fn parse_map(s: impl Into<String>) -> Result<Vec<Vec<TileType>>> {
    let mut res = Vec::new();

    let s_i: String = s.into();

    for line in s_i.lines() {
        let t = line.split(",").collect::<Vec<&str>>();
        let mut temp = Vec::new();
        for ind in t {
            temp.push(TileType::from(ind.parse::<u64>().ok().ok_or("Couldn't parse tile")?));
        }

        res.push(temp)
    }

    Ok(res)
}

pub fn parse_item(t: impl Into<String>) -> Option<(usize, u64)> {
    let t_str: String = t.into();

    let mut pair = t_str.split(':');
    let id: usize = pair.next()?.parse().ok()?;
    let quantity: u64 = pair.next()?.parse().ok()?;

    Some((id, quantity))
} 

pub fn parse_item_list(t: impl Into<String>) -> Option<Vec<(usize, u64)>> {
    let mut res = Vec::new();
    let inv_str: String = t.into();
    let x = inv_str.split(',');

    for indv in x {
        if let Some((id, quantity)) = parse_item(indv) {
            res.push((id, quantity));
        }
    }

    if res.is_empty() {
        None
    } else {
        Some(res)
    }
}

pub struct FieldsWrapper<'a>(&'a [&'a str]);

impl FieldsWrapper<'_> {
    fn get_field(&self, index: usize, err_str: impl Into<String>) -> Result<&str> {
        Ok(*self.0
            .get(index)
            .ok_or(err_str.into())?)
    }

    fn get_field_and_parse<T>(&self, index: usize, err_str: impl Into<String>) -> Result<T>
    where T: FromStr {
        let s: String = err_str.into();

        Ok(
            self
                .get_field(index, &s)?
                .parse::<T>()
                .ok()
                .ok_or(&s)?
        )
    }
}

pub fn extract_item_data(fields: &[&str]) -> Result<Item> {
    let err_str = "Invalid item in Object table";

    assert!(fields.len() == 8, "{}", err_str);

    let fw = FieldsWrapper(fields);

    let id: usize = fw
        .get_field_and_parse(0, err_str)?;

    let name: String = fw
        .get_field(1, err_str)?
        .to_owned()
        .replace('_', " ");

    let class_str: String = fw
        .get_field(2, err_str)?
        .to_owned();

    let exploit: f32 = fw
        .get_field_and_parse(3, err_str)?;
    
    let fishing: f32 = fw
        .get_field_and_parse(4, err_str)?;

    let edible: bool = fw.get_field(5, err_str)? != "0";
    
    let exploitable: u32 = fw
        .get_field_and_parse(6, err_str)?;

    let fishable: u32 = fw
        .get_field_and_parse(7, err_str)?;

    Ok(Item::new(id, name, class_str, exploit, fishing, edible, exploitable, fishable))
}