use amethyst::{
  ecs::prelude::*
};

pub mod minotaur;
pub use self::minotaur::Minotaur;


pub struct RegularEnemy {
  is_moving: bool,
  is_attacking: bool,
  is_dying: bool,
  pub dir_x: f32,
  pub dir_y: f32,
  pub speed: f32,
  past_action_time: f64,
  past_damage_taken_time: f64,
  health_points: i32,
}

impl RegularEnemy {
  pub fn new(speed: f32, health_points: i32) -> RegularEnemy {
    RegularEnemy {
      dir_x: 0.,
      dir_y: 0.,
      speed: speed,
      is_moving: false,
      is_attacking: false,
      is_dying: false,
      past_action_time: 0.,
      past_damage_taken_time: 0.,
      health_points
    }
  }

  pub fn is_moving(&self) -> bool {
    self.is_moving
  }

  pub fn is_attacking(&self) -> bool {
    self.is_attacking
  }

  pub fn stop_all_actions(&mut self) {
    self.is_moving = false;
    self.is_attacking = false;
  }

  pub fn get_past_action_time(&self) -> f64 {
    self.past_action_time
  }

  pub fn stay_idle(&mut self, time: f64) {
    self.stop_all_actions();
    self.past_action_time = time;
  }

  pub fn move_to_hero(&mut self, time: f64) {
    self.stop_all_actions();
    self.is_moving = true;
    self.past_action_time = time;
  }

  pub fn attack(&mut self, time: f64) {
    self.stop_all_actions();
    self.is_attacking = true;
    self.past_action_time = time;
  }

  pub fn take_damage(&mut self, damage: i32, time: f64) {
    if time - 2. > self.past_damage_taken_time {
      self.health_points -= damage;
      self.past_damage_taken_time = time;
      println!("HP {}.", self.health_points);
    }
  }

  pub fn get_health_points(&self) -> i32 {
    self.health_points
  }

  pub fn die(&mut self) {
    self.stop_all_actions();
    self.is_dying = true;
  }

  pub fn is_dying(&self) -> bool {
    self.is_dying
  }

  pub fn stop_moving(&mut self) {
    self.is_moving = false;
  }
}

impl Component for RegularEnemy {
  type Storage = DenseVecStorage<Self>;
}
