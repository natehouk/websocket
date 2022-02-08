# websocket

A simple toy websocket client to connect to Bitstamp.net and print the live order book written in Rust.

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
Time    : Tue Feb  8 19:22:53 2022 UTC

         Bids                    Asks
0.14540874 @ 42969.98	0.11634953 @ 42985.93
0.00283174 @ 42967.49	0.05000000 @ 42989.13
0.38757490 @ 42962.64	0.11635309 @ 42989.28
0.51000000 @ 42962.63	0.34000000 @ 42991.13
0.11634891 @ 42961.24	0.14527510 @ 42992.32
0.11635191 @ 42954.50	0.10157125 @ 42992.84
0.13600000 @ 42952.78	0.37730512 @ 42993.26
0.18614313 @ 42947.92	0.72126711 @ 42994.07
0.18615153 @ 42947.54	0.19910374 @ 42995.29
0.36223589 @ 42942.98	0.15124791 @ 42996.79
0.30996891 @ 42941.41	0.18614790 @ 43002.48
```

## Reference Implementation
https://assets.bitstamp.net/static/webapp/examples/diff_order_book_v2.5dabf12d7fe2ab5e69be923d5f1b1cf6e8d3bafb.html

## Reference API
https://www.bitstamp.net/websocket/v2/
