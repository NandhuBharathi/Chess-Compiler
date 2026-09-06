
mod board;
mod config;
mod game;
mod ir;
mod move_engine;
mod parser;
mod rules;

use config::GameConfig;
use game::Game;
use move_engine::{
    Color,
    PieceType,
};
use parser::parse_move;

fn display_board(game: &Game) {
    println!();

    for rank in (0..8).rev() {
        print!(
            "{}  ",
            rank + 1
        );

        for file in 0..8 {
            let square =
                rank * 8 + file;

            match game.board.piece_at(square) {
                Some(piece) => {
                    let symbol =
                        match (
                            piece.color,
                            piece.kind,
                        ) {
                            (
                                Color::White,
                                PieceType::King,
                            ) => 'K',

                            (
                                Color::White,
                                PieceType::Queen,
                            ) => 'Q',

                            (
                                Color::White,
                                PieceType::Rook,
                            ) => 'R',

                            (
                                Color::White,
                                PieceType::Bishop,
                            ) => 'B',

                            (
                                Color::White,
                                PieceType::Knight,
                            ) => 'N',

                            (
                                Color::White,
                                PieceType::Pawn,
                            ) => 'P',

                            (
                                Color::Black,
                                PieceType::King,
                            ) => 'k',

                            (
                                Color::Black,
                                PieceType::Queen,
                            ) => 'q',

                            (
                                Color::Black,
                                PieceType::Rook,
                            ) => 'r',

                            (
                                Color::Black,
                                PieceType::Bishop,
                            ) => 'b',

                            (
                                Color::Black,
                                PieceType::Knight,
                            ) => 'n',

                            (
                                Color::Black,
                                PieceType::Pawn,
                            ) => 'p',
                        };

                    print!(
                        "{} ",
                        symbol
                    );
                }

                None => {
                    print!(". ");
                }
            }
        }

        println!();
    }

    println!(
        "\n   a b c d e f g h"
    );

    println!(
        "Turn: {:?}",
        game.turn
    );

    println!(
        "Score: White {} | Black {}",
        game.score(Color::White),
        game.score(Color::Black)
    );

    println!(
        "Moves: {}",
        game.move_count()
    );

    if game.config.time_limit_enabled {
        println!(
            "White Time: {:?}",
            game.time_remaining(
                Color::White
            )
        );

        println!(
            "Black Time: {:?}",
            game.time_remaining(
                Color::Black
            )
        );
    } else {
        println!(
            "Time Limit: OFF"
        );
    }
}

fn execute_input(
    game: &mut Game,
    input: &str,
) -> bool {
    println!(
        "\nInput: {}",
        input
    );

    let chess_move =
        match parse_move(input) {
            Some(chess_move) =>
                chess_move,

            None => {
                println!(
                    "Compiler: Invalid notation."
                );

                return false;
            }
        };

    game.play_move(
        chess_move
    )
}

fn main() {
    println!(
        "=== CHESS COMPILER ==="
    );

    // ==========================================
    // TIME CONFIGURATION
    // ==========================================

    let no_limit_config =
        GameConfig::no_time_limit();

    let timed_config =
        GameConfig::with_time_limit(
            600
        );

    println!(
        "\n=== TIME CONFIGURATION ==="
    );

    println!(
        "OFF:"
    );

    println!(
        "  Enabled: {}",
        no_limit_config.time_limit_enabled
    );

    println!(
        "  Seconds: {}",
        no_limit_config.time_limit_seconds
    );

    println!(
        "ON:"
    );

    println!(
        "  Enabled: {}",
        timed_config.time_limit_enabled
    );

    println!(
        "  Seconds: {}",
        timed_config.time_limit_seconds
    );

    // ==========================================
    // USE TIMED MODE
    // ==========================================

    let mut game =
        Game::with_config(
            timed_config
        );

    // ==========================================
    // EN PASSANT TEST POSITION
    // ==========================================

    game.board.squares =
        [None; 64];

    // White King = e1
    game.board.squares[4] =
        Some(board::Piece {
            color: Color::White,
            kind: PieceType::King,
        });

    // White Pawn = e5
    game.board.squares[36] =
        Some(board::Piece {
            color: Color::White,
            kind: PieceType::Pawn,
        });

    // Black King = e8
    game.board.squares[60] =
        Some(board::Piece {
            color: Color::Black,
            kind: PieceType::King,
        });

    // Black Pawn = d7
    game.board.squares[51] =
        Some(board::Piece {
            color: Color::Black,
            kind: PieceType::Pawn,
        });

    game.turn =
        Color::Black;

    println!(
        "\n=== EN PASSANT TEST POSITION ==="
    );

    display_board(
        &game
    );

    // ==========================================
    // BLACK d7-d5
    // ==========================================

    if !execute_input(
        &mut game,
        "d7d5",
    ) {
        println!(
            "\nCompiler Test: SETUP FAIL"
        );

        return;
    }

    display_board(
        &game
    );

    // ==========================================
    // WHITE e5-d6 EN PASSANT
    // ==========================================

    if !execute_input(
        &mut game,
        "e5d6",
    ) {
        println!(
            "\nCompiler Test: EN PASSANT FAIL"
        );

        return;
    }

    display_board(
        &game
    );

    // ==========================================
    // BOARD VERIFICATION
    // ==========================================

    let white_pawn_on_d6 =
        matches!(
            game.board.piece_at(43),
            Some(piece)
                if piece.color == Color::White
                    && piece.kind == PieceType::Pawn
        );

    let black_pawn_removed =
        game.board
            .piece_at(35)
            .is_none();

    // ==========================================
    // SCORE VERIFICATION
    // ==========================================

    let white_score_correct =
        game.score(
            Color::White
        ) == 1;

    // ==========================================
    // TURN VERIFICATION
    // ==========================================

    let correct_turn =
        game.turn == Color::Black;

    // ==========================================
    // MOVE HISTORY VERIFICATION
    // ==========================================

    let correct_move_count =
        game.move_count() == 2;

    // ==========================================
    // RESULT
    // ==========================================

    println!(
        "\n=== EN PASSANT TEST ==="
    );

    println!(
        "White Pawn on d6: {}",
        white_pawn_on_d6
    );

    println!(
        "Black Pawn removed from d5: {}",
        black_pawn_removed
    );

    println!(
        "White Score = 1: {}",
        white_score_correct
    );

    println!(
        "Turn = Black: {}",
        correct_turn
    );

    println!(
        "Move Count = 2: {}",
        correct_move_count
    );

    if white_pawn_on_d6
        && black_pawn_removed
        && white_score_correct
        && correct_turn
        && correct_move_count
    {
        println!(
            "\nCompiler Test: EN PASSANT PASS"
        );
    } else {
        println!(
            "\nCompiler Test: EN PASSANT FAIL"
        );
    }

    // ==========================================
    // IR VERIFICATION
    // ==========================================

    if let Some(ir) =
        game.move_history.last()
    {
        println!(
            "\n=== EN PASSANT IR ==="
        );

        println!(
            "Piece: {:?}",
            ir.piece
        );

        println!(
            "Color: {:?}",
            ir.color
        );

        println!(
            "From: {}",
            ir.from
        );

        println!(
            "To: {}",
            ir.to
        );

        println!(
            "Capture: {}",
            ir.capture_description()
        );
    }

    // ==========================================
    // CLOCK VERIFICATION
    // ==========================================

    println!(
        "\n=== CLOCK TEST ==="
    );

    println!(
        "White Remaining: {:?}",
        game.time_remaining(
            Color::White
        )
    );

    println!(
        "Black Remaining: {:?}",
        game.time_remaining(
            Color::Black
        )
    );

    // ==========================================
    // FINAL
    // ==========================================

    println!(
        "\n=== FINAL GAME STATE ==="
    );

    println!(
        "White Score: {}",
        game.score(
            Color::White
        )
    );

    println!(
        "Black Score: {}",
        game.score(
            Color::Black
        )
    );

    println!(
        "Total Moves: {}",
        game.move_count()
    );

    println!(
        "Game Over: {}",
        game.game_over
    );

    println!(
        "Chess Compiler Core: READY"
    );
}
