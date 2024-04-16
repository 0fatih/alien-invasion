use crate::world::{self, World};

#[derive(Debug)]
pub struct Alien {
    pub name: String,
    pub current_city: String,
    pub is_trapped: bool,
    pub is_dead: bool,
}

impl Alien {
    pub fn new(name: &str, current_city: &str) -> Alien {
        Alien {
            name: name.to_string(),
            current_city: current_city.to_string(),
            is_trapped: false,
            is_dead: false,
        }
    }
}
