extern crate amethyst;
extern crate rand;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{ALPHA, DisplayConfig, DrawFlat2D, DepthMode, ColorMask, Pipeline, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
};

mod states;
mod entities;
mod utils;
mod components;
mod systems;

use crate::states::Menu;

fn main() -> amethyst::Result<()> {
    // amethyst::start_logger(Default::default());

    let root = application_root_dir();
    let display_config_path = format!("{}{}", root, "\\resources\\display_config.ron");
    let display_config = DisplayConfig::load(&display_config_path);

    let binding_config_path = format!("{}{}", root, "\\resources\\binding_config.ron");

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(&binding_config_path)?;

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(
                    DrawFlat2D::new()
                        .with_transparency(
                            ColorMask::all(),
                            ALPHA,
                            Some(DepthMode::LessEqualWrite),
                    )
                )
                .with_pass(DrawUi::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
        RenderBundle::new(pipe, Some(display_config))
            .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::animation_system::SimpleAnimationSystem, "simple_animation_system", &[])
        .with(systems::animation_system::ComplexAnimationsSystem, "complex_animation_system", &[])
        .with(systems::hero_move_system::HeroMoveSystem, "hero_move_system", &["input_system"])
        .with(systems::menu_system::MenuSystem::default(), "menu_system", &["input_system"])
        .with(systems::hero_attack_system::HeroAttackSystem, "hero_attack_system", &["input_system"])
        .with(
            systems::hero_animation_system::HeroAnimationSystem,
            "hero_animation_system",
            &["hero_move_system", "hero_attack_system", "complex_animation_system"]
            )
        .with(
            systems::enemies_system::RegularEnemySystem,
            "regular_enemy_system",
            &["hero_move_system", "complex_animation_system"]
        );

    let mut game = Application::new("./", Menu::default(), game_data)?;

    game.run();

    Ok(())
}