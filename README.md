# Ruka Chess #
## About ##
Ruka is a high-performance chess engine written in Rust, designed to combine efficiency with competitive gameplay. It leverages advanced algorithms like alpha-beta pruning and employs bitboard representation for compact and efficient handling of chess positions.
Ruka interfaces seamlessly with the Universal Chess Interface (UCI) protocol, making it compatible with popular chess platforms like lichess.org.
A passion project of ours, Ruka is a chess engine that we hope can perform at a high level.
## Installation ##
Clone the repository:

```shell
git clone https://github.com/Joel-VO/Ruka-Chess.git
cd ruka
```
To run it directly, run:
```shell
cargo run --release
```
Or, you can do as follows:
Build the project:

```shell
cargo build --release
```

Run the engine:
```shell
./target/release/ruka
```

## Features ##
* `Bitboard Representation`: Efficient data structure for managing chess positions.
* `Alpha-Beta Pruning`: Optimized move selection with search depth control.
* `Transposition Table`: 
* `Rust Crates Integration`: Leveraging specialized crates for move generation and FEN parsing.
* `UCI Compatibility`: Supports standard chess communication protocols for testing and gameplay.
* `Compact Design`: Prioritizes speed and resource efficiency.
## Getting Started ##
### Prerequisites ###
Rust (latest stable version)
A UCI-compatible chess GUI (e.g., Arena or Cute Chess).

## Usage ##
#### To use it with a chess GUI:

Add Ruka as a new engine in your preferred GUI.
Provide a FEN position or start a new game.
Ruka will generate moves and respond to commands such as go and stop.
Example UCI Command:

#### To play against Ruka:
Go to [lichess.org](#https://lichess.org/) and go to [Ruka](#https://lichess.org/@/Ruka-Chess) or search directly for ruka in the search bar above.
Challenge her to a game and she'll play her best :)
