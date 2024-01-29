use bevy::{prelude::*, time::Stopwatch};
use crate::{health::*, water_desire::*, food_desire::*, species::*};
use rand::Rng;

const MIN_REPRODUCE_THRESHOLD: f32 = 0.95;
const REPRODUCE_GRACE_PERIOD_SEC: f32 = 10.0;


#[derive(Event)]
pub struct Reproduce(pub Entity);


#[derive(Component)]
pub struct Reproduction {
    pub time_since: Stopwatch,
    pub genes: Vec<f32>,
    pub fitness: f32,
}


impl Reproduction {
    pub fn default() -> Self {
        Self {
            time_since: Stopwatch::new(),
            genes: vec![0.0],
            fitness: 1.,
        }
    }
}




pub fn react_to_reproduction_event(
    mut reproduce_event: EventReader<Reproduce>,
    mut query: Query<(&mut Species, &mut Reproduction)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
){
    for reproduce_event in reproduce_event.read() {

        //info!("Reproduce!");

        // indexing .0 gives the Entity, or the parent species that is going to reproduce
        let e = reproduce_event.0;

        match query.get_component_mut::<Species>(e) {
            Ok(mut this_species) => {
                this_species.need_to_reproduce = false;
                let new_race = &this_species.race;
                let texture: Handle<Image>; 
                // TODO instead of loading the species texture again, have it stored somewhere??
                match &new_race {
                    SpeciesRace::Red => {texture = asset_server.load("textures/species/red_species.png")},
                    SpeciesRace::Blue => {texture = asset_server.load("textures/species/blue_species.png")},
                    SpeciesRace::Yellow => {texture = asset_server.load("textures/species/yellow_species.png")},
                    SpeciesRace::Green => {texture = asset_server.load("textures/species/green_species.png")},
                }
                // birth a new member of this type of species at the parent's location
                // TODO make baby species smaller in size, grow over time?
                let parent_pos = this_species.position;
                commands.spawn(
                    (SpriteBundle {
                        texture,
                        transform: Transform {
                            translation: parent_pos,
                            scale: Vec3::splat(SPECIES_TEXTURE_SCALE),
                            rotation: Quat::default()
                        },
                        ..default()},
                    Species::new(
                        parent_pos, 
                        this_species.race, 
                        this_species.homebase, 
                        this_species.aggressiveness, 
                        this_species.avoidance),
                    Health::default(),
                    WaterDesire::default(),
                    FoodDesire::default(),
                    Reproduction::default(),
                    ),
                );
            },
            Err(_) => info!("Could not find Species component for reproduce event on this entity."),
        }

        // reset the parent species reproduction_factor
        match query.get_component_mut::<Reproduction>(e) {
            Ok(mut rep) => {
                rep.time_since.reset();
            },
            Err(_) => info!("Could not find Reproduction component for reproduce event on this entity."),
        }
    }
    reproduce_event.clear();
}


pub fn update_reproduction(
    mut query: Query<(Entity, &mut Reproduction, &mut Species, &FoodDesire, &WaterDesire, &Health)>,
    time: Res<Time>,
    mut reproduce_event_sender: EventWriter<Reproduce>,
    
) {
    for (e, mut reproduction, mut sp, food_des, water_des, health) in query.iter_mut() {

        // tick time since reproduce stopwatch
        reproduction.time_since.tick(time.delta());


        // TODO reproduce if stats are over some threshold? Or reproduce on random chance weighted according to good
        // stats? Genetic algorithm?
        let mut rng = rand::thread_rng();
        let threshold = rng.gen_range(MIN_REPRODUCE_THRESHOLD..1.0);
        let mut score = 0.0;
        //info!("Health: {}, Water: {}, Food: {}", health.val, water_des.val, food_des.val);
        if health.val > 0. { score += health.val};
        if water_des.val > 0. { score += water_des.val };
        if food_des.val > 0. { score += food_des.val };
        score /= MAX_HEALTH + MAX_WATER + MAX_HUNGER;
        //info!("Score: {}", score);

        // trigger reproduction event
        if score > threshold && reproduction.time_since.elapsed_secs() > REPRODUCE_GRACE_PERIOD_SEC {
            reproduce_event_sender.send(Reproduce(e));
        }


    }
}

//// react to the event
//fn birth_species(
    //mut events: EventReader<Reproduce>,
    //mut query: Query<&mut Species>,
    //asset_server: Res<AssetServer>,
    //mut commands: Commands,
//) {
    //for reproduce_event in events.read() {
        //// indexing .0 gives the Entity
        //let en = reproduce_event.0;

        //match query.get_component_mut::<Species>(en) {
            //Ok(mut this_species) => {
                //this_species.need_to_reproduce = false;
                //let new_race = &this_species.race;
                //let mut texture: Handle<Image> = asset_server.load("textures/species/blue_species.png");
                //match &new_race {
                    //SpeciesRace::Red => {texture = asset_server.load("textures/species/red_species.png")},
                    //SpeciesRace::Blue => {texture = asset_server.load("textures/species/blue_species.png")},
                    //SpeciesRace::Yellow => {texture = asset_server.load("textures/species/blue_species.png")},
                    //SpeciesRace::Green => {texture = asset_server.load("textures/species/blue_species.png")},
                //}
                //// birth a new member of this type of species at the parent's location
                //let parent_x = this_species.x;
                //let parent_y = this_species.y;
                //commands.spawn(
                    //(SpriteBundle {
                        //texture,
                        //transform: Transform::from_xyz(parent_x, parent_y, 0.),
                        //..default()},
                    //Species::new(parent_x, parent_y))
                //);
            //},
            //Err(_) => info!("Could not find Species component for reproduce event on this entity."),
        //}
    //}
//}
