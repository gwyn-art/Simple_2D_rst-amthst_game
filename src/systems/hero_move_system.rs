use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::{
  Hero
};

pub struct HeroMoveSystem;

impl<'s> System<'s> for HeroMoveSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    WriteStorage<'s, Hero>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transform, mut heroes, input): Self::SystemData) {
    for (hero, transform) in (&mut heroes, &mut transform).join() {
      if hero.is_attacking() || hero.is_dead() {
        return;
      }

      let movement_vertical = input.axis_value("vertical");
      let movement_horizontal = input.axis_value("horizontal");
      let mut is_walking = false; 

      if let Some(mv_amount) = movement_vertical {
        let scaled_amount = 1.2 * mv_amount as f32;
        let hero_y = transform.translation().y;

        if scaled_amount != 0. && !is_walking {
          is_walking = true;
        }

        transform.set_y(hero_y + scaled_amount);
      }

      if let Some(mv_amount) = movement_horizontal {
        let scaled_amount = 1.2 * mv_amount as f32;
        let hero_x = transform.translation().x;
        let scale = transform.scale_mut();

        if scaled_amount != 0. && !is_walking {
          is_walking = true;

          if scaled_amount > 0. {
            scale.x = scale.x.abs();
          } else if scaled_amount < 0. {
            scale.x = scale.x.abs() * -1.;
          }
        }

        transform.set_x(hero_x + scaled_amount);
      }

      hero.is_walking = is_walking;
    }
  }
}