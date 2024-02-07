use bevy::prelude::*;
use crate::food_desire::FoodDesire;
use crate::health::*;
use crate::species::*;

const ATTACK_THRESHOLD: f32 = 0.9;
const SPECIES_FOOD_EAT_RESTORE: f32 = 25.0;


#[derive(Component)]
pub struct Fight {
    pub score: f32,
    pub attack_val: f32,
    pub defense_val: f32,
}

impl Fight {
    pub fn new(score: f32, attack_val: f32, defense_val: f32) -> Self {
        Self {
            score,
            attack_val,
            defense_val,
        }
    }
    
    pub fn default() -> Self {
        Self {
            score: 1.0,
            attack_val: 1.0,
            defense_val: 1.0,
        }
    }
}


pub fn fight_species(
    mut query: Query<(&mut Fight, &mut Health, &Transform, &Species, &mut FoodDesire)>,
    time: Res<Time>,
) {
    let mut count: i32 = 0;
    let mut combinations = query.iter_combinations_mut::<2>();
    while let Some([mut this, other]) = combinations.fetch_next() {
        let this_fight = this.0;
        let mut this_health = this.1;
        let this_tf = this.2;
        let this_sp = this.3;
        let mut this_hunger = this.4;

        let other_fight = other.0;
        let mut other_health = other.1;
        let other_tf = other.2;
        let other_sp = other.3;

        if (this_tf.translation - other_tf.translation).length() < 20.0
            && this_sp.race != other_sp.race {
            if this_fight.score > ATTACK_THRESHOLD {
                other_health.val -= this_fight.attack_val * time.delta_seconds();
                this_health.val -= other_fight.attack_val * time.delta_seconds();
            // fill hunger if kill other species
            if other_health.val < 0. {
                this_hunger.val += SPECIES_FOOD_EAT_RESTORE;
            }
            }
        }
        count += 1;
    }
    // info!("Pairs: {}", count);


}
