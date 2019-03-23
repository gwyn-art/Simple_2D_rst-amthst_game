use amethyst::core::{Transform, timing::Time};
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::{
  Hero
};

pub struct HeroAttackSystem;

impl<'s> System<'s> for HeroAttackSystem {
  type SystemData = (
    WriteStorage<'s, Hero>,
    Read<'s, InputHandler<String, String>>,
    Read<'s, Time>,
  );

  fn run(&mut self, (mut heroes, input, time): Self::SystemData) {
    for (hero) in (&mut heroes).join() {
      let attack = input.action_is_down("attack");
      if attack.is_none() {
        return;
      }

      let is_attacking = attack.unwrap();

      match is_attacking {
        true => hero.attack(time.absolute_real_time_seconds()),
        false => hero.stop_attack(time.absolute_real_time_seconds())
      }
    }
  }
}