use amethyst::{
  ecs::prelude::*
};

pub mod minotaur;
pub use self::minotaur::Minotaur;


pub struct RegularEnemy {
  is_moving: bool,
  is_attacking: bool,
  pub dir_x: f32,
  pub dir_y: f32,
  pub speed: f32,
  past_action_time: f64
}

impl RegularEnemy {
  pub fn new(speed: f32) -> RegularEnemy {
    RegularEnemy {
      dir_x: 0.,
      dir_y: 0.,
      speed: speed,
      is_moving: false,
      is_attacking: false,
      past_action_time: 0.,
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

  pub fn stop_moving(&mut self) {
    self.is_moving = false;
  }
}

impl Component for RegularEnemy {
  type Storage = DenseVecStorage<Self>;
}
