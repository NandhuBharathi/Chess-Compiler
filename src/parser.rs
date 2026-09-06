use crate::move_engine::ChessMove;

pub fn parse_move(input: &str) -> Option<ChessMove> {
    let input = input.trim().to_lowercase();

    if input.len() != 4 {
        return None;
    }

    let bytes = input.as_bytes();

    let from_file = bytes[0];
    let from_rank = bytes[1];
    let to_file = bytes[2];
    let to_rank = bytes[3];

    if !(b'a'..=b'h').contains(&from_file)
        || !(b'1'..=b'8').contains(&from_rank)
        || !(b'a'..=b'h').contains(&to_file)
        || !(b'1'..=b'8').contains(&to_rank)
    {
        return None;
    }

    let from = ((from_rank - b'1') as usize * 8) + (from_file - b'a') as usize;

    let to = ((to_rank - b'1') as usize * 8) + (to_file - b'a') as usize;

    Some(ChessMove::new(from, to))
}
