pub mod game {

    pub struct Game {
        maze: Maze,
        entity_vector: Vec<Box<Entity>>,
        // UI
        // GameRules
    }

    impl Game {
        pub fn new(maze: Maze) -> Game {
            Game{maze}
        }

        pub fn set_maze(&mut self, maze: Maze) {
            self.maze = maze;
        }

        pub fn get_maze(&self) -> Maze {
            self.maze
        }

        pub fn add_entity(&self, entity: Entity) {
            let boxed_entity = Box::new(entity);
            self.entity_vector.push(boxed_entity);
        }

        pub fn get_entity_at(&self, position: Position) -> Option<Entity> {
            // @return Option<Entity> because there might not be an entity that matches, so Option<> allows for None
            for entity in self.entity_vector.iter() {
                if(entity.has_property('m')) {
                    return Some(entity.clone()); // Some() is an enum variant that needs to be used w/Option
                }
            }
            None
        }

        pub fn get_entities_with_property(&self, property: String) -> Vec<Box<Entity>> {
            let mut matching_entities = Vec::new();

            for entity in self.entity_vector.iter() {
                if entity.has_property(property) {
                    matching_entities.push(entity.clone()); // Cloning creates a new heap allocated Entity that isn't tied to lifetime of entity_vector
                }
            }
            matching_entities
        }

        pub fn get_entities(&self) -> Vec<Box<Entity>> {
            self.entity_vector
        }
    }

}