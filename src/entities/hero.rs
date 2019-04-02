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
  Hero
};

pub fn create_hero(world: &mut World, sprite_sheet: SpriteSheetHandle) -> Entity {
  let mut transform = Transform::default();
  let hero = Hero::new();
  transform.set_xyz(500. / 2., 500. / 2., 0.);
  
  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet.clone(),
    sprite_number: 0,
  };
  
  let mut animation: Vec<(String, i32, SimpleAnimation)> = Vec::new();
  animation.push((String::from("Idle"), 100, SimpleAnimation::new(0, 10, 0.20)));
  animation.push((String::from("Walking"), 80, SimpleAnimation::new(10, 10, 0.1)));
  animation.push(
    (
      String::from("Attacking"), 
      60, 
      SimpleAnimation::new(20, 10, (hero.get_attack_time() as f32) / 10.)
    ));
  
  world
    .create_entity()
    .with(sprite_render)
    .with(ComplexAnimations::new(animation, String::from("Idle")))
    .with(hero)
    .with(transform)
    .build()
}