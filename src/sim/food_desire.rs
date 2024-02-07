use bevy::prelude::*;

pub const MAX_HUNGER: f32 = 5.;
const HUNGER_RATE_HZ: f32 = 1.0;
const EAT_RADIUS: f32 = 20.;

#[derive(Component)]
pub struct FoodDesire {
    pub val: f32,
    pub in_range_eat: f32,
}


impl Default for FoodDesire {
    fn default() -> Self {
        Self {
            val: MAX_HUNGER,
            in_range_eat: EAT_RADIUS,
        }
    }
}



pub fn update_hunger(
    mut query: Query<(Entity, &mut FoodDesire)>,
    time: Res<Time>,
){
    for (e, mut food) in query.iter_mut() {
        
        // constantly remove hunger each update. Different from water desire which only removes if
        // it is not replenishing, because eating food should be instantaneous
        food.val -= HUNGER_RATE_HZ * time.delta_seconds();
    }
}



