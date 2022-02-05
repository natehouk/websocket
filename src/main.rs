use serde::{Deserialize, Serialize};
use tungstenite::{connect, Message};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct Trade {
    data: Data,
    channel: String,
    event: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    id: u32,
    amount: f32,
    amount_str: String,
    buy_order_id: u64,
    microtimestamp: String,
    price: f32,
    price_str: String,
    sell_order_id: u64,
    timestamp: String,
    #[serde(rename = "type")]
    _type: u8,
}

fn main() {
    let (mut socket, _response) =
        connect(Url::parse("wss://ws.bitstamp.net").unwrap()).expect("Can't connect");

    socket
        .write_message(Message::Text(
            r#"{
        "event": "bts:subscribe",
        "data": {
            "channel": "live_trades_btcusd"
        }
    }"#
            .into(),
        ))
        .expect("Error sending message");

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let result: Result<Trade, serde_json::Error> = serde_json::from_str(msg.to_text().unwrap());

        let value = match result {
            Ok(msg) => msg,
            Err(err) => {
                println!("{:?}", err);
                continue;
            }
        };
        println!("{:?}", value);
    }
}
