use std::str::FromStr;

pub struct GameMessage {
    code: u16,
    data: Vec<String> 
}

pub enum Dispatcher {
    Exploit,
    GetUser,
    GetRecipes,
    GetRecipe,
    Unknown
}

impl Dispatcher {
    pub fn from_code(c: u16) -> Self {
        match c {
            0 => Dispatcher::Exploit,
            1 => Dispatcher::GetUser,
            2 => Dispatcher::GetRecipes,
            3 => Dispatcher::GetRecipe,
            _ => Dispatcher::Unknown
        }
    }
}

impl GameMessage {
    pub fn new(text: String) -> Result<Self, String> {
        let mut data = text.lines();
        if let Some(code_str) = data.nth(0) {
            if let Ok(code) = code_str.parse::<u16>() {
                Ok(Self {
                    code,
                    data: data.map(|s| s.to_owned()).collect()
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

    pub fn get_line(&self, number: usize) -> Result<String, String> {
        match self.data.get(number-1) {
            Some(line) => Ok(line.to_owned()),
            None => Err("Malformed request".to_owned())
        }
    }

    pub fn get_numeric_line<T>(&self, number: usize) -> Result<T, String> 
    where T: FromStr {
        let res = self.get_line(number)?;
        match res.parse() {
            Ok(l) => Ok(l),
            Err(_) => Err("Expected numeric lined".to_owned())
        }
    }
}