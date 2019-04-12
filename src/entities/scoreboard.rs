use amethyst::{
    prelude::*,
    assets::Loader,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    ecs::prelude::Entity
};

use crate::components::Score;

pub fn initialise_score(world: &mut World) -> (Entity) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    let item1_transform = UiTransform::new(
        String::from("score_board"), Anchor::TopRight,
        -50., -50., 5., 400., 50., 0,
    );

    return world
        .create_entity()
        .with(item1_transform)
        .with(Score)
        .with(UiText::new(
            font.clone(),
            String::from("0"),
            [ 0.5, 0.95, 0.5, 1.],
            30.,
        )).build();
}