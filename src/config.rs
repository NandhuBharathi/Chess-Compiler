
use crate::move_engine::PieceType;

#[derive(Clone, Copy, Debug)]
pub struct GameConfig {
    pub time_limit_enabled: bool,
    pub time_limit_seconds: u64,
}

impl GameConfig {
    pub fn no_time_limit() -> Self {
        Self {
            time_limit_enabled: false,
            time_limit_seconds: 0,
        }
    }

    pub fn with_time_limit(seconds: u64) -> Self {
        Self {
            time_limit_enabled: true,
            time_limit_seconds: seconds,
        }
    }
}

pub fn piece_points(piece: PieceType) -> u32 {
    match piece {
        PieceType::Pawn => 1,
        PieceType::Knight => 3,
        PieceType::Bishop => 3,
        PieceType::Rook => 5,
        PieceType::Queen => 9,
        PieceType::King => 0,
    }
}
