use crate::config::{colors_config, lattice_config};
use crate::resources::RandomSource;
use bevy::prelude::*;
use rand::distributions::Uniform;
use rand::Rng;

//////////////////////////////////////////////////
/// NODE
//////////////////////////////////////////////////

#[derive(Component)]
pub struct Node {
    pos: Vec3,        // meters
    vel: Vec3,        // meters/sec
    sum_forces: Vec3, //newtons
    mass: f32,        // kg
}

impl Node {
    /// Create the mesh for the node
    fn create_mesh(&self) -> Mesh {
        Sphere::new(lattice_config::NODE_RADIUS).mesh().uv(32, 18)
    }
}

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

//////////////////////////////////////////////////
/// LINK
//////////////////////////////////////////////////

#[derive(Component)]
pub struct Link {
    // Links are massless.
    // Using link / spring interchangably throughout the code
    spring_const: f32,
    orig_length: f32, //TODO: maybe change to int so that all the scaling math is easier?
    pub to: Entity,
    pub from: Entity,
}

impl Link {
    /// Create a new link.
    /// From denotes from which node the link is connected and
    /// to denotes to which node the link is connected
    fn new(spring_const: f32, orig_length: f32, to: Entity, from: Entity) -> Self {
        // function returns an instance of Link
        // When function names are the same as field names, don't need to type it twice.
        Link {
            spring_const,
            orig_length,
            to,
            from,
        }
    }

    /// Create the mesh for the link
    fn create_mesh(&self) -> Mesh {
        Cuboid::new(
            lattice_config::LINK_RADIUS,
            lattice_config::LINK_RADIUS,
            -self.orig_length,
        )
        .mesh()
        .into()
    }
}

//////////////////////////////////////////////////
/// LATTICE
//////////////////////////////////////////////////

struct LatticeNodes {
    dim: u32,
    data: Vec<Entity>,
}

impl LatticeNodes {
    // this has fucked me up a lot.
    // if you pass in self, then when u call this function, ownership will be transferred?
    // so u need the arg to be reference or mutable reference

    /// Creates a data struct that holds all of the nodes so that during lattice
    /// generation, links can query them.
    /// Dim is the number of "1x1x1 cubes" on one side of the cube lattice.
    fn new(dim: u32) -> Self {
        let total_nodes = calc_num_lattice_nodes(dim) as usize;
        Self {
            // all internal math is based on num nodes on one side
            // so dim + 1
            dim: dim + 1,
            data: Vec::with_capacity(total_nodes),
        }
    }

    /// Get the data from the array given xyz index in the lattice.
    fn get_data_idx(&self, x: u32, y: u32, z: u32) -> usize {
        (z * self.dim * self.dim + y * self.dim + x) as usize
    }

    /// Get the entity given the xyz index in the lattice.
    fn get(&mut self, UVec3 { x, y, z }: UVec3) -> Entity {
        debug_assert!(x <= (self.dim + 1));
        debug_assert!(y <= (self.dim + 1));
        debug_assert!(z <= (self.dim + 1));
        // println!("accessing {} {} {}", x,y,z);
        let idx = self.get_data_idx(x, y, z);
        self.data[idx]
    }

    /// Add a node to the lattice.
    fn add(&mut self, node: Entity) {
        self.data.push(node);
    }
}

//////////////////////////////////////////////////
/// HELPER FUNCTIONS
//////////////////////////////////////////////////

/// Get number of nodes in a lattice given dim.
/// Dim is the number of "1x1x1 cubes" on one side of the cube lattice.
fn calc_num_lattice_nodes(dim: u32) -> u32 {
    debug_assert!(dim > 0);
    match u32::checked_pow(dim + 1, 3) {
        None => panic!("overflow while calculating number of lattice nodes"),
        Some(val) => val,
    }
}

/// Get number of links in a lattice given dim.
/// Dim is the number of "1x1x1 cubes" on one side of the cube lattice.
fn calc_num_lattice_links(dim: u32) -> u32 {
    debug_assert!(dim > 0);
    match u32::checked_mul(3 * dim * (dim + 1), 3 * dim + 1) {
        None => panic!("overflow while calculating number of lattice links"),
        Some(val) => val,
    }
}

// TODO: maybe this function could just be absorbed into lattice nodes and
// then it just returns an object to generate_lattice.
/// Spawn all of the nodes into the environment and store them
/// in the lattice nodes struct
fn create_all_nodes(
    lattice_nodes: &mut LatticeNodes,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    random_source: &mut ResMut<RandomSource>,
) {
    const NODES_DIM: u32 = lattice_config::DIM + 1;

    // think you can only do this because there is one resource inserted?
    // A: There can only be one type of a resource inserted. Otherwise, need to use ECS
    let rng = &mut random_source.0;
    let dist: Uniform<f32> =
        Uniform::new_inclusive(lattice_config::START_VEL_MIN, lattice_config::START_VEL_MAX);

    println!("Entering for loop");
    for z in 0..NODES_DIM {
        for y in 0..NODES_DIM {
            for x in 0..NODES_DIM {
                let starting_pos = Vec3::new(
                    x as f32 * lattice_config::STARTING_LINK_LEN,
                    y as f32 * lattice_config::STARTING_LINK_LEN,
                    z as f32 * lattice_config::STARTING_LINK_LEN,
                );

                let starting_vel = Vec3::new(rng.sample(dist), rng.sample(dist), rng.sample(dist));

                println!("Staring vel {}", starting_vel);

                let node = Node {
                    pos: starting_pos,
                    vel: starting_vel,
                    ..default()
                };
                lattice_nodes.add(
                    commands
                        .spawn((
                            PbrBundle {
                                mesh: meshes.add(node.create_mesh()), // not ideal
                                material: materials.add(colors_config::NODE_COLOR),
                                transform: Transform::from_translation(node.pos),
                                ..default()
                            },
                            node,
                        ))
                        .id(),
                );
            }
        }
    }

    debug_assert_eq!(
        (calc_num_lattice_nodes(lattice_config::DIM)) as usize,
        lattice_nodes.data.len()
    );
}

//////////////////////////////////////////////////
/// SPRING SYSTEMS
//////////////////////////////////////////////////

pub fn generate_lattice(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut random_source: ResMut<RandomSource>,
) {
    println!("Generating lattice");
    // Turns out, you don't need all the directions cause you
    // are only constructing the cube in one direction.
    // This gets rid of of the duplication problem.
    let dir_arr = [
        IVec3::new(1, 0, 0),
        IVec3::new(0, 1, 0),
        IVec3::new(0, 0, 1),
        IVec3::new(1, 1, 0),
        IVec3::new(0, 1, 1),
        IVec3::new(1, 0, 1),
        IVec3::new(1, -1, 0),
        IVec3::new(0, 1, -1),
        IVec3::new(-1, 0, 1),
    ];

    // Generate all of the nodes first
    let mut node_data = LatticeNodes::new(lattice_config::DIM);
    create_all_nodes(
        &mut node_data,
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut random_source,
    );

    // fill out and spawn all links
    let nodes_dim = lattice_config::DIM + 1;
    let mut counter: u32 = 0;
    for z in 0..nodes_dim {
        for y in 0..nodes_dim {
            for x in 0..nodes_dim {
                // need to check 18 directions, jk 9 directions
                // (because duplicates while building lattice)
                for dir in dir_arr {
                    // TODO: this u32 and i32 conversions are messy
                    let curr_node_pos = UVec3 { x, y, z };
                    let to_node_pos = curr_node_pos.as_ivec3() + dir;

                    if to_node_pos.x < 0
                        || to_node_pos.x >= nodes_dim as i32
                        || to_node_pos.y < 0
                        || to_node_pos.y >= nodes_dim as i32
                        || to_node_pos.z < 0
                        || to_node_pos.z >= nodes_dim as i32
                    {
                        continue;
                    }

                    // Get the entities for both nodes
                    let to_node = node_data.get(to_node_pos.as_uvec3());
                    let from_node = node_data.get(curr_node_pos);

                    // Determine the length of the spring
                    let length = (to_node_pos.as_vec3() * lattice_config::STARTING_LINK_LEN
                        - curr_node_pos.as_vec3() * lattice_config::STARTING_LINK_LEN)
                        .length();
                    println!("{length}");

                    // Create a new Link / Spring
                    let link = Link::new(lattice_config::SPRING_CONST, length, to_node, from_node);
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(link.create_mesh()),
                            material: materials.add(colors_config::SPRING_COLOR),
                            transform: Transform::from_translation(
                                // This is left as is instead of feeding length because
                                // node_data stores it is a 1x1x1 cube internally so it can use indexes directly
                                // to access the node data.
                                curr_node_pos.as_vec3() * lattice_config::STARTING_LINK_LEN,
                            ),
                            visibility: Visibility::Visible,
                            ..default()
                        },
                        link,
                    ));

                    counter += 1; // update counter and make sure we get all the links
                }
            }
        }
    }

    let num_links = calc_num_lattice_links(lattice_config::DIM);
    println!("number of springs generated is {counter} and expected was {num_links}",);
    debug_assert_eq!(counter, num_links);
}

// to start the springs are not compressed at all and should not exert any forces.
// we initialize the sum forces on Node to zero.
// the nodes are moving though.
// the nodes moving cause the spring to resize
// when the springs resize, they are exerting force on the nodes
// TODO: implement them separetly and then curious the performance boost when you combine them

/// Update the state of the nodes and their positions
pub fn update_nodes_state(time: Res<Time>, mut query: Query<(&mut Node, &mut Transform)>) {
    let delta_t = time.delta_seconds();
    // println!("Elasped time is {}", delta_t);

    for (mut node, mut transform) in &mut query {
        // update vel and pos
        let acc = node.sum_forces / node.mass;
        node.vel = node.vel + acc * delta_t;
        node.pos = node.pos + node.vel * delta_t;

        transform.translation = node.pos;

        // zero out sum forces
        node.sum_forces = Vec3::ZERO;
    }
}

/// Update spring phyiscs
pub fn update_link_physics(links: Query<&Link>, mut nodes: Query<(&mut Node, &mut Transform)>) {
    for link in &links {
        // let (mut node_from,mut node_to) = match (nodes.get_mut(link.from), nodes.get_mut(link.to)) {
        //     (Ok(n1), Ok(n2)) => (n1, n2),
        //     _ => panic!("Unable to get one of the nodes")
        // };

        //TODO: clean up where internal position vs transform is used.

        let node_from = match nodes.get(link.from) {
            Ok(node) => node,
            Err(error) => panic!("Unable to get the node {error:?}"),
        };
        let node_to = match nodes.get(link.to) {
            Ok(node) => node,
            Err(error) => panic!("Unable to get the node {error:?}"),
        };

        let node_to_pos = node_to.0.pos;
        let node_from_pos = node_from.0.pos;
        let length = (node_to_pos - node_from_pos).length();
        let spring_displacement = length - link.orig_length;
        // positive -> spring is expanded
        // negative -> spring is contracted
        let force = -1. * link.spring_const * spring_displacement;

        let mut node_from_mut = match nodes.get_mut(link.from) {
            Ok(node) => node,
            Err(error) => panic!("Unable to get the node {error:?}"),
        };

        // this force is applied in the axis colinear from node 1 to node 2
        node_from_mut.0.sum_forces += force / 2.0 * (node_from_pos - node_to_pos).normalize();

        let mut node_to_mut = match nodes.get_mut(link.to) {
            Ok(node) => node,
            Err(error) => panic!("Unable to get the node {error:?}"),
        };

        node_to_mut.0.sum_forces += force / 2.0 * (node_to_pos - node_from_pos).normalize();
    }
}

/// Update the springs
pub fn update_spring(
    mut links: Query<(&Link, &mut Transform), Without<Node>>,
    nodes: Query<&mut Transform, With<Node>>,
) {
    for (link, mut transform) in &mut links {
        // expect and unwrap not encouraged to use, see
        // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
        let node1 = nodes
            .get(link.to)
            .expect("The node should exist as no nodes are ever despawned.");
        let node2 = nodes
            .get(link.from)
            .expect("The node should exist as no nodes are ever despawned.");

        // TODO: separate function?
        // all of this is updating the position and angle of the spring
        let dir = node1.translation - node2.translation;
        let length = dir.length();
        let res = dir.normalize() * (length / 2.) + node2.translation;
        // apply transform to the link
        transform.translation = res;
        let fwd = transform.forward().xyz().normalize();
        //TODO: replace with the other function now that you upgraded engine version
        transform.rotate(Quat::from_rotation_arc(fwd, dir.normalize()));
        // scale the link so that it connects the nodes
        transform.scale.z = length / link.orig_length;
    }
}
