use rand::Rng;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use crate::food_desire::*;
use crate::reproduce::Reproduction;
use crate::water_desire::*;
use crate::fight::*;
use bevy::math::f32::{Vec2, Vec3};

use crate::health::*;
use crate::homebase::*;




const SPAWN_SPREAD: f64 = 200.;
pub const SPECIES_TEXTURE_SCALE: f32 = 2.0;
const MIN_SPECIES_SPAWN: u32 = 10;
const MAX_SPECIES_SPAWN: u32 = 50;
const PERCEPTION_RADIUS: f32 = 100.;


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SpeciesRace {
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Component)]
pub struct Species {
    pub aggressiveness: f32,
    pub engineering: f32,
    pub tribalism: f32,
    pub avoidance: f32, // large avoidance means the species will steer away from other species of a different race
    pub reproducibility: f32,
    pub fighting_score: f32,
    pub need_to_reproduce: bool,
    pub race: SpeciesRace,
    pub target_pos: Vec3,
    pub homebase: Vec3,
    pub steering_forces: Vec3,

    pub n_neighbors: u32,
    pub reproduction_factor: f32,
    pub perception_radius: f32,
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub position: Vec3,
}


impl Species {
    pub fn new(position: Vec3, race: SpeciesRace, homebase: Vec3, aggressiveness: f32, avoidance: f32) -> Self {
        let mut rng = rand::thread_rng();
        // let x_vel = rng.gen_range(-10.0..10.0);
        // let y_vel = rng.gen_range(-10.0..10.0);
        Self {
            race,
            reproduction_factor: 0.0,
            acceleration: Vec3::ZERO,
            velocity: Vec3::ZERO,
            target_pos: homebase,
            position,
            perception_radius: 100.0,
            n_neighbors: 0,
            homebase,
            steering_forces: Vec3::ZERO,

            aggressiveness,
            engineering: 1.0,
            tribalism: 1.0,
            avoidance,
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
            target_pos: Vec3::ZERO,
            perception_radius: PERCEPTION_RADIUS,
            n_neighbors: 0,
            homebase: Vec3::ZERO,
            steering_forces: Vec3::ZERO,
            
            aggressiveness: 1.0,
            engineering: 1.0,
            tribalism: 1.0,
            avoidance: 1.0,
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

        let number_sprites = rng.gen_range(MIN_SPECIES_SPAWN..MAX_SPECIES_SPAWN);
        // let number_sprites = 2;
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
            let mut aggressiveness: f32 = 0.;
            let mut avoidance: f32 = 0.;
            match race {
                SpeciesRace::Blue => {
                    species_image = blue_species_handle.clone();
                    aggressiveness = 14.;
                    avoidance = 30.;
                },
                SpeciesRace::Red => {
                    species_image = red_species_handle.clone();
                    aggressiveness = 8.;
                    avoidance = 50.;
                },
                SpeciesRace::Yellow => {
                    species_image = yellow_species_handle.clone();
                    aggressiveness = 4.;
                    avoidance = 50.;
                },
                SpeciesRace::Green => {
                    species_image = green_species_handle.clone();
                    aggressiveness = 1.;
                    avoidance = 300.;
                },
            };
            
            // SPAWN ALL SPECIES COMPONENTS
            commands.spawn((
                SpriteBundle {
                    texture: species_image,
                    transform: Transform {
                         translation: Vec3::new(x_coord, y_coord, 1.),
                         rotation: Quat::default(),
                         scale: Vec3::splat(SPECIES_TEXTURE_SCALE),
                    },
                    ..default()},
                Species::new(
                    Vec3::new(x_coord, y_coord, 10.), 
                    race, 
                    home.position,
                    aggressiveness,
                    avoidance,
                ),
                WaterDesire::default(),
                FoodDesire::default(),
                Health::default(),
                Fight::default(),
                Reproduction::default(),
            ));
        }
    }
}
