use rand::Rng;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};



pub enum SpeciesRace {
    Red,
    Blue,
    Green,
    Yellow,
}


#[derive(Component)]
pub struct Species {
    pub id: i32,
    pub name: String,
    pub aggressiveness: f32,
    pub engineering: f32,
    pub tribalism: f32,
    pub reproducibility: f32,
    pub fighting_score: f32,
    pub need_to_reproduce: bool,
    pub race: SpeciesRace,
    pub reproduction_factor: f32,
    pub x: f32,
    pub y: f32
}

impl Species {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            id: 1,
            name: String::from("Red"),
            race: SpeciesRace::Red,
            reproduction_factor: 0.0,
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
            id: 1,
            name: String::from("Red"),
            race: SpeciesRace::Red,
            reproduction_factor: 0.0,
            aggressiveness: 1.0,
            engineering: 1.0,
            tribalism: 1.0,
            reproducibility: 1.0,
            fighting_score: 1.0,
            need_to_reproduce: false,
            x: 0.0,
            y: 0.0,
        }
    }

}

//      TODO
//      Circular radius instead of rectangular?
pub fn initial_species_group_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    /* Generate a cluster of sprites as some center coordinate with random offsets from the center
     * with Perlin noise. 
     */
    let blue_species_handle: Handle<Image> = asset_server.load("textures/species/blue_species.png");
    let red_species_handle: Handle<Image>  = asset_server.load("textures/species/red_species.png");


    let number_species_types: i32 = 2;

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


    for species_i in 0..number_species_types {
        info!("Spawned species group");
        let number_sprites = rng.gen_range(1..20);
        let x_center = rng.gen_range(-SCREEN_WIDTH/2.0..SCREEN_WIDTH/2.0);
        let y_center = rng.gen_range(-SCREEN_HEIGHT/2.0..SCREEN_HEIGHT/2.0);
        // generate the number of sprites with offsets chosen randomly from the Perlin noise map
        for _ in 0..number_sprites {
            
            let distance_scale: f64 = 100.;
            
            // choose a random point for the x offset
            let xx_i: u32 = rng.gen_range(0..perlin_x);
            let xy_i: u32 = rng.gen_range(0..perlin_y);
            // choose a random point for the y offset
            let yx_i: u32 = rng.gen_range(0..perlin_x);
            let yy_i: u32 = rng.gen_range(0..perlin_y);

            let x_offset = distance_scale * noise_values[(xx_i * perlin_x + xy_i) as usize];
            let y_offset = distance_scale * noise_values[(yx_i * perlin_x + yy_i) as usize];

            let x_coord: f32 = x_center + x_offset as f32;
            let y_coord: f32 = y_center + y_offset as f32;

            let mut species_image: Handle<Image> = blue_species_handle.clone();
            match species_i {
                0 => species_image = blue_species_handle.clone(),
                1 => species_image = red_species_handle.clone(),
                _ => info!("Species index out of range for initial spawn.")
            };


            commands.spawn((
                SpriteBundle {
                    texture: species_image,
                    transform: Transform::from_xyz(x_coord, y_coord , 0.),
                    ..default()
                },
                Species::new(x_coord, y_coord)));


        }
    }

}


pub fn species_movement(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Species)>,
    ) {

    let mut rng = rand::thread_rng();
    let max_x_movement_step: f32 = 4.0;
    let max_y_movement_step: f32 = 4.0;

    for (mut tf, mut spe) in query.iter_mut() {
       spe.x += rng.gen_range(0.0..max_x_movement_step);
       spe.y += rng.gen_range(0.0..max_y_movement_step);

       tf.translation.x += rng.gen_range(-max_x_movement_step..max_x_movement_step);
       tf.translation.y += rng.gen_range(-max_y_movement_step..max_y_movement_step);


    }

}




fn fight_species(
    mut query: Query<&mut Species>,
){
    for mut sp in query.iter_mut() {
        sp.engineering += 1.;
        info!("{}", sp.engineering);
    }
}
