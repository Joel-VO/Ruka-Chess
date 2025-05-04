<div align="center">

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/Rust-1.85.1%2B-orange.svg)](https://www.rust-lang.org/)


[![GitHub Release](https://img.shields.io/github/v/release/Joel-VO/Ruka-Chess?display_name=tag&color=green&label=Release)](https://github.com/Joel-VO/Ruka-Chess/releases)
</div>

<p align="center">
  <img src="https://github.com/user-attachments/assets/613145a1-e65c-4aca-ba22-5978d9e25e28" alt="Ruka" width="300">
</p>


# Ruka Chess

## Overview

Ruka Chess is a high-performance chess engine developed in Rust. Combining efficiency, speed, and strategic depth, 
Ruka represents my personal approach and passion to computational chess.

## Key Features

- **Search Algorithms**
  - Minimax algorithm with Principal Variation Search (PVS)
  - Alpha-beta pruning
  - Null move pruning
  - Late Move Reduction (LMR)

- **Efficient Data Structures**
  - Bitboard representations for compact position handling(using chess crate)
  - Transposition Tables using Zobrist hashing
  - Optimized move generation and ordering

- **Robust Evaluation**
  - Pesto evaluation method
  - Comprehensive piece-square tables
  - Tactical position analysis

- **Universal Chess Interface (UCI) Compatibility**
  - Integration with major chess platforms
  - Support for common UCI communication protocols

## Installation

### Prerequisites
- Rust programming language (latest stable version)
- Cargo package manager
- A UCI compatible game manager like [CutechessCLI](https://github.com/cutechess/cutechess) or a similar game manager. 

### Cloning the Repository

```shell
git clone https://github.com/Joel-VO/Ruka-Chess.git
cd Ruka-Chess
```

### Running the Engine

#### Linux
```shell
# Run directly
cargo run --release

# Alternative method
cargo build --release
./target/release/Ruka
```
- Next, link the bin file to [CutechessCLI](https://github.com/cutechess/cutechess) or follow the direction provided by [lichess-bot](https://github.com/lichess-bot-devs/lichess-bot?tab=readme-ov-file) to link the engine to lichess.

#### Windows
```shell
# Run directly
cargo run --release

# Alternative method
cargo build --release
Ruka.exe
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

Contributions are welcome! Please submit pull requests or open issues on the GitHub repository. This is a passion project and i intend to develop it further down the line.

I'm learning and continuously doing so, and so helpful and constructive feedback is highly appreciated.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

Image credit: DALL.E 3 and image further edited by [Joel-VO](https://github.com/Joel-VO) in GIMP

## Contact

Project Maintainer: Joel VO
- GitHub: [Joel-VO](https://github.com/Joel-VO)
- Linkedin: [Joel](https://www.linkedin.com/in/joel-oommen-63bb89271/)
