use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub vel: Vec3,
}


impl Default for Player {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vel: Vec3::ZERO,
        }
    }
}


pub fn move_player(
    mut query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
) {
    let mut q = query.single_mut();
    let mut player = q.0;
    let mut tf = q.1;
    
    let const_acc = Vec3::new(1., 0.0, 0.0) * time.delta_seconds();
    player.vel += const_acc;
    tf.translation += player.vel; // pixels per second

}
