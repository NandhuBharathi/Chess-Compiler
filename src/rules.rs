
use crate::board::Board;
use crate::move_engine::{
    is_legal_move,
    path_is_clear,
    ChessMove,
    Color,
    PieceType,
};

pub fn find_king(
    board: &Board,
    color: Color,
) -> Option<usize> {
    for square in 0..64 {
        if let Some(piece) = board.piece_at(square) {
            if piece.color == color
                && piece.kind == PieceType::King
            {
                return Some(square);
            }
        }
    }

    None
}

pub fn is_square_attacked(
    board: &Board,
    square: usize,
    by_color: Color,
) -> bool {
    for from in 0..64 {
        let piece = match board.piece_at(from) {
            Some(piece) => piece,
            None => continue,
        };

        if piece.color != by_color {
            continue;
        }

        // Pawn attacks
        if piece.kind == PieceType::Pawn {
            let from_file = (from % 8) as i32;
            let from_rank = (from / 8) as i32;

            let to_file = (square % 8) as i32;
            let to_rank = (square / 8) as i32;

            let direction = match by_color {
                Color::White => 1,
                Color::Black => -1,
            };

            if (to_file - from_file).abs() == 1
                && to_rank - from_rank == direction
            {
                return true;
            }

            continue;
        }

        if !is_legal_move(
            piece.kind,
            piece.color,
            from,
            square,
        ) {
            continue;
        }

        // Sliding pieces need clear path.
        if matches!(
            piece.kind,
            PieceType::Bishop
                | PieceType::Rook
                | PieceType::Queen
        ) {
            let mut occupancy = [None; 64];

            for i in 0..64 {
                occupancy[i] =
                    board.squares[i]
                        .map(|p| (p.color, p.kind));
            }

            if !path_is_clear(
                &occupancy,
                from,
                square,
            ) {
                continue;
            }
        }

        return true;
    }

    false
}

pub fn is_in_check(
    board: &Board,
    color: Color,
) -> bool {
    let king_square = match find_king(board, color) {
        Some(square) => square,
        None => return false,
    };

    let attacker = match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    is_square_attacked(
        board,
        king_square,
        attacker,
    )
}

pub fn has_legal_move(
    board: &Board,
    color: Color,
) -> bool {
    for from in 0..64 {
        let piece = match board.piece_at(from) {
            Some(piece) => piece,
            None => continue,
        };

        if piece.color != color {
            continue;
        }

        for to in 0..64 {
            let chess_move = ChessMove::new(from, to);

            if !board.compile_move(chess_move) {
                continue;
            }

            let mut test_board = board.clone_board();

            test_board.execute_move(chess_move);

            if !is_in_check(&test_board, color) {
                return true;
            }
        }
    }

    false
}

pub fn is_checkmate(
    board: &Board,
    color: Color,
) -> bool {
    is_in_check(board, color)
        && !has_legal_move(board, color)
}

pub fn is_stalemate(
    board: &Board,
    color: Color,
) -> bool {
    !is_in_check(board, color)
        && !has_legal_move(board, color)
}
