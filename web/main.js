import init, { ChessCompiler } from "./pkg/chess_wasm.js";

let compiler;
let selected = null;

const board = document.getElementById("board");
const status = document.getElementById("status");

const pieces = {
    K: "♔",
    Q: "♕",
    R: "♖",
    B: "♗",
    N: "♘",
    P: "♙",

    k: "♚",
    q: "♛",
    r: "♜",
    b: "♝",
    n: "♞",
    p: "♟"
};

function drawBoard() {
    const state = compiler.board();

    board.innerHTML = "";

    for (let row = 7; row >= 0; row--) {
        for (let col = 0; col < 8; col++) {

            const square = document.createElement("div");

            square.className =
                "square " +
                ((row + col) % 2 === 0 ? "light" : "dark");

            const index = row * 8 + col;
            const piece = state[index];

            if (piece !== ".") {
                square.textContent = pieces[piece];
            }

            if (
                selected &&
                selected.row === row &&
                selected.col === col
            ) {
                square.classList.add("selected");
            }

            square.onclick = () => selectSquare(row, col);

            board.appendChild(square);
        }
    }

    status.textContent =
        compiler.game_over()
            ? "Game Over"
            : `${capitalize(compiler.turn())} to move`;
}

function selectSquare(row, col) {
    if (compiler.game_over()) {
        return;
    }

    if (!selected) {
        selected = { row, col };
        drawBoard();
        return;
    }

    const from = squareName(selected.row, selected.col);
    const to = squareName(row, col);

    const success = compiler.play_move(from + to);

    selected = null;

    if (!success) {
        status.textContent = "Illegal move";
    }

    drawBoard();
}

function squareName(row, col) {
    const files = "abcdefgh";
    return files[col] + (row + 1);
}

function capitalize(text) {
    return text.charAt(0).toUpperCase() + text.slice(1);
}

async function start() {
    await init();

    compiler = new ChessCompiler();

    drawBoard();
}

start();
