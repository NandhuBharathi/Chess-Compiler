mod board;
mod config;
mod draw;
mod game;
mod ir;
mod move_engine;
mod parser;
mod rules;

use board::Board;
use config::GameConfig;
use game::Game;
use move_engine::{Color, PieceType};
use parser::parse_move;

fn compile_and_execute(game: &mut Game, input: &str) -> bool {
    println!();
    println!("Input: {}", input);

    // --------------------------------------------------
    // FRONTEND: PARSER
    // --------------------------------------------------

    let chess_move = match parse_move(input) {
        Some(chess_move) => {
            println!(
                "Parser: {} -> {} -> {}",
                input, chess_move.from, chess_move.to
            );

            chess_move
        }

        None => {
            println!("Parser: Invalid move input.");

            return false;
        }
    };

    // --------------------------------------------------
    // COMPILER / RUNTIME
    // --------------------------------------------------

    game.play_move(chess_move)
}

fn empty_board() -> Board {
    let mut board = Board {
        squares: [None; 64],
    };

    board.squares[4] = Some(board::Piece {
        color: Color::White,
        kind: PieceType::King,
    });

    board.squares[60] = Some(board::Piece {
        color: Color::Black,
        kind: PieceType::King,
    });

    board
}

fn add_piece(board: &mut Board, square: usize, color: Color, kind: PieceType) {
    board.squares[square] = Some(board::Piece { color, kind });
}

fn main() {
    println!("=== CHESS COMPILER ===");

    // ==================================================
    // TIME CONFIGURATION
    // ==================================================

    println!();
    println!("=== TIME CONFIGURATION ===");

    let no_clock = GameConfig::no_time_limit();

    let timed = GameConfig::with_time_limit(600);

    println!(
        "OFF: Enabled = {}, Seconds = {}",
        no_clock.time_limit_enabled, no_clock.time_limit_seconds
    );

    println!(
        "ON: Enabled = {}, Seconds = {}",
        timed.time_limit_enabled, timed.time_limit_seconds
    );

    let clock_game = Game::with_config(timed);

    println!("White Time: {:?}", clock_game.time_remaining(Color::White));

    println!("Black Time: {:?}", clock_game.time_remaining(Color::Black));

    // ==================================================
    // THREEFOLD REPETITION TEST
    // ==================================================

    println!();
    println!("=== THREEFOLD REPETITION TEST ===");

    let mut repetition_game = Game::with_config(no_clock);

    let repetition_moves = [
        "g1f3", "g8f6", "f3g1", "f6g8", "g1f3", "g8f6", "f3g1", "f6g8",
    ];

    for input in repetition_moves {
        if !compile_and_execute(&mut repetition_game, input) {
            break;
        }

        if repetition_game.game_over {
            break;
        }
    }

    println!();
    println!(
        "Threefold Repetition: {}",
        repetition_game.is_threefold_repetition()
    );

    println!("Repetition Count: {}", repetition_game.repetition_count());

    // ==================================================
    // 50-MOVE RULE TEST
    // ==================================================

    println!();
    println!("=== 50-MOVE RULE TEST ===");

    let mut fifty_game = Game::with_config(no_clock);

    let fifty_test_moves = ["g1f3", "g8f6", "f3g1", "f6g8"];

    for input in fifty_test_moves {
        if !compile_and_execute(&mut fifty_game, input) {
            break;
        }

        if fifty_game.game_over {
            break;
        }
    }

    println!();
    println!("Halfmove Clock: {}", fifty_game.halfmove_clock());

    println!("50-Move Rule: {}", fifty_game.is_fifty_move_rule());

    // ==================================================
    // DRAW MODULE TEST
    // ==================================================

    println!();
    println!("=== DRAW MODULE TEST ===");

    let positions = vec![
        String::from("POSITION_A"),
        String::from("POSITION_B"),
        String::from("POSITION_A"),
        String::from("POSITION_C"),
        String::from("POSITION_A"),
    ];

    let repetition_count = draw::repetition_count(&positions);

    let threefold = draw::is_threefold_repetition(&positions);

    println!("Draw Module Repetition Count: {}", repetition_count);

    println!("Draw Module Threefold: {}", threefold);

    let fifty_start = 99;

    let fifty_after_move = draw::update_halfmove_clock(fifty_start, PieceType::Knight, false);

    let fifty_rule = draw::is_fifty_move_rule(fifty_after_move);

    println!("Halfmove Before: {}", fifty_start);

    println!("Halfmove After: {}", fifty_after_move);

    println!("Draw Module 50-Move Rule: {}", fifty_rule);

    // ==================================================
    // INSUFFICIENT MATERIAL TEST
    // ==================================================

    println!();
    println!("=== INSUFFICIENT MATERIAL TEST ===");

    // K vs K

    let king_vs_king = empty_board();

    let king_vs_king_result = draw::is_insufficient_material(&king_vs_king);

    println!("K vs K: {}", king_vs_king_result);

    println!(
        "Description: {}",
        draw::insufficient_material_description(&king_vs_king)
    );

    // K+B vs K

    let mut bishop_vs_king = empty_board();

    add_piece(&mut bishop_vs_king, 18, Color::White, PieceType::Bishop);

    let bishop_vs_king_result = draw::is_insufficient_material(&bishop_vs_king);

    println!("K+B vs K: {}", bishop_vs_king_result);

    // K+N vs K

    let mut knight_vs_king = empty_board();

    add_piece(&mut knight_vs_king, 18, Color::White, PieceType::Knight);

    let knight_vs_king_result = draw::is_insufficient_material(&knight_vs_king);

    println!("K+N vs K: {}", knight_vs_king_result);

    // K+B vs K+B
    // Same-colored bishops

    let mut bishops_same_color = empty_board();

    add_piece(&mut bishops_same_color, 18, Color::White, PieceType::Bishop);

    add_piece(&mut bishops_same_color, 34, Color::Black, PieceType::Bishop);

    let bishops_same_color_result = draw::is_insufficient_material(&bishops_same_color);

    println!("K+B vs K+B (same color): {}", bishops_same_color_result);

    // K+B vs K+B
    // Opposite-colored bishops

    let mut bishops_opposite_color = empty_board();

    add_piece(
        &mut bishops_opposite_color,
        18,
        Color::White,
        PieceType::Bishop,
    );

    add_piece(
        &mut bishops_opposite_color,
        35,
        Color::Black,
        PieceType::Bishop,
    );

    let bishops_opposite_color_result = draw::is_insufficient_material(&bishops_opposite_color);

    println!(
        "K+B vs K+B (opposite color): {}",
        bishops_opposite_color_result
    );

    // K+R vs K

    let mut rook_vs_king = empty_board();

    add_piece(&mut rook_vs_king, 18, Color::White, PieceType::Rook);

    let rook_vs_king_result = draw::is_insufficient_material(&rook_vs_king);

    println!("K+R vs K: {}", rook_vs_king_result);

    // ==================================================
    // IR CAPTURE TEST
    // ==================================================

    println!();
    println!("=== IR CAPTURE TEST ===");

    let mut capture_game = Game::with_config(no_clock);

    compile_and_execute(&mut capture_game, "e2e4");

    compile_and_execute(&mut capture_game, "d7d5");

    compile_and_execute(&mut capture_game, "e4d5");

    if let Some(ir) = capture_game.move_history.last() {
        println!("Piece: {:?}", ir.piece);

        println!("Color: {:?}", ir.color);

        println!("From: {}", ir.from);

        println!("To: {}", ir.to);

        println!("Captured Piece: {:?}", ir.captured_piece);

        println!("Captured Color: {:?}", ir.captured_color);

        println!("Capture Description: {}", ir.capture_description());
    }

    println!("White Score: {}", capture_game.score(Color::White));

    // ==================================================
    // COMPILER TEST RESULTS
    // ==================================================

    println!();
    println!("=== COMPILER TEST RESULTS ===");

    let threefold_pass = repetition_game.is_threefold_repetition();

    let draw_module_threefold_pass = threefold && repetition_count == 3;

    let draw_module_fifty_pass = fifty_rule && fifty_after_move == 100;

    let insufficient_material_pass = king_vs_king_result
        && bishop_vs_king_result
        && knight_vs_king_result
        && bishops_same_color_result
        && !bishops_opposite_color_result
        && !rook_vs_king_result;

    let ir_capture_pass = capture_game.score(Color::White) == 1 && capture_game.move_count() == 3;

    println!("Threefold Repetition: {}", threefold_pass);

    println!("Draw Module Threefold: {}", draw_module_threefold_pass);

    println!("Draw Module 50-Move Rule: {}", draw_module_fifty_pass);

    println!("Insufficient Material: {}", insufficient_material_pass);

    println!("IR Capture: {}", ir_capture_pass);

    if threefold_pass
        && draw_module_threefold_pass
        && draw_module_fifty_pass
        && insufficient_material_pass
        && ir_capture_pass
    {
        println!();
        println!("Compiler Test: DRAW RULES PASS");
    } else {
        println!();
        println!("Compiler Test: DRAW RULES FAIL");
    }

    // ==================================================
    // FINAL
    // ==================================================

    println!();
    println!("=== FINAL ===");

    println!("Threefold: {}", threefold_pass);

    println!("50-Move Rule Module: {}", draw_module_fifty_pass);

    println!("Insufficient Material: {}", insufficient_material_pass);

    println!("IR Capture: {}", ir_capture_pass);

    println!("Chess Compiler Core: READY");
}
