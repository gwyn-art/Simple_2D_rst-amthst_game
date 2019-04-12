use amethyst::{
  prelude::*,
  ecs::prelude::Entity,
  renderer::{
    DebugLinesParams,
    Transparent,
    SpriteSheetHandle
  }
};
use rand::{ thread_rng, Rng};

use crate::entities::{
  create_camera,
  create_hero,
  create_background,
  initialise_score,
  enemies::{
    create_minotaur
  }
};

use crate::components::{ 
  Minotaur,
  Score
};

use crate::utils::load_spritesheet;
use crate::states::{
  Game,
  UserAction,
  GameAction,
  CurrentState,
  Menu
};

pub struct GameRunning {
  level_entities: Vec<Entity>,
  enemy_sprite_sheet: Option<SpriteSheetHandle>
}

impl Default for GameRunning {

  fn default () -> Self {
    GameRunning {
      level_entities: Vec::new(),
      enemy_sprite_sheet: None
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

    self.enemy_sprite_sheet = Some(minotaur_sprite_sheet.clone());

    world.register::<Minotaur>();
    world.register::<Transparent>();

    world.add_resource(DebugLinesParams { line_width: 10. });

    self.level_entities.append(&mut create_background(&mut world, background_sprite_sheet));
    self.level_entities.append(&mut create_minotaur(&mut world, minotaur_sprite_sheet.clone(), 800. / 1.8, 600. / 1.8));
    self.level_entities.append(&mut create_hero(&mut world, hero_sprite_sheet));
    self.level_entities.push(initialise_score(&mut world));

    create_camera(&mut world);
  }

  fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    let mut world = &mut data.world;

    if let Some(UserAction::OpenMenu) = world.write_resource::<Game>().user_action.take() {
        return Trans::Pop;
    }

    let game = world.write_resource::<Game>().game_action.take();
    if let Some(GameAction::SpawnEnemy) = game {
      let sprite_sheet = self.enemy_sprite_sheet.clone().unwrap();
      let mut rand = thread_rng();
      let x: f32 = rand.gen();
      let y: f32 = rand.gen();
      self.level_entities.append(&mut create_minotaur(&mut world, sprite_sheet, 800. * x, 600. * y));
      world.write_resource::<Game>().game_action = None;
    }

    Trans::None
  }

  fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
    data
      .world
      .delete_entities(self.level_entities.as_slice())
      .expect("Failed to delete entities from level.");
  }
}