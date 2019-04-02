use amethyst::{
  prelude::*,
  ecs::prelude::Entity
};

use crate::entities::{
  create_camera,
  initialise_menu
};

use crate::states::{
  Game,
  UserAction,
  CurrentState,
  GameRunning
};

pub struct Menu {
  state_entities: Vec<Entity>
}

impl Default for Menu {

  fn default() -> Self {
    Menu {
      state_entities: Vec::new()
    }
  }
}

impl SimpleState for Menu {

  fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    _data.world.write_resource::<Game>().current_state = CurrentState::MainMenu;
    let mut world = _data.world;

    let buttons = initialise_menu(&mut world, true);
    self.state_entities.push(buttons.0);
    self.state_entities.push(buttons.1);
    
    create_camera(&mut world);
  }

  fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    let mut game = data.world.write_resource::<Game>();

    match game.user_action.take() {
        Some(UserAction::StartGame) => Trans::Switch(Box::new(GameRunning::default())),
        Some(UserAction::Quit) => {
            Trans::Quit
        },
        _ => Trans::None,
    }
  }

  fn on_resume(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
    // mark that the current state is a main menu state.
    data.world.write_resource::<Game>().current_state = CurrentState::MainMenu;
  }

  fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
    data
      .world
      .delete_entities(self.state_entities.as_slice())
      .expect("Failed to delete entities from level.");
  }
}