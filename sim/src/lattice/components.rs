use crate::config::{colors_config, lattice_config};
use bevy::prelude::*;

//-------------------------------------------------------
// STRUCTS
//-------------------------------------------------------

/// If a node is spawned with a static component, it should not move
#[derive(Component)]
pub struct Static;

/// Nodes!
#[derive(Component)]
pub struct Node {
    pub pos: Vec3,        // meters
    pub vel: Vec3,        // meters/sec
    pub sum_forces: Vec3, //newtons
    pub mass: f32,        // kg
}

/// Links are massless.
/// Using link / spring interchangably throughout the code
#[derive(Component)]
pub struct Link {
    pub spring_const: f32,
    pub delta_spring_length_pre: f32,
    pub orig_length: f32,
    pub to: Option<Entity>,
    pub from: Entity,
}

//-------------------------------------------------------
// IMPLEMENTATIONS
//-------------------------------------------------------

impl Default for Node {
    fn default() -> Self {
        Node {
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            sum_forces: Vec3::ZERO,
            mass: lattice_config::NODE_MASS,
        }
    }
}

impl Link {
    /// Create a new link.
    /// From denotes from which node the link is connected and
    /// to denotes to which node the link is connected
    pub fn new(spring_const: f32, orig_length: f32, to: Option<Entity>, from: Entity) -> Self {
        // function returns an instance of Link
        // When function names are the same as field names, don't need to type it twice.
        Link {
            spring_const,
            orig_length,
            to,
            from,
            delta_spring_length_pre: 0.0,
        }
    }

    /// Create the mesh for the link
    /// Can't clone the mesh because it will depend on original length
    pub fn create_mesh(&self) -> Mesh {
        Cuboid::new(
            lattice_config::LINK_RADIUS,
            lattice_config::LINK_RADIUS,
            -self.orig_length,
        )
        .mesh()
        .into()
    }
}
