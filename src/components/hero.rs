use amethyst::{
  ecs::prelude::*
};

pub struct Hero {
  width: f32,
  height: f32,
  pub is_walking: bool,
  is_attacking: bool,
  // Time need to spend for one attack
  attack_time: f64,
  // Time when prev attack was started
  attack_start_time: f64
}

impl Hero {
  pub fn new () -> Hero {
    Hero {
      width: 20.,
      height: 31.,
      is_walking: false,
      is_attacking: false,
      attack_time: 1.0,
      attack_start_time: 0.
    }
  }

  pub fn set_walking(&mut self, is_walking: bool) {
    self.is_walking = is_walking;
  }

  pub fn get_attack_time(&self) -> f64 {
    self.attack_time
  }

  pub fn is_attacking(&self) -> bool {
    self.is_attacking
  }

  pub fn attack(&mut self, time: f64) {
    if time > self.attack_time + self.attack_start_time {
      self.attack_start_time = time;
      self.is_attacking = true;
    }
  }

  pub fn stop_attack(&mut self, time: f64) {
    if self.attack_start_time + self.attack_time < time {
      self.is_attacking = false;
    } 
  }
}

impl Component for Hero {
  type Storage = DenseVecStorage<Self>;
}