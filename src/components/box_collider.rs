use amethyst::{
  ecs::{
    prelude::{
      Entity,
      Component,
      VecStorage
    }
  }
};
use rand::{ thread_rng, Rng};


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ColliderType {
  HeroAttack,
  HeroBody,
  EnemyAttack,
  EnemyBody
}

#[derive(Clone)]
pub struct BoxCollider2D {
  pub id: i32,
  pub width: f32,
  pub height: f32,
  pub x: f32,
  pub y: f32,
  tag: ColliderType,
  colliding_with: Vec<ColliderType>,
  pub is_active: bool
}

impl BoxCollider2D {
  
  pub fn new(x: f32, y: f32, width: f32, height: f32, tag: ColliderType, is_active: bool) -> Self {
    let mut rand = thread_rng();
    let id = rand.gen_range(0, 843921);

    BoxCollider2D {
      id,
      x,
      y,
      width,
      height,
      tag: tag,
      colliding_with: Vec::new(),
      is_active: is_active
    }
  }

  pub fn change_colliding_with(&mut self, colliding_with: Vec<ColliderType>) {
    self.colliding_with = colliding_with;
  }

  pub fn get_colliding_with(&self) -> Vec<ColliderType> {
    self.colliding_with.clone()
  }

  pub fn get_tag(&self) -> ColliderType {
    self.tag
  }
}

impl Component for BoxCollider2D {
  type Storage = VecStorage<Self>;
}