use bevy::prelude::*;
use crate::WaterDesire;
use crate::Species;
use crate::food_desire;
use crate::food_desire::FoodDesire;

//TODO should health be float or int? If int, need to check on multiples of seconds
// If float, can damage every frame and multiply by time.delta_seconds()

const HEALTH_SPAWN: f32 = 10.;
const LOW_WATER_DAMAGE_RATE: f32 = 0.1;
const LOW_HUNGER_DAMAGE_RATE: f32 = 0.1;


#[derive(Component)]
pub struct Health {
    pub val: f32,
    pub full: f32,
}




impl Health {
    pub fn default() -> Self {
        Self {
            val: HEALTH_SPAWN,
            full: HEALTH_SPAWN,
        }
    }

    pub fn new(val: f32, full: f32) -> Self {
        Self {
            val,
            full,
        }
    }
}




pub fn kill_zero_health(
    mut query: Query<(Entity, &Health)>,
    mut commands: Commands,
){
    for (e, h) in query.iter_mut() {
        if h.val < 0. {
            commands.entity(e).despawn();
        }
    }
}





// TODO make this a function for all cases where it should lose health? Food, water, fighting? Or
// separate?
// Different damage rates for each damage type?
pub fn damage_low_stats(
    mut query: Query<(&mut Species, &mut Health, &mut WaterDesire, &mut FoodDesire)>,
    time: Res<Time>,
) {
    for (spec, mut health, water_desire, food_desire) in query.iter_mut() {
        
        if water_desire.curr_val < 0.0 {
            health.val -= LOW_WATER_DAMAGE_RATE * time.delta_seconds();
        }
        if food_desire.curr_val < 0.0 {
            health.val -= LOW_HUNGER_DAMAGE_RATE * time.delta_seconds();
        }
    }
}
