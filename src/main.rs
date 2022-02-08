use derivative::Derivative;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::process::exit;
use tungstenite::{connect, Message};
use url::Url;
use std::time::{Duration, Instant};

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
#[derivative(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct LimitPrice {
    price: OrderedFloat<f64>,

    #[derivative(Hash="ignore", PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    size: OrderedFloat<f64>,
    #[derivative(Hash="ignore", PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
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
    amount: f64,
    amount_str: String,
    buy_order_id: u64,
    microtimestamp: String,
    price: f64,
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
    amount: f64,
    amount_str: String,
    #[derivative(Ord = "ignore", PartialEq = "ignore", PartialOrd = "ignore")]
    price: f64,
    price_str: String,
}

fn print_order_book(order_book: &OrderBook) {
    clearscreen::clear().expect("Error clearing screen");
    let mut i = 0;
    for bid in &order_book.bids {
        if order_book.asks.len() > i {
            println!("{:010.2} @ {:08.2}\t{:010.2} @ {:08.2}", bid.size, bid.price, order_book.asks[i].size, order_book.asks[i].price);
        } else {
            println!("{:010.2} @ {:08.2}\t{:010.2} @ {:08.2}\t", bid.size, bid.price, 0.0, 0.0);
        }
        i += 1;
        if i > 50 {
            break;
        }
    }
}

fn main() {

    // Create Order Book data structure
    let mut order_book = OrderBook {
        bids: Vec::new(),
        asks: Vec::new(),
    };

    let mut start = Instant::now();

    let mut x = 0;
    // if let Data::Order(order) = Data::Order(Order { id: 1455821016551424, id_str: "1455821016551424".to_string(), order_type: 0, datetime: "1644260028".to_string(), microtimestamp: "1644260027526000".to_string(), amount: 0.006, amount_str: "0.00600000".to_string(), price: 43921.93, price_str: "43921.93".to_string() }) {
    //     let limit_price = LimitPrice {
    //         price: OrderedFloat(order.price),
    //         size: OrderedFloat(order.amount),
    //         orders: vec![order.clone()],
    //     };
    //     let _value = match order_book.bids.binary_search(&limit_price) {
    //         Ok(i) => {
    //             order_book.bids[i].size += order.amount;
    //             order_book.bids[i].orders.push(order);
    //         }
    //         Err(i) => {
    //             order_book.bids.insert(i, limit_price);
    //         }
    //     };
    // }

    // if let Data::Order(order) = Data::Order(Order{ id: 1455821016551424, id_str: "1455821016551425".to_string(), order_type: 0, datetime: "1644260028".to_string(), microtimestamp: "1644260027526000".to_string(), amount: 0.006, amount_str: "0.00600000".to_string(), price: 43921.93, price_str: "43921.93".to_string() }) {
    //     let limit_price = LimitPrice {
    //         price: OrderedFloat(order.price),
    //         size: OrderedFloat(order.amount),
    //         orders: vec![order.clone()],
    //     };
    //     let _value = match order_book.bids.binary_search(&limit_price) {
    //         Ok(i) => {
    //             order_book.bids[i].size += order.amount;
    //             order_book.bids[i].orders.push(order);
    //         }
    //         Err(i) => {
    //             order_book.bids.insert(i, limit_price);
    //         }
    //     };
    // }
    
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



    loop {
        let msg = socket.read_message().expect("Error reading message");
        let result: Result<Msg, serde_json::Error> = serde_json::from_str(msg.to_text().unwrap());

        let _value = match result {
            Ok(msg) => {
                if msg.event == "bts:subscription_succeeded" {
                } else if msg.event == "trade" {
                } else if msg.event == "order_created" {
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
                                        order_book.bids.insert(i, limit_price.clone());
                                        if order_book.asks.len() > 0 {
                                            if &limit_price.price >= &order_book.asks[0].price {
                                                let mut i = 0;
                                                for ask in order_book.asks.clone() {
                                                    if limit_price.price >= ask.price {
                                                        order_book.asks.remove(0);
                                                    }
                                                    i += 1;
                                                }
                                            }
                                        }
                                    }
                                };
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
                                        if order_book.bids.len() > 0 {
                                            if &limit_price.price >= &order_book.bids[0].price {
                                                let mut i = 0;
                                                for bid in order_book.bids.clone() {
                                                    if limit_price.price >= bid.price {
                                                        order_book.bids.remove(0);
                                                    }
                                                    i += 1;
                                                }
                                            }
                                        }
                                    }
                                };
                            }
                            _ => (),
                        }
                    }
                } else if msg.event == "order_deleted" {
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
                                                if order_book.bids[i].orders.len() == 0 {
                                                    order_book.bids.remove(i);
                                                }
                                            }
                                            Err(_) => ()
                                        };
                                    }
                                    Err(_) => ()
                                };
                                
                                x += 1;
                            }
                            ask if ask == OrderType::Sell as u8 => {
                                let _value = match order_book.asks.binary_search(&limit_price) {
                                    Ok(i) => {
                                        let _value = match order_book.asks[i].orders.binary_search(&order) {
                                            Ok(j) => {
                                                order_book.asks[i].orders.remove(j);
                                                if order_book.asks[i].orders.len() == 0 {
                                                    order_book.asks.remove(i);
                                                }
                                            }
                                            Err(_) => ()
                                        };
                                    }
                                    Err(_) => ()
                                };
                                
                            }
                            _ => (),
                        }
                    }
                } else {
                    println!("UNKNOWN\n{:?}", msg);
                }
            }
            Err(_) => ()
        };
        if start.elapsed().as_millis() > 100 {
            start = Instant::now();
            print_order_book(&order_book);
        }
    }
}
