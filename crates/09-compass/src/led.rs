#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

const NORTH: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

const NORTH_EAST: [[u8; 5]; 5] = [
    [1, 1, 0, 0, 0],
    [1, 1, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0],
];

const EAST: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

const SOUTH_EAST: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [1, 1, 0, 0, 0],
    [1, 1, 0, 0, 0],
];

const SOUTH: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
];

const SOUTH_WEST: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 1, 1],
    [0, 0, 0, 1, 1],
];

const WEST: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0],
];

const NORTH_WEST: [[u8; 5]; 5] = [
    [0, 0, 0, 1, 1],
    [0, 0, 0, 1, 1],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

impl Direction {
    pub const fn to_led_display(self) -> [[u8; 5]; 5] {
        match self {
            Self::North => NORTH,
            Self::NorthEast => NORTH_EAST,
            Self::East => EAST,
            Self::SouthEast => SOUTH_EAST,
            Self::South => SOUTH,
            Self::SouthWest => SOUTH_WEST,
            Self::West => WEST,
            Self::NorthWest => NORTH_WEST,
        }
    }
}
