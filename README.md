# Ruka Chess #
## About ##
Ruka is a high-performance chess engine written in Rust, designed to combine efficiency with competitive gameplay. It leverages advanced algorithms like alpha-beta pruning and employs bitboard representation for compact and efficient handling of chess positions.
Ruka interfaces seamlessly with the Universal Chess Interface (UCI) protocol, making it compatible with popular chess platforms like lichess.org.
A passion project of ours, Ruka is a chess engine that we hope can perform at a high level.
## Installation ##
First, clone the repository locally using:

```shell
git clone https://github.com/Joel-VO/Ruka-Chess.git
cd ruka
```
This works for both Windows(after installing git) / Linux

### For Linux ###

To run it directly, run:
```shell
cargo run --release
```
Or for compiling and then running:

Build the project:

```shell
cargo build --release
```

Run the engine:
```shell
./target/release/ruka
```
### For Windows ###

To run it directly, run:
```shell
cargo run --release
```
Or for compiling and then running:

Build the project:
```shell
cargo build --release
```

Run the engine:
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

ucinewgame (server) -> position startpos (server) -> go maketime 1000 (server) -> bestmove e2e4 (engine)
#### To play against Ruka:
Go to [lichess.org](#https://lichess.org/@/Ruka-Chess) and challenge her to a game. Check if she's online before issuing her a challenge. 

