use amethyst::{
    ecs::prelude::{Component, VecStorage},
};

pub struct MenuItem {
  pub order: i32,
}

impl Component for MenuItem {
    type Storage = VecStorage<Self>;
}