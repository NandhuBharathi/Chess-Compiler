use wasm_bindgen::prelude::*;

#[path = "../../src/board.rs"]
mod board;

#[path = "../../src/config.rs"]
mod config;

#[path = "../../src/draw.rs"]
mod draw;

#[path = "../../src/game.rs"]
mod game;

#[path = "../../src/ir.rs"]
mod ir;

#[path = "../../src/move_engine.rs"]
mod move_engine;

#[path = "../../src/parser.rs"]
mod parser;

#[path = "../../src/rules.rs"]
mod rules;

use config::GameConfig;
use game::Game;
use move_engine::Color;
use parser::parse_move;

#[wasm_bindgen]
pub struct ChessCompiler {
    game: Game,
}

#[wasm_bindgen]
impl ChessCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            game: Game::with_config(GameConfig::no_time_limit()),
        }
    }

    pub fn play_move(&mut self, input: &str) -> bool {
        let chess_move = match parse_move(input) {
            Some(chess_move) => chess_move,
            None => return false,
        };

        self.game.play_move(chess_move)
    }

    pub fn board(&self) -> String {
        let mut output = String::with_capacity(64);

        for square in 0..64 {
            let symbol = match self.game.board.piece_at(square) {
                None => '.',

                Some(piece) => match (piece.color, piece.kind) {
                    (Color::White, move_engine::PieceType::King) => 'K',
                    (Color::White, move_engine::PieceType::Queen) => 'Q',
                    (Color::White, move_engine::PieceType::Rook) => 'R',
                    (Color::White, move_engine::PieceType::Bishop) => 'B',
                    (Color::White, move_engine::PieceType::Knight) => 'N',
                    (Color::White, move_engine::PieceType::Pawn) => 'P',

                    (Color::Black, move_engine::PieceType::King) => 'k',
                    (Color::Black, move_engine::PieceType::Queen) => 'q',
                    (Color::Black, move_engine::PieceType::Rook) => 'r',
                    (Color::Black, move_engine::PieceType::Bishop) => 'b',
                    (Color::Black, move_engine::PieceType::Knight) => 'n',
                    (Color::Black, move_engine::PieceType::Pawn) => 'p',
                },
            };

            output.push(symbol);
        }

        output
    }

    pub fn turn(&self) -> String {
        match self.game.turn {
            Color::White => "white".to_string(),
            Color::Black => "black".to_string(),
        }
    }

    pub fn game_over(&self) -> bool {
        self.game.game_over
    }

    pub fn move_count(&self) -> usize {
        self.game.move_count()
    }

    pub fn white_score(&self) -> u32 {
        self.game.score(Color::White)
    }

    pub fn black_score(&self) -> u32 {
        self.game.score(Color::Black)
    }
}
