use bevy::prelude::*;
use rand::{self, Rng};
use noise::{NoiseFn, Perlin, Fbm};
use crate::{MAP_HEIGHT, MAP_WIDTH};
use ndarray::Array;

const PERLIN_X_POINTS: usize = 50;
const PERLIN_Y_POINTS: usize = 50;
const FOOD_REPLENISH_MAX: f32 = 50.;
const FOOD_REPLENISH_MIN: f32 = 10.;
const SPAWN_SPREAD: f32 = 30.0;
const PERLIN_ELEVATION_THRESHOLD: f64 = 0.9999; // between 0.0 and 1.0
const N_FOOD_MAX: i32 = 20;
const REPLENISH_CHANCE: f32 = 0.001;

#[derive(Component)]
pub struct FoodSource {
    pub value: f32,
    pub position: Vec3,
}

#[derive(Resource, Default)]
pub struct FoodLocations {
    pub position: Vec<Vec2>,
}




// random value for hunger replenishment
impl FoodSource {
    fn new(position: Vec3) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            value: rng.gen_range(FOOD_REPLENISH_MIN..=FOOD_REPLENISH_MAX),
            position,
        }
    }
}


pub fn spawn_food_sources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut food_locations: ResMut<FoodLocations>,
){

    let mut rng = rand::thread_rng();

    // generate a 2D perlin noise surface for the entire map. Size in pixels
    let perlin = Perlin::new(2);
    // let fractal_brownian_motion: Fbm<Perlin> = Fbm::default();
    let mut noise_values: Vec<f64> = Vec::new();
    let x_coords = Array::linspace(-MAP_WIDTH/2., MAP_WIDTH/2., PERLIN_X_POINTS);
    let y_coords = Array::linspace(-MAP_HEIGHT/2., MAP_HEIGHT/2., PERLIN_Y_POINTS);
    for x in x_coords.iter() {
        for y in y_coords.iter() {
            let value = perlin.get([*x as f64, *y as f64]);
            // let value = fractal_brownian_motion.get([*x as f64, *y as f64]);
            noise_values.push(value);
            // info!("Perlin values: {}", value);
        }
    }



    // loop through grid of centers and spawn food
    let food_handle: Handle<Image> = asset_server.load("/Users/matthewbarbattini/Desktop/evolution-sim-bevy/textures/food/food_1.png");
    for x in 0..PERLIN_X_POINTS {
        for y in 0..PERLIN_Y_POINTS {
            // if the elevation of the perlin noise surface is above some threshold, spawn a cluster of food there
            let index = PERLIN_X_POINTS * x + y;
            if noise_values[index] > PERLIN_ELEVATION_THRESHOLD {

                let x_world = x_coords[x];
                let y_world = y_coords[y];

                food_locations.position.push(Vec2::new(x_world, y_world));
                
                let n_food = rng.gen_range(1..N_FOOD_MAX);
                for _ in 0..n_food {
                    let x_offset = rng.gen_range(-SPAWN_SPREAD..SPAWN_SPREAD);
                    let y_offset = rng.gen_range(-SPAWN_SPREAD..SPAWN_SPREAD);
                    
                    let x_coord = x_world + x_offset;
                    let y_coord = y_world + y_offset;

                
                    commands.spawn((SpriteBundle{
                            transform: Transform::from_xyz(x_coord, y_coord, 0.),
                            texture: food_handle.clone(), 
                            ..default()},
                        FoodSource::new(Vec3::new(x_coord, y_coord, 0.)),
                    ));
                }
            }
        }
    }
}



pub fn spawn_food_replenish(
    mut commands: Commands,
    food_locations: Res<FoodLocations>,
    asset_server: Res<AssetServer>,
){
    let mut rng = rand::thread_rng();

    let food_handle: Handle<Image> = asset_server.load("/Users/matthewbarbattini/Desktop/evolution-sim-bevy/textures/food/food_1.png");
    for pos in food_locations.position.iter() {

        let chance = rng.gen_range(0.0..1.0);
        if chance < REPLENISH_CHANCE {
            // info!("Spawn new food");

            let x_spawn = pos.x + rng.gen_range(-SPAWN_SPREAD..SPAWN_SPREAD);
            let y_spawn = pos.y + rng.gen_range(-SPAWN_SPREAD..SPAWN_SPREAD);


            commands.spawn((
                SpriteBundle{
                    transform: Transform::from_xyz(x_spawn, y_spawn, 0.),
                    texture: food_handle.clone(),
                    ..default()},
                FoodSource::new(Vec3::new(x_spawn, y_spawn, 0.)),
            ));
        }
    }
}
