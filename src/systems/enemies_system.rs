use amethyst::core::{Transform, timing::Time};
use amethyst::ecs::{Join, Read, System, ReadStorage, WriteStorage};
use rand::{ thread_rng, Rng};

use crate::components::{
  enemies::{
    RegularEnemy,
    Minotaur,
  },
  ComplexAnimations,
  Hero
};

pub struct RegularEnemySystem;

impl<'s> System<'s> for RegularEnemySystem {
  type SystemData = (
    WriteStorage<'s, RegularEnemy>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, ComplexAnimations>,
    ReadStorage<'s, Hero>,
    Read<'s, Time>,
  );

  fn run(&mut self, (mut enemies, mut transform, mut animations, heroes, time): Self::SystemData) {

    for (enemy, enemy_transform) in (&mut enemies, &transform).join() {
      let mut distance_to_hero = 0.;
      // Find Hero and write directions to him
      for (_, hero_transform) in (&heroes, &transform).join() {
        let (enemy_x, enemy_y) = (enemy_transform.translation().x, enemy_transform.translation().y);
        let (hero_x, hero_y) = (hero_transform.translation().x, hero_transform.translation().y);

        let dif_x = enemy_x - hero_x;
        let dif_y = enemy_y - hero_y;
        distance_to_hero = dif_x.abs().max(dif_y.abs());

        if dif_x.abs() > 20. {
          if dif_x > 0. {
            enemy.dir_x = -1.;
          } else {
            enemy.dir_x = 1.;
          }
        } else {
          enemy.dir_x = 0.;
        }

        if dif_y.abs() > 20. {
          if dif_y > 0. {
            enemy.dir_y = -1.;
          } else {
            enemy.dir_y = 1.;
          }
        } else {
          enemy.dir_y = 0.;
        }
      }

      // Decide what to do
      let mut rand = thread_rng();
      let action = rand.gen_range(0, 10);
      let time_now = time.absolute_real_time_seconds();
      
      if time_now - enemy.get_past_action_time() > 2. {
        if action > 6 {
          enemy.attack(time_now);
        } else if action > 3 {
          enemy.move_to_hero(time_now);
        } else {
          enemy.stay_idle(time_now);
        }
      }

      if distance_to_hero < 30. { 
        enemy.stop_moving();
      }
    }

    // Move enemy
    for (enemy, transform_enemy) in (&mut enemies, &mut transform).join() {
      if !enemy.is_moving() { continue; }
      let (enemy_x, enemy_y) = (transform_enemy.translation().x, transform_enemy.translation().y);
      let scale = transform_enemy.scale_mut();
      
      if enemy.dir_x != 0. {
        if enemy.dir_x > 0. {
          scale.x = -1.;
        } else {
          scale.x = 1.;
        }
      }

      transform_enemy.set_x(enemy_x + enemy.dir_x * enemy.speed);
      transform_enemy.set_y(enemy_y + enemy.dir_y * enemy.speed);
      // transform_enemy.set_y(enemy_x + enemy.get_speed_y());
    }

    // Animate enemy
    for (animation, enemy) in (&mut animations, &enemies).join() {
      println!("Is attacking: {}", enemy.is_attacking());
      let is_moving = enemy.is_moving() ; // && (enemy.dir_x.abs() + enemy.dir_y.abs()) > 0.
      animation.change_condition_activity(
        is_moving,
        &String::from("Walking")
      );
      animation.change_condition_activity(
        enemy.is_attacking(),
        &String::from("Attacking")
      );
    }
  }
}