use bevy::prelude::*;
use rand::{distributions::Uniform, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::config::{colors_config, lattice_config};

use crate::lattice::components::{Link, Node, Static};

// use crate::lattice::components

//-------------------------------------------------------
// STRUCTS
//-------------------------------------------------------

/// A resource for used for generating random numbers for
/// the lattice's starting velocities
#[derive(Resource)]
pub struct RandomSource(pub ChaCha8Rng);

/// A plugin for the random source
pub struct RandomSourcePlugin;

/// Data structure for holding all of the nodes for lattice generation.
#[derive(Resource)]
pub struct LatticeGen {
    /// Dim is the number of nodes along one side, NOT the number of 1x1x1 cubes along face
    pub nodes_dim: u32,
    /// A 1D array of all the node elements
    pub data: Vec<Entity>,
}

//-------------------------------------------------------
// LATTICE GENERATION FUNCTIONS
//-------------------------------------------------------

/// Get a vector of indices specifying which nodes should receive
/// the static component during generation
fn get_static_node_indices() -> Vec<(u32, u32, u32)> {
    let corner_index = lattice_config::DIM;
    vec![
        (0, 0, 0),
        (corner_index, corner_index, corner_index),
        (corner_index, 0, 0),
        (0, corner_index, 0),
        (0, 0, corner_index),
        (corner_index, corner_index, 0),
        (corner_index, 0, corner_index),
        (0, corner_index, corner_index),
    ]
}

/// Check if the link end position is out of bounds of the cube
fn link_out_of_bounds(vec: IVec3, bounds: i32) -> bool {
    return vec.x < 0
        || vec.x >= bounds
        || vec.y < 0
        || vec.y >= bounds
        || vec.z < 0
        || vec.z >= bounds;
}

/// Spawn all nodes into the world
pub fn create_all_nodes(
    mut lattice_gen: ResMut<LatticeGen>,
    mut rng_source: ResMut<RandomSource>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a uniform distribution for random numbers
    let rng = &mut rng_source.0;
    // TODO: want to move the following as a resource function, but confused on the syntax,
    // see https://doc.rust-lang.org/std/keyword.struct.html
    let dist: Uniform<f32> =
        Uniform::new_inclusive(lattice_config::START_VEL_MIN, lattice_config::START_VEL_MAX);

    // Get indexes for which nodes should be static
    let corners = get_static_node_indices();

    // Define some variables for generating all the nodes
    const NODES_DIM: u32 = lattice_config::DIM + 1;
    let node_mesh = Sphere::new(lattice_config::NODE_RADIUS).mesh().uv(32, 18);

    // Generate all nodes
    for z in 0..NODES_DIM {
        for y in 0..NODES_DIM {
            for x in 0..NODES_DIM {
                let starting_pos =
                    Vec3::new(x as f32, y as f32, z as f32) * lattice_config::STARTING_LINK_LEN;
                let starting_vel = Vec3::new(rng.sample(dist), rng.sample(dist), rng.sample(dist));

                let node = Node {
                    pos: starting_pos,
                    vel: starting_vel,
                    ..default()
                };

                let bundle = PbrBundle {
                    mesh: meshes.add(node_mesh.clone()),
                    material: materials.add(colors_config::NODE_COLOR),
                    transform: Transform::from_translation(node.pos),
                    ..default()
                };

                // Check if it's a corner node and anchor it by spawning it with the static component.
                if corners.contains(&(x, y, z)) {
                    lattice_gen.add(commands.spawn((bundle, node, Static)).id());
                } else {
                    lattice_gen.add(commands.spawn((bundle, node)).id());
                }
            }
        }
    }

    println!("Number of lattice nodes is {}", lattice_gen.data.len());
    debug_assert_eq!(
        (calc_num_nodes(lattice_config::DIM)) as usize,
        lattice_gen.data.len()
    );
}

pub fn generate_lattice(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    lattice_gen: Res<LatticeGen>,
) {
    println!("Generating Lattice");

    // Turns out, you don't need all the directions cause you
    // are only constructing the lattice in one direction.
    // This gets rid of the duplication problem.
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

    let nodes_dim: i32 = lattice_config::DIM as i32 + 1;
    let mut counter: u32 = 0;

    // Fill out and spawn all links
    for z in 0..nodes_dim {
        for y in 0..nodes_dim {
            for x in 0..nodes_dim {
                for dir in dir_arr {
                    // Get the two and from position of the nodes
                    let curr_node_pos = IVec3 { x, y, z };
                    let to_node_pos = curr_node_pos + dir;

                    // Check if we are out of bounds
                    if link_out_of_bounds(to_node_pos, nodes_dim) {
                        continue;
                    }

                    // Get the entities for both nodes
                    let to_node = lattice_gen.get(to_node_pos.as_uvec3());
                    let from_node = lattice_gen.get(curr_node_pos.as_uvec3());

                    // Determine the length of the spring, diagonal springs will not be the same starting length
                    // as horizontal and vertical ones
                    let length = (to_node_pos.as_vec3() - curr_node_pos.as_vec3()).length();

                    // Generate a color that creates a gradient across the cube
                    let position = curr_node_pos.as_vec3() / nodes_dim as f32;
                    let color = Color::srgb(position.x, position.y, position.z);

                    // Create a new Link / Spring and spawn
                    let link = Link::new(lattice_config::SPRING_CONST, length, to_node, from_node);
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(link.create_mesh()),
                            material: materials.add(color),
                            // transform will be corrected once springs positions update
                            transform: Transform::from_translation(Vec3::ZERO),
                            visibility: lattice_config::LINK_VISIBILITY,
                            ..default()
                        },
                        link,
                    ));

                    counter += 1; // update counter and make sure we get all the links
                }
            }
        }
    }

    let num_links = calc_num_links(lattice_config::DIM);
    println!("number of springs generated is {counter} and expected was {num_links}",);
    debug_assert_eq!(counter, num_links);
}

//-------------------------------------------------------
// LatticeGen IMPL
//-------------------------------------------------------

impl LatticeGen {
    /// Create a new lattice data structure for the given dimension
    pub fn new(lattice_dimension: u32) -> Self {
        Self {
            // all internal math is based on num nodes on one side so dim + 1
            nodes_dim: lattice_dimension + 1,
            data: Vec::with_capacity(calc_num_nodes(lattice_dimension) as usize),
        }
    }

    /// Get the data from the array given xyz index in the lattice.
    pub fn get_data_idx(&self, x: u32, y: u32, z: u32) -> usize {
        (z * self.nodes_dim * self.nodes_dim + y * self.nodes_dim + x) as usize
    }

    /// Get the entity given the xyz index in the lattice.
    pub fn get(&self, UVec3 { x, y, z }: UVec3) -> Entity {
        debug_assert!(x <= (self.nodes_dim + 1));
        debug_assert!(y <= (self.nodes_dim + 1));
        debug_assert!(z <= (self.nodes_dim + 1));
        // println!("accessing {} {} {}", x,y,z);
        let idx = self.get_data_idx(x, y, z);
        self.data[idx]
    }

    /// Add a node to the lattice.
    pub fn add(&mut self, node: Entity) {
        self.data.push(node);
    }
}

/// Get number of nodes in a lattice
pub fn calc_num_nodes(lattice_dim: u32) -> u32 {
    debug_assert!(lattice_dim > 0);
    match u32::checked_pow(lattice_dim + 1, 3) {
        None => panic!("overflow while calculating number of lattice nodes"),
        Some(val) => val,
    }
}

/// Get number of links in a lattice given dim.
/// Dim is the number of "1x1x1 cubes" on one side of the cube lattice.
pub fn calc_num_links(lattice_dim: u32) -> u32 {
    debug_assert!(lattice_dim > 0);
    match u32::checked_mul(3 * lattice_dim * (lattice_dim + 1), 3 * lattice_dim + 1) {
        None => panic!("overflow while calculating number of lattice links"),
        Some(val) => val,
    }
}

//-------------------------------------------------------
// RandomSourcePlugin IMPL
//-------------------------------------------------------

impl Plugin for RandomSourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::add_rng);
    }
}

impl RandomSourcePlugin {
    fn add_rng(mut commands: Commands) {
        // https://rust-random.github.io/book/guide-rngs.html
        // TODO: currently deterministic but change to get varying results.
        let keyboard_mashing: u64 = 459347051375372;
        let seeded_rng = ChaCha8Rng::seed_from_u64(keyboard_mashing);
        commands.insert_resource(RandomSource(seeded_rng));
        println!("Successfully added RNG");
    }
}
