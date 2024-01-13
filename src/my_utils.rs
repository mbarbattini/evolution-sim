use crate::{species::*, water_desire::WaterDesire, health::Health};


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
    mut query: Query<(Entity, &Species, &Transform, &WaterDesire, &Health)>,
) {
    let mut chose_blue: bool = false;
    let mut chose_red: bool = false;
    let mut chose_green: bool = false;
    let mut chose_yellow: bool = false;
    for (entity, spec, transform, water_desire, health) in query.iter() {
        // get only the first species in each race
        let species_index = entity.index();
        if spec.race == SpeciesRace::Blue {
            if !chose_blue {
                // info!("Blue {}", spec.position);
                // info!("Blue health: {}", health.val);
                // if water_desire.amount < 0.0 {info!("Blue is out of water")}
                // info!("Blue vel: {}", spec.velocity.length());
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
                info!("Yellow water amount: {}", water_desire.amount);
                // info!("Yellow perception radius: {}", spec.perception_radius);
                // info!("Yellow health: {}", health.val)
            }
            chose_yellow = true;

        }



    }

}



