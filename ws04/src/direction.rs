use std::ops::Mul;

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

/// Implements the `Mul` trait for the `Direction` struct, allowing it to be multiplied by an `i32` scalar.
/// 
/// # Parameters
/// - `self`: The `Direction` instance to be multiplied.
/// - `scalar`: The `i32` value by which the `Direction` instance will be multiplied.
/// 
/// # Returns
/// A new `Direction` instance with its `x` and `y` fields multiplied by the given scalar.
/// 
/// # Example
/// ```
/// let direction = Direction { x: 2, y: 3 };
/// let scaled_direction = direction * 4;
/// assert_eq!(scaled_direction.x, 8);
/// assert_eq!(scaled_direction.y, 12);
/// ```
impl Mul<i32> for Direction {
    type Output = Direction;

    fn mul(self, scalar: i32) -> Self::Output {
        Direction {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
