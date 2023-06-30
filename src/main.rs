mod handler;
mod server;
mod data;
mod parser;

const LOCAL_ADDR: &'static str = "127.0.0.1";
const LOCAL_PORT: u16 = 7878;

fn main() {
    let mut data = data::Data::new();
    let mut ann = data.get_player(331431342438875137);

    //ann.add_item(3, 47, &data);
    //ann.save(&data);
    //dbg!(ann.exploit(&data));

    dbg!(ann);

    server::Server::new(
        LOCAL_ADDR,
        LOCAL_PORT,
        handler::BaseHandler::new()
        )
        .serve()
        .unwrap();
}

