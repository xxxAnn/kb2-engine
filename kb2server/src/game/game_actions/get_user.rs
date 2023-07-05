use kb2lib::prelude::Dump;

use crate::{Data, game::message::GameMessage, Result, Error};

use super::{Summarize};

pub struct GetUser<'a> {
    data: &'a mut Data,
    userid: u64   
}

impl<'a> GetUser<'a> {
    pub fn new(data: &'a mut Data, userid: u64) -> Self {
        Self {
            data,
            userid
        }
    }
}

impl<'a> Summarize<'a> for GetUser<'a> {
    type ResultSummary = String;

    fn call(self) -> Result<String> {
        let user = self.data.player_mut(self.userid)
            .ok_or(Error::CantCreateUser)?;

        Ok(user.dump())
    }

    fn from_message(data: &'a mut Data, gm: &GameMessage) -> Result<Self> {
        Ok(GetUser::new(data, gm.get_numeric_line(1)?))
    }
}

