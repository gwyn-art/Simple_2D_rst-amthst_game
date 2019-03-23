use amethyst::{
  prelude::*,
  renderer::{
    Camera,
    Projection
  },
  core::Transform
};

pub fn create_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_z(1.0);

  world
    .create_entity()
    .with(
      Camera::from(Projection::orthographic(
        0.0,
        500.,
        0.0,
        500.
      )))
    .with(transform)
    .build();
}