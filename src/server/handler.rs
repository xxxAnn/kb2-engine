pub trait Handler {
    fn handle(&mut self, recv: impl Into<String>) -> String {
        println!("Received: '{}'", recv.into());

        "ACK".to_string()
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