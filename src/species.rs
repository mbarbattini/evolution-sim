use rand::Rng;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use bevy::math::f32::{Vec2, Vec3};

use crate::water_source::*;

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
    pub target_pos: Vec3,
    pub water_desire: f32,
    pub n_neighbors: u32,
    pub reproduction_factor: f32,
    pub perception_radius: f32,
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub position: Vec3,
}

impl Species {
    pub fn new(position: Vec3) -> Self {
        let mut rng = rand::thread_rng();
        let x_vel = rng.gen_range(-1.0..1.0);
        let y_vel = rng.gen_range(-1.0..1.0);
        Self {
            id: 1,
            name: String::from("Red"),
            race: SpeciesRace::Red,
            reproduction_factor: 0.0,
            acceleration: Vec3::ZERO,
            velocity: Vec3::new(x_vel, y_vel, 0.0),
            target_pos: Vec3::ZERO,
            position,
            perception_radius: 25.0,
            n_neighbors: 0,
            water_desire: 1.0,

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
            acceleration: Vec3::ZERO,
            velocity: Vec3::new(0., 0., 0.),
            position: Vec3::ZERO,
            target_pos: Vec3::ZERO,
            perception_radius: 25.0,
            n_neighbors: 0,
            water_desire: 1.0,
            
            aggressiveness: 1.0,
            engineering: 1.0,
            tribalism: 1.0,
            reproducibility: 1.0,
            fighting_score: 1.0,
            need_to_reproduce: false,
        }
    }

}

//TODO Circular radius instead of rectangular?
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
        let number_sprites = rng.gen_range(30..50);
        let x_center = rng.gen_range(-SCREEN_WIDTH/2.0..SCREEN_WIDTH/2.0);
        let y_center = rng.gen_range(-SCREEN_HEIGHT/2.0..SCREEN_HEIGHT/2.0);
        // generate the number of sprites with offsets chosen randomly from the Perlin noise map
        for _ in 0..number_sprites {
            
            let distance_scale: f64 = 1000.;
            
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
            
            // TODO match the species enum instead of ints
            let mut species_image: Handle<Image> = blue_species_handle.clone();
            match species_i {
                0 => species_image = blue_species_handle.clone(),
                1 => species_image = red_species_handle.clone(),
                _ => info!("Species index out of range for initial spawn.")
            };

            commands.spawn((
                SpriteBundle {
                    texture: species_image,
                    transform: Transform {
                         translation: Vec3::new(x_coord, y_coord, 0.),
                         rotation: Quat::default(),
                         scale: Vec3::splat(2.),

                    },
                    ..default()},
                Species::new(Vec3::new(x_coord, y_coord, 0.))));
        }
    }

}


pub fn update_species(
    mut query: Query<(Entity, &mut Transform, &mut Species)>,
    time: Res<Time>,
){
    const MAXIMUM_ACCELERATION: f32 = 0.1;


    let mut rng = rand::thread_rng();
    for (entity, mut transform, mut spec) in query.iter_mut() {

        // update physics
        let mut new_acceleration = Vec3::ZERO;
        let mut new_velocity = Vec3::splat(0.1);
        
        // random acceleration in some direction
        //if rng.gen_range(0.0..1.0) < 0.1 {
            //let factor = 0.1;
            //new_acceleration = Vec3::new(rng.gen_range(-factor..factor), rng.gen_range(-factor..factor), 0.0) * time.delta_seconds();
        //}

        // constant negative y acceleration
        //new_acceleration = Vec3::new(0., -0.1, 0.) * time.delta_seconds();
        
        // TODO do we need the line below if other functions are adding acceleration?
        //spec.acceleration += new_acceleration;
        new_acceleration = spec.acceleration;

        // give the species some maximum acceleration
        // don't need to update the species.acceleration because it gets reset in next update
        if new_acceleration.length() > MAXIMUM_ACCELERATION {
            new_acceleration = new_acceleration.normalize() * MAXIMUM_ACCELERATION;
            info!("Raw Acc: {}", new_acceleration);
        }
        spec.velocity += new_acceleration;
        new_velocity = spec.velocity;
    
        spec.position += new_velocity;

        // update sprite transform
        transform.translation = spec.position;

        // reset data
        //spec.acceleration = Vec3::ZERO;
        spec.n_neighbors = 0;

         if entity.index() == 7 {
             info!("{}", spec.velocity);
         }
    }
}



pub fn go_to_water_source(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(&mut Transform, &mut WaterSource)>,
        Query<(&mut Transform, &mut Species)>,
    )>,
    time: Res<Time>
) {
    let mut water_locations: Vec<Vec3> = vec![];
    let mut water_radii: Vec<f32> = vec![];

    for (mut water_transform, mut water_source) in set.p0().iter_mut() {
        water_locations.push(water_transform.translation);
        water_radii.push(water_source.radius);
    }

    for (mut species_transform, mut spec) in set.p1().iter_mut() {
        for (i, water_pos) in water_locations.iter().enumerate() {

            let distance = *water_pos - species_transform.translation;
            if distance.length() - water_radii[i] < spec.perception_radius {
                // add a force to steer towards this water source
                // make the magnitude proprotional to the species desire for water
                let steering = distance.normalize() * spec.water_desire * time.delta_seconds();
                spec.acceleration += steering;
            }
        }
    }


}




//pub fn species_movement(
    //mut commands: Commands,
    //mut water_query: Query<(Entity, &mut Transform, &mut WaterSource)>,
    //mut species_query: Query<(Entity, &mut Transform, &mut Species)>,

    //time: Res<Time>,
//) {

    //let mut rng = rand::thread_rng();

    //for (entity, mut transform, mut water_source) in water_query.iter_mut() {


    //}



     //////iterate through all distinct pair of species 
     ////let mut pair_combinations = query.iter_combinations_mut::<2>();
     ////while let Some([spec1, spec2]) = pair_combinations.fetch_next() {
         ////let entity1 = spec1.0;
         ////let entity2 = spec2.0;
         ////let translation1 = spec1.1.translation;
         ////let translation2 = spec2.1.translation;
         ////let species1 = spec1.2;
         ////let species2 = spec2.2;
     ////}

     ////iterate through each individual species
     //for (entity, mut transform, mut spec) in query.iter_mut() {

         //////GOAL POSITION
         ////let goal = Vec3::new(220.0, 200.0, 0.0);
         ////goal_velocity = goal - transform.translation;
         ////desired_velocity += goal_velocity;

    


         //if entity.index() == 7 {
             //info!("{}", new_acceleration);
         //}
    //}
//}




