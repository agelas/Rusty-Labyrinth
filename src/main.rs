use parts::maze::Maze;
use std::env;

mod parts;
mod entities;
mod game;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let m:Maze = Maze::read(file_path);
    m.print();
}
