use bevy::{prelude::*};
use crate::{species::*, water_desire::WaterDesire, health::Health, food_desire::*};
use bevy::input::mouse::MouseWheel;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

const ZOOM_SPEED: f32 = 0.1;
const MIN_ZOOM: f32 = 0.01;
const MAX_ZOOM: f32 = 5.0;


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



pub fn key_h_go_home(
    mut query: Query<&mut Transform, With<Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
){
    if let Ok(mut transform) = query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::H) {
            transform.translation = Vec3::ZERO;
        }
    } 
}



pub fn debug_single_species(
    query: Query<(Entity, &Species, &Transform, &WaterDesire, &Health, &FoodDesire)>,
) {
    let mut chose_blue: bool = false;
    let mut chose_red: bool = false;
    let mut chose_green: bool = false;
    let mut chose_yellow: bool = false;
    for (entity, spec, transform, water_desire, health, food_desire) in query.iter() {
        // get only the first species in each race
        if spec.race == SpeciesRace::Blue {
            if !chose_blue {
                //  info!("Blue vel {}, acc: {}, pos: {}", spec.velocity.length(), spec.acceleration.length(), spec.position);
                // info!("Blue health: {}", health.val);
                // if water_desire.amount < 0.0 {info!("Blue is out of water")}
                // info!("Blue water: {}", water_desire.curr_val);
                // info!("Blue hunger: {}", food_desire.curr_val);
                // info!("Elapsed secs: {}", water_desire.timer.elapsed_secs());
                // let angle = f32::atan2(spec.velocity.x, spec.velocity.y);
                // info!("angle : {}", angle);

                // info!("x: {}, y: {}, angle: {}", spec.velocity.x, spec.velocity.y, angle);
            } 
            chose_blue = true;
        }
        if spec.race == SpeciesRace::Red {
            if !chose_red {
                // info!("Red {}", spec.position);
            }
            chose_red = true;
        }
        if spec.race == SpeciesRace::Green {
            if !chose_green {
                // info!("Green {}", spec.position);
            }
            chose_green = true;
        }
        if spec.race == SpeciesRace::Yellow {
            if !chose_yellow {
                // info!("Yellow water amount: {}", water_desire.amount);
                // info!("Yellow perception radius: {}", spec.perception_radius);
                // info!("Yellow health: {}", health.val)
                //info!("Yellow acc: {}", spec.acceleration.length());
            }
            chose_yellow = true;
        }
    }
}





pub fn zoom_system(
    mut wheel: EventReader<MouseWheel>,
    mut cam: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let delta_zoom: f32 = wheel.read().map(|e| e.y).sum();
    if delta_zoom == 0. {
        return;
    }

    let (mut pos, mut cam) = cam.single_mut();

    cam.scale += ZOOM_SPEED * delta_zoom * cam.scale;
    cam.scale = cam.scale.clamp(MIN_ZOOM, MAX_ZOOM);
    
    //TODO make the zoom center around where the mouse currently is on the screen
    //let screen_width = q_windows.single().width(); 
    //let screen_height = q_windows.single().height(); 
    //let screen_size = Vec2::new(screen_width, screen_height);

    //if let Some(position) = q_windows.single().cursor_position() {

        //let mouse_normalized_screen_pos = (position / screen_size) * 2. - Vec2::ONE;
        //let mouse_world_pos = pos.translation.truncate() + mouse_normalized_screen_pos * cam.scale;

        //pos.translation = (mouse_world_pos - mouse_normalized_screen_pos * cam.scale).extend(pos.translation.z);

    //} else {
        //println!("Cursor is not in the game window.");
    //}
}


