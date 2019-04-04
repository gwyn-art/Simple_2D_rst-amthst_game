use amethyst::core::timing::Time;
use amethyst::core::Parent;
use amethyst::ecs::{Join, Read, ReadStorage, System, Entities, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::{
  Hero,
  BoxCollider2D,
  box_collider::ColliderType,
};

pub struct HeroAttackSystem;

impl<'s> System<'s> for HeroAttackSystem {
  type SystemData = (
    WriteStorage<'s, Hero>,
    WriteStorage<'s, BoxCollider2D>,
    ReadStorage<'s, Parent>,
    Entities<'s>,
    Read<'s, InputHandler<String, String>>,
    Read<'s, Time>,
  );

  fn run(&mut self, (mut heroes, mut box_colliders, parents, entities, input, time): Self::SystemData) {
    for (hero, entity) in (&mut heroes, &entities).join() {
      let attack = input.action_is_down("attack");
      if attack.is_none() {
        return;
      }

      let is_attacking = attack.unwrap();

      match is_attacking {
        true => hero.attack(time.absolute_real_time_seconds()),
        false => hero.stop_attack(time.absolute_real_time_seconds())
      }

      for (box_collider, parent) in (&mut box_colliders, &parents).join() {
        if parent.entity == entity && box_collider.get_tag() == ColliderType::HeroAttack {
          box_collider.is_active = hero.is_attacking();
        }

        if parent.entity != entity || box_collider.get_tag() != ColliderType::HeroBody {
          continue;
        }

        for tag in box_collider.get_colliding_with() {
          if tag == ColliderType::EnemyAttack {
            hero.take_damage(25, time.absolute_real_time_seconds());
          }
        }
      }
    }
  }
}