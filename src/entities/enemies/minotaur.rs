use amethyst::{
  prelude::*,
  renderer::{
    SpriteSheetHandle,
    SpriteRender
  },
  core::Transform,
  ecs::prelude::Entity,
};

use crate::components::{
  SimpleAnimation,
  ComplexAnimations,
  Minotaur,
  enemies::RegularEnemy
};


pub fn create_minotaur(world: &mut World, sprite_sheet: SpriteSheetHandle, x: f32, y: f32) -> Entity {
  let mut transform = Transform::default();
  let minotaur = Minotaur::new();
  let enemy = RegularEnemy::new(1.8);
  transform.set_xyz(x, y, 0.);
  transform.set_scale(2., 2., 0.);
  
  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet.clone(),
    sprite_number: 0,
  };
  
  let mut animation: Vec<(String, i32, SimpleAnimation)> = Vec::new();
  animation.push((String::from("Idle"), 100, SimpleAnimation::new(0, 10, 0.2)));
  animation.push((String::from("Walking"), 80, SimpleAnimation::new(10, 10, 0.02)));
  animation.push((String::from("Attacking"), 60, SimpleAnimation::new(20, 10, 0.18)));

  world
    .create_entity()
    .with(sprite_render)
    .with(ComplexAnimations::new(animation, String::from("Idle")))
    .with(minotaur)
    .with(enemy)
    .with(transform)
    .build()
}