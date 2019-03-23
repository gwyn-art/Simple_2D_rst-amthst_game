use amethyst::{
  ecs::prelude::*
};

pub struct Minotaur {
  width: f32,
  height: f32
}


impl Minotaur {
  pub fn new () -> Minotaur {
    Minotaur {
      width: 27.,
      height: 34.
    }
  }
}

impl Component for Minotaur {
  type Storage = DenseVecStorage<Self>;
}