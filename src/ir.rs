
use crate::move_engine::{
    ChessMove,
    Color,
    PieceType,
};

#[derive(Clone, Copy, Debug)]
pub struct MoveIR {
    pub piece: PieceType,
    pub color: Color,

    pub from: usize,
    pub to: usize,

    pub captured_piece: Option<PieceType>,
    pub captured_color: Option<Color>,
}

impl MoveIR {
    pub fn from_move(
        chess_move: ChessMove,
        piece: PieceType,
        color: Color,
        captured_piece: Option<PieceType>,
        captured_color: Option<Color>,
    ) -> Self {
        Self {
            piece,
            color,
            from: chess_move.from,
            to: chess_move.to,
            captured_piece,
            captured_color,
        }
    }

    pub fn capture_description(&self) -> String {
        match (
            self.captured_color,
            self.captured_piece,
        ) {
            (Some(color), Some(piece)) => {
                format!(
                    "{:?} {:?}",
                    color,
                    piece
                )
            }

            _ => {
                "None".to_string()
            }
        }
    }
}
