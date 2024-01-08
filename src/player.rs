use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub x: f32,
    pub y: f32,
}


impl Default for Player {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

}
