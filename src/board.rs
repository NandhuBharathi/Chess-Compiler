
use crate::move_engine::{
    is_legal_move,
    path_is_clear,
    ChessMove,
    Color,
    PieceType,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceType,
}

pub struct Board {
    pub squares: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            squares: [None; 64],
        };

        let back_rank = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for file in 0..8 {
            board.squares[file] = Some(Piece {
                color: Color::White,
                kind: back_rank[file],
            });

            board.squares[8 + file] = Some(Piece {
                color: Color::White,
                kind: PieceType::Pawn,
            });

            board.squares[48 + file] = Some(Piece {
                color: Color::Black,
                kind: PieceType::Pawn,
            });

            board.squares[56 + file] = Some(Piece {
                color: Color::Black,
                kind: back_rank[file],
            });
        }

        board
    }

    pub fn piece_at(
        &self,
        square: usize,
    ) -> Option<Piece> {
        if square >= 64 {
            return None;
        }

        self.squares[square]
    }

    pub fn compile_move(
        &self,
        chess_move: ChessMove,
    ) -> bool {
        if chess_move.from >= 64
            || chess_move.to >= 64
            || chess_move.from == chess_move.to
        {
            return false;
        }

        let piece =
            match self.piece_at(chess_move.from) {
                Some(piece) => piece,
                None => return false,
            };

        let target =
            self.piece_at(chess_move.to);

        let from_file =
            (chess_move.from % 8) as i32;

        let from_rank =
            (chess_move.from / 8) as i32;

        let to_file =
            (chess_move.to % 8) as i32;

        let to_rank =
            (chess_move.to / 8) as i32;

        let dx =
            (to_file - from_file).abs();

        // ======================================
        // PAWN
        // ======================================

        if piece.kind == PieceType::Pawn {
            let direction =
                match piece.color {
                    Color::White => 1,
                    Color::Black => -1,
                };

            let rank_delta =
                to_rank - from_rank;

            // Normal one-square move
            if dx == 0
                && rank_delta == direction
            {
                return target.is_none();
            }

            // Initial two-square move
            if dx == 0
                && rank_delta == direction * 2
            {
                let start_rank =
                    match piece.color {
                        Color::White => 1,
                        Color::Black => 6,
                    };

                if from_rank != start_rank {
                    return false;
                }

                let middle_rank =
                    from_rank + direction;

                let middle_square =
                    (middle_rank * 8 + from_file)
                        as usize;

                return self.squares[middle_square]
                    .is_none()
                    && target.is_none();
            }

            // Normal diagonal capture
            if dx == 1
                && rank_delta == direction
            {
                return match target {
                    Some(target_piece) => {
                        target_piece.color != piece.color
                    }

                    None => false,
                };
            }

            // Empty diagonal is handled by
            // Game::validate_en_passant().
            return false;
        }

        // ======================================
        // CASTLING GEOMETRY
        // ======================================

        if piece.kind == PieceType::King
            && from_file == 4
            && from_rank == to_rank
            && dx == 2
        {
            return self.compile_castle(
                chess_move,
                piece.color,
            );
        }

        // ======================================
        // NORMAL PIECES
        // ======================================

        if !is_legal_move(
            piece.kind,
            piece.color,
            chess_move.from,
            chess_move.to,
        ) {
            return false;
        }

        if let Some(target_piece) = target {
            if target_piece.color == piece.color {
                return false;
            }
        }

        if matches!(
            piece.kind,
            PieceType::Bishop
                | PieceType::Rook
                | PieceType::Queen
        ) {
            let mut occupancy =
                [None; 64];

            for i in 0..64 {
                occupancy[i] =
                    self.squares[i]
                        .map(|p| (p.color, p.kind));
            }

            if !path_is_clear(
                &occupancy,
                chess_move.from,
                chess_move.to,
            ) {
                return false;
            }
        }

        true
    }

    fn compile_castle(
        &self,
        chess_move: ChessMove,
        color: Color,
    ) -> bool {
        let rank =
            match color {
                Color::White => 0,
                Color::Black => 7,
            };

        if chess_move.from != rank * 8 + 4 {
            return false;
        }

        if chess_move.to == rank * 8 + 6 {
            let rook_square =
                rank * 8 + 7;

            match self.piece_at(rook_square) {
                Some(piece)
                    if piece.kind == PieceType::Rook
                        && piece.color == color => {}

                _ => return false,
            }

            let f_square =
                rank * 8 + 5;

            let g_square =
                rank * 8 + 6;

            if self.squares[f_square].is_some() {
                return false;
            }

            if self.squares[g_square].is_some() {
                return false;
            }

            return true;
        }

        if chess_move.to == rank * 8 + 2 {
            let rook_square =
                rank * 8;

            match self.piece_at(rook_square) {
                Some(piece)
                    if piece.kind == PieceType::Rook
                        && piece.color == color => {}

                _ => return false,
            }

            let b_square =
                rank * 8 + 1;

            let c_square =
                rank * 8 + 2;

            let d_square =
                rank * 8 + 3;

            if self.squares[b_square].is_some()
                || self.squares[c_square].is_some()
                || self.squares[d_square].is_some()
            {
                return false;
            }

            return true;
        }

        false
    }

    // ======================================
    // NORMAL / CASTLING EXECUTION
    // ======================================

    pub fn execute_move(
        &mut self,
        chess_move: ChessMove,
    ) {
        let piece =
            match self.squares[chess_move.from] {
                Some(piece) => piece,
                None => return,
            };

        let from_file =
            chess_move.from % 8;

        let to_file =
            chess_move.to % 8;

        // ==================================
        // KING SIDE CASTLING
        // ==================================

        if piece.kind == PieceType::King
            && from_file == 4
            && to_file == 6
        {
            let rank =
                chess_move.from / 8;

            let rook_from =
                rank * 8 + 7;

            let rook_to =
                rank * 8 + 5;

            self.squares[chess_move.from] =
                None;

            self.squares[chess_move.to] =
                Some(piece);

            let rook =
                self.squares[rook_from];

            self.squares[rook_from] =
                None;

            self.squares[rook_to] =
                rook;

            return;
        }

        // ==================================
        // QUEEN SIDE CASTLING
        // ==================================

        if piece.kind == PieceType::King
            && from_file == 4
            && to_file == 2
        {
            let rank =
                chess_move.from / 8;

            let rook_from =
                rank * 8;

            let rook_to =
                rank * 8 + 3;

            self.squares[chess_move.from] =
                None;

            self.squares[chess_move.to] =
                Some(piece);

            let rook =
                self.squares[rook_from];

            self.squares[rook_from] =
                None;

            self.squares[rook_to] =
                rook;

            return;
        }

        // ==================================
        // NORMAL MOVE
        // ==================================

        self.squares[chess_move.from] =
            None;

        self.squares[chess_move.to] =
            Some(piece);
    }

    // ======================================
    // EN PASSANT EXECUTION
    // ======================================

    pub fn execute_en_passant(
        &mut self,
        chess_move: ChessMove,
    ) -> Option<Piece> {
        let moving_piece =
            match self.squares[chess_move.from] {
                Some(piece)
                    if piece.kind == PieceType::Pawn =>
                {
                    piece
                }

                _ => return None,
            };

        let captured_square =
            match moving_piece.color {
                Color::White =>
                    chess_move.to.checked_sub(8)?,

                Color::Black =>
                    chess_move.to.checked_add(8)?,
            };

        if captured_square >= 64 {
            return None;
        }

        let captured_piece =
            match self.squares[captured_square] {
                Some(piece)
                    if piece.kind == PieceType::Pawn
                        && piece.color != moving_piece.color =>
                {
                    piece
                }

                _ => return None,
            };

        self.squares[chess_move.from] =
            None;

        self.squares[captured_square] =
            None;

        self.squares[chess_move.to] =
            Some(moving_piece);

        Some(captured_piece)
    }

    pub fn clone_board(&self) -> Board {
        Board {
            squares: self.squares,
        }
    }
}
