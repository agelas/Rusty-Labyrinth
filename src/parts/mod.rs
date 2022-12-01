mod tiles;
pub(crate) mod position;

pub mod maze {
    use super::{tiles::{Tile, Floor}, position::Position};
    use std::fs;

    pub struct Maze {
        height: i8,
        width: i8,
        grid: Vec<Box<dyn Tile>>, // Vector of objects that implement Tile trait
    }

    impl Maze {
        pub fn new(in_height: i8, in_width: i8) -> Maze {
            let mut new_grid: Vec<Box<dyn Tile>> = Vec::new();
            for _i in 0..(in_height*in_width) {
                new_grid.push(Box::new(Floor::new()));
            }
            Maze{height: in_height, width: in_width, grid: new_grid}
        }

        pub fn get_height(&self) -> i8 {
            self.height
        }

        pub fn get_width(&self) -> i8 {
            self.width
        }

        pub fn in_bounds(&self, position: &Position) -> bool {
            position.in_bounds(&self.width, &self.height)
        }

        pub fn set_tile(& mut self, position: &Position, tile: Box<dyn Tile>) {
            let index = position.get_y() * self.width + position.get_x();
            self.grid[index as usize] = tile; // All indexes are of type usiz?
        }

        pub fn get_tile(&self, position: &Position) -> &Box<dyn Tile> {
            let index = position.get_y() * self.width + position.get_x();
            &self.grid[index as usize]
        }

        pub fn read(file_path: &String) {
            let file_contents = fs::read_to_string(file_path).expect("Should read file");
            println!("{}", file_contents);
        }

        pub fn print() {
            todo!();
        }
    }
}