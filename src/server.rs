use std::{net::TcpListener};

use crate::handler::Handler;

pub struct Server<T>
where T: Handler {
    addr: String,
    port: u16,
    handler: T
}

impl<T> Server<T> 
where T: Handler {
    pub fn new(addr: impl Into<String>, port: impl Into<u16>, handler: T) -> Self {
        Self {
            addr: addr.into(),
            port: port.into(),
            handler: handler.into()
        }
    }

    fn __create_binding_addr(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }


    pub fn serve(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.__create_binding_addr())?;

        for strm in listener.incoming() {
            match strm {
                Ok(mut conn) => {
                    self.handler.handle(&mut conn);
                },
                _ => {}
            }
        }

        Ok(())
    }
}