use std::collections::HashMap;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use amethyst::core::{
  Parent,
  Transform
};

use crate::components::{
  BoxCollider2D,
  box_collider::ColliderType
};

pub struct ColliderSystem;

impl<'s> System<'s> for ColliderSystem {
  type SystemData = (
    WriteStorage<'s, BoxCollider2D>,
    ReadStorage<'s, Parent>,
    ReadStorage<'s, Transform>
  );

  fn run(&mut self, (mut box_colliders, parents, transforms) : Self::SystemData) {

    // Find all colliders that needed to update
    let mut new_colliders = HashMap::<i32, BoxCollider2D>::new();
    for (box_collider_f, parent_f) in (&box_colliders, &parents).join() {
      if !box_collider_f.is_active {
        continue;
      }
      // println!("Collider F {}.", box_collider_f.id);
      let transform_f = transforms.get(parent_f.entity).unwrap();
      let mut collided_with = Vec::<ColliderType>::new();

      // Get absolute positions of center point in collider F
      let (x_f, y_f) = (transform_f.translation()[0] + box_collider_f.x, transform_f.translation()[1] + box_collider_f.y);
      let (center_x_f, center_y_f) = (x_f + box_collider_f.width / 2., y_f + box_collider_f.height / 2.);

      for (box_collider_s, parent_s) in (&box_colliders, &parents).join() {
        if box_collider_s.id == box_collider_f.id ||
          !box_collider_s.is_active ||
          parent_s.entity == parent_f.entity {
          continue;
        }
        // println!("Collider S {}.", box_collider_s.id);
        let transform_s = transforms.get(parent_s.entity).unwrap();
        // Get absolute positions of center point in collider S
        let (x_s, y_s) = (transform_s.translation()[0] + box_collider_s.x, transform_s.translation()[1] + box_collider_s.y);
        let (center_x_s, center_y_s) = (x_s + box_collider_s.width / 2., y_s + box_collider_s.height / 2.);

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