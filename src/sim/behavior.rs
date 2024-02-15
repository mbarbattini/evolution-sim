use bevy::prelude::*;
use rand::Rng;
use crate::rect_utils::{magnify_rect, transform_to_rect};
use crate::{debug_ui::*, species::*, food_source::*, food_desire::*, water_desire::*, water_source::*, reproduce::*};
use std::f32::consts::PI;
use crate::quadtree::*;
use crate::health::*;
use crate::entity_wrapper::*;

use crate::physics::*;

const MAX_ACCELERATION: f32 = 5.;
const PERCEPTION_RADIUS: f32 = 200.;

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
        Query<(Entity, &mut Species, &mut Physics, &mut Health, &mut Transform)>,
        Query<(Entity, &mut Transform, &mut Species, &mut FoodDesire, &mut WaterDesire, &mut Physics)>,
    )>,
    mut reproduce_event_writer: EventWriter<Reproduce>,
    mut qt: ResMut<EntityQuadtree>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    ui_state: ResMut<UiState>,
    time: Res<Time>,
) {

    // --------------------------------------------------
    //               SPECIES PAIR COMPARISONS
    // --------------------------------------------------

    for (this_entity, mut sp, mut phys, mut health, tf) in species_set.p0().iter_mut() {
        let this_rect = transform_to_rect(&tf);
        let perception_rect = magnify_rect(&this_rect, Vec2::splat(PERCEPTION_RADIUS));
        if ui_state.show_perception_radius {
            gizmos.rect_2d(perception_rect.center(), 0.0, perception_rect.size(), Color::GREEN);
        }

        if let Some(node) = qt.query_rect(&perception_rect) {
            let mut nearby_count = 0;
            let mut min_distance: f32 = 100000.;
            let mut avoid_force = Vec3::ZERO;             
            for near in node
                .get_all_descendant_values()
                .filter(|v| v.entity != this_entity)
                {
                    // do stuff based on nearby species

                    // avoid other species.
                    let other_to_this = phys.pos - near.pos;
                    let distance = other_to_this.length();

                    // if temp_distance < min_distance {
                    avoid_force = other_to_this.normalize_or_zero()  * ui_state.avoid_other_strength;
                    phys.steering +=  avoid_force * time.delta_seconds();           
                        // min_distance = temp_distance;
                    // }






                    nearby_count += 1;
            }
            if ui_state.show_physics_vectors {
                gizmos.ray(phys.pos, avoid_force * ui_state.vector_scaling, Color::YELLOW);
            } 
            // info!("Nearby: {}", nearby_count);

        }
    }


    // // OLD ITER COMBINATIONS
    // // species pair comparisons
    // let mut species_only = species_set.p2();
    // let mut combinations = species_only.iter_combinations_mut::<2>();
    // while let Some([mut this, other]) = combinations.fetch_next() {
    //     let this_sp = this.1;
    //     let mut this_phys = this.2;

    //     let other_sp = other.1;
    //     let other_phys = other.2;
 
    //     let mut avoid_force = Vec3::ZERO;
    //     let other_to_this = this_phys.pos - other_phys.pos;
    //     let distance = other_to_this.length();

    //     // other race
    //     if this_sp.race != other_sp.race {
    //         // avoid other races. Scale perception radius by species avoidance value
    //         if distance > 0. && distance < 200. {//this.perception_radius + this.avoidance {
    //             avoid_force += ui_state.avoid_other_strength * other_to_this.normalize_or_zero();
    //             this_phys.steering += avoid_force;
    //             if ui_state.show_physics_vectors {
    //                 gizmos.ray(this_phys.pos, avoid_force * ui_state.vector_scaling, Color::YELLOW);
    //             }
    //         }
    //     // same race
    //     } else {

    //         // avoid species of the same race 
    //         if distance > 0. && distance <  200. {//this.perception_radius {
    //             avoid_force += ui_state.avoid_same_strength * other_to_this.normalize_or_zero();
    //             this_phys.steering += avoid_force;
    //         }
    //     }
    // }


    // steer towards food and water
    for (
        e,
        mut tf,
        mut sp, 
        mut food_des, 
        mut water_des,
        mut phys,
    ) in species_set.p1().iter_mut() {
        // steer towards food
        let mut min_distance: f32 = 1000000.0;
        let mut food_force = Vec3::ZERO;
        for (food_source_e, food_source) in food_source_query.iter_mut() {

            // don't search if food_desire is greater than 0.
            if food_des.val > 0. { break; }
            // if food_des.timer.percent_left() > food_des.grace_period_percent { break };

            let species_to_target = food_source.position.xy() - phys.pos.xy();
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
        //sp.steering_forces += food_force;
        // phys.steering += food_force * time.delta_seconds();


        // steer to water sources
        min_distance = 10000000.0;
        let mut water_force = Vec3::ZERO;
        for (water_source_e, mut water_source) in water_source_query.iter_mut() {
            // if water_des.timer.percent_left() > water_des.grace_period_percent { break; };
            if water_des.val > 0. { break; }

            let species_to_target = water_source.position.xy() - phys.pos.xy();
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
                    phys.vel *= 0.95;
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
        // phys.steering += water_force * time.delta_seconds();

        // steer towards homebase. If other behaviors are close to 0, this one will dominate, even though it has no strength factor
        // TODO add strength factor? Maybe increase strength when the species health is low, or it has no food, water, etc.
        let steering_homebase = (sp.homebase - phys.pos).normalize_or_zero();
        // phys.steering += steering_homebase * time.delta_seconds();



    }

    //let mut query_species_reproduction = species_set.p3();
    //let mut reproduction_combinations = query_species_reproduction.iter_combinations_mut::<2>();
    //while let Some([(this_e, this_sp, mut this_phys, this_rep), (other_e, other_sp, other_phys, other_rep)]) = reproduction_combinations.fetch_next() {

        //// TODO steer towards closest one? Would then be O(n^2), not big of a deal if # species
        //// in mating pool are small

        //if this_rep.in_mating_pool {
            //// steer towards other members in the mating pool
            //let this_to_other = this_phys.pos - other_phys.pos;
            //let reproduction_force = this_to_other.normalize_or_zero() * (1. / this_to_other.length());
            //this_phys.steering += reproduction_force * time.delta_seconds();
            //// if close enough, send reproduction event
            //if this_to_other.length() < 20.0 {
                //info!("Fucked and had a baby!");
                //reproduce_event_writer.send(Reproduce(this_e, other_e));
            //}
        //}


    //}





// END OF GIANT FUNCTION
}


