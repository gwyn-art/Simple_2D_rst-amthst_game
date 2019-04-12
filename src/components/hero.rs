use amethyst::{
  ecs::prelude::*
};

pub struct Hero {
  health_points: i32,
  pub is_walking: bool,
  is_attacking: bool,
  is_dead: bool,
  // Time need to spend for one attack
  attack_time: f64,
  // Time when prev attack was started
  attack_start_time: f64,
  // Time when prev taken damage
  past_damage_taken_time: f64,
}

impl Hero {
  pub fn new () -> Hero {
    Hero {
      is_walking: false,
      is_attacking: false,
      is_dead: false,
      attack_time: 1.0,
      attack_start_time: 0.,
      health_points: 100,
      past_damage_taken_time: 0.,
    }
  }

  pub fn set_walking(&mut self, is_walking: bool) {
    if self.is_dead {
      return;
    }

    self.is_walking = is_walking;
  }

  pub fn get_attack_time(&self) -> f64 {
    self.attack_time
  }

  pub fn is_attacking(&self) -> bool {
    self.is_attacking
  }

  pub fn attack(&mut self, time: f64) {
    if time > self.attack_time + self.attack_start_time && !self.is_dead {
      self.attack_start_time = time;
      self.is_attacking = true;
    }
  }

  pub fn stop_attack(&mut self, time: f64) {
    if self.attack_start_time + self.attack_time < time {
      self.is_attacking = false;
    } 
  }

  pub fn take_damage(&mut self, damage: i32, time: f64) {
    if time - 2. > self.past_damage_taken_time {
      self.health_points -= damage;
      self.past_damage_taken_time = time;
      println!("Hero HP: {}", self.health_points);
      if self.health_points <= 0 {
        self.die();
      }
    }
  }

  pub fn get_health_points(&self) -> i32 {
    self.health_points
  }

  fn stop_all_actions(&mut self) {
    self.is_attacking = false;
    self.is_walking = false;
    self.is_dead = false;
  }

  pub fn die(&mut self) {
    self.stop_all_actions();
    self.is_dead = true;
  }

  pub fn is_dead(&self) -> bool {
    self.is_dead
  }
}

impl Component for Hero {
  type Storage = DenseVecStorage<Self>;
}