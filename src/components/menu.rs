use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Entity},
};

pub struct MenuItem {
  pub is_active: bool,
  pub order: i32,
}

impl Component for MenuItem {
    type Storage = DenseVecStorage<Self>;
}