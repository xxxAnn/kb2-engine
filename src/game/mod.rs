mod game_modules;

use game_modules::{Exploit, Summarize, ExploitSummary};

use crate::prelude::{Data, Handler};

pub use game_modules::Summary;

pub struct Game {
    data: Data
}

impl Game {
    pub fn new(data: Data) -> Self {
        Self {
            data
        }
    }

    pub fn data(&mut self) -> &mut Data {
        &mut self.data
    }

    pub fn exploit(&mut self, userid: u64) -> ExploitSummary {
        Exploit::new(self.data(), userid).call()
    }
}

impl Handler for Game {
    fn handle(&mut self, recv: impl Into<String>) -> String {
        let recv_str: String = recv.into();
        let mut data: std::str::Lines<'_> = recv_str.lines();
        let mfirst_line = data.nth(0);
        if let Some(first_line) = mfirst_line {
            match first_line {
                "000" => {
                    match data.nth(0) {
                        Some(userid) => {
                            match userid.parse() {
                                Ok(id) => {
                                    let k = self.exploit(id).text();
                                    k
                                },
                                _ => "User id is not a number".to_owned()
                            }
                        },
                        _ => {
                            "No user id provided".to_owned()
                        }
                    }
                },
                "001" => {
                    match data.nth(0) {
                        Some(userid) => {
                            assert_eq!("331431342438875137", userid);
                            match userid.parse() {
                                Ok(id) => {
                                    self.data().get_player(id).text()
                                },
                                _ => "User id is not a number".to_owned()
                            }
                        },
                        _ => {
                            "No user id provided".to_owned()
                        }
                    }
                }
                _ => {
                    "Malformed request".to_owned()
                }
            }
        } else {
            "Malformed request".to_owned()
        }
    }
}