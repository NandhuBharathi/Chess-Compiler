#[derive(Clone, Copy, Debug)]
pub struct ChessMove {
    pub from: usize,
    pub to: usize,
}

impl ChessMove {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

pub fn is_legal_move(piece: PieceType, color: Color, from: usize, to: usize) -> bool {
    if from >= 64 || to >= 64 || from == to {
        return false;
    }

    let from_file = (from % 8) as i32;
    let from_rank = (from / 8) as i32;

    let to_file = (to % 8) as i32;
    let to_rank = (to / 8) as i32;

    let dx = (to_file - from_file).abs();
    let dy = (to_rank - from_rank).abs();

    match piece {
        PieceType::Pawn => {
            if color == Color::White {
                dx == 0 && (to_rank - from_rank == 1 || (from_rank == 1 && to_rank == 3))
            } else {
                dx == 0 && (from_rank - to_rank == 1 || (from_rank == 6 && to_rank == 4))
            }
        }

        PieceType::Knight => (dx == 1 && dy == 2) || (dx == 2 && dy == 1),

        PieceType::Bishop => dx == dy,

        PieceType::Rook => dx == 0 || dy == 0,

        PieceType::Queen => dx == dy || dx == 0 || dy == 0,

        PieceType::King => dx <= 1 && dy <= 1,
    }
}

pub fn path_is_clear(squares: &[Option<(Color, PieceType)>; 64], from: usize, to: usize) -> bool {
    let from_file = (from % 8) as i32;
    let from_rank = (from / 8) as i32;

    let to_file = (to % 8) as i32;
    let to_rank = (to / 8) as i32;

    let step_file = (to_file - from_file).signum();
    let step_rank = (to_rank - from_rank).signum();

    let mut file = from_file + step_file;
    let mut rank = from_rank + step_rank;

    while file != to_file || rank != to_rank {
        let square = (rank * 8 + file) as usize;

        if squares[square].is_some() {
            return false;
        }

        file += step_file;
        rank += step_rank;
    }

    true
}
