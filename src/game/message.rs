use std::str::FromStr;

use crate::{prelude::Data, defs::{ErrorType, Kb2Result}, utils::Kb2Error};

use super::{Summary, game_actions::{Exploit, Summarize, AvailableRecipes, GetUser, Unknown, GetRecipe, GetRecipes, Craft}};

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
            _ => Dispatcher::Unknown
        }
    }

    pub fn call(&self, gm: &GameMessage, data: &mut Data) -> Kb2Result<Box<dyn Summary>> {

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
            Unknown
        )
    }
}

impl GameMessage {
    pub fn new(text: &str) -> Kb2Result<Self> {
        let mut data = text.lines();
        if let Some(code_str) = data.next() {
            if let Ok(code) = code_str.parse::<u16>() {
                Ok(Self {
                    code,
                    data: data.map(std::borrow::ToOwned::to_owned).collect()
                })
            } else {
                Err(ErrorType::from("Code wasn't numeric"))
            }
        } else {
            Err(ErrorType::MalformedRequest)
        }
    }

    pub fn dispatch(&self) -> Dispatcher {
        Dispatcher::from_code(self.code)
    }

    pub fn get_line(&self, number: usize) -> Kb2Result<String> {
        match self.data.get(number-1) {
            Some(line) => Ok(line.clone()),
            None => Err(ErrorType::MalformedRequest)
        }
    }

    pub fn get_numeric_line<T>(&self, number: usize) -> Kb2Result<T> 
    where T: FromStr {
        let res = self.get_line(number)?;
        match res.parse() {
            Ok(l) => Ok(l),
            Err(_) => Err(ErrorType::from("Expected numeric lined"))
        }
    }
}