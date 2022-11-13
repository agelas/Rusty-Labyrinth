mod parts;

use crate::parts::position::Position;
fn main() {
    let p = Position::new(0, 0);
    println!("x: {}, y: {}", p.get_x(), p.get_y());
}
