<div align="center">

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/Rust-1.85.1%2B-orange.svg)](https://www.rust-lang.org/)


[![GitHub Release](https://img.shields.io/github/v/release/Joel-VO/Ruka-Chess?color=green&label=Release)](https://github.com/Joel-VO/Ruka-Chess/releases)
</div>

# Ruka Chess Engine

## Overview

Ruka Chess is a high-performance chess engine developed in Rust, designed to deliver exceptional gameplay through cutting-edge algorithmic techniques. Combining efficiency, speed, and strategic depth, Ruka represents a sophisticated approach to computational chess.

## Key Features

- **Advanced Search Algorithms**
  - Minimax algorithm with Principal Variation Search (PVS)
  - Alpha-beta pruning
  - Null move pruning
  - Late Move Reduction (LMR)

- **Efficient Data Structures**
  - Bitboard representations for compact position handling
  - Transposition Tables using Zobrist hashing
  - Optimized move generation and ordering

- **Robust Evaluation**
  - Pesto evaluation method
  - Comprehensive piece-square tables
  - Tactical position analysis

- **Universal Chess Interface (UCI) Compatibility**
  - Seamless integration with major chess platforms
  - Support for standard UCI communication protocols

## Installation

### Prerequisites
- Rust programming language (latest stable version)
- Cargo package manager
- A UCI compatible game manager like [CutechessCLI](https://github.com/cutechess/cutechess) or a similar game manager. 

### Cloning the Repository

```shell
git clone https://github.com/Joel-VO/Ruka-Chess.git
cd ruka
```

### Running the Engine

#### Linux
```shell
# Run directly
cargo run --release

# Alternative method
cargo build --release
./target/release/ruka
```
- Next, link the bin file to [CutechessCLI](https://github.com/cutechess/cutechess) or follow the direction provided by [lichess-bot](https://github.com/lichess-bot-devs/lichess-bot?tab=readme-ov-file) to link the engine to lichess.

#### Windows
```shell
# Run directly
cargo run --release

# Alternative method
cargo build --release
ruka.exe
```

- Next, link the bin file to [CutechessCLI](https://github.com/cutechess/cutechess) or follow the direction provided by [lichess-bot](https://github.com/lichess-bot-devs/lichess-bot?tab=readme-ov-file) to link the engine to lichess.

## Usage

### Chess GUI Integration

Ruka supports standard UCI protocol commands. Here's a typical interaction sequence:

```
uci                 # Initialize UCI protocol
-> id name Ruka-Chess
-> id author JoelVO
-> uciok

isready             # Check engine readiness
-> readyok

ucinewgame          # Start a new game
position startpos   # Set starting position
go movetime 1000    # Calculate best move with 1-second time limit
-> bestmove e2e4    # Engine's recommended move
```

### Online Play

Challenge Ruka on [Lichess.org](https://lichess.org/@/Ruka-Chess)

**Note:** Ensure the engine is online before initiating a game.

## Contributing

Contributions are welcome! Please submit pull requests or open issues on the GitHub repository. This is a passion project and i intend to develop it further down the line. However if my code is not upto profession standards, kindly excuse my lack of knowledge :)

I'm learning and continuously doing so, and so helpful and constructive feedback is highly appreciated.

Also, ignore a lot of the commented/redundant code, i'll clean that up in the upcoming patch.


## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.


## Contact

Project Maintainer: Joel VO
- GitHub: [Joel-VO](https://github.com/Joel-VO)
