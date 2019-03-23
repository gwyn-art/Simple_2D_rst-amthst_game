use amethyst::{
  prelude::*,
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

pub fn load_spritesheet (world: &mut World, name: &str) -> SpriteSheetHandle {

  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      format!("sprites/{}.png", name),
      PngFormat,
      TextureMetadata::srgb_scale(),
      (),
      &texture_storage,
    )
  };

  let loader = world.read_resource::<Loader>();
  let spritesheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    format!("sprites/{}.ron", name),
    SpriteSheetFormat,
    texture_handle,
    (),
    &spritesheet_store,
  )
}