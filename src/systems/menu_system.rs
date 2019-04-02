use amethyst::ecs::{Join, Read, Write, System, WriteStorage, Entities};
use amethyst::input::InputHandler;
use amethyst::ui::UiText;
use amethyst::core::Time;
use amethyst::Trans;

use crate:: {
  components::{
    MenuItem
  },
  entities::menu::MENU_COUNT,
  states::{
    Game,
    CurrentState,
    UserAction
  }
};

pub struct MenuSystem {
  active_menu: i32,
  min_passed_time: f64,
  last_time: f64,
}

impl Default for MenuSystem {
  fn default() -> MenuSystem { 
    MenuSystem {
      active_menu: 0,
      min_passed_time: 0.1,
      last_time: 0.,
    }
  }
}

impl<'s> System<'s> for MenuSystem {
  type SystemData = (
    Entities<'s>,
    WriteStorage<'s, MenuItem>,
    WriteStorage<'s, UiText>,
    Read<'s, InputHandler<String, String>>,
    Write<'s, Game>,
    Read<'s, Time>,
  );

  fn run(&mut self, (entities, mut menu_items, mut ui_texts, input, mut game, time): Self::SystemData) {
    let active_color: [f32; 4] = [ 0.5, 0.95, 0.5, 1.];
    let default_color: [f32; 4] = [1., 1., 1., 1.];

    if time.absolute_real_time_seconds() - self.last_time < self.min_passed_time {
      return;
    }

    self.last_time = time.absolute_real_time_seconds();

    let step = input.axis_value("vertical");
    let select = input.action_is_down("select");
    let back_to_menu = input.action_is_down("back_to_menu");

    // Menu changes
    if let Some(step_value) = step {
      self.active_menu = (self.active_menu + step_value as i32).abs() % MENU_COUNT;
    }
    
    // Menu navigation part
    if game.current_state == CurrentState::MainMenu {
      for (e, menu_item, ui_text) in (&*entities, &mut menu_items, &mut ui_texts).join() {
        if menu_item.order == self.active_menu {
          ui_text.color = active_color;
        } else {
          ui_text.color = default_color;
        }

        if !select.is_none() && select.unwrap() && self.active_menu == 1 {
          game.user_action = Some(UserAction::Quit);
        } else if !select.is_none() && select.unwrap() && self.active_menu == 0 {
          game.user_action = Some(UserAction::StartGame);
        }
      }
    }

    println!("Game current state is gamepaly: {}", game.current_state == CurrentState::Gameplay);
    // Back to Menu from a game
    if game.current_state == CurrentState::Gameplay && back_to_menu.unwrap() {
      game.user_action = Some(UserAction::OpenMenu);
    }
  }
}