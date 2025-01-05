#![allow(warnings)]

use bevy::{ecs::query, prelude::*};
use rand::prelude::*;

#[derive(Component)]
pub struct Node {
    pos: Vec3,
    vel: Vec3,
    sum_forces: Vec3,
    mesh: Mesh,
}

#[derive(Component)]
pub struct Link {
    spring_const: f32,
    orig_length: f32,
    pub to: Entity,
    pub from: Entity,
    mesh: Mesh,
}

impl Node {
    fn new(pos: Vec3, vel: Vec3, rad: f32) -> Self {
        Node {
            pos,
            vel,
            sum_forces: Vec3::ZERO,
            mesh: Sphere::new(rad).mesh().uv(32, 18),
        }
    }
}

impl Link {
    // from denotes from which node the link is connected and
    // to denotes to which node the link is connected
    fn new(spring_const: f32, orig_length: f32, girth: f32, to: Entity, from: Entity) -> Self {
        // function returns an instance of Link
        // When function names are the same as field names, don't need to type it twice.
        Link {
            spring_const,
            orig_length,
            to,
            from,
            // we shall use x as the length and then we will write functions to rotate and move accordingly.
            mesh: Cuboid::new(girth, girth, -orig_length).mesh(),
        }
    }
}

// maybe can make a custom mesh to solve this but this is easier
// https://www.christopherbiscardi.com/why-do-bevy-sprites-spawn-with-the-center-at-0-0
// fn set_cuboid_pos_by_end(target_position: Vec3, length_in_dir: f32, dir: CuboidOffsetDir) -> Vec3
// {
//     // By default, it sets it based on origin.
//     // Returns a modified transform that will place the cuboid correctly.
//     match dir {
//         CuboidOffsetDir::Y => Vec3::new(target_position.x, target_position.y + length_in_dir/2.0, target_position.z),
//         _ => Vec3::default(),
//     }
// }

pub fn create_random_vector(rng: &mut ThreadRng) -> Vec3 {
    Vec3 {
        x: rng.gen_range(0.0..10.0),
        y: rng.gen_range(0.0..10.0),
        z: rng.gen_range(0.0..10.0),
    }
}

pub fn create_spring(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Get two position vectors
    let mut rng = rand::thread_rng();
    let node1_pos = create_random_vector(&mut rng);
    let node2_pos = create_random_vector(&mut rng);

    // Create node components
    let sphere_rad: f32 = 0.3;
    let node1 = Node::new(node1_pos, Vec3::ZERO, sphere_rad);
    let node2 = Node::new(node2_pos, Vec3::ZERO, sphere_rad);

    // Spawn nodes
    let node1_ent = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(node1.mesh.clone()), // not ideal
                material: materials.add(Color::YELLOW),
                transform: Transform::from_translation(node1.pos.clone()),
                ..default()
            },
            node1,
        ))
        .id();

    let node2_ent = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(node2.mesh.clone()),
                material: materials.add(Color::YELLOW),
                transform: Transform::from_translation(node2.pos.clone()),
                ..default()
            },
            node2,
        ))
        .id();

    let dir = node1_pos - node2_pos;
    let link = Link::new(2., dir.length(), 0.2, node1_ent, node2_ent);

    // Link origin is at the center of the mesh
    let center_pos = dir.normalize() * (dir.length() / 2.) + node2_pos;
    let mut link_trans = Transform::from_translation(center_pos);

    link_trans.rotate(Quat::from_rotation_arc(
        link_trans.forward().xyz(),
        dir.normalize(),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(link.mesh.clone()),
            material: materials.add(Color::YELLOW),
            transform: link_trans,
            ..default()
        },
        link,
    ));
}

// lets start by fixing node1 and only doing node2
// F = -k(delta X (from nominal))
// a = F /m
// v = v_i + a * dt
// x = x_i + v * dt
// pub fn animate_spring(time: Res<Time>, mut query: Query<&mut Transform, With<Node>>) {

//     for mut transform in &mut query {

//     }

// }
