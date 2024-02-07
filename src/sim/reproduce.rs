use bevy::{prelude::*, time::Stopwatch};
use crate::{health::*, water_desire::*, food_desire::*, species::*, physics::Physics};
use rand::Rng;
use rand_distr::{Distribution, Normal, num_traits::Float};

const ADD_TO_POOL_STD: f32 = 3.; // 3 standard deviations is 99.7% chance
const REPRODUCE_GRACE_PERIOD_SEC: f32 = 10.0;


#[derive(Event)]
pub struct Reproduce(pub Entity, pub Entity);


#[derive(Component)]
pub struct Reproduction {
    pub time_since: Stopwatch,
    pub genes: Vec<f32>,
    pub fitness: f32,
    pub in_mating_pool: bool,
}

//// a resource for entities in the mating pool.
//#[derive(Resource, Default)]
//pub struct MatingPool {
    //pub members: Vec<Entity>,
//}


impl Reproduction {
    pub fn default() -> Self {
        Self {
            time_since: Stopwatch::new(),
            genes: vec![0.0],
            fitness: 1.,
            in_mating_pool: false,
        }
    }

    pub fn new(genes: Vec<f32>, fitness: f32) -> Self {
        Self {
            time_since: Stopwatch::new(),
            genes,
            fitness,
            in_mating_pool: false,
        }
    }
}




pub fn react_to_reproduction_event(
    mut reproduce_event: EventReader<Reproduce>,
    mut query: Query<(&mut Species, &mut Reproduction, &mut Transform)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
){
    for reproduce_event in reproduce_event.read() {

        info!("Reproduce!");

        // indexing .0 gives the Entity, or the parent species that is going to reproduce
        let first_parent_e = reproduce_event.0;
        let second_parent_e = reproduce_event.1;
        let parent_pos = query.get_component::<Transform>(first_parent_e).unwrap().clone().translation;

        match query.get_component_mut::<Species>(first_parent_e) {
            Ok (this_species) => {
                let new_race = &this_species.race;
                let texture: Handle<Image>; 
                // TODO instead of loading the species texture again, have it stored somewhere??
                match &new_race {
                    SpeciesRace::Red => {texture = asset_server.load("/Users/matthewbarbattini/Desktop/evolution-sim-bevy/textures/species/red_species.png")},
                    SpeciesRace::Blue => {texture = asset_server.load("/Users/matthewbarbattini/Desktop/evolution-sim-bevy/textures/species/blue_species.png")},
                    SpeciesRace::Yellow => {texture = asset_server.load("/Users/matthewbarbattini/Desktop/evolution-sim-bevy/textures/species/yellow_species.png")},
                    SpeciesRace::Green => {texture = asset_server.load("/Users/matthewbarbattini/Desktop/evolution-sim-bevy/textures/species/green_species.png")},
                }
                // birth a new member of this type of species at the parent's location
                // TODO make baby species smaller in size, grow over time?
                let new_fitness = 1.0;
                let new_genes = vec![1.0];
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
                        this_species.race, 
                        this_species.homebase, 
                        this_species.aggressiveness, 
                        this_species.avoidance),
                    Health::default(),
                    WaterDesire::default(),
                    FoodDesire::default(),
                    Reproduction::new(new_genes, new_fitness),
                    Physics::new(parent_pos),
                    ),
                );
            },
            Err(_) => info!("Could not find Species component for reproduce event on this entity."),
        }

        // reset the parent species reproduction stopwatch
        match query.get_component_mut::<Reproduction>(first_parent_e) {
            Ok(mut rep) => {
                rep.time_since.reset();
                rep.in_mating_pool = false;
            },
            Err(_) => info!("Could not find Reproduction component for reproduce event on this entity."),
        }
        match query.get_component_mut::<Reproduction>(second_parent_e) {
            Ok(mut rep) => {
                rep.time_since.reset();
                rep.in_mating_pool = false;
            },
            Err(_) => info!("Could not find Reproduction component for reproduce event on this entity."),
        }
    } // clear all events in the buffer
    reproduce_event.clear();
}


pub fn update_reproduction(
    mut query: Query<(Entity, &mut Reproduction)>,
    time: Res<Time>,
    mut reproduce_event_sender: EventWriter<Reproduce>,
    
) {
    for (e, mut reproduction) in query.iter_mut() {

        // tick time since reproduce stopwatch
        reproduction.time_since.tick(time.delta());

        if reproduction.time_since.elapsed_secs() < REPRODUCE_GRACE_PERIOD_SEC { return };

        // add to the mating pool based on random chance
        let normal = Normal::new(0., 1.).unwrap();
        let v = normal.sample(&mut rand::thread_rng());
        //info!("{}", v);
        if v > ADD_TO_POOL_STD {
            info!("In mating pool!");
            reproduction.in_mating_pool = true;
        }
    }
}

