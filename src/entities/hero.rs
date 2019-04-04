use amethyst::{
  prelude::*,
  renderer::{
    SpriteSheetHandle,
    SpriteRender
  },
  core::{
    Transform,
    Parent
  },
  ecs::prelude::Entity,
};

use crate::components::{
  SimpleAnimation,
  ComplexAnimations,
  Hero,
  BoxCollider2D,
  ColliderType
};

pub fn create_hero(world: &mut World, sprite_sheet: SpriteSheetHandle) -> Entity {
  let mut transform = Transform::default();
  transform.set_xyz(500. / 2., 500. / 2., 0.);
  transform.set_scale(1.6, 1.6, 1.);

  let body_collider = BoxCollider2D::new(0., 0., 31. * 1.6, 20. * 1.6, ColliderType::HeroBody, true);
  let attack_collider = BoxCollider2D::new(10. * 1.6, 15. * 1.6, 10., 15., ColliderType::HeroAttack, false);
  let hero = Hero::new();

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
  animation.push((String::from("Dying"), 10, SimpleAnimation::new(30, 10, 0.20)));
  
  let hero = world
    .create_entity()
    .with(sprite_render)
    .with(ComplexAnimations::new(animation, String::from("Idle")))
    .with(hero)
    .with(transform)
    .build();

  world
    .create_entity()
    .with(body_collider)
    .with(Parent {
      entity: hero
    })
    .build();

  world
    .create_entity()
    .with(attack_collider)
    .with(Parent {
      entity: hero
    })
    .build();

  return hero;
}