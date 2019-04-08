use amethyst::{
  prelude::*,
  renderer::{
    SpriteSheetHandle,
    SpriteRender,
    Transparent
  },
  core::{
    Transform
  },
  ecs::prelude::Entity,
};


pub fn create_background(world: &mut World, sprite_sheet: SpriteSheetHandle) -> Vec<Entity> {
  let mut transform = Transform::default();
  transform.set_xyz(400., 300., -10.);
  
  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet.clone(),
    sprite_number: 0,
  };

  let background = world
    .create_entity()
    .with(sprite_render)
    .with(Transparent)
    .with(transform)
    .build();

  let mut res = Vec::new();
  res.push(background);

  return res;
}