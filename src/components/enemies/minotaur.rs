use amethyst::{
  ecs::prelude::*
};

pub struct Minotaur;


impl Minotaur {
  pub fn new () -> Minotaur {
    Minotaur {
    }
  }
}

impl Component for Minotaur {
  type Storage = DenseVecStorage<Self>;
}