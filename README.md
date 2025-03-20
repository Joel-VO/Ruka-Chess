# Ruka Chess

## About

Ruka Chess is a high-performance chess engine written in Rust. It is designed to combine efficiency with competitive gameplay through advanced algorithms such as alpha-beta pruning and bitboard representations for compact and efficient position handling. Ruka is fully compatible with the Universal Chess Interface (UCI) protocol, allowing seamless integration with popular chess platforms like lichess.org.

This passion project aims to deliver a robust chess engine capable of high-level performance.

## Installation

Clone the repository locally:

```shell
git clone https://github.com/Joel-VO/Ruka-Chess.git
cd ruka
```
### For Linux 

To run the engine directly:
```shell
cargo run --release
```
Alternatively, compile and then run by:

* Build the project:

```shell
cargo build --release
```

* Run the engine:
```shell
./target/release/ruka
```
### For Windows ###

To run it directly, run:
```shell
cargo run --release
```
Alternatively, compile and then run:

* Build the project:
```shell
cargo build --release
```

* Run the engine:
```shell
ruka.exe
```
## Features ##
* `Chess Crate`: Efficient data structures and functions for managing chess board and move generation.
* `UCI compatible`: Supports UCI format and can connect with any UCI compatible framework.
* `Move ordering`: Uses mover ordering to prioritise checks and captures.
* `MiniMax`: Uses the age-old minimax algorithm for search tree.
* `Principal Variation Search`: Uses PVS to reduce search space without sacrificing accuracy.
* `Reduction algorithms`: Uses LMR to reduce depth to search based on whether the position is tactical or not.
* `Pruning`: Uses alpha beta pruning and Null move pruning.
* `Evaluation`: Uses Pesto evaluation with its piece square tables taken from the Chess Programming Wiki. 
* `Transposition Tables`: Added TT implementation using Zobrist hashing technique to speed up search...

## Usage ##
#### To use it with a chess GUI:

After compiling, run commands in the uci format to get outputs. If you have a locally running chess GUI, use that, else, use the lichess-bot for configuring Ruka.

#### Example UCI Command: ####
uci (server) -> id name Ruka-Chess (engine) -> id author JoelVO (engine) -> uciok (engine)-> isready (server)-> readyok (engine)-> 
```scss
uci                 (server)
id name Ruka-Chess  (engine)
id author JoelVO    (engine)
uciok               (engine)
isready             (server)
readyok             (engine)
ucinewgame          (server)
position startpos   (server)
go maketime 1000    (server)
bestmove e2e4       (engine)
```

#### To play against Ruka:
Go to [lichess.org](#https://lichess.org/@/Ruka-Chess) and challenge her to a game.Ensure the engine is online before issuing a challenge.

