// TODO Include if loading an entire folder
// use bevy::{asset::LoadedFolder, prelude::*};

use bevy::prelude::*;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Evolution Sim v1.0".into(),
                        ..default()
                    }),
                    ..default()
                })
            .set(ImagePlugin::default_nearest())
        )
        .add_systems(Startup, setup)
        .add_systems(Update, camera_movement)
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

    let blue_species_handle = asset_server.load("textures/species/blue_species.png");
    let red_species_handle = asset_server.load("textures/species/red_species.png");

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(5.0),
            ..default()
        },
        texture: blue_species_handle,
        ..default()
    });

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(100., 0.0, 0.0),
            scale: Vec3::splat(5.0),
            ..default()
        },
        texture: red_species_handle,
        ..default()
    });

    commands.spawn(Camera2dBundle::default());
}


fn camera_movement(
        mut camera_query: Query<(&mut Transform, With<Camera>)>,
        keyboard_input: Res<Input<KeyCode>>,
    ){
    /*  Moves the camera with WSAD keyboard input  */
    //TODO Add acceleration
    let movement_speed: f32 = 5.;

    let mut movement_up: bool = false;
    let mut movement_down: bool = false;
    let mut movement_left: bool = false;
    let mut movement_right: bool = false;

    if keyboard_input.pressed(KeyCode::W) {
        movement_up = true; 
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement_down = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement_right = true;
    }
    if keyboard_input.pressed(KeyCode::A) {
        movement_left = true;  
    }

    let mut camera_transform = camera_query.single_mut();

    if movement_up {
        camera_transform.0.translation.y += movement_speed;
    } else if movement_down {
        camera_transform.0.translation.y -= movement_speed;
    } else if movement_right {
        camera_transform.0.translation.x += movement_speed;
    } else if movement_left {
        camera_transform.0.translation.x -= movement_speed;
    }

}

// this function iterates over all entities that have a Sprite Component and a Transform Component
fn sprite_do_all(
        mut sprite_query: Query<(&Sprite, &mut Transform)>,
        keyboard_input: Res<Input<KeyCode>>,
    ){

    for (mut sp, mut tr) in sprite_query.iter_mut() {


    }
    
}





