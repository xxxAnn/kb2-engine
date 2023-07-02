use crate::prelude::{Data, Item};

pub trait Summary {
    fn text(&self) -> String;
}

pub trait Summarize {
    type ResultSummary: Summary;

    fn call(self) -> Self::ResultSummary;
}

pub struct Exploit<'a> {
    data: &'a mut Data,
    userid: u64   
}

pub struct ExploitSummary {
    item_obtained: Item,
    quantity_obtained: u64
}

impl ExploitSummary {
    fn new(item_obtained: Item, quantity_obtained: u64) -> ExploitSummary {
        Self { 
            item_obtained,
            quantity_obtained
        }
    }
}

impl Summary for ExploitSummary {
    fn text(&self) -> String {
        format!("{}\r\n{}\r\n{}", "exploit_summary_", self.quantity_obtained, self.item_obtained.as_string())
    }
}

impl<'a> Exploit<'a> {
    pub fn new(data: &'a mut Data, userid: u64) -> Self {
        Self {
            data,
            userid
        }
    }
}

impl Summarize for Exploit<'_> {
    type ResultSummary = ExploitSummary;

    fn call(self) -> ExploitSummary {
        let gamedata = self.data.gamedata();
        let mut user = self.data.get_player(self.userid);

        let temp = user.exploit(&gamedata);
        let res = temp
            .first()
            .unwrap();

        ExploitSummary::new(res.0.clone(), res.1)
    }
}