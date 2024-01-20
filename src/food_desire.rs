use std::time::Duration;
use bevy::prelude::*;
use bevy::ecs::entity::Entities;

use crate::food_source;
use crate::food_source::*;
use crate::species::*;

// const TIMER_DURATION: f32 = 20.;
// const CAPACITY: f32 = 20.;
const SPAWN_VAL: f32 = 5.;
// const GRACE_PERIOD_FRACTION: f32 = 0.8;
const HUNGER_RATE_HZ: f32 = 1.0;
const EAT_RADIUS: f32 = 20.;

#[derive(Component)]
pub struct FoodDesire {
    // pub timer: Timer,
    pub curr_val: f32,
    // pub capacity: f32,
    pub in_range_eat: f32,
    // pub grace_period_percent: f32,
}


impl Default for FoodDesire {
    fn default() -> Self {
        Self {
            // timer: Timer::from_seconds(TIMER_DURATION, TimerMode::Once),
            curr_val: SPAWN_VAL,
            // capacity: CAPACITY,
            in_range_eat: EAT_RADIUS,
            // grace_period_percent: GRACE_PERIOD_FRACTION,
        }
    }
}

// decrases hunger according to hunger timer
// removes health if hunger is below 0
pub fn update_hunger(
    mut query: Query<(Entity, &mut FoodDesire)>,
    time: Res<Time>,
){
    for (e, mut food) in query.iter_mut() {
        // decrease timer
        // food.timer.tick(time.delta());
        
        // constantly remove hunger each update. Different from water desire which only removes if
        // it is not replenishing, because eating food should be instantaneous
        food.curr_val -= HUNGER_RATE_HZ * time.delta_seconds();
    }
}



