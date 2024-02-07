use std::collections::binary_heap::Iter;

use bevy::math::Rect;
use bevy::prelude::*;
use bevy::gizmos::*;
//use quadtree_rs::Quadtree;
use crate::species::*;
use crate::physics::*;
use crate::{MAP_WIDTH, MAP_HEIGHT};

pub const QUADTREE_CAPACITY: u32 = 10;

#[derive(Resource, Default)]
pub struct Quadtree {
    pub capacity: u32,
    pub members: Vec<Entity>, // the entities of species that are currently in this Quadtree window
    pub divided: bool,
    pub boundary: Rect, // the rectangle boundary, origin is in center
    pub southwest: Option<Box<Quadtree>>,
    pub northwest: Option<Box<Quadtree>>,
    pub northeast: Option<Box<Quadtree>>,
    pub southeast: Option<Box<Quadtree>>,
}


//impl Default for Quadtree {
    //fn default() -> Self { 
        //Self {
            //capacity: CAPACITY,
            //boundary: Rect::new(-MAP_WIDTH/2., -MAP_HEIGHT/2., MAP_WIDTH/2., MAP_HEIGHT/2.), 
            //divided: false,
            //members: vec![],
            //southwest: None,
            //southeast: None,
            //northwest: None,
            //northeast: None,
        //}
    //}
//}

impl Quadtree {
    // new should be used once at startup. Initialize boundary to the entire map size
    pub fn new(capacity: u32, boundary: Rect) -> Self {
        Self {
            capacity,
            boundary,
            members: vec![],
            divided: false,
            southwest: None,
            northwest: None,
            northeast: None,
            southeast: None,
        }
    }

    pub fn insert(&mut self, species_pos: Vec2, entity: Entity) -> bool {

        if !self.boundary.contains(species_pos) {
            return false;
        }

        if (self.members.len() as u32) < self.capacity {
            self.members.push(entity); // push the entity into quadtree data storage
            return true;
        } else {
            if !self.divided {
                //info!("Subdivided!");
                self.subdivide();
            }
            if 
                // recursively add the entity to children if already subdivided
                self.northeast
                    .as_mut()
                    .expect("NE does not exist")
                    .insert(species_pos, entity) ||
                self.northwest
                    .as_mut()
                    .expect("NW does not exist")
                    .insert(species_pos, entity) ||
                self.southeast
                    .as_mut()
                    .expect("SE does not exist")
                    .insert(species_pos, entity) ||
                self.southwest
                    .as_mut()
                    .expect("SW does not exist")
                    .insert(species_pos, entity) 
            {
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn subdivide(&mut self) {

        let center = self.boundary.center();
        let w = self.boundary.width();
        let h = self.boundary.height();

        let ne = Rect::new(center.x, center.y, center.x + w/2., center.y + h/2.);
        let nw = Rect::new(center.x - w/2., center.y, center.x, center.y + h/2.);
        let se = Rect::new(center.x, center.y - h/2., center.x + w/2., center.y);
        let sw = Rect::new(center.x - w/2., center.y - h/2., center.x, center.y);

        self.northeast = Some(Box::new(Quadtree::new(self.capacity, ne)));
        self.northwest = Some(Box::new(Quadtree::new(self.capacity, nw)));
        self.southeast = Some(Box::new(Quadtree::new(self.capacity, se)));
        self.southwest = Some(Box::new(Quadtree::new(self.capacity, sw)));

        self.divided = true;
    }
}



// Update the quadtree each frame
pub fn update_quadtree(
    species_query: Query<(Entity, &Physics, With<Species>)>,
    mut qt: ResMut<Quadtree>,
    mut gizmos: Gizmos,
) {
    //info!("Quadtree update");
    // only work with xy coordinates of species
    for q in &species_query {
        let entity = q.0;
        let pos = q.1.pos.xy();
        qt.insert(pos, entity);
    }
    
    // let nw_boundary = qt.northwest.as_ref().expect("").boundary;
    // if qt.as_ref().divided {
    //     qt.as_ref().northeast;
    // }
    // gizmos.rect_2d(nw_boundary.center(), 0.0, nw_boundary.size(), Color::WHITE);

    let curr_qt = qt.as_ref();
    while curr_qt.divided {


    }



    let mut current_ne_qt = qt.as_mut();
    while current_ne_qt.divided {
        gizmos.rect_2d(current_ne_qt.boundary.center(), 0.0, current_ne_qt.boundary.size(), Color::WHITE);
        if current_ne_qt.northeast.is_some() {
            current_ne_qt = current_ne_qt.northeast.as_mut().expect("");
        }
    }
    let mut current_nw_qt = qt.as_mut();
    while current_nw_qt.divided {
        gizmos.rect_2d(current_nw_qt.boundary.center(), 0.0, current_nw_qt.boundary.size(), Color::WHITE);
        if current_nw_qt.northwest.is_some() {
            current_nw_qt = current_nw_qt.northwest.as_mut().expect("");
        }
    }
    let mut current_se_qt = qt.as_mut();
    while current_se_qt.divided {
        gizmos.rect_2d(current_se_qt.boundary.center(), 0.0, current_se_qt.boundary.size(), Color::WHITE);
        if current_se_qt.southeast.is_some() {
            current_se_qt = current_se_qt.southeast.as_mut().expect("");
        }
    }
    let mut current_sw_qt = qt.as_mut();
    while current_sw_qt.divided {
        gizmos.rect_2d(current_sw_qt.boundary.center(), 0.0, current_sw_qt.boundary.size(), Color::WHITE);
        if current_sw_qt.southwest.is_some() {
            current_sw_qt = current_sw_qt.southwest.as_mut().expect("");
        }
    }
    
}
