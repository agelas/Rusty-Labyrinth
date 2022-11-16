pub mod entities{
    use crate::parts::position::Position;

    /*
        An Entity is meant to represent a generic game
        entity like the hero or minotaur.
    */
    pub struct Entity {
        glyph: String,
        properties: String,
        position: Position,
    }

    impl Entity {

        pub fn set_glyph(& mut self, glyph: String) {
            self.glyph = glyph;
        }

        pub fn get_glyph(&self) -> String {
            self.glyph.clone() // Idk why 
        }

        pub fn set_properties(& mut self, props: String) {
            self.properties = props;
        }

        pub fn get_properties(&self) -> String {
            self.properties.clone()
        }

        pub fn has_property(&self, prop: String) -> bool {
            self.properties.contains(&prop)
        }

        pub fn set_position(& mut self, pos: Position) {
            self.position = pos;
        }

        pub fn get_position(&self) -> &Position {
            &self.position // I guess we "borrow" the Entity's Position struct??
        }
    }
}