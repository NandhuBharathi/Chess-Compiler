use crate::board::Board;
use crate::ir::MoveIR;
use crate::move_engine::{Color, PieceType};

pub const FIFTY_MOVE_LIMIT: u32 = 100;

pub struct DrawState {
    position_history: Vec<String>,
    halfmove_clock: u32,
}

impl DrawState {
    pub fn new() -> Self {
        Self {
            position_history: Vec::new(),
            halfmove_clock: 0,
        }
    }

    pub fn halfmove_clock(&self) -> u32 {
        self.halfmove_clock
    }

    pub fn is_fifty_move_rule(&self) -> bool {
        is_fifty_move_rule(self.halfmove_clock)
    }

    pub fn update_halfmove_clock(&mut self, piece: PieceType, captured: bool) {
        self.halfmove_clock = update_halfmove_clock(self.halfmove_clock, piece, captured);
    }

    pub fn record_position(
        &mut self,
        board: &Board,
        turn: Color,
        castling_rights: [bool; 6],
        previous_move: Option<&MoveIR>,
    ) {
        let key = position_key(board, turn, castling_rights, previous_move);

        self.position_history.push(key);
    }

    pub fn is_threefold_repetition(&self) -> bool {
        is_threefold_repetition(&self.position_history)
    }

    pub fn repetition_count(&self) -> usize {
        repetition_count(&self.position_history)
    }
}

pub fn is_threefold_repetition(position_history: &[String]) -> bool {
    repetition_count(position_history) >= 3
}

pub fn repetition_count(position_history: &[String]) -> usize {
    let current = match position_history.last() {
        Some(position) => position,
        None => return 0,
    };

    position_history
        .iter()
        .filter(|position| *position == current)
        .count()
}

pub fn is_fifty_move_rule(halfmove_clock: u32) -> bool {
    halfmove_clock >= FIFTY_MOVE_LIMIT
}

pub fn update_halfmove_clock(current: u32, piece: PieceType, captured: bool) -> u32 {
    if piece == PieceType::Pawn || captured {
        0
    } else {
        current.saturating_add(1)
    }
}

pub fn is_insufficient_material(board: &Board) -> bool {
    let mut non_king_pieces = Vec::new();

    for square in 0..64 {
        let piece = match board.piece_at(square) {
            Some(piece) => piece,
            None => continue,
        };

        if piece.kind == PieceType::King {
            continue;
        }

        non_king_pieces.push((square, piece.color, piece.kind));
    }

    if non_king_pieces.is_empty() {
        return true;
    }

    if non_king_pieces
        .iter()
        .any(|(_, _, piece)| matches!(piece, PieceType::Pawn | PieceType::Rook | PieceType::Queen))
    {
        return false;
    }

    if non_king_pieces.len() == 1 {
        return matches!(non_king_pieces[0].2, PieceType::Bishop | PieceType::Knight);
    }

    if non_king_pieces.len() == 2 {
        let first = non_king_pieces[0];

        let second = non_king_pieces[1];

        if first.2 == PieceType::Bishop && second.2 == PieceType::Bishop && first.1 != second.1 {
            return square_color(first.0) == square_color(second.0);
        }
    }

    false
}

fn square_color(square: usize) -> bool {
    let file = square % 8;
    let rank = square / 8;

    (file + rank) % 2 == 0
}

pub fn insufficient_material_description(board: &Board) -> String {
    let mut pieces = Vec::new();

    for square in 0..64 {
        if let Some(piece) = board.piece_at(square) {
            if piece.kind != PieceType::King {
                pieces.push(format!("{:?} {:?}", piece.color, piece.kind));
            }
        }
    }

    if pieces.is_empty() {
        return "King vs King".to_string();
    }

    pieces.join(" vs ")
}

fn position_key(
    board: &Board,
    turn: Color,
    castling_rights: [bool; 6],
    previous_move: Option<&MoveIR>,
) -> String {
    let mut key = String::with_capacity(128);

    for square in 0..64 {
        match board.piece_at(square) {
            Some(piece) => {
                let color_code = match piece.color {
                    Color::White => 'w',
                    Color::Black => 'b',
                };

                let piece_code = match piece.kind {
                    PieceType::King => 'k',
                    PieceType::Queen => 'q',
                    PieceType::Rook => 'r',
                    PieceType::Bishop => 'b',
                    PieceType::Knight => 'n',
                    PieceType::Pawn => 'p',
                };

                key.push(color_code);
                key.push(piece_code);
            }

            None => {
                key.push('.');
                key.push('.');
            }
        }
    }

    key.push('|');

    match turn {
        Color::White => key.push('w'),
        Color::Black => key.push('b'),
    }

    key.push('|');

    for allowed in castling_rights {
        if allowed {
            key.push('1');
        } else {
            key.push('0');
        }
    }

    key.push('|');

    match en_passant_target(previous_move) {
        Some(square) => {
            key.push_str(&square.to_string());
        }

        None => {
            key.push('-');
        }
    }

    key
}

fn en_passant_target(previous_move: Option<&MoveIR>) -> Option<usize> {
    let previous = previous_move?;

    if previous.piece != PieceType::Pawn {
        return None;
    }

    let delta = previous.to as i32 - previous.from as i32;

    if delta.abs() != 16 {
        return None;
    }

    let midpoint = (previous.from as i32 + previous.to as i32) / 2;

    Some(midpoint as usize)
}
