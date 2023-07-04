use crate::{prelude::Data, game::message::GameMessage, defs::{Kb2Result}};

use super::{Summary, Summarize};

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

    fn call(self) -> Kb2Result<String> {
        let user = self.data.player_mut(self.userid);

        Ok(user.text())
    }

    fn from_message(data: &'a mut Data, gm: &GameMessage) -> Kb2Result<Self> {
        Ok(GetUser::new(data, gm.get_numeric_line(1)?))
    }
}

