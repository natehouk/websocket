use derivative::Derivative;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::process::exit;
use tungstenite::{connect, Message};
use url::Url;

use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

enum OrderType {
    Buy = 0,
    Sell = 1,
}

#[derive(Debug)]
struct OrderBook {
    bids: Vec<LimitPrice>,
    asks: Vec<LimitPrice>,
}

#[derive(Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct LimitPrice {
    price: OrderedFloat<f32>,

    #[derivative(Hash="ignore", PartialEq = "ignore", PartialOrd = "ignore")]
    size: OrderedFloat<f32>,
    #[derivative(Hash="ignore", PartialEq = "ignore", PartialOrd = "ignore")]
    orders: Vec<Order>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Data {
    Trade(Trade),
    Order(Order),
    None {},
}

#[derive(Serialize, Deserialize, Debug)]
struct Msg {
    channel: String,
    event: String,
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
struct Trade {
    id: u64,
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

#[derive(Derivative, Serialize, Deserialize, Clone)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Order {
    id: u64,
    id_str: String,
    order_type: u8,
    datetime: String,
    microtimestamp: String,
    #[derivative(Ord = "ignore", PartialEq = "ignore", PartialOrd = "ignore")]
    amount: f32,
    amount_str: String,
    #[derivative(Ord = "ignore", PartialEq = "ignore", PartialOrd = "ignore")]
    price: f32,
    price_str: String,
}

fn main() {
    let (mut socket, _response) =
        connect(Url::parse("wss://ws.bitstamp.net").unwrap()).expect("Can't connect");

    socket
        .write_message(
            Message::Text(
                json!({
                    "event": "bts:subscribe",
                    "data": {
                        "channel": "live_trades_btcusd"
                    }
                })
                .to_string(),
            )
            .into(),
        )
        .expect("Error sending message");

    socket
        .write_message(
            Message::Text(
                json!({
                    "event": "bts:subscribe",
                    "data": {
                        "channel": "live_orders_btcusd"
                    }
                })
                .to_string(),
            )
            .into(),
        )
        .expect("Error sending message");

    let mut order_book = OrderBook {
        bids: Vec::new(),
        asks: Vec::new(),
    };
    let mut x = 0;

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let result: Result<Msg, serde_json::Error> = serde_json::from_str(msg.to_text().unwrap());

        let _value = match result {
            Ok(msg) => {
                if msg.event == "bts:subscription_succeeded" {
                    println!("CONNECTED\n{:?}", msg);
                } else if msg.event == "trade" {
                    println!("TRADE\n{:?}", msg);
                } else if msg.event == "order_created" {
                    println!("ORDER CREATED\n{:?}", msg.data);
                    if let Data::Order(order) = msg.data {
                        let limit_price = LimitPrice {
                            price: OrderedFloat(order.price),
                            size: OrderedFloat(order.amount),
                            orders: vec![order.clone()],
                        };
                        match (&order).order_type {
                            buy if buy == OrderType::Buy as u8 => {
                                let _value = match order_book.bids.binary_search(&limit_price) {
                                    Ok(i) => {
                                        order_book.bids[i].size += order.amount;
                                        order_book.bids[i].orders.push(order);
                                    }
                                    Err(i) => {
                                        order_book.bids.insert(i, limit_price);
                                    }
                                };
                                println!("ORDER BOOK\n{:?}", order_book);
                                x += 1;
                            }
                            ask if ask == OrderType::Sell as u8 => {
                                let _value = match order_book.asks.binary_search(&limit_price) {
                                    Ok(i) => {
                                        order_book.asks[i].size += order.amount;
                                        order_book.asks[i].orders.push(order);
                                    }
                                    Err(i) => {
                                        order_book.asks.insert(i, limit_price);
                                    }
                                };
                                println!("ORDER BOOK\n{:?}", order_book)
                            }
                            _ => (),
                        }
                    }
                } else if msg.event == "order_deleted" {
                    println!("ORDER DELETED\n{:?}", msg);
                    if let Data::Order(order) = msg.data {
                        let limit_price = LimitPrice {
                            price: OrderedFloat(order.price),
                            size: OrderedFloat(order.amount),
                            orders: vec![order.clone()],
                        };
                        match (&order).order_type {
                            buy if buy == OrderType::Buy as u8 => {
                                let _value = match order_book.bids.binary_search(&limit_price) {
                                    Ok(i) => {
                                        let _value = match order_book.bids[i].orders.binary_search(&order) {
                                            Ok(j) => {
                                                order_book.bids[i].orders.remove(j);
                                                pause();
                                            }
                                            Err(_j) => {
                                                println!("UNABLE TO DELETE ORDER ID {} error {}", order.id, _j);
                                            }
                                        };
                                    }
                                    Err(_i) => {
                                        println!("UNABLE TO DELETE ORDER ID {} error {}", order.id, _i);
                                    }
                                };
                                println!("\n\nORDER BOOK\n\n{:?}\n\n\n", order_book);
                                
                                x += 1;
                            }
                            ask if ask == OrderType::Sell as u8 => {
                                let _value = match order_book.asks.binary_search(&limit_price) {
                                    Ok(i) => {
                                        let _value = match order_book.asks[i].orders.binary_search(&order) {
                                            Ok(j) => {
                                                order_book.asks[i].orders.remove(j);
                                                pause();
                                            }
                                            Err(_j) => {
                                                println!("UNABLE TO DELETE ORDER ID {} error {}", order.id, _j);
                                            }
                                        };
                                    }
                                    Err(_i) => {
                                        println!("UNABLE TO DELETE ORDER ID {} error {}", order.id, _i);
                                    }
                                };
                                println!("\n\nORDER BOOK\n\n{:?}\n\n\n", order_book);
                                
                            }
                            _ => (),
                        }
                    }
                } else {
                    println!("UNKNOWN\n{:?}", msg);
                }
            }
            Err(err) => {
                println!("ERROR\n{:?}", err);
            }
        };
        //if x == 2 {
        //    exit(0);
        //}
    }
}
