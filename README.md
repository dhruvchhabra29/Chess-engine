# Rust Chess Engine

## Overview
A command-line chess engine implemented in Rust using the [chess crate](https://crates.io/crates/chess) for game logic.

## Features
- CLI interface for move input
- Chess game logic via chess crate
- Board display and move validation

## Setup
1. Install Rust via rustup:
   ```bash
   curl --proto =https://sh.rustup.rs -sSf | sh
   ```
2. Add chess crate:
   ```bash
   cargo add chess
   ```
3. Build the project:
   ```bash
   cargo build
   ```

## Usage
```bash
cargo run
```

### Controls
- Enter moves in standard algebraic notation (e.g., e2e4)
- Type "quit" to exit

### Example
```
Welcome to the chess engine!

Current board:
rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Enter your move (e.g., e2e4): e2e4
Move applied.
```

## Notes
- Requires Rust 1.100.0+
- Uses chess crate v3.2.0
- Remove unused dependency `anyhow` from Cargo.toml if not needed

## Repository
[View on GitHub](https://github.com/dhruvchhabra29/Chess-engine.git)
