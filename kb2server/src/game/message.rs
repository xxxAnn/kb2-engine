use std::str::FromStr;

use crate::{Data, Result, Error};

use super::Summary;

use super::{game_actions::{Exploit, Summarize, AvailableRecipes, GetUser, Unknown, GetRecipe, GetRecipes, Craft, GetLocation}};

pub struct GameMessage {
    code: u16,
    data: Vec<String> 
}

pub enum Dispatcher {
    Exploit,
    GetUser,
    GetRecipes,
    GetRecipe,
    AvailableRecipes,
    Craft,
    GetLocation,
    Unknown
}

macro_rules! dispatch {
    ($i:ident, $($arg:expr),+) => {
        Ok(Box::new($i::from_message($($arg),+)?.call()?))
    }
}

macro_rules! caller {
    ($t:ident, $d:ident, $g:ident, $($i:ident),+) => {
        match $t {
            $(
                Dispatcher::$i => dispatch!($i, $d, $g)
            ),+
        }
    }
}

impl Dispatcher {
    pub fn from_code(c: u16) -> Self {
        match c {
            0 => Dispatcher::Exploit,
            1 => Dispatcher::GetUser,
            2 => Dispatcher::GetRecipes,
            3 => Dispatcher::GetRecipe,
            4 => Dispatcher::AvailableRecipes,
            5 => Dispatcher::Craft,
            6 => Dispatcher::GetLocation,
            _ => Dispatcher::Unknown
        }
    }

    pub fn call(&self, gm: &GameMessage, data: &mut Data) -> Result<Box<dyn Summary>> {

        caller!(
            self,
            data,
            gm,
            Exploit,
            GetUser,
            GetRecipes,
            GetRecipe,
            AvailableRecipes,
            Craft,
            GetLocation,
            Unknown
        )
    }
}

impl GameMessage {
    pub fn new(text: &str) -> Result<Self> {
        let mut data = text.lines();
        if let Some(code_str) = data.next() {
            if let Ok(code) = code_str.parse::<u16>() {
                Ok(Self {
                    code,
                    data: data.map(std::borrow::ToOwned::to_owned).collect()
                })
            } else {
                Err(Error::from("Code wasn't numeric"))
            }
        } else {
            Err(Error::MalformedRequest)
        }
    }

    pub fn dispatch(&self) -> Dispatcher {
        Dispatcher::from_code(self.code)
    }

    pub fn get_line(&self, number: usize) -> Result<String> {
        match self.data.get(number-1) {
            Some(line) => Ok(line.clone()),
            None => Err(Error::MalformedRequest)
        }
    }

    pub fn get_numeric_line<T>(&self, number: usize) -> Result<T> 
    where T: FromStr {
        let res = self.get_line(number)?;
        match res.parse() {
            Ok(l) => Ok(l),
            Err(_) => Err(Error::from("Expected numeric lined"))
        }
    }
}