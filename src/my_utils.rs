use bevy::prelude::*;
use crate::species::*;


pub fn despawn_all_enemies(
    mut commands: Commands,
    query: Query<Entity, With<Species>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Q) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}



// Moves the camera with WSAD keyboard input
pub fn camera_movement(
        mut camera_query: Query<(&mut Transform, With<Camera>)>,
        keyboard_input: Res<Input<KeyCode>>,
    ){
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
