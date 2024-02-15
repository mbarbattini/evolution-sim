use std::hash::Hash;
use bevy::prelude::Resource;

use bevy::{
    prelude::{Entity, Transform, Vec3},
    math::Rect,
};
use crate::quadtree_value::*;
use crate::quadtree::*;

#[derive(Clone)]
pub struct EntityWrapper {
    pub entity: Entity,
    pub pos: Vec3,
    pub rect: Rect,
}

impl EntityWrapper {
    pub fn new(entity: Entity, pos: Vec3, rect: Rect) -> Self {
        EntityWrapper {
            entity,
            pos,
            rect,
        }
    }
}

impl QuadtreeValue for EntityWrapper {
    fn get_rect(&self) -> &Rect {
        &self.rect
    }
}

impl PartialEq for EntityWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl Hash for EntityWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entity.hash(state);
    }
}

impl Eq for EntityWrapper {}

pub type EntityQuadtree = Quadtree<EntityWrapper>;

impl Resource for EntityQuadtree {

}

// #[derive(Resource)]
// pub struct EntityQuadtree {
//     pub inst: Quadtree<EntityWrapper>,
// }