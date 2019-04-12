use amethyst::ecs::{Join, System, Entities, ReadStorage, WriteStorage};
use amethyst::ui::UiText;

use crate::{
  components::{
    Score,
    enemies::RegularEnemy
  }
};

pub struct ScoreSystem;

impl<'s> System<'s> for ScoreSystem {
  type SystemData = (
    WriteStorage<'s, UiText>,
    ReadStorage<'s, Score>,
    ReadStorage<'s, RegularEnemy>,
  );

  fn run (&mut self, (mut texts, scores, enemies): Self::SystemData) {
    let mut count_dead_enemies = 0;

    for enemy in (&enemies).join() {
      if enemy.is_dying() {
        count_dead_enemies += 1;
      }
    }

    for (ui_text, _) in (&mut texts, &scores).join() {
      ui_text.text = (count_dead_enemies * 138).to_string();
    }
  }
}