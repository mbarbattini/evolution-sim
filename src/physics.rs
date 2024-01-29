use bevy::prelude::*;
use crate::{debug_ui::*};
use std::f32::consts::PI;

const MAX_VELOCITY: f32 = 2.;

#[derive(Component)]
pub struct Physics {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub steering: Vec2,
}


impl Physics {
    pub fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            steering: Vec2::ZERO,
        }
    }
}


pub fn update_physics(
    mut query: Query<(&mut Physics, &mut Transform)>,
    ui_state: Res<UiState>,
    time: Res<Time>,
){
    for (mut phys, mut tf) in query.iter_mut() {
        // update species physics
        let mut cur_acc = phys.acc;
        let mut cur_vel = phys.vel;
        let mut cur_pos = phys.pos;

        // info!("acc: {}, vel: {}, pos: {}", cur_acc.length(), cur_vel.length(), cur_pos.length());
        
        if cur_acc.is_nan() {
            cur_acc = Vec2::ZERO;
        } 
        if cur_vel.is_nan() {
            cur_vel = Vec2::ZERO;
        }
        if cur_pos.is_nan() {
            cur_pos = Vec2::ZERO;
        }

        if ui_state.steering_strength != 0. {
            phys.steering = phys.steering.clamp_length_max(ui_state.steering_strength);
        } else {
            phys.steering = phys.steering.clamp_length_max(1.0);
        }

        let mut new_acc = (cur_acc + phys.steering) * time.delta_seconds();

        if ui_state.max_acceleration != 0. {
            new_acc = new_acc.clamp_length_max(ui_state.max_acceleration);
        } else {
            new_acc = new_acc.clamp_length_max(1.0);
        }

        let mut new_vel = cur_vel + new_acc;

        new_vel = new_vel.clamp_length_max(MAX_VELOCITY);
        
        let new_pos = cur_pos + new_vel;

        // now set species physics data
        phys.vel = new_vel;
        phys.pos = new_pos;

        // always render the sprite in front of everything else with Z coord
        let sprite_position = Vec3::new(phys.pos.x, phys.pos.y, 10.);
        tf.translation = sprite_position;
        let angle = f32::atan2(phys.vel.y, phys.vel.x);
        // subtracting PI/2 makes the sprite in line with y axis, travels facing the top
        // not subtracting makes it in line with x axis, travels facing the side
        tf.rotation = Quat::from_euler(EulerRot::XYZ, 0., 0., angle - PI/2.);
    }
}