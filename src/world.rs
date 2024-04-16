use std::{collections::HashMap, str::FromStr};
use tracing::error;
use crate::error::AppError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
  North,
  South,
  East,
  West,
}

impl Direction {
  pub fn get_opposite(&self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::South => Direction::North,
      Direction::East => Direction::West,
      Direction::West => Direction::East,
    }
  }
}

impl FromStr for Direction {
  type Err = AppError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s.to_lowercase().as_str() {
          "north" => Ok(Direction::North),
          "south" => Ok(Direction::South),
          "east" => Ok(Direction::East),
          "west" => Ok(Direction::West),
          _ => Err(AppError::WrongDirection)
      }
  }
}

#[derive(Debug)]
pub struct World {
  pub cities: Vec<String>,
  // pub routes: MultiMap<String, Direction, String>
  pub routes: HashMap<String, HashMap<Direction, String>>
}

impl World {
  pub fn new() -> World {
    World {
      cities: Vec::new(),
      routes: HashMap::new(),
    }
  }

  pub fn destroy_city(&mut self, city_name: &str) {
    self.cities.retain(|city| city != city_name);
    self.routes.remove(city_name);
    for (_, routes) in self.routes.iter_mut() {
      routes.retain(|_, destination| destination != city_name);
    }
  }

  pub fn load_map(&mut self, file_name: &str) {
    // Load map from file
    let file = std::fs::read_to_string(file_name).expect("Failed to read map file");

    for line in file.lines() {
      let line = line.trim();
      let parts: Vec<&str> = line.split(' ').collect();
      let city_name = parts[0];

      self.cities.push(city_name.to_string());

      let mut rts = if self.routes.contains_key(city_name) {
        self.routes.get(city_name).unwrap().clone()
      } else {
        HashMap::new()
      };

      for part in parts.iter().skip(1) {
        let route_parts: Vec<&str> = part.split('=').collect();
        if route_parts.len() != 2 {
          error!("Invalid route: {}", part);
          continue;
        }
        
        let direction = Direction::from_str(route_parts[0]).unwrap();
        let destination = route_parts[1];

        let mut dest_rts = if self.routes.contains_key(destination) {
          self.routes.get(destination).unwrap().clone()
        } else {
          HashMap::new()
        };
        
        // Make route bidirectional
        dest_rts.insert(direction.get_opposite(), city_name.to_string());
        rts.insert(direction, destination.to_string());

        self.routes.insert(destination.to_string(), dest_rts);
      }
      self.routes.insert(city_name.to_string(), rts);
    }
  }

  pub fn print_map(&self) {
    for (city, routes) in self.routes.iter() {
      println!("{}:", city);
      for (direction, destination) in routes.iter() {
        println!("  {:?} -> {}", direction, destination);
      }
    }
  }
}