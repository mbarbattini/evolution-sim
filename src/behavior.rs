use bevy::prelude::*;
use crate::{debug_ui::*, species::*, food_source::*, food_desire::*, water_desire::*, water_source::*};
use std::f32::consts::PI;

const MAX_VELOCITY: f32 = 2.;
const MAX_ACCELERATION: f32 = 5.;

/*
NOTES:

- The velocity is always clamped to MAX_VELOCITY. 2.0 seems like a good value for the current setup.

- The steering forces should not be clamped when they are added to the species. They should only be clamped at the
  end, when we are adding the new acceleration to the current velocity to get the new velocity. This way if there is one dominating
  behavior, it's strength is not diminished. We add all steering forces, the final vector looks most like the stongest one, then clamp.

- ALWAYS normalize_or_zero steering forces before multiplying by its strength factor. If not, then it will look like a bug because one 
  steering force is much larger than others when it shouldn't be





*/






pub fn behaviors(
    mut food_source_query: Query<(Entity, &mut FoodSource)>,
    mut water_source_query: Query<(Entity, &mut WaterSource)>,
    mut species_set: ParamSet<(
        Query<(&mut Transform, &mut Species, &mut FoodDesire, &mut WaterDesire)>,
        Query<(&mut Transform, &mut Species)>,
        Query<(Entity, &mut Species)>,
    )>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    ui_state: ResMut<UiState>,
    time: Res<Time>,
) {

    // species pair comparisons
    let mut species_only = species_set.p2();
    let mut combinations = species_only.iter_combinations_mut::<2>();
    while let Some([mut this, other]) = combinations.fetch_next() {
        let this_e = this.0;
        let mut this_sp = this.1;

        let other_e = other.0;
        let other_sp = other.1;

        let mut avoid_force = Vec3::ZERO;
        let mut cohesion_force = Vec3::ZERO;
        let other_to_this = this_sp.position - other_sp.position;
        let distance = other_to_this.length();

        // other race
        if this_sp.race != other_sp.race {
            // avoid other races. Scale perception radius by species avoidance value
            if distance > 0. && distance < 200. {//this.perception_radius + this.avoidance {
                avoid_force += ui_state.avoid_other_strength * other_to_this.normalize_or_zero();
                this_sp.steering_forces += avoid_force;
                if ui_state.show_physics_vectors {
                    gizmos.ray(this_sp.position, avoid_force * ui_state.vector_scaling, Color::YELLOW);
                }
            }


        // same race
        } else {

            // avoid species of the same race 
            if distance > 0. && distance <  200. {//this.perception_radius {
                avoid_force += ui_state.avoid_same_strength * other_to_this.normalize_or_zero();
                this_sp.steering_forces += avoid_force;
            }
        }
    }


    // steer towards food and water
    for (
        mut tf,
        mut sp, 
        mut food_des, 
        mut water_des
    ) in species_set.p0().iter_mut() {

        // steer towards food
        let mut min_distance: f32 = 1000000.0;
        let mut food_force = Vec3::ZERO;
        for (food_source_e, food_source) in food_source_query.iter_mut() {

            // don't search if food_desire is greater than 0.
            if food_des.val > 0. { break; }
            // if food_des.timer.percent_left() > food_des.grace_period_percent { break };

            let species_to_target = food_source.position.xy() - sp.position.xy();
            let distance = species_to_target.length();

            // make sure food entity still exists before species goes to it
            if let Some(_) = commands.get_entity(food_source_e) { 
                //TODO case where there is no food, and as soon as it spawns all species steer
                //towards it. Make it so they steer towards it only if food is within their perception radius? Then the
                //species might never see the food
                //if distance < min_distance && distance < sp.perception_radius {
                if distance < min_distance {
                    let acc_vec2 = species_to_target.normalize_or_zero() * food_des.val.abs();

                    food_force = Vec3::new(acc_vec2.x, acc_vec2.y, 0.);
                    min_distance = distance;
                }
                // eat if within range
                if distance < food_des.in_range_eat {
                    food_des.val += food_source.value;
                    commands.entity(food_source_e).despawn();
                }
            } else { // if it doesn't exist, just move on to the next food source
                continue;
            }
        }
        sp.steering_forces += food_force;


        // steer to water sources
        min_distance = 10000000.0;
        let mut water_force = Vec3::ZERO;
        for (water_source_e, mut water_source) in water_source_query.iter_mut() {
            // if water_des.timer.percent_left() > water_des.grace_period_percent { break; };
            if water_des.val > 0. { break; }

            let species_to_target = water_source.position.xy() - sp.position.xy();
            let distance = species_to_target.length();

            if let Some(_) = commands.get_entity(water_source_e) {

                if distance < min_distance {
                    // let max_steer = species_to_target.normalize_or_zero() * 10.;
                    // let acc_vec2 = (species_to_target.normalize_or_zero()).lerp(max_steer, water_des.timer.percent());
                    let acc_vec2 = species_to_target.normalize_or_zero() * water_des.val.abs();

                    water_force = Vec3::new(acc_vec2.x, acc_vec2.y, 0.);
                    min_distance = distance;
                }
                // drink
                if distance < water_des.in_range_drink {
                    sp.velocity *= 0.95;
                    water_des.is_consuming = true;
                    let val = water_des.drink_rate_hz * time.delta_seconds();
                    water_des.val += val;
                    water_source.value -= val;
                } 
                // if it is greater than 0, then start over so it has a grace period by setting value to capacity
                if water_des.val > 0. {
                    water_des.val = water_des.spawn_val;
                    min_distance = 1000000.0;
                }
            }
        }
        sp.steering_forces += water_force;

        // steer towards homebase. If other behaviors are close to 0, this one will dominate, even though it has no strength factor
        // TODO add strength factor? Maybe increase strength when the species health is low, or it has no food, water, etc.
        let steering_homebase = (sp.homebase - sp.position).normalize_or_zero();
        sp.steering_forces += steering_homebase;
    }


    // update species physics
    for (mut tf, mut sp) in species_set.p1().iter_mut() {
        // update species physics
        let mut cur_acc = sp.acceleration;
        let mut cur_vel = sp.velocity;
        let mut cur_pos = sp.position;

        // info!("acc: {}, vel: {}, pos: {}", cur_acc.length(), cur_vel.length(), cur_pos.length());
        
        if cur_acc.is_nan() {
            cur_acc = Vec3::ZERO;
        } 
        if cur_vel.is_nan() {
            cur_vel = Vec3::ZERO;
        }
        if cur_pos.is_nan() {
            cur_pos = sp.homebase;
        }

        if ui_state.steering_strength != 0. {
            sp.steering_forces = sp.steering_forces.clamp_length_max(ui_state.steering_strength);
        } else {
            sp.steering_forces = sp.steering_forces.clamp_length_max(1.0);
        }

        let mut new_acc = (cur_acc + sp.steering_forces) * time.delta_seconds();

        if ui_state.max_acceleration != 0. {
            new_acc = new_acc.clamp_length_max(ui_state.max_acceleration);
        } else {
            new_acc = new_acc.clamp_length_max(1.0);
        }

        let mut new_vel = cur_vel + new_acc;

        new_vel = new_vel.clamp_length_max(MAX_VELOCITY);
        
        let new_pos = cur_pos + new_vel;

        // acceleration is always smaller than velocity, so scale even more
        if ui_state.show_physics_vectors {
            gizmos.ray(new_pos, new_vel * ui_state.vector_scaling, Color::GREEN);
            gizmos.ray(new_pos, new_acc * ui_state.vector_scaling, Color::RED);
        }
        if ui_state.show_perception_radius {
            gizmos.circle(new_pos, Vec3::Z, sp.perception_radius, Color::WHITE);
        }


        // now set species physics data
        sp.velocity = new_vel;
        sp.position = new_pos;

        // always render the sprite in front of everything else with Z coord
        let sprite_position = Vec3::new(sp.position.x, sp.position.y, 10.);
        tf.translation = sprite_position;
        let angle = f32::atan2(sp.velocity.y, sp.velocity.x);
        // subtracting PI/2 makes the sprite in line with y axis, travels facing the top
        // not subtracting makes it in line with x axis, travels facing the side
        tf.rotation = Quat::from_euler(EulerRot::XYZ, 0., 0., angle - PI/2.);
    }






}


