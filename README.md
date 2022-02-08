# websocket

A simple toy websocket to connect to Bitstamp.net and print the live order book written in Rust.

## How to Run
To run the project, first install the Rust tool chain: https://www.rust-lang.org/tools/install

Then run the following commands:

```
$ git clone git@github.com:natehouk/websocket.git
$ cd websocket
$ cargo run
```

## Example Output

```
Status  : Connected
Exchange: Bitstamp.net
Symbol  : BTC/USD

         Bids                    Asks
0.01373496 @ 43005.37	0.10500000 @ 42981.09
0.08933164 @ 42997.33	0.60973700 @ 42994.87
0.23461279 @ 42996.27	0.14521883 @ 42997.23
0.00437412 @ 42993.72	0.64744221 @ 42999.91
4.66843494 @ 42989.99	1.11188394 @ 42999.92
0.01300000 @ 42988.94	0.11630924 @ 43000.88
0.11632652 @ 42982.02	0.58445243 @ 43003.49
0.13824516 @ 42981.09	0.18602231 @ 43004.21
0.11332652 @ 42980.95	0.11623247 @ 43006.09
0.11632545 @ 42972.40	0.13600000 @ 43006.59
0.15119658 @ 42972.19	1.00000000 @ 43008.09
```

## Reference Implementation
https://assets.bitstamp.net/static/webapp/examples/diff_order_book_v2.5dabf12d7fe2ab5e69be923d5f1b1cf6e8d3bafb.html
