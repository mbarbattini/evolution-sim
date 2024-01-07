// TODO Include if loading an entire folder
// use bevy::{asset::LoadedFolder, prelude::*};

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::ecs::query::ReadOnlyWorldQuery; use bevy::asset::LoadState;
use rand::Rng;
use core::time;
use std::thread::sleep;
use noise::{NoiseFn, Perlin, Seedable};
use std::time::Duration;


use species::*;
use my_utils::*;

mod species;
mod my_utils;

pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Evolution Sim v1.0".into(),
                        resolution: {WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT)},
                        ..default()
                    }),
                    ..default()
                })
            .set(ImagePlugin::default_nearest())
        )
        .add_systems(Startup, setup)
        .add_systems(Startup, initial_species_group_spawn)
        .add_systems(Update, camera_movement)
        .add_systems(Update, species_movement)
        .add_systems(Update, trigger_event_single_species)
        .add_systems(Update, react_to_event_single_species)
        .add_systems(Update, despawn_all_enemies)
        .add_systems(Update, update_loop_single_species)

        .add_event::<Reproduce>()

        .run();
}







fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {


    // TODO Load all assets from a folder with asset_server.load_folder() instead of loading them
    // individually with asset_server.load().
    // Get weird error when I try to do this: "All loaded files need an extension"
    //let loaded_folder: Handle<LoadedFolder> = asset_server.load_folder("textures/species");


    commands.spawn(Camera2dBundle::default());
}




// loop through all the species in the world and check for events that can be triggered
fn trigger_event_single_species(
    query: Query<(Entity, &Species)>,
    time: Res<Time>,
    mut reproduce_event: EventWriter<Reproduce>,
    //TODO mut other_event: EventWrite<OtherEvent>,
){
    for (entity, species) in query.iter() {
      // reproduction event
      if species.reproduction_factor > 10.0 {
        reproduce_event.send(Reproduce(entity));
      }

      //if () {
        //other_event.send();
      //}
    }
}


fn update_loop_single_species(
    mut query: Query<&mut Species>,
){
    for mut species in query.iter_mut() {
        let mut rng = rand::thread_rng();
        let reproduce_chance = rng.gen_range(0.0..1.0);
        info!("{}", reproduce_chance);
        if reproduce_chance < 0.001 {
            species.reproduction_factor = 11.0;
        }
    }

}





fn react_to_event_single_species(
    mut reproduce_event: EventReader<Reproduce>,
    mut query: Query<&mut Species>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
){
    for reproduce_event in reproduce_event.read() {

        // indexing .0 gives the Entity
        let en = reproduce_event.0;

        match query.get_component_mut::<Species>(en) {
            Ok(mut this_species) => {
                this_species.need_to_reproduce = false;
                let new_race = &this_species.race;
                // TODO how to initalize to nothing in Rust??
                let mut texture: Handle<Image> = asset_server.load("textures/species/blue_species.png");
                // TODO instead of loading the species texture again, have it stored somewhere??
                match &new_race {
                    SpeciesRace::Red => {texture = asset_server.load("textures/species/red_species.png")},
                    SpeciesRace::Blue => {texture = asset_server.load("textures/species/blue_species.png")},
                    SpeciesRace::Yellow => {texture = asset_server.load("textures/species/blue_species.png")},
                    SpeciesRace::Green => {texture = asset_server.load("textures/species/blue_species.png")},
                }
                // birth a new member of this type of species at the parent's location
                let parent_x = this_species.x;
                let parent_y = this_species.y;
                commands.spawn(
                    (SpriteBundle {
                        texture,
                        transform: Transform::from_xyz(parent_x, parent_y, 0.),
                        ..default()},
                    Species::new(parent_x, parent_y))
                );
                // reset the parent species reproduction_factor
                this_species.reproduction_factor = 0.0;
            },
            Err(_) => info!("Could not find Species component for reproduce event on this entity."),
        }
    }
    reproduce_event.clear();

}





#[derive(Event)]
struct Reproduce(Entity);



// react to the event
fn birth_species(
    mut events: EventReader<Reproduce>,
    mut query: Query<&mut Species>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for reproduce_event in events.read() {
        // indexing .0 gives the Entity
        let en = reproduce_event.0;

        match query.get_component_mut::<Species>(en) {
            Ok(mut this_species) => {
                //TODO instead of loading the species texture again, have it stored somewhere??
                this_species.need_to_reproduce = false;
                let new_race = &this_species.race;
                // TODO how to initalize to nothing in Rust??
                let mut texture: Handle<Image> = asset_server.load("textures/species/blue_species.png");
                match &new_race {
                    SpeciesRace::Red => {texture = asset_server.load("textures/species/red_species.png")},
                    SpeciesRace::Blue => {texture = asset_server.load("textures/species/blue_species.png")},
                    SpeciesRace::Yellow => {texture = asset_server.load("textures/species/blue_species.png")},
                    SpeciesRace::Green => {texture = asset_server.load("textures/species/blue_species.png")},
                }
                // birth a new member of this type of species at the parent's location
                let parent_x = this_species.x;
                let parent_y = this_species.y;
                commands.spawn(
                    (SpriteBundle {
                        texture,
                        transform: Transform::from_xyz(parent_x, parent_y, 0.),
                        ..default()},
                    Species::new(parent_x, parent_y))
                );
            },
            Err(_) => info!("Could not find Species component for reproduce event on this entity."),
        }
    }
}


