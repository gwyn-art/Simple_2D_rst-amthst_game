use amethyst::core::{timing::Time};
use amethyst::ecs::{Read, System, Entity, Write};
use rand::{ thread_rng, Rng};

use crate::{
  states::{
    Game,
    CurrentState,
    GameAction
  }
};

pub struct EnemiesSpawnSystem {
  pub passed_time: f64
}

impl Default for EnemiesSpawnSystem {

  fn default() -> Self {
    EnemiesSpawnSystem {
      passed_time: 0.,
    }
  }
}

impl<'s> System<'s> for EnemiesSpawnSystem {
  type SystemData = (
    Read<'s, Time>,
    Write<'s, Game>
  );

  fn run (&mut self, (time, mut game): Self::SystemData) {
    if game.current_state != CurrentState::Gameplay || time.absolute_real_time_seconds() - self.passed_time < 5. {
      return;
    }

    println!("spawn!");
    game.game_action = Some(GameAction::SpawnEnemy);

    self.passed_time = time.absolute_real_time_seconds();
  }
}