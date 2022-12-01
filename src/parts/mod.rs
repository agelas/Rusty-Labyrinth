mod tiles;
pub(crate) mod position;

mod maze {
    use super::{tiles::Tile, position::Position};


    struct Maze {
        height: i8,
        width: i8,
        grid: Vec<Box<dyn Tile>>, // Vector of objects that implement Tile trait
    }

    impl Maze {
        pub fn get_height(&self) -> i8 {
            self.height
        }

        pub fn get_width(&self) -> i8 {
            self.width
        }

        pub fn in_bounds(&self, position: &Position) -> bool {
            position.in_bounds(self.width, self.height)
        }

        pub fn set_tile(& mut self, position: &Position, tile: Box<dyn Tile>) {
            let index = position.get_y() * self.width + position.get_x();
            self.grid[index as usize] = tile; // All indexes are of type usiz?
        }

        pub fn get_tile(&self, position: &Position) -> &Box<dyn Tile> {
            let index = position.get_y() * self.width + position.get_x();
            &self.grid[index as usize]
        }

        pub fn read() -> Vec<Box<dyn Tile>> {
            todo!();
        }

        pub fn print() {
            todo!();
        }
    }
}