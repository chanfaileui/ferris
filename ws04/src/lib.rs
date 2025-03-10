#[warn(missing_docs)]
pub mod coordinate;
pub mod direction;

#[cfg(test)]
mod tests {
    use crate::coordinate::Coordinate;
    use crate::direction::Direction;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn addAssign_works() {
        let mut a = Coordinate::new(1, 2);
        a += Direction {x: 1, y: 0};
        assert_eq!(a, Coordinate::new(2, 2));
    }
}
