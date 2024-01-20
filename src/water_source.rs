use bevy::prelude::*;
use rand::Rng;

use crate::{MAP_WIDTH, MAP_HEIGHT};

const NUMBER_SOURCES: u32 = 4;
const RADIUS: f32 = 20.;
const CAPACITY: f32 = 10.;


#[derive(Component)]
pub struct WaterSource {
    pub position: Vec3,
    pub capacity: f32,
    pub value: f32,
    pub radius: f32,
}


// TODO make the water source radius proportional to the sprite size in pixels
impl WaterSource {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            capacity: CAPACITY,
            value: CAPACITY,
            radius: RADIUS,
        }
    }
}


pub fn spawn_water_sources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let mut rng = rand::thread_rng();

    let water_source_handle: Handle<Image> = asset_server.load("textures/simple_water_source.png");


    for _ in 0..NUMBER_SOURCES {

        let position = Vec3::new(rng.gen_range(-MAP_WIDTH/2.0..MAP_WIDTH/2.), rng.gen_range(-MAP_HEIGHT/2.0..MAP_HEIGHT/2.), -5.0);
        commands.spawn((
            SpriteBundle {
                texture: water_source_handle.clone(),
                transform: Transform {
                     translation: position,
                     rotation: Quat::default(),
                     scale: Vec3::splat(3.)},
                ..default()
            },
            WaterSource::new(position),    
        ));

    }



}

