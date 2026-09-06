
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

pub fn square(file: char, rank: u8) -> usize {
    let file_index = file as usize - 'a' as usize;
    let rank_index = (rank - 1) as usize;

    rank_index * 8 + file_index
}
