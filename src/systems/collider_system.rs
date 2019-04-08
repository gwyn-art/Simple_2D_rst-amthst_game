use std::collections::HashMap;
use amethyst::ecs::{Join, ReadStorage, Write, System, WriteStorage};
use amethyst::core::{
  Parent,
  Transform
};
use amethyst::renderer::{
  DebugLinesComponent,
  Rgba
};
use amethyst::core::nalgebra::Point3;

use crate::{
  components::{
    BoxCollider2D,
    box_collider::ColliderType
  },
  build_settings::{
    get_build_settings,
    BuildMode
  },
};

pub struct ColliderSystem;

impl<'s> System<'s> for ColliderSystem {
  type SystemData = (
    WriteStorage<'s, BoxCollider2D>,
    WriteStorage<'s, DebugLinesComponent>,
    ReadStorage<'s, Parent>,
    ReadStorage<'s, Transform>,
  );

  fn run(&mut self, (mut box_colliders, mut debug_lines, parents, transforms) : Self::SystemData) {

    // Find all colliders that needed to update
    let mut new_colliders = HashMap::<i32, BoxCollider2D>::new();
    for (debug_line, box_collider_f, parent_f) in (&mut debug_lines, &box_colliders, &parents).join() {
      debug_line.clear();
      if !box_collider_f.is_active {
        continue;
      }
      // println!("Collider F {}.", box_collider_f.id);
      let transform_f = transforms.get(parent_f.entity).unwrap();
      let mut collided_with = Vec::<ColliderType>::new();

      // Get absolute positions of center point in collider F
      let mut flipped = 1.;
        if transform_f.scale().x < 0. {
          flipped = -1.
        }
      let (x_f, y_f) = (
        transform_f.translation().x + box_collider_f.x * flipped,
        transform_f.translation().y + box_collider_f.y
      );
      let (center_x_f, center_y_f) = (x_f + box_collider_f.width * flipped / 2., y_f + box_collider_f.height / 2.);
      

      // Draw lines for debug
      if *get_build_settings().build_mode() == BuildMode::Debug { 
        draw_lines(debug_line, x_f, y_f, box_collider_f.width * flipped, box_collider_f.height);
      }

      for (box_collider_s, parent_s) in (&box_colliders, &parents).join() {
        if box_collider_s.id == box_collider_f.id ||
          !box_collider_s.is_active ||
          parent_s.entity == parent_f.entity {
          continue;
        }
        // println!("Collider S {}.", box_collider_s.id);
        let transform_s = transforms.get(parent_s.entity).unwrap();
        // Get absolute positions of center point in collider S
        let mut flipped = 1.;
        if transform_s.scale().x < 0. {
          flipped = -1.
        }
        let (x_s, y_s) = 
          (
            transform_s.translation()[0] + box_collider_s.x * flipped,
            transform_s.translation()[1] + box_collider_s.y
          );
        let (center_x_s, center_y_s) = (x_s + box_collider_s.width * flipped / 2., y_s + box_collider_s.height / 2.);

        // Calc distance between centers points of colliders F and S
        let (dist_horizontal, dist_vertical) = ((center_x_f - center_x_s).abs(), (center_y_f - center_y_s).abs());
        // println!("Distance Horizontal {}, Distance Vertical {}.", dist_horizontal, dist_vertical);

        if is_colliding(
            dist_horizontal,
            dist_vertical,
            (box_collider_f.width, box_collider_f.height),
            (box_collider_s.width, box_collider_s.height)
          ) {
          //  println!("Colliding!");
          collided_with.push(box_collider_s.get_tag().clone());
        }        
      }

      // Clone collider into new colliders
      let mut temp_collider = box_collider_f.clone();
      temp_collider.change_colliding_with(collided_with);
      new_colliders.insert(temp_collider.id, temp_collider);
    }

    for box_collider in (&mut box_colliders).join() {
      let new_collider = new_colliders.get_mut(&box_collider.id);

      if let Some(new_collider_value) = new_collider {
        // println!("Collider {} collided with {} objects.", 
        //   new_collider_value.id,
        //   new_collider_value.get_colliding_with().len()
        // );
        box_collider.change_colliding_with(new_collider_value.get_colliding_with());
      }
    }
  }

}


fn is_colliding(
  dist_horizontal: f32,
  dist_vertical: f32,
  (width_a, height_a): (f32, f32),
  (width_b, height_b): (f32, f32)) -> bool {
    dist_horizontal < (width_a / 2. + width_b / 2.) && dist_vertical < (height_a / 2. + height_b / 2.)
}

fn draw_lines(debug_lines: &mut DebugLinesComponent, x: f32, y: f32, width: f32, height: f32) {
  debug_lines.add_line(
        Point3::new(x, y, 10.),
        Point3::new(x + width, y,
        10.), Rgba::RED);
  debug_lines.add_line(
        Point3::new(x + width, y, 10.),
        Point3::new(x + width, y + height,
        10.), Rgba::RED);
  debug_lines.add_line(
        Point3::new(x, y, 10.),
        Point3::new(x, y + height,
        10.), Rgba::RED);
  debug_lines.add_line(
        Point3::new(x, y + height, 10.),
        Point3::new(x + width, y + height,
        10.), Rgba::RED);
}