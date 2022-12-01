use parts::maze::Maze;
use crate::parts::position::Position;
use std::env;

mod parts;
mod entities;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    Maze::read(file_path)
}
