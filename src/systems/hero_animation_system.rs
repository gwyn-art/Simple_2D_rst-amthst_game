use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::{
  Hero,
  ComplexAnimations
};

pub struct HeroAnimationSystem;

impl<'s> System<'s> for HeroAnimationSystem {
  type SystemData = (
    WriteStorage<'s, ComplexAnimations>,
    ReadStorage<'s, Hero>,
  );

  fn run(&mut self, (mut animations, heroes) : Self::SystemData) {
    for (animation, hero) in (&mut animations, &heroes).join() {
      animation.change_condition_activity(hero.is_walking, &String::from("Walking"));
      animation.change_condition_activity(hero.is_attacking(), &String::from("Attacking"));
      if hero.is_dying() {
        animation.stop_on_last_frame = true;
        animation.change_condition_activity(true, &String::from("Dying"));
      }
    }
  }
}