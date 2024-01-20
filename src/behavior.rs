use bevy::{prelude::*, ecs::system::adapter::new};
use bevy_egui::egui::special_emojis;
use crate::{debug_ui::*, species::*, food_source::*, food_desire::*, water_desire::{*, self}, water_source::*,};


const IN_RANGE_FOOD: f32 = 5.;
const MAX_VELOCITY: f32 = 50.;
const MAX_ACCELERATION: f32 = 5.;
const DRINK_RATE_HZ: f32 = 0.1;

// Instead of scaling the species perception radius according to how desperate they are for food/water,
// just do all decision making on where the species updates its target position here. Weight the options
// by water desire, food desire, and the species inherent traits (aggressiveness, solitude, engineering, )

// TODO trigger events (fight, reproduce) in this function?

// TWO OPTIONS:
// 1. Target position
//    The chosen behavior modifies the species target position. They will travel to only this position
// 2. All behaviors apply a force on the species 
//    The force vectors are weighted according to the species traits. If one behavior is very strong, 
//    then it will move towards that goal position more often than if a behavior is not as strong. 
//     



// 1. Species wander around when there are no strong influencing behaviors
// 2. Species move to a target at a stronger velocity when their food and water timers are done
// 3. Species slow down as they approach the target


pub fn behaviors(
    mut food_source_query: Query<(Entity, &mut FoodSource)>,
    mut water_source_query: Query<(Entity, &mut WaterSource)>,
    mut species_set: ParamSet<(
        Query<(&mut Transform, &mut Species, &mut FoodDesire, &mut WaterDesire)>,
        Query<(&mut Transform, &mut Species)>,
        Query<&mut Species>,
    )>,
    mut commands: Commands,
    ui_state: ResMut<UiState>,
    time: Res<Time>,
) {

    // avoid species of other race
    let mut avoid_force = Vec3::ZERO;
    let mut species_only = species_set.p2();
    let mut combinations = species_only.iter_combinations_mut::<2>();
    while let Some([mut this, mut other]) = combinations.fetch_next() {
        let vector = other.position - this.position;
        let distance = vector.length();
        if this.race != other.race {
            // avoid other races. Scale perception radius by species avoidance value
            if distance > 0. && distance < this.perception_radius * this.avoidance {
                avoid_force += -1.0 * (vector / distance);
                this.steering_forces += avoid_force;
            }
        } else { // avoid species of the same race
            if distance > 0. && distance < this.perception_radius {
                avoid_force += -1.0 * (vector / distance);
                this.steering_forces += avoid_force;
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
            if food_des.curr_val > 0. { break; }
            // if food_des.timer.percent_left() > food_des.grace_period_percent { break };

            let species_to_target = food_source.position.xy() - sp.position.xy();
            let distance = species_to_target.length();

            // make sure food entity still exists before species goes to it
            if let Some(_) = commands.get_entity(food_source_e) { 

                if distance < min_distance {
                    // let max_steer = species_to_target.normalize_or_zero() * 10.;
                    // let acc_vec2 = (species_to_target.normalize_or_zero()).lerp(max_steer, food_des.timer.percent());   
                    let acc_vec2 = species_to_target.normalize_or_zero() * food_des.curr_val.abs();

                    food_force = Vec3::new(acc_vec2.x, acc_vec2.y, 0.);
                    min_distance = distance;
                }
                // eat if within range
                if distance < food_des.in_range_eat {
                    food_des.curr_val += food_source.value;
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
            if water_des.curr_val > 0. { break; }

            let species_to_target = water_source.position.xy() - sp.position.xy();
            let distance = species_to_target.length();

            if let Some(_) = commands.get_entity(water_source_e) {

                if distance < min_distance {
                    // let max_steer = species_to_target.normalize_or_zero() * 10.;
                    // let acc_vec2 = (species_to_target.normalize_or_zero()).lerp(max_steer, water_des.timer.percent());
                    let acc_vec2 = species_to_target.normalize_or_zero() * water_des.curr_val.abs();

                    water_force = Vec3::new(acc_vec2.x, acc_vec2.y, 0.);
                    min_distance = distance;
                }
                // drink
                if distance < water_des.in_range_drink {
                    sp.velocity *= 0.95;
                    water_des.is_consuming = true;
                    let val = water_des.drink_rate_hz * time.delta_seconds();
                    water_des.curr_val += val;
                    water_source.value -= val;
                } 
                // if it is greater than 0, then start over so it has a grace period by setting value to capacity
                if water_des.curr_val > 0. {
                    water_des.curr_val = water_des.spawn_val;
                    min_distance = 1000000.0;
                }
            }
        }
        sp.steering_forces += water_force;
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
            sp.steering_forces *= ui_state.steering_strength;
        }

        let mut new_acc = (cur_acc + sp.steering_forces) * time.delta_seconds();
        let mut new_vel = cur_vel + new_acc;

        if ui_state.max_velocity != 0. {
            new_vel = new_vel.clamp_length_max(ui_state.max_velocity);
        } else {
            new_vel = new_vel.clamp_length_max(1.0);
        }
        
        let new_pos = cur_pos + new_vel;

        // now set species physics data
        sp.velocity = new_vel;
        sp.position = new_pos;

        // always render the sprite in front of everything else with Z coord
        let sprite_position = Vec3::new(sp.position.x, sp.position.y, 10.);
        tf.translation = sprite_position;
        // transform.rotation = Quat::from_rotation_y(spec.velocity.angle_between(Vec3::X));
    }



}


