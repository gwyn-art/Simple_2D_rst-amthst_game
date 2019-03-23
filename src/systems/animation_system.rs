use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, System, WriteStorage},
    renderer::{SpriteRender},
};

use crate::components::{SimpleAnimation, ComplexAnimations};

pub struct SimpleAnimationSystem;

impl<'s> System<'s> for SimpleAnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, SimpleAnimation>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut animations, time): Self::SystemData) {
        for (sprite_render, animation) in (&mut sprite_renders, &mut animations).join() {
            animation.elapsed_time += time.delta_seconds();
            let frame_count = (animation.elapsed_time / animation.time_per_frame) as usize % animation.frames;
            if frame_count != animation.current_frame {
                animation.current_frame = frame_count;
                sprite_render.sprite_number = frame_count + animation.start_sprite_index;
            }
        }
    }
}

pub struct ComplexAnimationsSystem;

impl<'s> System<'s> for ComplexAnimationsSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, ComplexAnimations>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut animations, time): Self::SystemData) {
        for (sprite_render, animation) in (&mut sprite_renders, &mut animations).join() {
            // Sorting conditions and getting o<name> of current animation
            let current_animation_name = animation.conditions_sort().get_conditions().get_top();
            if current_animation_name.is_none() {
                return;
            }

            let current_animation_name = current_animation_name.unwrap().clone();

            // Check is it same animation as prev or new one by start_sprite_index
            let is_new = 
                match animation.get_active() {
                    Some(active) => 
                        active.start_sprite_index != 
                        animation.get_animation_by_name(&current_animation_name).unwrap().start_sprite_index,
                    None => true
                };
            
            // If animation is new we will get it not mutated variant from animation stack 
            if is_new {
                animation.update_active(&current_animation_name);
            }

            // Updating animation frames
            let delta_time = time.delta_seconds();
            animation.add_elapsed_time(delta_time);
            let active_animation = animation.get_active().unwrap();
            let frame_count = (active_animation.elapsed_time / active_animation.time_per_frame) as usize % active_animation.frames;
            if frame_count != active_animation.current_frame {
                sprite_render.sprite_number = frame_count + active_animation.start_sprite_index;
                animation.change_frame(frame_count);
            }

        }
    }

}
