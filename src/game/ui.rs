use crate::parts::position::Direction;
use std::io;

pub struct UI {
    dir : Direction,
    message : String
}

impl UI {
    pub fn get_move_direction(&mut self) -> &Direction {
        let mut move_direction: String = String::new();
        let mut valid = false;

        while !valid {
            println!("Your move (w/a/s/d): ");
            io::stdin().read_line(&mut move_direction).expect("Failed to read line");

            match move_direction.trim() {
                "w" => {
                    self.dir = Direction::UP;
                    valid = true;
                }
                "a" => {
                    self.dir = Direction::LEFT;
                    valid = true;
                }
                "s" => {
                    self.dir = Direction::DOWN;
                    valid = true;
                }
                "d" => {
                    self.dir = Direction::RIGHT;
                    valid = true;
                }
                _ => {
                    println!("Unknown Direction");
                    self.dir = Direction::NONE;
                    valid = false;
                }
            }
        }
        &self.dir
    }
}