/// Represents a direction as a 2D vector with x and y components.
/// 
/// # Examples
/// 
/// ```
/// use ws04::direction::Direction;
/// 
/// let dir = Direction { x: 1, y: -1 };
/// assert_eq!(dir.x, 1);
/// assert_eq!(dir.y, -1);
/// ```
pub struct Direction {
    pub x: i32,
    pub y: i32
}

/// Represents the four cardinal directions.
/// 
/// # Examples
/// 
/// ```
/// use ws04::direction::{CardinalDirection, Direction};
/// 
/// let north = CardinalDirection::North;
/// let dir: Direction = north.into();
/// assert_eq!(dir.x, 0);
/// assert_eq!(dir.y, 1);
/// ```
pub enum CardinalDirection {
    North,
    East,
    South,
    West
}

impl From<CardinalDirection> for Direction {
    fn from(dir: CardinalDirection) -> Direction {
        match dir {
            CardinalDirection::North => Direction { x: 0, y: 1 },
            CardinalDirection::East => Direction { x: 1, y: 0 },
            CardinalDirection::South => Direction { x: 0, y: -1 },
            CardinalDirection::West => Direction { x: -1, y: 0 },
        }
    }
}
