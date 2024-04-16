use tracing::info;
use crate::{error::{AppError, Result}, world::Direction};

#[derive(Debug, Clone)]
pub struct City<'a> {
  pub name: String,
  pub is_destroyed: bool,
  pub north: Option<&'a City<'a>>,
  pub south: Option<&'a City<'a>>,
  pub east: Option<&'a City<'a>>,
  pub west: Option<&'a City<'a>>,
}

impl<'a> City<'a> {
  pub fn new(name: String) -> City<'a> {
    City {
      name: name,
      is_destroyed: false,
      north: None,
      south: None,
      east: None,
      west: None,
    }
  }

  pub fn set_destinations(&mut self, direction: Direction, destination: &'a City) -> Result<()> {
    match direction {
      Direction::North => {
        if self.north.is_some() {
          return Err(AppError::RouteAlreadyExists);
        }
        self.north = Some(destination);
      },
      Direction::South => {
        if self.south.is_some() {
          return Err(AppError::RouteAlreadyExists);
        }
        self.south = Some(destination);
      },
      Direction::East => {
        if self.east.is_some() {
          return Err(AppError::RouteAlreadyExists);
        }
        self.east = Some(destination);
      },
      Direction::West => {
        if self.west.is_some() {
          return Err(AppError::RouteAlreadyExists);
        }
        self.west = Some(destination);
      },
    }

    Ok(())
  }


  pub fn destroy_self(&mut self, alien1: i32, alien2: i32) {
    info!("{} has been destroyed by alien {} and alien {}", self.name, alien1, alien2);
    self.is_destroyed = true;
    self.north = None;
    self.south = None;
    self.east = None;
    self.west = None;
  }
}