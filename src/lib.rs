use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// Cell states
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Cell {
    Empty,
    X,  // Player
    O,  // AI
}

// Game state
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct GameState {
    board: [Cell; 9],
    current_player: Cell,
}

// Move result structure
#[derive(Serialize, Deserialize)]
pub struct MoveResult {
    pub position: Option<usize>,
    pub game_over: bool,
    pub winner: Option<String>,
    pub winning_line: Option<Vec<usize>>,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameState {
        GameState {
            board: [Cell::Empty; 9],
            current_player: Cell::X,
        }
    }

    // Make a move at the given position
    pub fn make_move(&mut self, position: usize, player: String) -> Result<JsValue, JsValue> {
        if position >= 9 {
            return Err(JsValue::from_str("Invalid position"));
        }

        if self.board[position] != Cell::Empty {
            return Err(JsValue::from_str("Cell already occupied"));
        }

        let cell = if player == "X" { Cell::X } else { Cell::O };
        self.board[position] = cell;

        let result = MoveResult {
            position: Some(position),
            game_over: self.is_game_over(),
            winner: self.get_winner(),
            winning_line: self.get_winning_line(),
        };

        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    // Get AI move using minimax with alpha-beta pruning
    pub fn get_ai_move(&self, difficulty: String) -> Result<JsValue, JsValue> {
        let max_depth = match difficulty.as_str() {
            "easy" => 2,
            "medium" => 4,
            _ => 9,  // hard
        };

        let (best_move, _) = self.minimax(Cell::O, max_depth, std::i32::MIN, std::i32::MAX, true);
        
        let result = MoveResult {
            position: best_move,
            game_over: false,
            winner: None,
            winning_line: None,
        };

        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    // Reset the game
    pub fn reset(&mut self) {
        self.board = [Cell::Empty; 9];
        self.current_player = Cell::X;
    }

    // Get current board state
    pub fn get_board(&self) -> Result<JsValue, JsValue> {
        let board_strings: Vec<String> = self.board.iter().map(|cell| {
            match cell {
                Cell::Empty => "".to_string(),
                Cell::X => "X".to_string(),
                Cell::O => "O".to_string(),
            }
        }).collect();
        
        Ok(serde_wasm_bindgen::to_value(&board_strings)?)
    }

    // Check if position is empty
    pub fn is_empty(&self, position: usize) -> bool {
        position < 9 && self.board[position] == Cell::Empty
    }
}

// Private implementation methods
impl GameState {
    // Minimax algorithm with alpha-beta pruning
    fn minimax(&self, player: Cell, depth: i32, mut alpha: i32, mut beta: i32, is_maximizing: bool) -> (Option<usize>, i32) {
        // Terminal state evaluation
        if let Some(winner) = self.check_winner() {
            let score = if winner == Cell::O { 10 } else { -10 };
            // Adjust score by depth to prefer quicker wins
            let adjusted_score = if score > 0 { score - depth } else { score + depth };
            return (None, adjusted_score);
        }

        if self.is_board_full() {
            return (None, 0); // Draw
        }

        if depth == 0 {
            return (None, self.evaluate_board());
        }

        let mut best_move = None;
        let mut best_score = if is_maximizing { std::i32::MIN } else { std::i32::MAX };

        for i in 0..9 {
            if self.board[i] == Cell::Empty {
                // Make move
                let mut new_state = self.clone();
                new_state.board[i] = player;

                // Recursively evaluate
                let next_player = if player == Cell::X { Cell::O } else { Cell::X };
                let (_, score) = new_state.minimax(next_player, depth - 1, alpha, beta, !is_maximizing);

                // Update best move
                if is_maximizing {
                    if score > best_score {
                        best_score = score;
                        best_move = Some(i);
                    }
                    alpha = alpha.max(best_score);
                } else {
                    if score < best_score {
                        best_score = score;
                        best_move = Some(i);
                    }
                    beta = beta.min(best_score);
                }

                // Alpha-beta pruning
                if beta <= alpha {
                    break;
                }
            }
        }

        (best_move, best_score)
    }

    // Evaluate board position (heuristic for non-terminal states)
    fn evaluate_board(&self) -> i32 {
        let lines = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
            [0, 4, 8], [2, 4, 6],            // Diagonals
        ];

        let mut score = 0;
        for line in &lines {
            let line_score = self.evaluate_line(line[0], line[1], line[2]);
            score += line_score;
        }
        score
    }

    // Evaluate a single line
    fn evaluate_line(&self, a: usize, b: usize, c: usize) -> i32 {
        let mut o_count = 0;
        let mut x_count = 0;

        for &pos in &[a, b, c] {
            match self.board[pos] {
                Cell::O => o_count += 1,
                Cell::X => x_count += 1,
                Cell::Empty => {}
            }
        }

        if x_count > 0 && o_count > 0 {
            0 // Mixed line, no advantage
        } else if o_count > 0 {
            o_count * o_count // AI advantage
        } else if x_count > 0 {
            -(x_count * x_count) // Player advantage
        } else {
            0
        }
    }

    // Check for winner
    fn check_winner(&self) -> Option<Cell> {
        let lines = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
            [0, 4, 8], [2, 4, 6],            // Diagonals
        ];

        for line in &lines {
            if self.board[line[0]] != Cell::Empty &&
               self.board[line[0]] == self.board[line[1]] &&
               self.board[line[1]] == self.board[line[2]] {
                return Some(self.board[line[0]]);
            }
        }
        None
    }

    // Get winning line positions
    fn get_winning_line(&self) -> Option<Vec<usize>> {
        let lines = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
            [0, 4, 8], [2, 4, 6],            // Diagonals
        ];

        for line in &lines {
            if self.board[line[0]] != Cell::Empty &&
               self.board[line[0]] == self.board[line[1]] &&
               self.board[line[1]] == self.board[line[2]] {
                return Some(line.to_vec());
            }
        }
        None
    }

    // Check if board is full
    fn is_board_full(&self) -> bool {
        self.board.iter().all(|&cell| cell != Cell::Empty)
    }

    // Check if game is over
    fn is_game_over(&self) -> bool {
        self.check_winner().is_some() || self.is_board_full()
    }

    // Get winner as string
    fn get_winner(&self) -> Option<String> {
        self.check_winner().map(|winner| {
            match winner {
                Cell::X => "X".to_string(),
                Cell::O => "O".to_string(),
                Cell::Empty => unreachable!(),
            }
        })
    }
}

// Initialize wasm module
#[wasm_bindgen(start)]
pub fn main() {
    // Set panic hook for better error messages in browser
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
} 