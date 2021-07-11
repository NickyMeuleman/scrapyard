#[derive(Debug)]
pub struct ChessPosition(i8, i8);

#[derive(Debug)]
pub struct Queen(i8, i8);

impl ChessPosition {
    pub fn new(rank: i8, file: i8) -> Option<Self> {
        // match (rank, file) {
        //     (rank, file) if((0..=7).contains(&rank) && (0..=7).contains(&file)) => Some(ChessPosition(rank, file)),
        //     _ => None
        // }
        // equivalent code with @ binding
        // match (rank, file) {
        //     (rank @ 0..=7, file @ 0..=7) => Some(ChessPosition(rank, file)),
        //     _ => None,
        // }
        // That @ sign is unnecessary in this case
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition(rank, file)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position.0, position.1)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // if the slope of the line between the queens is +/-1, they're on the same diagonal
        self.0 == other.0
            || self.1 == other.1
            || (self.0 - other.0).abs() == (self.1 - other.1).abs()
    }
}
