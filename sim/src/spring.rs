use bevy::prelude::*;

//////////////////////////////////////////////////
/// CONSTS
//////////////////////////////////////////////////

const LATTICE_DIM: u32 = 3;
const LATTICE_STARTING_LINK_LEN: f32 = 3.;
const LATTICE_NODE_RADIUS: f32 = 0.1;
const LATTICE_LINK_RADIUS: f32 = 0.1;

//////////////////////////////////////////////////
/// NODE
//////////////////////////////////////////////////

#[derive(Component)]
pub struct Node {
    pos: Vec3,
    vel: Vec3,
    sum_forces: Vec3,
}

impl Node {
    fn new(pos: Vec3, vel: Vec3) -> Self {
        Node {
            pos,
            vel,
            sum_forces: Vec3::ZERO,
        }
    }

    /// Create the mesh for the node
    fn create_mesh(&self) -> Mesh {
        Sphere::new(LATTICE_NODE_RADIUS).mesh().uv(32, 18)
    }
}

//////////////////////////////////////////////////
/// LINK
//////////////////////////////////////////////////

#[derive(Component)]
pub struct Link {
    spring_const: f32,
    orig_length: f32,
    pub to: Entity,
    pub from: Entity,
}

impl Link {
    /// Create a new link
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
        Cuboid::new(LATTICE_LINK_RADIUS, LATTICE_LINK_RADIUS, -self.orig_length).mesh()
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
        return self.data[idx];
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
/// in the latticenodes struct
fn create_all_nodes(
    lattice_nodes: &mut LatticeNodes,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let nodes_dim = LATTICE_DIM + 1;

    for z in 0..nodes_dim {
        for y in 0..nodes_dim {
            for x in 0..nodes_dim {
                // TODO: provide randomized vel.
                let starting_pos = Vec3::new(
                    x as f32 * LATTICE_STARTING_LINK_LEN,
                    y as f32 * LATTICE_STARTING_LINK_LEN,
                    z as f32 * LATTICE_STARTING_LINK_LEN,
                );
                let node1 = Node::new(starting_pos, Vec3::ZERO);
                lattice_nodes.add(
                    commands
                        .spawn((
                            PbrBundle {
                                mesh: meshes.add(node1.create_mesh()), // not ideal
                                material: materials.add(Color::WHITE),
                                transform: Transform::from_translation(node1.pos.clone()),
                                ..default()
                            },
                            node1,
                        ))
                        .id(),
                );
            }
        }
    }

    debug_assert_eq!(
        (calc_num_lattice_nodes(LATTICE_DIM)) as usize,
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
) {
    // Turns out, you don't need all the directions cause you
    // are only constructing the cube in one direction.
    // This gets rid of duplicates.
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
    let mut node_data = LatticeNodes::new(LATTICE_DIM);
    create_all_nodes(&mut node_data, &mut commands, &mut meshes, &mut materials);

    // fill out and spawn all links
    let nodes_dim = LATTICE_DIM + 1;
    let mut counter: u32 = 0;
    for z in 0..nodes_dim {
        for y in 0..nodes_dim {
            for x in 0..nodes_dim {
                // need to check 18 directions
                for dir in dir_arr {
                    // TODO: this u32 and i32 conversions are messy
                    let lattice_node_pos = UVec3 { x, y, z };
                    let to_node_pos = lattice_node_pos.as_ivec3() + dir;

                    if to_node_pos.x < 0
                        || to_node_pos.x >= nodes_dim as i32
                        || to_node_pos.y < 0
                        || to_node_pos.y >= nodes_dim as i32
                        || to_node_pos.z < 0
                        || to_node_pos.z >= nodes_dim as i32
                    {
                        continue;
                    }

                    // println!("getting node {:?}. input dir is {:?}. current pos is {:?}", to_node_pos, dir, lattice_node_pos);
                    let to_node = node_data.get(to_node_pos.as_uvec3());
                    let from_node = node_data.get(lattice_node_pos);

                    let link = Link::new(2., LATTICE_STARTING_LINK_LEN, to_node, from_node);
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(link.create_mesh()),
                            material: materials.add(Color::BLUE),
                            transform: Transform::from_translation(
                                lattice_node_pos.as_vec3() * LATTICE_STARTING_LINK_LEN,
                            ),
                            ..default()
                        },
                        link,
                    ));

                    counter += 1; // update counter and make sure we get all the links
                }
            }
        }
    }

    let num_links = calc_num_lattice_links(LATTICE_DIM);
    println!("number of springs generated is {counter} and expected was {num_links}",);
    debug_assert_eq!(counter, num_links);
}

/// Update the springs position
pub fn update_spring(
    mut query: Query<(&mut Transform, &Link), Without<Node>>,
    nodes: Query<(Entity, &mut Node, &mut Transform), Without<Link>>,
) {
    for (mut transform, link) in query.iter_mut() {
        let (_, _, node1) = nodes
            .get(link.to)
            .expect("The node should exist as no nodes are ever despawned.");
        let (_, _, node2) = nodes
            .get(link.from)
            .expect("The node should exist as no nodes are ever despawned.");
        let dir = node1.translation - node2.translation;
        let length = dir.length();
        let res = dir.normalize() * (length / 2.) + node2.translation;

        // this applies to link
        transform.translation = res;
        let fwd = transform.forward().xyz().normalize();
        transform.rotate(Quat::from_rotation_arc(fwd, dir.normalize()));
    }
}
