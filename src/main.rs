mod handler;
mod server;
mod data;
mod parser;

const local_addr: &'static str = "127.0.0.1";
const local_port: u16 = 7878;

fn main() {
    let mut data = data::Data::new();
    let ann = data.get_player(331431342438875137);

    //ann.add_item(2, 47, &data);
    //ann.save(&data);

    dbg!(ann);
    //dbg!(data);

    server::Server::new(
        local_addr,
        local_port,
        handler::BaseHandler::new()
        )
        .serve()
        .unwrap();
}

