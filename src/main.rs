use bevy::prelude::*;
use bevy::window::WindowResolution;
use sim::*;
use bevy_egui::EguiPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

mod sim;

use species::*;
use my_utils::*;
use fps_counter::*;
use water_source::*;
use water_desire::*;
use food_desire::*;
use food_source::*;
use debug_ui::*;
use behavior::*;
use fight::*;
use health::*;
use homebase::*;


// mod species;
// mod my_utils;
// mod player;
// mod fps_counter;
// mod water_source;
// mod water_desire;
// mod health;
// mod food_desire;
// mod food_source;
// mod homebase;
// mod behavior;
// mod debug_ui;
// mod physics;
// mod fight;

pub const SCREEN_WIDTH: f32 = 1920.;
pub const SCREEN_HEIGHT: f32 = 1080.;
pub const MAP_WIDTH: f32 = 1920.;
pub const MAP_HEIGHT: f32 = 1080.;


fn main() {

    App::new()
        .add_plugins(
            DefaultPlugins
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
        .init_resource::<UiState>()
        .init_resource::<FoodLocations>()
        .init_resource::<EntityQuadtree>()
        // PLUGINS
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EguiPlugin)

        .add_systems(Startup, 
            (setup, setup_fps_counter, create_homebases, spawn_water_sources, spawn_food_sources))

        .add_systems(PostStartup, initial_species_group_spawn)

        .add_systems(PreUpdate, 
            (camera_movement, key_h_go_home, fps_text_update_system, fps_counter_showhide, zoom_system))

        .add_systems(Update, 
            (debug_menu_ui, damage_low_stats, update_hunger, update_water_desire, behaviors, spawn_food_replenish, fight_species, fade_out_blood))

        .add_systems(PostUpdate,
            (despawn_all_enemies, kill_zero_health, debug_single_species))
        // EVENTS
        // .add_systems(Update, 
        //     (trigger_event_single_species, react_to_event_single_species))
        // .add_event::<Reproduce>()

        .run();
}



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let origin_texture = asset_server.load("textures/origin_marker_slim.png");

    // create an origin sprite marker
    commands.spawn((
        SpriteBundle{
            texture: origin_texture,
            transform: Transform {
                 translation: Vec3::new(0., 0., 0.),
                 rotation: Quat::default(),
                 scale: Vec3::splat(1.0),
            },
            ..default()},
    ));

    commands.spawn(Camera2dBundle::default());
    
}

