use bevy::asset::io::AssetWriter;
use rand::Rng;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use crate::{MAP_WIDTH, MAP_HEIGHT};
use crate::{water_desire::WaterDesire};
use bevy::math::f32::{Vec2, Vec3};

use crate::water_source::*;
use crate::health::*;
use crate::homebase::*;


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SpeciesRace {
    Red,
    Blue,
    Green,
    Yellow,
}


const SPAWN_SPREAD: f64 = 200.;
pub const MAX_VELOCITY_HEALTHY: f32 = 10.;
const MAX_ACCELERATION: f32 = 0.5;
const WANDER_STRENGTH: f32 = 2.;
const DESIRED_STRENGTH: f32 = 3.;


#[derive(Component)]
pub struct Species {
    pub aggressiveness: f32,
    pub engineering: f32,
    pub tribalism: f32,
    pub reproducibility: f32,
    pub fighting_score: f32,
    pub need_to_reproduce: bool,
    pub race: SpeciesRace,
    pub target_pos: Vec3,
    pub homebase: Vec3,

    pub n_neighbors: u32,
    pub reproduction_factor: f32,
    pub perception_radius: f32,
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub position: Vec3,
    pub max_velocity: f32,
}


impl Species {
    pub fn new(position: Vec3, race: SpeciesRace, homebase: Vec3) -> Self {
        let mut rng = rand::thread_rng();
        let x_vel = rng.gen_range(-1.0..1.0);
        let y_vel = rng.gen_range(-1.0..1.0);
        Self {
            race,
            reproduction_factor: 0.0,
            acceleration: Vec3::ZERO,
            velocity: Vec3::new(x_vel, y_vel, 0.0),
            max_velocity: MAX_VELOCITY_HEALTHY,
            target_pos: homebase,
            position,
            perception_radius: 25.0,
            n_neighbors: 0,
            homebase,

            aggressiveness: 1.0,
            engineering: 1.0,
            tribalism: 1.0,
            reproducibility: 1.0,
            fighting_score: 1.0,
            need_to_reproduce: false,
        }
    }

}


impl Default for Species {
    fn default() -> Self {
        Self {
            race: SpeciesRace::Red,
            reproduction_factor: 0.0,
            acceleration: Vec3::ZERO,
            velocity: Vec3::new(0., 0., 0.),
            position: Vec3::ZERO,
            max_velocity: MAX_VELOCITY_HEALTHY,
            target_pos: Vec3::ZERO,
            perception_radius: 25.0,
            n_neighbors: 0,
            homebase: Vec3::ZERO,
            
            aggressiveness: 1.0,
            engineering: 1.0,
            tribalism: 1.0,
            reproducibility: 1.0,
            fighting_score: 1.0,
            need_to_reproduce: false,
        }
    }

}

pub fn initial_species_group_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut home_query: Query<&mut Homebase>,
){
    /* Generate a cluster of sprites as some center coordinate with random offsets from the center
     * with Perlin noise. 
     */
    let blue_species_handle: Handle<Image> = asset_server.load("textures/species/blue_species.png");
    let red_species_handle: Handle<Image>  = asset_server.load("textures/species/red_species.png");
    let yellow_species_handle: Handle<Image> = asset_server.load("textures/species/yellow_species.png");
    let green_species_handle: Handle<Image> = asset_server.load("textures/species/green_species.png");

    let mut rng = rand::thread_rng();

    // generate a 2D perlin noise map
    let perlin_x: u32 = 50;
    let perlin_y: u32 = 50;
    let perlin = Perlin::new(2);
    let scale: f64 = 0.7;
    let mut noise_values: Vec<f64> = Vec::new();
    for x in 0..perlin_x {
        for y in 0..perlin_y {
            let value = perlin.get([x as f64 * scale, y as f64 * scale]);
            noise_values.push(value);
        }
    }

    for home in home_query.iter_mut() {

        let race = home.species_race;

        let number_sprites = rng.gen_range(30..50);
        // generate the number of sprites with offsets chosen randomly from the Perlin noise map
        for _ in 0..number_sprites {
            
            // choose a random point for the x offset
            let xx_i: u32 = rng.gen_range(0..perlin_x);
            let xy_i: u32 = rng.gen_range(0..perlin_y);
            // choose a random point for the y offset
            let yx_i: u32 = rng.gen_range(0..perlin_x);
            let yy_i: u32 = rng.gen_range(0..perlin_y);

            let x_offset = SPAWN_SPREAD * noise_values[(xx_i * perlin_x + xy_i) as usize];
            let y_offset = SPAWN_SPREAD * noise_values[(yx_i * perlin_x + yy_i) as usize];

            let x_coord: f32 = home.position.x + x_offset as f32;
            let y_coord: f32 = home.position.y + y_offset as f32;
            
            let mut species_image: Handle<Image> = blue_species_handle.clone();
            match race {
                SpeciesRace::Blue => {species_image = blue_species_handle.clone();},
                SpeciesRace::Red => {species_image = red_species_handle.clone();},
                SpeciesRace::Yellow => {species_image = yellow_species_handle.clone();},
                SpeciesRace::Green => {species_image = green_species_handle.clone();},
            };
            
            // SPAWN ALL SPECIES COMPONENTS
            commands.spawn((
                SpriteBundle {
                    texture: species_image,
                    transform: Transform {
                         translation: Vec3::new(x_coord, y_coord, 0.),
                         rotation: Quat::default(),
                         scale: Vec3::splat(2.),
                    },
                    ..default()},
                Species::new(Vec3::new(x_coord, y_coord, 0.), race, home.position),
                WaterDesire::default(),
                Health::default(),
            ));
        }
    }

}

//pub fn update_species_velocity_only(
    //mut query: Query<(Entity, &mut Transform, &mut Species)>,
    //time: Res<Time>,
//){
    //const MAX_VELOCITY: f32 = 200.0;
    //const WANDER_RANGE: f32 = 0.707;
    //const WANDER_STRENGTH: f32 = 1500.;
    //const DESIRED_STRENGTH: f32 = 1.;

    //let mut rng = rand::thread_rng();
    //for (entity, mut transform, mut spec) in query.iter_mut() {
        //let desired_direction = (spec.target_pos - spec.position).normalize();

        //let wander_x = rng.gen_range(desired_direction.x - WANDER_RANGE..desired_direction.x + WANDER_RANGE);
        //let wander_y = rng.gen_range(desired_direction.y - WANDER_RANGE..desired_direction.y + WANDER_RANGE);
        //let wander_direction = Vec3::new(wander_x, wander_y, 0.0).normalize();
        ////info!("desire: {}, wander: {}", desired_direction * DESIRED_STRENGTH, wander_direction * WANDER_STRENGTH );
        //let new_vel = (desired_direction + wander_direction * WANDER_STRENGTH).clamp_length_max(MAX_VELOCITY);
        //info!("{}", new_vel);
        //spec.position += new_vel * time.delta_seconds();
        //transform.translation = spec.position;
    //}

//}

// TODO make wandering smoother
pub fn update_species_with_acceleration(
    mut query: Query<(Entity, &mut Transform, &mut Species)>,
    time: Res<Time>,
){


    let mut rng = rand::thread_rng();
    for (entity, mut transform, mut spec) in query.iter_mut() {

        let new_velocity: Vec3;

        // some random wander vector
        let wander_x = rng.gen_range(-1.0..1.0);
        let wander_y = rng.gen_range(-1.0..1.0);

        let desired_direction = (spec.target_pos - spec.position).normalize();
        let wander_direction = Vec3::new(wander_x, wander_y, 0.0).normalize();

        let desired_velocity = desired_direction * DESIRED_STRENGTH;
        let wander_velocity = wander_direction * WANDER_STRENGTH;

        let desired_steering_force = desired_velocity - spec.velocity;
        let wander_force = wander_velocity - spec.velocity;

        let new_acceleration = desired_steering_force + wander_force; 

        let current_max_vel: f32 = spec.max_velocity;
        spec.velocity += (new_acceleration * time.delta_seconds()).clamp_length_max(current_max_vel);
        new_velocity = spec.velocity;
            
        spec.position += new_velocity;

        transform.translation = spec.position;

        if entity.index() == 14 {
             //info!("{}", spec.water_amount);
        }
    }
}
