use std::io::{Read, Write};
use std::{net::TcpListener};

mod handler;

pub use handler::Handler;
pub use handler::BaseHandler;

use crate::defs::{ErrorType, Kb2Result};

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
            handler
        }
    }

    fn __create_binding_addr(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }


    pub fn serve(&mut self) -> Kb2Result<()> {
        let listener = TcpListener::bind(self.__create_binding_addr()).unwrap();

        for strm in listener.incoming() {
            if let Ok(mut conn) = strm {
                let mut buf = [0; 1024];

                conn.read(&mut buf).unwrap_or_default();
                
                let recv = std::str::from_utf8(&buf).unwrap_or("");

                let res = match self.handler.handle(recv) {
                    Ok(s) => s,
                    Err(s) => format!("{}", s)
                };

                conn.write(res.as_bytes())?;
            }
        }

        Ok(())
    }
}