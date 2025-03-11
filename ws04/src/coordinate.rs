use crate::direction::Direction;
use std::cmp::{max, min};
use std::convert::From;
use std::default::Default;
use std::ops::{Add, AddAssign};

/// Represent a 2D coordinate.
/// 
/// # Examples
/// 
/// ```
/// use ws04::coordinate::Coordinate;
/// 
/// let coord = Coordinate::new(3, 4);
/// assert_eq!(coord.x, 3);
/// assert_eq!(coord.y, 4);
/// ```
#[derive(PartialEq, Debug)]
pub struct Coordinate {
    /// The x coordinate.
    pub x: i32,
    /// The y coordinate.
    pub y: i32,
}

impl Coordinate {
    /// Create a new coordinate.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// 
    /// let coord = Coordinate::new(5, -2);
    /// assert_eq!(coord.x, 5);
    /// assert_eq!(coord.y, -2);
    /// ```
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    // Check if the coordinate is within the rectangle defined by two other coordinates.
    /// 
    /// The rectangle is defined by the two coordinates, regardless of their relative positions.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// 
    /// let point = Coordinate::new(3, 3);
    /// let corner1 = Coordinate::new(1, 1);
    /// let corner2 = Coordinate::new(5, 5);
    /// 
    /// assert!(point.is_within(&corner1, &corner2));
    /// 
    /// // Works even if corners are not in top-left/bottom-right order
    /// let corner3 = Coordinate::new(5, 1);
    /// let corner4 = Coordinate::new(1, 5);
    /// assert!(point.is_within(&corner3, &corner4));
    /// 
    /// // Point outside the rectangle
    /// let outside = Coordinate::new(6, 6);
    /// assert!(!outside.is_within(&corner1, &corner2));
    /// ```
    pub fn is_within(&self, v1: &Coordinate, v2: &Coordinate) -> bool {
        let left = min(v1.x, v2.x);
        let right = max(v1.x, v2.x);
        let bottom = min(v1.y, v2.y);
        let top = max(v1.y, v2.y);
    
        self.x >= left && self.x <= right && self.y >= bottom && self.y <= top
    }

    /// Calculate the Euclidean distance between two coordinates.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// 
    /// let a = Coordinate::new(0, 0);
    /// let b = Coordinate::new(3, 4);
    /// assert_eq!(a.distance(&b), 5.0);
    /// ```
    pub fn distance(&self, other: &Coordinate) -> f64 {
        let x_dist = (self.x - other.x) as f64;
        let y_dist = (self.y - other.y) as f64;

        (x_dist * x_dist + y_dist * y_dist).sqrt()
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Coordinate { x: 0, y: 0 }
    }
}

/// Allows adding two coordinates together.
/// 
/// # Examples
/// 
/// ```
/// use ws04::coordinate::Coordinate;
/// 
/// let a = Coordinate::new(1, 2);
/// let b = Coordinate::new(3, 4);
/// let result = a + b;
/// 
/// assert_eq!(result, Coordinate::new(4, 6));
/// ```
impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Allows adding a Direction to a Coordinate.
/// 
/// # Examples
/// 
/// ```
/// use ws04::coordinate::Coordinate;
/// use ws04::direction::Direction;
/// 
/// let pos = Coordinate::new(5, 5);
/// let dir = Direction { x: 2, y: -1 };
/// let new_pos = pos + dir;
/// 
/// assert_eq!(new_pos, Coordinate::new(7, 4));
/// ```
impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Direction) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Allows modifying a Coordinate by adding a Direction to it.
/// 
/// # Examples
/// 
/// ```
/// use ws04::coordinate::Coordinate;
/// use ws04::direction::{Direction, CardinalDirection};
/// 
/// let mut pos = Coordinate::new(5, 5);
/// pos += Direction::from(CardinalDirection::East);
/// 
/// assert_eq!(pos, Coordinate::new(6, 5));
/// ```
impl AddAssign<Direction> for Coordinate {
    fn add_assign(&mut self, rhs: Direction) {
        *self = Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Allows converting a Direction to a Coordinate.
/// 
/// # Examples
/// 
/// ```
/// use ws04::coordinate::Coordinate;
/// use ws04::direction::Direction;
/// use std::convert::From;
/// 
/// let dir = Direction { x: 3, y: 4 };
/// let coord = Coordinate::from(dir);
/// 
/// assert_eq!(coord.x, 3);
/// assert_eq!(coord.y, 4);
/// ```
impl From<Direction> for Coordinate {
    fn from(dir: Direction) -> Coordinate {
        Coordinate { x: dir.x, y: dir.y }
    }
}

