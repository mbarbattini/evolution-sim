use bevy::ecs::entity;
use bevy::math::Rect;
use bevy::prelude::*;

use crate::{quadtree_node::QuadtreeNode, quadtree_value::QuadtreeValue};
use crate::quadtree_stats::*;
use crate::{physics::*, EntityQuadtree, EntityWrapper};
use crate::species::*;

pub const MAX_DEPTH: usize = 8;
pub const THRESHOLD: usize = 10;
pub const PERCEPTION_RADIUS: f32 = 50.;


pub struct Quadtree<T: QuadtreeValue> {
    pub rect: Rect,
    pub root: QuadtreeNode<T>,
}

impl<T: QuadtreeValue> Quadtree<T> {
    pub fn empty(size: Rect) -> Self {
        Quadtree {
            rect: size,
            root: QuadtreeNode::<T>::empty(size.clone(), 0),
        }
    }

    pub fn add(&mut self, value: T) {
        //only add if value is contained within our rect
        if self.root.contains_rect(value.get_rect()) {
            self.root.add(value);
        }
    }

    pub fn delete(&mut self, value: &T) -> Option<T> {
        match self.query_value_mut(value) {
            Some(node) => node.delete(value),
            None => None,
        }
    }

    pub fn query_value_mut(&mut self, value: &T) -> Option<&mut QuadtreeNode<T>> {
        self.root.find_value_mut(value)
    }

    pub fn query_rect(&self, rect: &Rect) -> Option<&QuadtreeNode<T>> {
        self.root.query_rect(rect)
    }

    pub fn query_rect_mut(&mut self, rect: &Rect) -> Option<&mut QuadtreeNode<T>> {
        self.root.query_rect_mut(rect)
    }
}



pub fn update_quadtree(
    query: Query<(Entity, &Physics)>,
    mut qt: ResMut<EntityQuadtree>,
) {
    for (e, phys) in query.iter() {
        let perception_rect = Rect::new(phys.pos.x - PERCEPTION_RADIUS, phys.pos.y - PERCEPTION_RADIUS, phys.pos.x + PERCEPTION_RADIUS, phys.pos.y + PERCEPTION_RADIUS);
        let entity_wrap = EntityWrapper::new(e, phys.pos, perception_rect);
        
        if let Some(node) = qt.query_rect_mut(entity_wrap.get_rect()) {
            if !node.contains_value(&entity_wrap) {
                qt.delete(&entity_wrap);
                qt.add(entity_wrap);
            }
        }


        // QuadtreeStats::calculate(&qt).print();
    }
}
