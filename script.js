// Import the WASM module
let wasmModule;
let gameState;

// Game state variables
let isPlayerTurn = true;
let gameActive = true;
let scores = {
    player: 0,
    ai: 0,
    draws: 0
};

// Initialize the game when the DOM is loaded
document.addEventListener('DOMContentLoaded', async () => {
    await initializeWasm();
    setupEventListeners();
    updateUI();
});

// Initialize WASM module
async function initializeWasm() {
    try {
        // Import the generated WASM module
        wasmModule = await import('./pkg/tictactoe_ai.js');
        await wasmModule.default();
        
        // Create a new game state
        gameState = new wasmModule.GameState();
        
        console.log('WASM module loaded successfully');
    } catch (error) {
        console.error('Failed to load WASM module:', error);
        document.getElementById('game-status').textContent = 
            'Error loading game engine. Please refresh the page.';
    }
}

// Setup event listeners
function setupEventListeners() {
    // Cell click events
    const cells = document.querySelectorAll('.cell');
    cells.forEach(cell => {
        cell.addEventListener('click', handleCellClick);
    });

    // New game button
    document.getElementById('new-game').addEventListener('click', resetGame);

    // Difficulty change
    document.getElementById('difficulty').addEventListener('change', resetGame);
}

// Handle cell click
async function handleCellClick(event) {
    if (!gameActive || !isPlayerTurn || !wasmModule) return;

    const cell = event.target;
    const position = parseInt(cell.dataset.index);

    // Check if cell is empty
    if (!gameState.is_empty(position)) return;

    // Make player move
    try {
        const result = gameState.make_move(position, 'X');
        const moveResult = result;  // Already a JS object from WASM
        
        // Update UI with player move
        updateCell(position, 'X');
        
        // Check game state
        if (moveResult.game_over) {
            handleGameOver(moveResult);
            return;
        }

        // Switch to AI turn
        isPlayerTurn = false;
        updateStatus('AI is thinking...');
        disableBoard();

        // Small delay for better UX
        setTimeout(() => makeAIMove(), 500);

    } catch (error) {
        console.error('Error making move:', error);
    }
}

// Make AI move
async function makeAIMove() {
    if (!gameActive || !wasmModule) return;

    try {
        const difficulty = document.getElementById('difficulty').value;
        const aiMoveResult = gameState.get_ai_move(difficulty);
        const aiMove = aiMoveResult;  // Already a JS object from WASM

        if (aiMove.position !== null && aiMove.position !== undefined) {
            // Make the AI move
            const result = gameState.make_move(aiMove.position, 'O');
            const moveResult = result;  // Already a JS object from WASM
            
            // Update UI with AI move
            updateCell(aiMove.position, 'O');
            
            // Check game state
            if (moveResult.game_over) {
                handleGameOver(moveResult);
                return;
            }
        }

        // Switch back to player turn
        isPlayerTurn = true;
        enableBoard();
        updateStatus('Your turn!');

    } catch (error) {
        console.error('Error making AI move:', error);
        isPlayerTurn = true;
        enableBoard();
        updateStatus('Error occurred. Your turn!');
    }
}

// Update cell display
function updateCell(position, symbol) {
    const cell = document.querySelector(`[data-index="${position}"]`);
    cell.textContent = symbol;
    cell.classList.add('occupied', symbol.toLowerCase());
}

// Handle game over
function handleGameOver(result) {
    gameActive = false;
    disableBoard();

    if (result.winner) {
        if (result.winner === 'X') {
            updateStatus('🎉 You win!');
            scores.player++;
        } else {
            updateStatus('🤖 AI wins!');
            scores.ai++;
        }

        // Highlight winning line
        if (result.winning_line) {
            result.winning_line.forEach(pos => {
                document.querySelector(`[data-index="${pos}"]`).classList.add('winning');
            });
        }
    } else {
        updateStatus("It's a draw!");
        scores.draws++;
    }

    updateScoreboard();
}

// Reset the game
function resetGame() {
    if (!wasmModule) return;

    // Reset game state
    gameState.reset();
    gameActive = true;
    isPlayerTurn = true;

    // Reset UI
    const cells = document.querySelectorAll('.cell');
    cells.forEach(cell => {
        cell.textContent = '';
        cell.classList.remove('occupied', 'x', 'o', 'winning', 'disabled');
    });

    updateStatus('Your turn! Click any cell to start.');
    updateUI();
}

// Update game status
function updateStatus(message) {
    document.getElementById('game-status').textContent = message;
}

// Update scoreboard
function updateScoreboard() {
    document.getElementById('player-score').textContent = scores.player;
    document.getElementById('ai-score').textContent = scores.ai;
    document.getElementById('draw-score').textContent = scores.draws;
}

// Disable board interaction
function disableBoard() {
    document.querySelectorAll('.cell').forEach(cell => {
        cell.classList.add('disabled');
    });
}

// Enable board interaction
function enableBoard() {
    document.querySelectorAll('.cell').forEach(cell => {
        if (!cell.classList.contains('occupied')) {
            cell.classList.remove('disabled');
        }
    });
}

// Update UI based on current game state
function updateUI() {
    if (!wasmModule || !gameState) return;

    try {
        const boardState = gameState.get_board();
        const board = boardState;  // Already a JS array from WASM

        board.forEach((symbol, index) => {
            if (symbol) {
                updateCell(index, symbol);
            }
        });
    } catch (error) {
        console.error('Error updating UI:', error);
    }
} 