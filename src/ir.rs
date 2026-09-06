use crate::move_engine::{ChessMove, Color, PieceType};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveKind {
    Normal,
    Capture,
    CastleKingSide,
    CastleQueenSide,
    EnPassant,
    Promotion,
}

#[derive(Clone, Copy, Debug)]
pub struct MoveIR {
    pub piece: PieceType,
    pub color: Color,

    pub from: usize,
    pub to: usize,

    pub kind: MoveKind,

    pub captured_piece: Option<PieceType>,
    pub captured_color: Option<Color>,

    pub promotion: Option<PieceType>,
}

impl MoveIR {
    pub fn from_move(
        chess_move: ChessMove,
        piece: PieceType,
        color: Color,
        captured_piece: Option<PieceType>,
        captured_color: Option<Color>,
    ) -> Self {
        let kind = if captured_piece.is_some() {
            MoveKind::Capture
        } else {
            MoveKind::Normal
        };

        Self {
            piece,
            color,
            from: chess_move.from,
            to: chess_move.to,
            kind,
            captured_piece,
            captured_color,
            promotion: None,
        }
    }

    pub fn with_kind(mut self, kind: MoveKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_promotion(mut self, promotion: PieceType) -> Self {
        self.kind = MoveKind::Promotion;
        self.promotion = Some(promotion);
        self
    }

    pub fn capture_description(&self) -> String {
        match (self.captured_color, self.captured_piece) {
            (Some(color), Some(piece)) => {
                format!("{:?} {:?}", color, piece)
            }

            _ => "None".to_string(),
        }
    }
}
