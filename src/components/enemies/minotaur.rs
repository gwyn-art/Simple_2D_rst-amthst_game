use amethyst::{
  ecs::prelude::*
};

pub struct Minotaur {
  pub health_points: i32,
  pub attack_damage: i32
}


impl Minotaur {
  pub fn new () -> Minotaur {
    Minotaur {
      health_points: 700,
      attack_damage: 20
    }
  }
}

impl Component for Minotaur {
  type Storage = DenseVecStorage<Self>;
}