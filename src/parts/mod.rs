mod tiles;

pub mod position {
    pub enum Direction {
        UP, DOWN, LEFT, RIGHT,
    }

    #[derive(Debug)]
    pub struct Position {
        x: i8,
        y: i8,
    }
    
    impl Position {

        pub fn new(x: i8, y:i8) -> Position { //so this is how we do constructors?
            Position { x: x, y: y }
        }

        pub fn get_x(&self) -> i8 {
            self.x
        }

        pub fn get_y(&self) -> i8 {
            self.y
        }

        pub fn displace(&mut self, dir: Direction) {
            // Return the Position that is displaced by moving 1 step in the specified Direction.
            // Useful for carrying out moves or evaluating hypothetical moves. 
            match dir {
                Direction::UP => {
                    self.y = self.y + 1;
                }
                Direction::DOWN => {
                    self.y = self.y - 1;
                }
                Direction::LEFT => {
                    self.x = self.x - 1;
                }
                Direction::RIGHT => {
                    self.x = self.x + 1;
                }
            }
        }

        pub fn distance_from(&self, other: Position) -> i8 {
            // Get distance from this Position instance to another one
            let mut x_diff: i8 = self.x - other.x;
            x_diff = x_diff.abs();
            let mut y_diff: i8 = self.y - other.y;
            y_diff = y_diff.abs();
            x_diff + y_diff
        }

        pub fn in_bounds(&self, width: i8, height: i8) -> bool {
            if self.x >= 0 && self.x < width && self.y >= 0 && self.y < height {
                true
            } else {
                false
            }
        }
    }

    impl PartialEq for Position {
        // Override equality
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
}

/*
    So unit tests go in the same file in rust?
*/
#[cfg(test)]
mod tests {
    use crate::parts::position::Direction;

    use super::position::Position;

    #[test]
    fn test_new() {
        let test_p: Position = Position::new(1, 1);
        assert_eq!(test_p.get_x(), 1);
        assert_eq!(test_p.get_y(), 1);
    }

    #[test]
    fn test_displace() {
        let mut test_p: Position = Position::new(0, 0);
        test_p.displace(Direction::UP);
        assert_eq!(test_p.get_y(), 1);
        assert_eq!(test_p.get_x(), 0);
        test_p.displace(Direction::RIGHT);
        assert_eq!(test_p.get_y(), 1);
        assert_eq!(test_p.get_x(), 1);
        test_p.displace(Direction::DOWN);
        assert_eq!(test_p.get_y(), 0);
        assert_eq!(test_p.get_x(), 1);
        test_p.displace(Direction::LEFT);
        assert_eq!(test_p.get_y(), 0);
        assert_eq!(test_p.get_x(), 0);
    }

    #[test]
    fn test_distance_from() {
        let test_p1: Position = Position::new(0, 0);
        let test_p2: Position = Position::new(2, 2);
        let result: i8 = test_p1.distance_from(test_p2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_in_bounds() {
        let test_p: Position = Position::new(2, 2);
        let result: bool = test_p.in_bounds(3, 3);
        assert_eq!(result, true);
    }
}