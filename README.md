# Tic-Tac-Toe with Minimax AI

A modern web-based Tic-Tac-Toe game featuring an AI opponent powered by the Minimax algorithm with alpha-beta pruning, implemented in Rust and compiled to WebAssembly for optimal performance.

![Tic-Tac-Toe Game](https://img.shields.io/badge/Built%20with-Rust%20%2B%20WASM-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## 🎮 Features

- **Unbeatable AI**: Implements the Minimax algorithm with alpha-beta pruning
- **Multiple Difficulty Levels**:
  - Easy: Limited search depth for beginners
  - Medium: Moderate challenge
  - Hard: Perfect play - the AI never loses!
- **Modern UI**: Clean, responsive design with smooth animations
- **Performance**: Rust/WASM ensures instant AI responses
- **Score Tracking**: Keeps track of wins, losses, and draws

## 🚀 Demo

Play the game by building and running it locally (instructions below).

## 🛠️ Technology Stack

- **Frontend**: HTML5, CSS3, Vanilla JavaScript
- **AI Logic**: Rust
- **Runtime**: WebAssembly (WASM)
- **Build Tool**: wasm-pack

## 📋 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Python 3 (for local web server) or any static file server

## 🔧 Installation & Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/rexbrahh/tictactoe-rust-wasm.git
   cd tictactoe
   ```

2. **Build the WebAssembly module**
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

3. **Start a local web server**
   ```bash
   python3 -m http.server 8000
   ```

4. **Open the game**
   
   Navigate to `http://localhost:8000` in your web browser

### Alternative: Use the build script

```bash
chmod +x build.sh
./build.sh
```

## 🎯 How to Play

1. You play as **X**, the AI plays as **O**
2. Click any empty cell to make your move
3. The AI will respond immediately
4. Try to get three in a row horizontally, vertically, or diagonally
5. Use the difficulty selector to change the AI's strength
6. Click "New Game" to reset the board

## 🧠 Algorithm Details

The AI uses the **Minimax algorithm** with **alpha-beta pruning**:

- **Minimax**: Recursively evaluates all possible game states
- **Alpha-Beta Pruning**: Eliminates branches that won't affect the final decision
- **Evaluation Function**: Scores positions based on winning potential
- **Depth Limiting**: Controls difficulty by limiting search depth

## 📁 Project Structure

```
tictactoe/
├── index.html          # Game UI
├── styles.css          # Styling
├── script.js           # Frontend logic
├── Cargo.toml          # Rust dependencies
├── src/
│   └── lib.rs          # Minimax AI implementation
├── pkg/                # Generated WASM output
└── build.sh            # Build script
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with Rust and WebAssembly
- Inspired by classic game AI algorithms
- UI design influenced by modern web aesthetics 