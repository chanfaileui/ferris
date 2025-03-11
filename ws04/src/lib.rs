#[warn(missing_docs)]
/// Module for handling coordinates.
pub mod coordinate;
/// Module for handling directions.
pub mod direction;

#[cfg(test)]
mod tests {
    use crate::coordinate::Coordinate;
    use crate::direction::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn add_assign_test() {
        let mut a = Coordinate::new(1, 2);
        a += Direction { x: 1, y: 0 };
        assert_eq!(a, Coordinate::new(2, 2));
    }

    #[test]
    fn test_cardinal_direction_conversion() {
        // Test North
        let north_dir = Direction::from(CardinalDirection::North);
        assert_eq!(north_dir.x, 0);
        assert_eq!(north_dir.y, 1);

        // Test East
        let east_dir = Direction::from(CardinalDirection::East);
        assert_eq!(east_dir.x, 1);
        assert_eq!(east_dir.y, 0);

        // Test South
        let south_dir = Direction::from(CardinalDirection::South);
        assert_eq!(south_dir.x, 0);
        assert_eq!(south_dir.y, -1);

        // Test West
        let west_dir = Direction::from(CardinalDirection::West);
        assert_eq!(west_dir.x, -1);
        assert_eq!(west_dir.y, 0);
    }

    #[test]
    fn test_direction_addition() {
        let mut pos = Coordinate::new(5, 5);

        // Move east
        pos += Direction::from(CardinalDirection::East);
        assert_eq!(pos, Coordinate::new(6, 5));

        // Move north
        pos += Direction::from(CardinalDirection::North);
        assert_eq!(pos, Coordinate::new(6, 6));

        // Move west
        pos += Direction::from(CardinalDirection::West);
        assert_eq!(pos, Coordinate::new(5, 6));

        // Move south
        pos += Direction::from(CardinalDirection::South);
        assert_eq!(pos, Coordinate::new(5, 5));
    }

    #[test]
    fn within_test() {
        let v1 = Coordinate::new(0, 0);
        let v2 = Coordinate::new(4, 4);
        let a = Coordinate::new(2, 2);
        assert!(a.is_within(&v1, &v2));
    }
}
