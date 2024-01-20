use std::time::Duration;

use bevy::prelude::*;
use crate::species::*;
use crate::water_source::*;


const TIMER_DURATION: f32 = 10.;
const SPAWN_VAL: f32 = 5.;
const CAPACITY: f32 = 5.;
const THIRST_RATE_HZ: f32 = 1.0;
const IN_RANGE_DRINK: f32 = 40.;
const DRINK_RATE_HZ: f32 = 20.0;
const GRACE_PERIOD_FRACTION: f32 = 0.8;


#[derive(Component)]
pub struct WaterDesire {
    pub curr_val: f32,
    pub spawn_val: f32,
    pub consume_rate: f32,
    pub in_range_drink: f32,
    pub drink_rate_hz: f32,
    pub is_consuming: bool,
    pub grace_period_percent: f32,
    pub timer: Timer,
}

impl WaterDesire {
    pub fn default() -> Self {
        Self {
            curr_val: SPAWN_VAL,
            spawn_val: SPAWN_VAL,
            consume_rate: 1.,
            in_range_drink: IN_RANGE_DRINK,
            drink_rate_hz: DRINK_RATE_HZ,
            is_consuming: false,
            grace_period_percent: GRACE_PERIOD_FRACTION,
            timer: Timer::from_seconds(TIMER_DURATION, TimerMode::Once),
        }
    }
}


pub fn update_water_desire(
    mut query: Query<(Entity, &mut WaterDesire)>,
    time: Res<Time>,
){
    for (e, mut w) in query.iter_mut() {
        // tick the water desire timer on update
        w.timer.tick(time.delta());

        // decrease its water if its not currently replenishing
        // if !w.is_consuming {
            w.curr_val -= THIRST_RATE_HZ * time.delta_seconds();
        // }
    }
}

