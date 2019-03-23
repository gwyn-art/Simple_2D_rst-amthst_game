use amethyst::{
  prelude::*
};

use amethyst::{
  renderer::{
    PngFormat,
    SpriteSheetHandle,
    TextureMetadata,
    SpriteSheetFormat,
    SpriteSheet,
    Texture
  },
  assets::{
    AssetStorage,
    Loader
  }
};

use crate::entities::{
  create_camera,
  create_hero,
  enemies::{
    create_minotaur
  }
};
use crate::components::Minotaur;
use crate::utils::load_spritesheet;

pub struct Game;

impl SimpleState for Game {

  fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    let mut world = _data.world;
    let hero_sprite_sheet = load_spritesheet(&mut world, "hero");
    let minotaur_sprite_sheet = load_spritesheet(&mut world, "minotaur");

    world.register::<Minotaur>();

    create_hero(&mut world, hero_sprite_sheet);
    create_minotaur(&mut world, minotaur_sprite_sheet.clone(), 500. / 1.8, 500. / 1.8);
    // create_minotaur(&mut world, minotaur_sprite_sheet, 500. / 3.8, 500. / 3.8);
    create_camera(&mut world);
  }
}