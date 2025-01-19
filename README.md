## Ruka Chess ##
A passion project written in Rust, Ruka is a chess engine that we hope can perform at a high level.
## Installation ##
First install Rust and cargo using given commands
For Linux/Mac
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
For Windows, follow the guide on the official rust webpage.
Ruka Chess Engine
Ruka is a high-performance chess engine written in Rust, designed to combine efficiency with competitive gameplay. It leverages advanced algorithms like alpha-beta pruning and employs bitboard representation for compact and efficient handling of chess positions.

Ruka interfaces seamlessly with the Universal Chess Interface (UCI) protocol, making it compatible with popular chess platforms like lichess.org.

Features
Bitboard Representation: Efficient data structure for managing chess positions.
Alpha-Beta Pruning: Optimized move selection with search depth control.
Rust Crates Integration: Leveraging specialized crates for move generation and FEN parsing.
UCI Compatibility: Supports standard chess communication protocols for testing and gameplay.
Compact Design: Prioritizes speed and resource efficiency.
Getting Started
Prerequisites
Rust (latest stable version)
A UCI-compatible chess GUI (e.g., Arena or Cute Chess)
Installation
Clone the repository:

bash
Copy
Edit
git clone https://github.com/yourusername/ruka.git
cd ruka
Build the project:

bash
Copy
Edit
cargo build --release
Run the engine:

bash
Copy
Edit
./target/release/ruka
Usage
Ruka communicates via the UCI protocol. To use it with a chess GUI:

Add Ruka as a new engine in your preferred GUI.
Provide a FEN position or start a new game.
Ruka will generate moves and respond to commands such as go and stop.
Example UCI Command:

uci
Copy
Edit
position startpos moves e2e4
go
Ruka will return its best move in UCI format, e.g., e7e5.

Development Roadmap
 Implement transposition tables for enhanced move selection.
 Add quiescence search for better tactical evaluation.
 Introduce neural network evaluation for strategic depth.
 Perform testing on lichess.org to gauge Elo rating.
Contributing
Contributions are welcome! Please feel free to submit issues or pull requests. Ensure your code adheres to the project's formatting and testing guidelines.

License
Ruka is licensed under the MIT License.

Acknowledgments
Inspired by the vibrant chess and Rust programming communities.
Thanks to lichess.org for providing a platform for testing and analysis.
