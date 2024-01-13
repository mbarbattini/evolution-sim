use std::time::Duration;

use bevy::prelude::*;
use crate::species::*;
use crate::water_source::*;


const TIMER_DURATION: u64 = 30;
const SPAWN_VALUE: f32 = 10.;
const LOSE_RATE: f32 = 1.0;


#[derive(Component)]
pub struct WaterDesire {
    pub amount: f32,
    pub capacity: f32,
    pub refill_rate: f32,
    pub consume_rate: f32,
    pub consuming: bool,
    pub grace_period_percent: f32,
    pub timer: Timer,
}

impl WaterDesire {
    pub fn default() -> Self {
        Self {
            amount: SPAWN_VALUE,
            capacity: SPAWN_VALUE,
            refill_rate: 1.,
            consume_rate: 1.,
            consuming: false,
            grace_period_percent: 0.8,
            timer: Timer::new(Duration::from_secs(TIMER_DURATION), TimerMode::Once),
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
        if !w.consuming {
            w.amount -= LOSE_RATE * time.delta_seconds();
        }
    }
}


//TODO still does not seem to be working because water amount will increase and then stop before it
//has reached max
pub fn move_to_water_source(
    mut set: ParamSet<(
        Query<(&mut Transform, &mut WaterSource, )>,
        Query<(&mut Transform, &mut Species, &mut WaterDesire)>,
    )>,
    time: Res<Time>
){
    // when the species is within range of the water source and can start drinking
    const IN_RANGE_WATER_RADIUS: f32 = 25.0;
    // the distance to scale the perception radius according to the timer
    const MAX_PERCEPTION_LOW_WATER: f32 = 2000.0;
    // how often it replenishes its water
    let drink_rate_hz: f32 = 2.0;
    // choose the closest water source if multiple are within range with this variable
    let mut min_distance: f32 = 1000000.0;

    let mut water_locations: Vec<Vec3> = vec![];
    let mut water_radii: Vec<f32> = vec![];

    for (water_transform, water_source) in set.p0().iter_mut() {
        water_locations.push(water_transform.translation);
        water_radii.push(water_source.radius);
    }

    for (species_transform, mut spec, mut water_desire) in set.p1().iter_mut() {
        // if just refilled and in grace period don't search
        if water_desire.timer.percent_left() > water_desire.grace_period_percent {
           return; 
        } else {
            for (i, water_pos) in water_locations.iter().enumerate() {
                // if the water location is within the species perception radius, update its target
                // position. Make the species perception radius proportional to its water
                // desire timer
                let distance = *water_pos - species_transform.translation;
                if water_desire.timer.elapsed_secs() != 0.0 {
                    spec.perception_radius = MAX_PERCEPTION_LOW_WATER * water_desire.timer.percent();
                } else {
                    // if timer is 0, just give the species a maximum perception radius
                    // TODO kill species? Make it slow?
                    spec.perception_radius = MAX_PERCEPTION_LOW_WATER;
                    spec.max_velocity = MAX_VELOCITY_HEALTHY * 0.1;
                }
                // update target. Choose closest water source
                if distance.length() - water_radii[i] < spec.perception_radius 
                && distance.length() < min_distance {
                    spec.target_pos = *water_pos;
                    min_distance = distance.length();
                }
                // consume water if it reached the water source
                if distance.length() - water_radii[i] < IN_RANGE_WATER_RADIUS {
                    // give it a little acceleration opposite of their velocity vector so they don't clump up
                    spec.acceleration = -1.0 * distance * 5.;
                    water_desire.consuming = true;
                    water_desire.amount += drink_rate_hz * time.delta_seconds();
                    // TODO decrease water at the water source when a species drinks
                }
                // if it reached its capacity, leave, reset timer
                if water_desire.amount > water_desire.capacity {
                    spec.target_pos = spec.homebase;
                    water_desire.timer.reset();
                    water_desire.consuming = false;
                    min_distance = 1000000.0;
                }
            }
        }
    }
}

