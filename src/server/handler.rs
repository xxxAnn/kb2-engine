use crate::defs::{ErrorType, Kb2Result};

pub trait Handler {
    fn handle(&mut self, recv: impl Into<String>) -> Kb2Result<String> {
        println!("Received: '{}'", recv.into());

        Ok("ACK".to_string())
    }
}

pub struct BaseHandler;

impl Handler for BaseHandler {}

#[allow(dead_code)]
impl BaseHandler {
    pub fn new() -> Self {
        Self {}
    }
}