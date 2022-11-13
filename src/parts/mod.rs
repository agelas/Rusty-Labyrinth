pub mod position {
    enum Direction {
        UP, DOWN, LEFT, RIGHT,
    }

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

        fn displace(&mut self, dir: Direction) {
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

        fn distance_from(&self, other: Position) -> i8 {
            // Get distance from this Position instance to another one
            let mut x_diff: i8 = self.x - other.x;
            x_diff = x_diff.abs();
            let mut y_diff: i8 = self.y - other.y;
            y_diff = y_diff.abs();
            x_diff + y_diff
        }

        fn in_bounds(&self, width: i8, height: i8) -> bool {
            if self.x >= 0 && self.x < width && self.y >= 0 && self.y < height {
                true
            } else {
                false
            }
        }
    }
}