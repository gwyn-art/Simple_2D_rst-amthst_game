use amethyst::{
  prelude::*,
  ecs::prelude::Entity,
  renderer::{
    DebugLinesParams,
    Transparent
  }
};

use crate::entities::{
  create_camera,
  create_hero,
  create_background,
  enemies::{
    create_minotaur
  }
};

use crate::components::Minotaur;
use crate::utils::load_spritesheet;
use crate::states::{
  Game,
  UserAction,
  CurrentState,
  Menu
};

pub struct GameRunning {
  level_entities: Vec<Entity>
}

impl Default for GameRunning {

  fn default () -> Self {
    GameRunning {
      level_entities: Vec::new()
    }
  }
}

impl SimpleState for GameRunning {

  fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    _data.world.write_resource::<Game>().current_state = CurrentState::Gameplay;
    let mut world = _data.world;
    let hero_sprite_sheet = load_spritesheet(&mut world, "hero");
    let minotaur_sprite_sheet = load_spritesheet(&mut world, "minotaur");
    let background_sprite_sheet = load_spritesheet(&mut world, "background");

    world.register::<Minotaur>();
    world.register::<Transparent>();

    world.add_resource(DebugLinesParams { line_width: 560. });

    self.level_entities.append(&mut create_background(&mut world, background_sprite_sheet));
    self.level_entities.append(&mut create_minotaur(&mut world, minotaur_sprite_sheet.clone(), 800. / 1.8, 600. / 1.8));
    self.level_entities.append(&mut create_minotaur(&mut world, minotaur_sprite_sheet.clone(), 800. / 2.8, 600. / 2.8));
    self.level_entities.append(&mut create_minotaur(&mut world, minotaur_sprite_sheet.clone(), 800. / 0.8, 600. / 0.8));
    self.level_entities.append(&mut create_hero(&mut world, hero_sprite_sheet));

    create_camera(&mut world);
  }

  fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    let mut game = data.world.write_resource::<Game>();

    if let Some(UserAction::OpenMenu) = game.user_action.take() {
        return Trans::Switch(Box::new(Menu::default()));
    }

    Trans::None
  }

  fn on_resume(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
    data.world.write_resource::<Game>().current_state = CurrentState::Gameplay;
  }

  fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
    data
      .world
      .delete_entities(self.level_entities.as_slice())
      .expect("Failed to delete entities from level.");
  }
}