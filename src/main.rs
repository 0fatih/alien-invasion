use std::borrow::Borrow;

mod city;
mod error;
mod world;

fn main() {
    let mut world = world::World::new();
    
    world.load_map("../map.txt");

    world.print_map();

    world.destroy_city("C8");

}
