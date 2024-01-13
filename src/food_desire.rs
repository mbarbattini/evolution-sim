use std::time::Duration;

use bevy::prelude::*;
use crate::Species;


const TIMER_AMOUNT: Duration = Duration::from_secs(30);
const CAPACITY: f32 = 20.;
const GRACE_PERIOD_FRACTION: f32 = 0.8;
const HUNGER_RATE_HZ: f32 = 0.5;

#[derive(Component)]
pub struct FoodDesire {
    pub timer: Timer,
    pub amount: f32,
    pub capacity: f32,
}


impl Default for FoodDesire {
    fn default() -> Self {
        Self {
            timer: Timer::new(TIMER_AMOUNT, TimerMode::Once),
            amount: CAPACITY,
            capacity: CAPACITY,
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
        food.timer.tick(time.delta());
        
        // constantly remove hunger each update. Different from water desire which only removes if
        // it is not replenishing, because eating food should be instantaneous
        food.amount -= HUNGER_RATE_HZ * time.delta_seconds();
    }
}


pub fn move_to_food(


){


}

pub fn eat( 

){


}
