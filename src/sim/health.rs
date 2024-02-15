use crate::fight::*;
use crate::food_desire::FoodDesire;
use crate::WaterDesire;
use bevy::prelude::*;

pub const MAX_HEALTH: f32 = 10.;
const LOW_WATER_DAMAGE_RATE: f32 = 0.1;
const LOW_HUNGER_DAMAGE_RATE: f32 = 0.1;
const BLOOD_FADE_TIME: f32 = 10.;

#[derive(Component)]
pub struct Health {
    pub val: f32,
    pub full: f32,
}

impl Health {
    pub fn default() -> Self {
        Self {
            val: MAX_HEALTH,
            full: MAX_HEALTH,
        }
    }

    pub fn new(val: f32, full: f32) -> Self {
        Self { val, full }
    }
}

#[derive(Component)]
pub struct Blood {
    pub timer: Timer,
}

impl Blood {
    pub fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BLOOD_FADE_TIME, TimerMode::Once),
        }
    }
}

pub fn fade_out_blood(
    mut query: Query<(Entity, &mut Blood, &mut Sprite, &Handle<Image>)>,
    time: ResMut<Time>,
    mut commands: Commands,
) {
    for (e, mut blood, mut sprite, mut image) in query.iter_mut() {
        if blood.timer.finished() {
            commands.entity(e).despawn();
        }
        blood.timer.tick(time.delta());
        let r = sprite.color.r();
        let g = sprite.color.g();
        let b = sprite.color.b();
        sprite.color = Color::rgba(r, g, b, blood.timer.percent_left());
    }
}

pub fn kill_zero_health(
    mut query: Query<(Entity, &Health, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut blood_handle: Handle<Image> = asset_server
        .load("/Users/matthewbarbattini/Desktop/evolution-sim-bevy/textures/blood_splat_1.png");
    for (e, h, tf) in query.iter_mut() {
        if h.val < 0. {
            commands.entity(e).despawn();
            commands.spawn((
                SpriteBundle {
                    texture: blood_handle.clone(),
                    transform: tf.clone(),
                    ..default()
                },
                Blood::default(),
            ));
        }
    }
}
// TODO make this a function for all cases where it should lose health? Food, water, fighting? Or
// separate?
// Different damage rates for each damage type?
pub fn damage_low_stats(
    mut query: Query<(&mut Health, &mut WaterDesire, &mut FoodDesire, &mut Fight)>,
    time: Res<Time>,
) {
    for (mut health, water_desire, food_desire, mut fight) in query.iter_mut() {
        if water_desire.val < 0.0 {
            health.val -= LOW_WATER_DAMAGE_RATE * time.delta_seconds();
        }
        if food_desire.val < 0.0 {
            health.val -= LOW_HUNGER_DAMAGE_RATE * time.delta_seconds();
        }
    }
}
