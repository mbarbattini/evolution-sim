use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        Render, RenderApp, RenderSet,
    },
    // FPS Tracking
    diagnostic::FrameTimeDiagnosticsPlugin,
    window::WindowPlugin,
};
use rand::prelude::*;
use bevy::math::{vec2, vec3};



#[derive(Component)]
struct Species {
    name: String
}

// Bevy query. Find all Components of a certain type in the game
fn find_all_species(species_query: Query<&Species>) {
    for species in species_query.iter() {
        println!("Species is {}", species.name);
    }
}
// query components with only a certain component
// Query<&Species, With<SomeComponent>>




// a struct for the Plugin that interfaces with Bevy
pub struct EvolutionSimPlugin;


// implements the Plugin trait from Bevy
impl Plugin for EvolutionSimPlugin  {
    // give the App all the required info
    // arguments are required 
    fn build(&self, mut app: &mut App) {




    }

    // this runs once all plugins have been loaded
    fn finish(&self, mut app: &mut App) {
        


    }


}
//
// this doesn't seem to find the Screen, which was created as a SpriteBundle or Sprite.
// This game of life example renders the screen in a different way than a traditional bevy games
// does, so maybe we can use this, maybe we can't
fn change_screen_color(mut sprite_query: Query<(&mut Sprite, &mut Handle<Image>)>) {
    //let dt = time.delta_seconds();

    let mut image = Image::new_fill(
        Extent3d {
            width: 1920 / 2,
            height: 1080 / 2,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[11, 22, 44],
        TextureFormat::Rgba8Unorm,
    );

    for (mut sprite, mut textu) in &mut sprite_query {
        textu = image;
    }
    
    //if let Ok(mut screen_sprite) = sprite_query.get_single_mut() {
        //screen_sprite.texture = ;

    //}

       

}


fn load_textures_from_tilemap(
    mut texture_map: ResMut<Assets<TextureAtlas>>,
    ) {
    const SPRITE_SHEET_PATH: &str = "test.png";
    const TILE_H: i32 = 100;
    const TILE_W: i32 = 100;
    const SPRITE_SHEET_W: i32 = 19;
    const SPRITE_SHEET_H: i32 = 19;
    const SPRITE_PADDING: f32 = 19;
    const SPRITE_SHEET_OFFSET: f32 = 19;

    let asset_server: Res<AssetServer>;
    let texture_handle = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        vec2(TILE_W as f32, TILE_H as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        Some(Vec2::splat(SPRITE_PADDING)),
        Some(Vec2::splat(SPRITE_SHEET_OFFSET)),
    );
    //let handle = texture_atlases.add(texture_atlas);

}


// the base struct of the Game. Contains all game logic. Separate from the Plugin
struct EvolutionSim {
    state: AppState,
}



// some deep rendering process in the engine
impl render_graph::Node for EvolutionSim {
    fn update(&mut self, _world: &mut World) {
        
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,) -> Result<(), render_graph::NodeRunError>  {
    
            
        // do something based on the state of the game
        match self.state {
            AppState::Loading => {},
            AppState::Init => {},
            AppState::Update => {},
        }


        Ok(())
    }


}

// the possible states of the entire application
pub enum AppState {
    Loading,
    Init,
    Update,
}


fn setup (mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    
    let scale_factor = 2;
    let screen_width = 1920 / scale_factor;
    let screen_height = 1080 / scale_factor;
    // fill every pixel with a random color
    let mut image = Image::new_fill(
        Extent3d {
            width: screen_width,
            height: screen_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[34, 122, 5, 255],
        TextureFormat::Rgba8Unorm,
    );

    image.texture_descriptor.usage =
            TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let image = images.add(image);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(screen_width as f32, screen_height as f32)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });

    // Create a 2D camera
    commands.spawn(Camera2dBundle::default());

    //commands.insert_resource(GameOfLifeImage(image));

}



fn main() {

    App::new()
        // snap an object to the nearest pixel
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .run();
}


