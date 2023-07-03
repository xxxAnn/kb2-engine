use std::str::FromStr;

use crate::{prelude::Data, defs::ErrorType};

use super::{Summary, game_actions::{Exploit, Summarize, AvailableRecipes, GetUser, Unknown, GetRecipe, GetRecipes}};

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
            _ => Dispatcher::Unknown
        }
    }

    pub fn call(&self, gm: &GameMessage, data: &mut Data) -> Result<Box<dyn Summary>, ErrorType> {

        caller!(
            self,
            data,
            gm,
            Exploit,
            GetUser,
            GetRecipes,
            GetRecipe,
            AvailableRecipes,
            Unknown
        )
    }
}

impl GameMessage {
    pub fn new(text: &str) -> Result<Self, ErrorType> {
        let mut data = text.lines();
        if let Some(code_str) = data.next() {
            if let Ok(code) = code_str.parse::<u16>() {
                Ok(Self {
                    code,
                    data: data.map(std::borrow::ToOwned::to_owned).collect()
                })
            } else {
                Err("Code wasn't numeric".to_owned())
            }
        } else {
            Err("Malformed request".to_owned())
        }
    }

    pub fn dispatch(&self) -> Dispatcher {
        Dispatcher::from_code(self.code)
    }

    pub fn get_line(&self, number: usize) -> Result<String, ErrorType> {
        match self.data.get(number-1) {
            Some(line) => Ok(line.clone()),
            None => Err("Malformed request".to_owned())
        }
    }

    pub fn get_numeric_line<T>(&self, number: usize) -> Result<T, ErrorType> 
    where T: FromStr {
        let res = self.get_line(number)?;
        match res.parse() {
            Ok(l) => Ok(l),
            Err(_) => Err("Expected numeric lined".to_owned())
        }
    }
}