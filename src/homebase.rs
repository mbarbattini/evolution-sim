use bevy::prelude::*;
use rand::Rng;
use crate::{MAP_WIDTH, MAP_HEIGHT};
use crate::SpeciesRace;


#[derive(Component)]
pub struct Homebase {
    pub position: Vec3,
    pub species_race: SpeciesRace,


}



impl Homebase {
    pub fn new(position: Vec3, species_race: SpeciesRace) -> Self {
        Self {
            position,
            species_race,
        }
    }
}




pub fn create_homebases(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let mut rng = rand::thread_rng();
    for i in 1..5 {
        // spawn homebases in 4 quadrants in the corners
        const OFFSET_X: f32 = MAP_WIDTH * 0.3;
        const OFFSET_Y: f32 = MAP_HEIGHT * 0.3;
        let mut min_x: f32 = 0.;
        let mut max_x: f32 = 0.;
        let mut min_y: f32 = 0.;
        let mut max_y: f32 = 0.;
        match i {
            1 => { // upper left
                min_x = -MAP_WIDTH/2.; max_x = -MAP_WIDTH/2. + OFFSET_X; min_y = MAP_HEIGHT/2. - OFFSET_Y ; max_y = MAP_HEIGHT/2.;},
            2 => { // upper right
                min_x = MAP_WIDTH/2. - OFFSET_X; max_x = MAP_WIDTH/2.; min_y = MAP_HEIGHT/2. - OFFSET_Y ; max_y = MAP_HEIGHT/2.;},
            3 => { // lower right
                min_x = MAP_WIDTH/2. - OFFSET_X; max_x = MAP_WIDTH/2.; min_y = -MAP_HEIGHT/2.; max_y = -MAP_HEIGHT/2. + OFFSET_Y;},
            4 => { // lower left
                min_x = -MAP_WIDTH/2.; max_x = -MAP_WIDTH/2. + OFFSET_X; min_y = -MAP_HEIGHT/2.; max_y = -MAP_HEIGHT/2. + OFFSET_Y;},
            _ => {},
        }
        // spawn at -10 Z so sprites are in front of it?
        let homebase_x = rng.gen_range(min_x..max_x);
        let homebase_y = rng.gen_range(min_y..max_y);
        let homebase_pos: Vec3 = Vec3::new(homebase_x, homebase_y, -10.);

        let mut texture_handle: Handle<Image> = asset_server.load("textures/Homebases/blue_homebase.png");

        let mut race: SpeciesRace = SpeciesRace::Red;
        match i {
            1 => {race = SpeciesRace::Blue; texture_handle = asset_server.load("textures/Homebases/blue_homebase.png")}
            2 => {race = SpeciesRace::Red; texture_handle = asset_server.load("textures/Homebases/red_homebase.png")}
            3 => {race = SpeciesRace::Yellow; texture_handle = asset_server.load("textures/Homebases/yellow_homebase.png")}
            4 => {race = SpeciesRace::Green; texture_handle = asset_server.load("textures/Homebases/green_homebase.png")},
            _ => continue,
        };

        commands.spawn((
            SpriteBundle {
                texture: texture_handle,
                transform: Transform {
                     translation: homebase_pos,
                     rotation: Quat::default(),
                     scale: Vec3::splat(2.),
                },
                ..default()
            },
            Homebase::new(homebase_pos, race)
        ));


    }

}
