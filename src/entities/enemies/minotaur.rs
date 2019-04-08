use amethyst::{
  prelude::*,
  renderer::{
    SpriteSheetHandle,
    SpriteRender,
    Transparent,
    DebugLinesComponent
  },
  core::{
    Transform,
    Parent,
  },
  ecs::prelude::Entity,
};

use crate::components::{
  SimpleAnimation,
  ComplexAnimations,
  Minotaur,
  enemies::RegularEnemy,
  BoxCollider2D,
  ColliderType
};


pub fn create_minotaur(world: &mut World, sprite_sheet: SpriteSheetHandle, x: f32, y: f32) -> Entity {
  let mut transform = Transform::default();
  let minotaur = Minotaur::new();
  let enemy = RegularEnemy::new(1.8, 50);
  transform.set_xyz(x, y, 0.);
  transform.set_scale(2., 2., 0.);
  
  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet.clone(),
    sprite_number: 0,
  };
  
  let mut animation: Vec<(String, i32, SimpleAnimation)> = Vec::new();
  animation.push((String::from("Idle"), 100, SimpleAnimation::new(0, 10, 0.2)));
  animation.push((String::from("Walking"), 80, SimpleAnimation::new(10, 10, 0.02)));
  animation.push((String::from("Attacking"), 60, SimpleAnimation::new(20, 20, 0.1)));
  animation.push((String::from("Dying"), 10, SimpleAnimation::new(40, 10, 0.18)));

  let body_collider = BoxCollider2D::new(-10. * 2., -19. * 2., 31. * 2., 39. * 2., ColliderType::EnemyBody, true);
  let attack_collider = BoxCollider2D::new(-15. * 2., -19. * 2., 10. * 2., 39. * 2., ColliderType::EnemyAttack, false);

  let minotaur = world
    .create_entity()
    .with(sprite_render)
    .with(ComplexAnimations::new(animation, String::from("Idle")))
    .with(minotaur)
    .with(enemy)
    .with(Transparent)
    .with(transform)
    .build();

  world
    .create_entity()
    .with(body_collider)
    .with(DebugLinesComponent::new())
    .with(Parent {
      entity: minotaur
    })
    .build();

  world
    .create_entity()
    .with(attack_collider)
    .with(DebugLinesComponent::new())
    .with(Parent {
      entity: minotaur
    })
    .build();

  return minotaur;
}