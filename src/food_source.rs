use bevy::prelude::*;
use rand::{self, Rng};
use noise::{NoiseFn, Perlin, Fbm};
use crate::{MAP_HEIGHT, MAP_WIDTH};
use ndarray::Array;

const PERLIN_X_POINTS: usize = 50;
const PERLIN_Y_POINTS: usize = 50;
const FOOD_REPLENISH_MAX: f32 = 50.;
const FOOD_REPLENISH_MIN: f32 = 10.;
const SPAWN_SPREAD: f64 = 500.0;
const PERLIN_ELEVATION_THRESHOLD: f64 = 0.8; // between 0.0 and 1.0

#[derive(Component)]
pub struct FoodSource {
    pub value: f32,
    pub position: Vec3,
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
){

let mut rng = rand::thread_rng();

// generate a grid of center spawn coordinates so they are evenly spread across the map
// let mut xcenters = vec![];
// let mut ycenters = vec![];

// for i in 1..(N_SPAWN_X) {
//     let spacing = 2.0 * MAP_WIDTH / N_SPAWN_X as f32;
//     xcenters.push(-MAP_WIDTH + i as f32 * spacing);
// }
// for i in 1..(N_SPAWN_Y) {
//     let spacing = 2.0 * MAP_HEIGHT / N_SPAWN_Y as f32;
//     ycenters.push(-MAP_HEIGHT + i as f32 * spacing);
// }

// generate a 2D perlin noise surface for the entire map. Size in pixels
let perlin = Perlin::new(2);
// let fractal_brownian_motion: Fbm<Perlin> = Fbm::default();
let scale: f64 = 0.7;
let mut noise_values: Vec<f64> = Vec::new();
let x_coords = Array::linspace(-MAP_WIDTH/2., MAP_WIDTH/2., PERLIN_X_POINTS);
let y_coords = Array::linspace(-MAP_HEIGHT/2., MAP_HEIGHT/2., PERLIN_Y_POINTS);
for x in x_coords.iter() {
    for y in y_coords.iter() {
        let value = perlin.get([*x as f64, *y as f64]);
        // let value = fractal_brownian_motion.get([*x as f64, *y as f64]);
        noise_values.push(value);
    }
}


// loop through grid of centers and spawn food
let food_handle: Handle<Image> = asset_server.load("textures/food/food_1.png");
for x in 0..PERLIN_X_POINTS {
    for y in 0..PERLIN_Y_POINTS {
        // if the elevation of the perlin noise surface is above some threshold, spawn a cluster of food there
        let index = PERLIN_X_POINTS * x + y;
        if noise_values[index] > PERLIN_ELEVATION_THRESHOLD {

            let x_world = x_coords[x];
            let y_world = y_coords[y];
            commands.spawn((
                SpriteBundle{
                    transform: Transform::from_xyz(x_world, y_world, 0.),
                    texture: food_handle.clone(), 
                    ..default()},
                FoodSource::new(Vec3::new(x_world, y_world, 0.)),
            ));
            }
            

            //TODO food clusters instead of a single food?
            // let n_food = rng.gen_range(0..N_FOOD_MAX);
            // for _ in 0..n_food {
            //     // choose a random point for the x offset in the perlin noise map
            //     let x_offset = rng.gen_range(-10..10 as u32) as usize;
            //     // let xy_i: u32 = rng.gen_range(0..PERLIN_Y_POINTS as u32);
            //     // choose a random point for the y offset
            //     let y_offset = rng.gen_range(-10..10 as u32) as usize;
            //     // let yy_i: u32 = rng.gen_range(0..PERLIN_Y_POINTS as u32);

            //     let x_coord = SPAWN_SPREAD * noise_values[((x + x_offset) * PERLIN_X_POINTS + y) as usize];
            //     let y_coord = SPAWN_SPREAD * noise_values[((x * PERLIN_X_POINTS) + (y + y_offset)) as usize];

            //     // let x_coord: f32 =  + x_offset as f32;
            //     // let y_coord: f32 = ycenters[j as usize] + y_offset as f32;
            
            //     commands.spawn(( //         SpriteBundle{
            //             transform: Transform::from_xyz(x_coord, y_coord, 0.),
            //             texture: food_handle.clone(), 
            //             ..default()},
            //         FoodSource::new(Vec3::new(x_coord, y_coord, 0.)),
            //     ));
            // }
        }
    }
}
