use amethyst::{
    ecs::prelude::{Component, VecStorage},
};

pub struct Score;

impl Component for Score {
    type Storage = VecStorage<Self>;
}