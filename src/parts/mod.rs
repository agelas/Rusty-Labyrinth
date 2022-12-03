mod tiles;
pub(crate) mod position;

pub mod maze {
    use crate::parts::{position, tiles::{Wall, Goal}};

    use super::{tiles::{Tile, Floor}, position::Position};
    use std::{fs, io::BufReader};

    pub struct Maze {
        height: i8,
        width: i8,
        grid: Vec<Box<dyn Tile>>, // Vector of objects that implement Tile trait
    }

    impl Maze {
        pub fn new(in_height: i8, in_width: i8) -> Maze {
            let mut new_grid: Vec<Box<dyn Tile>> = Vec::new();
            let vec_length:i16 = i16::from(in_height)*i16::from(in_width);
            for _i in 0..vec_length {
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
            let index = i16::from(position.get_y()) * i16::from(self.width) + i16::from(position.get_x());
            self.grid[index as usize] = tile; // All indexes are of type usize?
        }

        pub fn get_tile(&self, position: &Position) -> &Box<dyn Tile> {
            let index = position.get_y() * self.width + position.get_x();
            &self.grid[index as usize]
        }

        pub fn read(file_path: &String) {
            /* This function is so ugly */
            let file_contents = fs::read_to_string(file_path).expect("Should read file");
            let mut lines = file_contents.lines();

            let mut maze_height: i8 = 0;
            let mut maze_width: i8 =  0;
            let dimensions_line = lines.next(); // Grab first line of file
            let dimensions = dimensions_line.unwrap();
            let mut dimensions = dimensions.split_whitespace().map(|s| s.parse::<i8>());
            match (dimensions.next(), dimensions.next()) { // How did Rust manage to make I/O worse than C??
                (Some(Ok(width)), Some(Ok(height))) => {
                    maze_height = height;
                    maze_width = width;
                }
                _ => {} // What is this Javascript looking nonsense?
            }

            /* Iterator has moved past first line, now read in the lines of the maze */
            let mut new_maze:Maze = Maze::new(maze_height, maze_width);

            for _i in 0..maze_height {
                let cur_line = lines.next();
                for (j, c) in cur_line.unwrap().chars().enumerate() { // Go through chars in string representation of maze line
                    let position: Position = Position::new(j.try_into().unwrap(), _i);
                    let mut tile: Box<dyn Tile> = Box::new(Wall::new()); // Set to wall by default until I figure out how to make Rust pretty
                    if c == '#' {
                        tile = Box::new(Wall::new());
                    } else if c == '.' {
                        tile = Box::new(Floor::new());
                    } else if c == '<' {
                        tile = Box::new(Goal::new());
                    } // Should prolly throw an error if no match
                    new_maze.set_tile(&position, tile);
                } 
            }
            
        }

        pub fn print() {
            todo!();
        }
    }
}