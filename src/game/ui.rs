use crate::{parts::{position::{Direction, Position}, maze::Maze}, entities::entity::Entity};
use std::io;

use super::game::Game;

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

    pub fn display_message(&mut self, msg : &String) {
        self.message = msg;
    }

    pub fn render(&mut self, game : &Game) {
        let m : Maze = *game.get_maze();
        let mut e : Entity = Entity::new();

        for _i in 0..m.get_height() {
            for _j in 0..m.get_width() {
                let p : Position = Position::new(_j, _i); 
                e = game.get_entity_at(p);
                if (e) {
                    println!(e.get_glyph());
                } else {
                    println!(m.get_tile(p).get_glyph());
                }
            }
            println!("");
        }
        if (self.message != "") {
            println!("{}", self.message);
            self.message = "".to_string();
        }
    }
}