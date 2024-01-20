use std::f32::consts::PI;

use bevy::ecs::system::adapter::new;
use bevy::prelude::*;
use crate::debug_ui::*;
use crate::Species;
use rand::Rng;
use crate::water_source::*;
use crate::homebase::*;

const MAX_VELOCITY_PIXEL_PER_SEC: f32 = 100.;
const WANDER_STRENGTH: f32 = 1.0;
const DESIRED_STRENGTH: f32 = 1.;
const CLOSE_DIST: f32 = 50.;
const SEPARATION_STRENGTH: f32 = 0.1;
const STEER_STRENGTH: f32 = 0.5;


pub fn update_species_physics_simple(
    mut query: Query<(Entity, &mut Transform, &mut Species)>,
    time: Res<Time>,
    ui_state: Res<UiState>,
){
    for (entity, mut transform, mut spec) in query.iter_mut() {
        // species gets acceleration from other functions
        let mut current_acceleration = spec.acceleration;
        let mut current_velocity = spec.velocity;
        let mut current_position = spec.position;

        // current_acceleration = Vec3::new(0., 5.0, 0.0);
        // info!("acc: {}, vel: {}, pos: {}", current_acceleration.length(), current_velocity.length(), current_position.length());
        
        if current_acceleration.is_nan() {
            current_acceleration = Vec3::ZERO;
        } 
        if current_velocity.is_nan() {
            current_velocity = Vec3::ZERO;
        }
        if current_position.is_nan() {
            current_position = spec.homebase;
        }

        // current_acceleration = current_acceleration.clamp_length_max(ui_state.max_acceleration);
        let mut new_velocity = Vec3::X;

        // if current acceleration is low i.e. not one large influencing behavior, just go in the 
        // same direction as the species is currently going
        //let mut new_velocity = Vec3::ZERO;
        // if current_acceleration.length() < 0.1 {
        //     // info!("Continue vel {}", current_velocity.length());
        //     new_velocity = current_velocity;
        // } else {
        //     // steer towards desired velocity (new acceleration this update)
        //     let mut steering = (current_acceleration - current_velocity);// * ui_state.steering_strength;
        //     new_velocity = current_velocity + steering;
        // }
        let mut steering = (current_acceleration - current_velocity);// * ui_state.steering_strength;
        new_velocity = current_velocity + steering;

        // set velocity to some constant magnitude based on slider
        // new_velocity = new_velocity.normalize_or_zero() * ui_state.max_velocity * time.delta_seconds();

        new_velocity *= time.delta_seconds();
        
        let new_position = current_position + new_velocity;

        // now set species physics data
        spec.velocity = new_velocity;
        spec.position = new_position;
        // reset to 0 so we don't have runaway acceleration
        //spec.acceleration = Vec3::ZERO;

        // always render the sprite in front of everything else with Z coord
        let sprite_position = Vec3::new(spec.position.x, spec.position.y, 10.);
        transform.translation = sprite_position;
        // transform.rotation = Quat::from_rotation_y(spec.velocity.angle_between(Vec3::X));
        
    }    
}



pub fn update_species_physics(
    mut query: Query<(Entity, &mut Transform, &mut Species)>,
    time: Res<Time>,
){
    let mut rng = rand::thread_rng();
    for (entity, mut transform, mut spec) in query.iter_mut() {

        // set to current acceleration at the beginning
        let mut new_acceleration = spec.acceleration;
        let mut new_velocity = spec.velocity;
        let mut new_position = spec.position;

        let desired_direction = (spec.target_pos - spec.position).normalize();

        // some random wander vector within +-30 degrees of its desired velocity
        // only wander when its velocity is above a limit, aka when it is travelling somewhere, not when hovering at a location
        //if spec.velocity.length() > 0.5 {
            // let perp_vel = Vec3::new(-desired_direction.y, desired_direction.x, 0.0);
            // let wander_vec_max = f32::cos(PI/6.) * desired_direction + f32::sin(PI/6.) * perp_vel;
            // let wander_vec_min = f32::cos(-PI/6.) * desired_direction + f32::sin(-PI/6.) * perp_vel;
            // let wander_direction_x = rng.gen_range(wander_vec_min.x..wander_vec_max.x);
            // let wander_direction_y = rng.gen_range(wander_vec_min.y..wander_vec_max.y);
            // let wander_direction = Vec3::new(wander_direction_x, wander_direction_y, 0.0);
            // let wander_velocity = wander_direction * WANDER_STRENGTH
            // let wander_force = wander_velocity - spec.velocity;

            //let wander_direction = Vec3::new(rng.gen_range(-1.0..1.0),rng.gen_range(-1.0..1.0), 0.);
            //let wander_velocity = wander_direction.normalize() * WANDER_STRENGTH;
            //let wander_force = wander_velocity - spec.velocity;

            //new_acceleration += wander_force;
        //}

        let desired_velocity = desired_direction * DESIRED_STRENGTH;

        let desired_steering_force = desired_velocity - spec.velocity;

        new_acceleration += desired_steering_force;

        //new_acceleration = new_acceleration.normalize() * MAX_ACCELERATION * time.delta_seconds();

        new_velocity += new_acceleration;
        new_position += new_velocity;

        new_velocity = new_velocity.normalize() * MAX_VELOCITY_PIXEL_PER_SEC * time.delta_seconds();

        spec.acceleration = new_acceleration;
        spec.velocity = new_velocity;
        spec.position = new_position;

        // always render the sprite in front of everything else with Z coord
        let sprite_position = Vec3::new(new_position.x, new_position.y, 10.);
        transform.translation = sprite_position;
    }
}


pub fn species_separation(
    mut set: ParamSet<(
        Query<(&mut Species, &mut WaterSource)>,
        Query<&mut Species>,
        // Query<&mut Species, &Homebase>,
    )>,
){
    let mut species_and_water_source = set.p0();
    let mut species_only = set.p1();
    // let mut species_and_homebase = set.p2();

    let mut combos = species_only.iter_combinations_mut::<2>();
    while let Some([mut this, mut other]) = combos.fetch_next() {
        let direction = this.position - other.position;
        let distance = direction.length();
        if distance < CLOSE_DIST {
            // inverse so it gets stronger the closer they are together
            this.acceleration += SEPARATION_STRENGTH * (direction / distance);
        }
    }
}

