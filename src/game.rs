
use std::time::{
    Duration,
    Instant,
};

use crate::board::Board;
use crate::config::{
    piece_points,
    GameConfig,
};
use crate::ir::MoveIR;
use crate::move_engine::{
    ChessMove,
    Color,
    PieceType,
};
use crate::rules::{
    is_checkmate,
    is_in_check,
    is_square_attacked,
    is_stalemate,
};

pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub game_over: bool,
    pub move_history: Vec<MoveIR>,

    pub white_score: u32,
    pub black_score: u32,

    pub config: GameConfig,

    pub white_time: Duration,
    pub black_time: Duration,

    turn_started: Instant,

    white_king_moved: bool,
    black_king_moved: bool,

    white_left_rook_moved: bool,
    white_right_rook_moved: bool,

    black_left_rook_moved: bool,
    black_right_rook_moved: bool,
}

impl Game {
    pub fn with_config(
        config: GameConfig,
    ) -> Self {
        let initial_time =
            if config.time_limit_enabled {
                Duration::from_secs(
                    config.time_limit_seconds
                )
            } else {
                Duration::ZERO
            };

        Self {
            board: Board::new(),
            turn: Color::White,
            game_over: false,
            move_history: Vec::new(),

            white_score: 0,
            black_score: 0,

            config,

            white_time: initial_time,
            black_time: initial_time,

            turn_started: Instant::now(),

            white_king_moved: false,
            black_king_moved: false,

            white_left_rook_moved: false,
            white_right_rook_moved: false,

            black_left_rook_moved: false,
            black_right_rook_moved: false,
        }
    }

    // ======================================
    // CLOCK
    // ======================================

    fn update_clock(
        &mut self,
    ) -> bool {
        if !self.config.time_limit_enabled {
            return true;
        }

        let elapsed =
            self.turn_started.elapsed();

        let remaining =
            match self.turn {
                Color::White =>
                    &mut self.white_time,

                Color::Black =>
                    &mut self.black_time,
            };

        if elapsed >= *remaining {
            *remaining =
                Duration::ZERO;

            println!(
                "Compiler: {:?} TIME OUT!",
                self.turn
            );

            self.game_over = true;

            return false;
        }

        *remaining -= elapsed;

        self.turn_started =
            Instant::now();

        true
    }

    pub fn time_remaining(
        &self,
        color: Color,
    ) -> Option<Duration> {
        if !self.config.time_limit_enabled {
            return None;
        }

        match color {
            Color::White =>
                Some(self.white_time),

            Color::Black =>
                Some(self.black_time),
        }
    }

    // ======================================
    // CASTLING RIGHTS
    // ======================================

    fn castling_allowed(
        &self,
        color: Color,
        king_side: bool,
    ) -> bool {
        match color {
            Color::White => {
                if self.white_king_moved {
                    return false;
                }

                if king_side {
                    !self.white_right_rook_moved
                } else {
                    !self.white_left_rook_moved
                }
            }

            Color::Black => {
                if self.black_king_moved {
                    return false;
                }

                if king_side {
                    !self.black_right_rook_moved
                } else {
                    !self.black_left_rook_moved
                }
            }
        }
    }

    fn mark_piece_moved(
        &mut self,
        from: usize,
        piece: crate::board::Piece,
    ) {
        match (
            piece.color,
            piece.kind,
            from,
        ) {
            (
                Color::White,
                PieceType::King,
                _,
            ) => {
                self.white_king_moved = true;
            }

            (
                Color::Black,
                PieceType::King,
                _,
            ) => {
                self.black_king_moved = true;
            }

            (
                Color::White,
                PieceType::Rook,
                0,
            ) => {
                self.white_left_rook_moved = true;
            }

            (
                Color::White,
                PieceType::Rook,
                7,
            ) => {
                self.white_right_rook_moved = true;
            }

            (
                Color::Black,
                PieceType::Rook,
                56,
            ) => {
                self.black_left_rook_moved = true;
            }

            (
                Color::Black,
                PieceType::Rook,
                63,
            ) => {
                self.black_right_rook_moved = true;
            }

            _ => {}
        }
    }

    // ======================================
    // CASTLING VALIDATION
    // ======================================

    fn validate_castling(
        &self,
        chess_move: ChessMove,
        color: Color,
    ) -> bool {
        let (
            king_start,
            king_side,
        ) =
            match color {
                Color::White => {
                    match (
                        chess_move.from,
                        chess_move.to,
                    ) {
                        (4, 6) =>
                            (4, true),

                        (4, 2) =>
                            (4, false),

                        _ => return false,
                    }
                }

                Color::Black => {
                    match (
                        chess_move.from,
                        chess_move.to,
                    ) {
                        (60, 62) =>
                            (60, true),

                        (60, 58) =>
                            (60, false),

                        _ => return false,
                    }
                }
            };

        if !self.castling_allowed(
            color,
            king_side,
        ) {
            return false;
        }

        if is_in_check(
            &self.board,
            color,
        ) {
            return false;
        }

        let rank =
            king_start / 8;

        let rook_square =
            if king_side {
                rank * 8 + 7
            } else {
                rank * 8
            };

        match self.board.piece_at(
            rook_square
        ) {
            Some(piece)
                if piece.kind == PieceType::Rook
                    && piece.color == color => {}

            _ => return false,
        }

        let enemy =
            opposite_color(color);

        if king_side {
            let f =
                rank * 8 + 5;

            let g =
                rank * 8 + 6;

            if self.board.piece_at(f).is_some()
                || self.board.piece_at(g).is_some()
            {
                return false;
            }

            if is_square_attacked(
                &self.board,
                f,
                enemy,
            ) {
                return false;
            }

            if is_square_attacked(
                &self.board,
                g,
                enemy,
            ) {
                return false;
            }
        } else {
            let b =
                rank * 8 + 1;

            let c =
                rank * 8 + 2;

            let d =
                rank * 8 + 3;

            if self.board.piece_at(b).is_some()
                || self.board.piece_at(c).is_some()
                || self.board.piece_at(d).is_some()
            {
                return false;
            }

            if is_square_attacked(
                &self.board,
                d,
                enemy,
            ) {
                return false;
            }

            if is_square_attacked(
                &self.board,
                c,
                enemy,
            ) {
                return false;
            }
        }

        true
    }

    // ======================================
    // EN PASSANT VALIDATION
    // ======================================

    fn validate_en_passant(
        &self,
        chess_move: ChessMove,
        color: Color,
    ) -> bool {
        let moving_piece =
            match self.board.piece_at(
                chess_move.from
            ) {
                Some(piece)
                    if piece.color == color
                        && piece.kind == PieceType::Pawn =>
                {
                    piece
                }

                _ => return false,
            };

        let from_file =
            (chess_move.from % 8) as i32;

        let from_rank =
            (chess_move.from / 8) as i32;

        let to_file =
            (chess_move.to % 8) as i32;

        let to_rank =
            (chess_move.to / 8) as i32;

        let direction =
            match color {
                Color::White => 1,
                Color::Black => -1,
            };

        // Must move diagonally exactly one square.
        if (to_file - from_file).abs() != 1
            || to_rank - from_rank != direction
        {
            return false;
        }

        // Destination must be empty.
        if self.board.piece_at(
            chess_move.to
        ).is_some() {
            return false;
        }

        // There must be a previous move.
        let previous =
            match self.move_history.last() {
                Some(move_ir) =>
                    move_ir,

                None => return false,
            };

        // Previous move must be an opponent pawn.
        if previous.piece != PieceType::Pawn
            || previous.color
                != opposite_color(color)
        {
            return false;
        }

        // Previous move must be exactly two ranks.
        let previous_rank_delta =
            (previous.to as i32
                - previous.from as i32)
                .abs();

        if previous_rank_delta != 16 {
            return false;
        }

        // The opponent pawn must now be beside
        // our pawn.
        let adjacent_square =
            match color {
                Color::White =>
                    chess_move.to
                        .checked_sub(8)
                        .unwrap_or(64),

                Color::Black =>
                    chess_move.to
                        .checked_add(8)
                        .unwrap_or(64),
            };

        if adjacent_square >= 64 {
            return false;
        }

        let captured_pawn =
            match self.board.piece_at(
                adjacent_square
            ) {
                Some(piece)
                    if piece.kind == PieceType::Pawn
                        && piece.color
                            == opposite_color(color) =>
                {
                    piece
                }

                _ => return false,
            };

        let _ = moving_piece;
        let _ = captured_pawn;

        // The previous pawn must be exactly the
        // pawn beside the destination.
        if previous.to != adjacent_square {
            return false;
        }

        true
    }

    // ======================================
    // PROMOTION
    // ======================================

    fn promote_pawn(
        &mut self,
        square: usize,
        promotion: PieceType,
    ) {
        let piece =
            match self.board.piece_at(square) {
                Some(piece)
                    if piece.kind == PieceType::Pawn =>
                {
                    piece
                }

                _ => return,
            };

        let promoted_piece =
            crate::board::Piece {
                color: piece.color,
                kind: promotion,
            };

        self.board.squares[square] =
            Some(promoted_piece);

        println!(
            "Compiler: Pawn promoted to {:?}.",
            promotion
        );
    }

    // ======================================
    // PLAY MOVE
    // ======================================

    pub fn play_move(
        &mut self,
        chess_move: ChessMove,
    ) -> bool {
        if self.game_over {
            println!(
                "Compiler: Game is already over."
            );

            return false;
        }

        if !self.update_clock() {
            return false;
        }

        let piece =
            match self.board.piece_at(
                chess_move.from
            ) {
                Some(piece) => piece,

                None => {
                    println!(
                        "Compiler: Source square is empty."
                    );

                    return false;
                }
            };

        if piece.color != self.turn {
            println!(
                "Compiler: Wrong player's turn."
            );

            return false;
        }

        // ==================================
        // CASTLING
        // ==================================

        let is_castling =
            piece.kind == PieceType::King
                && (chess_move.to as i32
                    - chess_move.from as i32)
                    .abs()
                    == 2;

        // ==================================
        // EN PASSANT
        // ==================================

        let is_en_passant =
            piece.kind == PieceType::Pawn
                && self.board.piece_at(
                    chess_move.to
                ).is_none()
                && ((
                    chess_move.to as i32
                        - chess_move.from as i32
                )
                .abs()
                    != 8)
                && ((
                    chess_move.to as i32
                        - chess_move.from as i32
                )
                .abs()
                    != 16);

        if is_castling {
            if !self.validate_castling(
                chess_move,
                self.turn,
            ) {
                println!(
                    "Compiler: Illegal castling rejected."
                );

                return false;
            }
        } else if is_en_passant {
            if !self.validate_en_passant(
                chess_move,
                self.turn,
            ) {
                println!(
                    "Compiler: Illegal En Passant rejected."
                );

                return false;
            }

            // Test king safety after En Passant.
            let mut test_board =
                self.board.clone_board();

            if test_board
                .execute_en_passant(
                    chess_move
                )
                .is_none()
            {
                println!(
                    "Compiler: En Passant execution failed."
                );

                return false;
            }

            if is_in_check(
                &test_board,
                self.turn,
            ) {
                println!(
                    "Compiler: En Passant rejected — King would be in check."
                );

                return false;
            }
        } else {
            if !self.board.compile_move(
                chess_move
            ) {
                println!(
                    "Compiler: Illegal move rejected."
                );

                return false;
            }

            let mut test_board =
                self.board.clone_board();

            test_board.execute_move(
                chess_move
            );

            if is_in_check(
                &test_board,
                self.turn,
            ) {
                println!(
                    "Compiler: Move rejected — King would be in check."
                );

                return false;
            }
        }

        // ==================================
        // CAPTURE INFORMATION
        // ==================================

        let captured_piece =
            if is_en_passant {
                let captured_square =
                    match piece.color {
                        Color::White =>
                            chess_move.to
                                .checked_sub(8)
                                .unwrap_or(64),

                        Color::Black =>
                            chess_move.to
                                .checked_add(8)
                                .unwrap_or(64),
                    };

                if captured_square < 64 {
                    self.board.piece_at(
                        captured_square
                    )
                } else {
                    None
                }
            } else {
                self.board.piece_at(
                    chess_move.to
                )
            };

        let captured_kind =
            captured_piece.map(
                |p| p.kind
            );

        let captured_color =
            captured_piece.map(
                |p| p.color
            );

        // ==================================
        // IR
        // ==================================

        let ir =
            MoveIR::from_move(
                chess_move,
                piece.kind,
                piece.color,
                captured_kind,
                captured_color,
            );

        println!(
            "Compiler IR: {:?} {:?} {} -> {}",
            ir.color,
            ir.piece,
            ir.from,
            ir.to
        );

        // ==================================
        // SCORE
        // ==================================

        if let Some(captured) =
            captured_piece
        {
            let points =
                piece_points(
                    captured.kind
                );

            match piece.color {
                Color::White => {
                    self.white_score += points;

                    println!(
                        "Compiler: White captured {:?} (+{} points)",
                        captured.kind,
                        points
                    );
                }

                Color::Black => {
                    self.black_score += points;

                    println!(
                        "Compiler: Black captured {:?} (+{} points)",
                        captured.kind,
                        points
                    );
                }
            }
        }

        // ==================================
        // MARK MOVED
        // ==================================

        self.mark_piece_moved(
            chess_move.from,
            piece,
        );

        // ==================================
        // EXECUTE
        // ==================================

        if is_en_passant {
            self.board.execute_en_passant(
                chess_move
            );

            println!(
                "Compiler: EN PASSANT accepted."
            );
        } else {
            self.board.execute_move(
                chess_move
            );

            if is_castling {
                println!(
                    "Compiler: CASTLING accepted."
                );
            } else {
                println!(
                    "Compiler: Move accepted."
                );
            }
        }

        // ==================================
        // PROMOTION
        // ==================================

        if piece.kind == PieceType::Pawn {
            let destination_rank =
                chess_move.to / 8;

            let promotion_rank =
                match piece.color {
                    Color::White => 7,
                    Color::Black => 0,
                };

            if destination_rank ==
                promotion_rank
            {
                self.promote_pawn(
                    chess_move.to,
                    PieceType::Queen,
                );
            }
        }

        // ==================================
        // SAVE IR
        // ==================================

        self.move_history.push(ir);

        println!(
            "Runtime: Move executed."
        );

        println!(
            "Score: White {} | Black {}",
            self.score(Color::White),
            self.score(Color::Black)
        );

        self.turn_started =
            Instant::now();

        self.turn =
            opposite_color(
                self.turn
            );

        // ==================================
        // GAME STATE
        // ==================================

        if is_checkmate(
            &self.board,
            self.turn,
        ) {
            println!(
                "Compiler: CHECKMATE — {:?} loses.",
                self.turn
            );

            self.game_over = true;
        } else if is_stalemate(
            &self.board,
            self.turn,
        ) {
            println!(
                "Compiler: STALEMATE — Draw."
            );

            self.game_over = true;
        } else if is_in_check(
            &self.board,
            self.turn,
        ) {
            println!(
                "Compiler: {:?} is in CHECK.",
                self.turn
            );
        }

        true
    }

    pub fn move_count(
        &self,
    ) -> usize {
        self.move_history.len()
    }

    pub fn score(
        &self,
        color: Color,
    ) -> u32 {
        match color {
            Color::White =>
                self.white_score,

            Color::Black =>
                self.black_score,
        }
    }
}

fn opposite_color(
    color: Color,
) -> Color {
    match color {
        Color::White =>
            Color::Black,

        Color::Black =>
            Color::White,
    }
}
