use crate::{Data, game::message::GameMessage, Result, Error};

use super::{Summarize};

pub struct GetLocation<'a> {
    data: &'a mut Data,
    userid: u64   
}

impl<'a> GetLocation<'a> {
    pub fn new(data: &'a mut Data, userid: u64) -> Self {
        Self {
            data,
            userid
        }
    }
}

impl<'a> Summarize<'a> for GetLocation<'a> {
    type ResultSummary = String;

    fn call(self) -> Result<String> {
        let mut map = self.data.map();
        let user = self.data.player_mut(self.userid)
            .ok_or(Error::CantCreateUser)?;
        
        let tile = user.my_tile(&mut map);

        Ok(format!("get_location_\r\n{}\r\n", u64::from(tile)))
    }

    fn from_message(data: &'a mut Data, gm: &GameMessage) -> Result<Self> {
        Ok(GetLocation::new(data, gm.get_numeric_line(1)?))
    }
}

