// ECS System. Entity, Component, System

// An component implements the Component Trait
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
// a system is just a normal rust function
//
// an entity is a type that holds a unique integer
struct Entity(u64);




#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);



// a plugin seems like an organization method. Maybe how we will 
// create a "class" for some entity type. 
// Just add the plugin in the main function
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}


fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}




// transforms
Transform::from_xyz(1.0, 1.0, 1.0);


// keyboard input
let keyboard: Res<Input<KeyCode>>;
if keyboard.just_pressed(KeyCode::Space) { };


// logging. Bevy custom. Plugins required are in DefaultPlugins
error!("Unknown condition!");
warn!("Something unusual happened!");
info!("Entered game level: {}", level_id);
debug!("x: {}, state: {:?}", x, state);
trace!("entity transform: {:?}", transform);











