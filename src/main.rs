use url::Url;
use tungstenite::{connect, Message};

fn main() {
    let (mut socket, response) = connect(
        Url::parse("wss://ws.bitstamp.net").unwrap()
    ).expect("Can't connect");

    socket.write_message(Message::Text(r#"{
        "event": "bts:subscribe",
        "data": {
            "channel": "live_orders_btcusd"
        }
    }"#.into()));

    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}