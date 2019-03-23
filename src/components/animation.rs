use amethyst::ecs::prelude::{Component, VecStorage};
use std::collections::HashMap;
use std::cmp::Ordering;

/* Comment of @Willowblade
    // Note that for an animation, the frames must be continuous.
    // 
    // Alternative implementations in the future might use Vec to dynamically
    // allocate the frames in some way, as this allows more versatility and a
    // cleaner implementation. With Array, there were compiler errors.
    // For now it's okay like this.
*/

#[derive(Copy, Clone)]
pub struct SimpleAnimation {
    pub start_sprite_index: usize,
    pub frames: usize,
    pub current_frame: usize,
    pub time_per_frame: f32,
    pub elapsed_time: f32,
}

impl SimpleAnimation {
    pub fn new(start_sprite_index: usize, frames: usize, time_per_frame: f32) -> SimpleAnimation {
        SimpleAnimation {
            start_sprite_index: start_sprite_index,
            frames: frames,
            current_frame: 0,
            time_per_frame: time_per_frame,
            elapsed_time: 0.0,
        }
    }
}

impl Default for SimpleAnimation {
    fn default() -> SimpleAnimation {
        SimpleAnimation {
            start_sprite_index: 0,
            frames: 4,
            current_frame: 0,
            time_per_frame: 0.15,
            elapsed_time: 0.0,
        }
    }
}

pub struct ComplexAnimations {
    animations: HashMap<String, SimpleAnimation>,
    conditions: AnimationsConditions,
    active: Option<SimpleAnimation>
}

impl ComplexAnimations {
    pub fn new (animations_info: Vec<(String, i32, SimpleAnimation)>, init: String) -> ComplexAnimations  {
        let mut active: Option<SimpleAnimation> = Option::None;
        let mut animations: HashMap<String, SimpleAnimation> = HashMap::new();
        let for_conditions = animations_info.iter().map(|x| (x.0.clone(), x.1)).collect();

        for element in animations_info {
            animations.insert(element.0.clone(), element.2);
            if element.0 == init {
                active = Option::Some(element.2);
            }
        }

        if active.is_none() {
            panic!("ComplexAnimations constructor. There is no animations supplied with name {}.", init);
        }

        let conditions = AnimationsConditions::new(for_conditions, init);

        ComplexAnimations {
            animations,
            active,
            conditions
        }
    }

    pub fn get_conditions(&self) -> &AnimationsConditions {
        &self.conditions
    }

    pub fn get_animations(&self) -> &HashMap<String, SimpleAnimation> {
        &self.animations
    }

    pub fn get_animation_by_name(&self, name: &String) -> Option<&SimpleAnimation> {
        self.animations.get(name)
    }

    pub fn get_active(&self) -> Option<&SimpleAnimation> {
        self.active.as_ref()
    }

    // pub fn get_active_mut(&mut self) -> Option<&mut SimpleAnimation> {
    //     self.active.as_mut()
    // }

    pub fn update_active(&mut self, animation_name: &String) {
        self.active = self.animations.get(animation_name).cloned();
    }

    pub fn change_frame(&mut self, frame: usize) {
        match self.active {
            Some(ref mut animation) => animation.current_frame = frame,
            None => ()
        };
    }

    pub fn add_elapsed_time(&mut self, elapsed_time: f32) {
        match self.active {
            Some(ref mut animation) => animation.elapsed_time += elapsed_time,
            None => ()
        };
    }

    pub fn change_condition_activity(&mut self, is_active: bool, name: &String) {
        self.conditions.change_active(is_active, name);
    }

    pub fn conditions_sort(&mut self) -> &ComplexAnimations{
        self.conditions.sort();
        return self;
    }
}

pub struct AnimationsConditions {
    conditions: Vec<AnimationCondition>
}

impl AnimationsConditions {

    fn new (animations: Vec<(String, i32)>, init: String) -> AnimationsConditions {
        let mut conditions: Vec<AnimationCondition> = Vec::new();

        for animation in animations {
            conditions.push(
                AnimationCondition {
                    is_active: init == animation.0,
                    name: animation.0,
                    priority: animation.1
                }
            );
        }

        AnimationsConditions {
            conditions
        }
    }

    pub fn get_top(&self) -> Option<&String> {
        match self.conditions.get(0) {
            Some(condition) => Some(&condition.name),
            None => None
        }
    }

    pub fn change_active(&mut self, is_active: bool, name: &String) {
        for x in self.conditions.iter_mut() {
            if x.name == *name {
                x.is_active = is_active;
            }
        }
    }

    pub fn sort(&mut self) {
        self.conditions.sort_by(|a, b|
            if a.is_active && !b.is_active {
                Ordering::Less
            } else if b.is_active && !a.is_active {
                Ordering::Greater
            } else if a.priority < b.priority {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        );
    }
}

pub struct AnimationCondition {
    is_active: bool,
    name: String,
    priority: i32
}

impl Component for SimpleAnimation {
    type Storage = VecStorage<Self>;
}

impl Component for ComplexAnimations {
    type Storage = VecStorage<Self>;
}