use std::io::{Read, Write};
use std::{net::TcpListener};

mod handler;

pub use handler::Handler;
pub use handler::BaseHandler;

use crate::defs::ErrorType;

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


    pub fn serve(&mut self) -> Result<(), ErrorType> {
        let listener = TcpListener::bind(self.__create_binding_addr()).unwrap();

        for strm in listener.incoming() {
            match strm {
                Ok(mut conn) => {
                    let mut buf = [0; 1024];

                    conn.read(&mut buf).unwrap();
                    
                    let recv = std::str::from_utf8(&buf).unwrap_or("");

                    conn.write(self.handler.handle(recv)?.as_bytes()).unwrap();
                },
                _ => {}
            }
        }

        Ok(())
    }
}