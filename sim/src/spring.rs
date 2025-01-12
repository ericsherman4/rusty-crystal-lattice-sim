use crate::config::{colors_config, lattice_config};
use crate::resources::RandomSource;
use bevy::{color, prelude::*};
use rand::{distributions::Uniform, Rng};

//////////////////////////////////////////////////
/// GENERAL SIMULATION STUFF
//////////////////////////////////////////////////

#[derive(Resource)]
pub struct SimulationData {
    kinetic_energy: f32,
    center_of_mass: Transform,
}

impl Default for SimulationData {
    fn default() -> Self {
        SimulationData {
            kinetic_energy: 0.0,
            center_of_mass: Transform::from_translation(Vec3::ZERO),
        }
    }
}

pub fn rotate_around_center(
    data: Res<SimulationData>,
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
){
    // MUST UNLOCK THE CAMERA FOR IT TO WORK

    let mut camera = camera.single_mut();
    
    // let secondary_axis_rot_rate = 0.09* time.delta_seconds();
    let secondary_axis_rot_rate = 0.0;
    let primary_axis_rot_rate = 0.3 * time.delta_seconds();


    if keyboard.pressed(KeyCode::ArrowRight) {
        let rot = Quat::from_euler(EulerRot::YXZ, primary_axis_rot_rate, secondary_axis_rot_rate, secondary_axis_rot_rate);
        camera.rotate_around(data.center_of_mass.translation, rot);
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        let rot = Quat::from_euler(EulerRot::YXZ, -primary_axis_rot_rate, -secondary_axis_rot_rate, -secondary_axis_rot_rate);
        camera.rotate_around(data.center_of_mass.translation, rot);
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        let rot = Quat::from_axis_angle(camera.right().into(), -primary_axis_rot_rate);
        camera.rotate_around(data.center_of_mass.translation, rot);
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        let rot = Quat::from_axis_angle(camera.right().into(), primary_axis_rot_rate);
        camera.rotate_around(data.center_of_mass.translation, rot);
    }
}


//////////////////////////////////////////////////
/// NODE
//////////////////////////////////////////////////

/// If a node is spawned with a static component, it should not move
#[derive(Component)]
pub struct Static;

#[derive(Component)]
pub struct Node {
    pos: Vec3,        // meters
    vel: Vec3,        // meters/sec
    sum_forces: Vec3, //newtons
    mass: f32,        // kg
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

/// Links are massless.
/// Using link / spring interchangably throughout the code
#[derive(Component)]
pub struct Link {
    spring_const: f32,
    delta_spring_length_pre: f32, 
    orig_length: f32,
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
            delta_spring_length_pre: 0.0,
        }
    }

    /// Create the mesh for the link
    /// Can't clone the mesh because it will depend on original length
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


/// Data structure for holding all of the nodes for lattice generation. 
struct LatticeNodes {

    /// Dim is the number of "1x1x1 cubes" on one side of the cube lattice.
    dim: u32,

    /// A 1D array of all the node elements
    data: Vec<Entity>,
}

impl LatticeNodes {
    /// Creates a data struct that holds all of the nodes so that during lattice
    /// generation, links can query them.
    
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

/// Spawn all nodes into the world
fn create_all_nodes(
    lattice_nodes: &mut LatticeNodes,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    random_source: &mut ResMut<RandomSource>,
) {
    // Create a uniform distribution for random numbers
    let rng = &mut random_source.0;
    let dist: Uniform<f32> =
        Uniform::new_inclusive(lattice_config::START_VEL_MIN, lattice_config::START_VEL_MAX);

    // Use array to specify which ones should be static
    let corner_index = lattice_config::DIM;
    let corners = vec![
        (0,0,0),
        (corner_index,corner_index,corner_index),
        (corner_index,0,0),
        (0,corner_index,0),
        (0,0,corner_index),
        (corner_index,corner_index,0),
        (corner_index,0,corner_index),
        (0,corner_index,corner_index),
    ];

    // Define some variables for generating all the nodes
    const NODES_DIM: u32 = lattice_config::DIM + 1;
    let node_mesh = Sphere::new(lattice_config::NODE_RADIUS).mesh().uv(32, 18);
    
    // Generate all nodes
    for z in 0..NODES_DIM {
        for y in 0..NODES_DIM {
            for x in 0..NODES_DIM {
                let starting_pos = Vec3::new(
                    x as f32 * lattice_config::STARTING_LINK_LEN,
                    y as f32 * lattice_config::STARTING_LINK_LEN,
                    z as f32 * lattice_config::STARTING_LINK_LEN,
                );

                let starting_vel = Vec3::new(rng.sample(dist), rng.sample(dist), rng.sample(dist));

                let node = Node {
                    pos: starting_pos,
                    vel: starting_vel,
                    ..default()
                };

                let bundle = PbrBundle {
                    mesh: meshes.add(node_mesh.clone()), // not ideal
                    material: materials.add(colors_config::NODE_COLOR),
                    transform: Transform::from_translation(node.pos),
                    ..default()
                };

                // Check if it's a corner node and anchor it by spawning it with the static component.
                if corners.contains(&(x,y,z)) {
                    lattice_nodes.add(commands.spawn((bundle,node, Static)).id());
                }
                else {  
                    lattice_nodes.add(commands.spawn((bundle,node)).id());
                }
            }
        }
    }

    println!("Number of lattice nodes is {}", lattice_nodes.data.len());
    debug_assert_eq!(
        (calc_num_lattice_nodes(lattice_config::DIM)) as usize,
        lattice_nodes.data.len()
    );
}

fn link_out_of_bounds(vec: IVec3, bounds: i32) -> bool {
    return vec.x < 0 
        || vec.x >= bounds 
        || vec.y < 0                 
        || vec.y >= bounds 
        || vec.z < 0 
        || vec.z >= bounds;
 }


//////////////////////////////////////////////////
/// SPRING SYSTEMS
//////////////////////////////////////////////////

pub fn update_center_of_mass(
    mut data: ResMut<SimulationData>,
    nodes: Query<&Node>,
) {
    let mut accum = Vec3::ZERO;
    let mut count = 0;

    for node in nodes.iter() {
        count += 1;
        accum += node.pos;
    }

    if count == 0 { return; }
    
    data.center_of_mass.translation = accum / count as f32;
    // println!("initial COM is {}", data.center_of_mass.translation);
}



pub fn generate_lattice(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut random_source: ResMut<RandomSource>,
) {
    println!("Generating Lattice");

    // Generate all of the nodes first. They needed to be spawned so that we can 
    // obtain the entity ID and store it in the link
    let mut node_data = LatticeNodes::new(lattice_config::DIM);
    create_all_nodes( &mut node_data, &mut commands, &mut meshes, &mut materials, &mut random_source);
    
    // Turns out, you don't need all the directions cause you
    // are only constructing the lattice in one direction.
    // This gets rid of the duplication problem.
    let dir_arr = [
        IVec3::new(1, 0, 0), IVec3::new(0, 1, 0), IVec3::new(0, 0, 1), 
        IVec3::new(1, 1, 0), IVec3::new(0, 1, 1), IVec3::new(1, 0, 1), 
        IVec3::new(1, -1, 0), IVec3::new(0, 1, -1), IVec3::new(-1, 0, 1)
    ];
   
    let nodes_dim: i32 = (lattice_config::DIM + 1).try_into().expect("Could not fit DIM in i32");
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
                    if link_out_of_bounds(to_node_pos, nodes_dim){
                        continue;
                    }

                    // Get the entities for both nodes
                    let to_node = node_data.get(to_node_pos.as_uvec3());
                    let from_node = node_data.get(curr_node_pos.as_uvec3());

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

    let num_links = calc_num_lattice_links(lattice_config::DIM);
    println!("number of springs generated is {counter} and expected was {num_links}",);
    debug_assert_eq!(counter, num_links);
}




pub fn print_kinetic_energy(
    sim_data: Res<SimulationData>,
) {
    println!("{}", sim_data.kinetic_energy);
}



// to start the springs are not compressed at all and should not exert any forces.
// we initialize the sum forces on Node to zero.
// the nodes are moving though.
// the nodes moving cause the spring to resize
// when the springs resize, they are exerting force on the nodes
// TODO: implement them separetly and then curious the performance boost when you combine them

/// Update the state of the nodes and their positions
pub fn update_nodes_state(
    time: Res<Time>, 
    mut nodes: Query<(&mut Node, &mut Transform), Without<Static>>,
    mut sim_data: ResMut<SimulationData>,
) {
    let delta_t = time.delta_seconds();
    // println!("Elasped time is {}", delta_t);

    let mut total_kinetic_energy = 0.0;

    for (mut node, mut transform) in nodes.iter_mut() {
        // update vel and pos
        let acc = node.sum_forces / node.mass;
        node.vel = node.vel + acc * delta_t;
        node.pos = node.pos + node.vel * delta_t;

        // zero out sum forces
        node.sum_forces = Vec3::ZERO;

        // calculate the total kinetic energy
        let velocity_magnitude = node.vel.length();
        total_kinetic_energy += 0.5 * node.mass * velocity_magnitude * velocity_magnitude;

        // Update the nodes mesh transform
        transform.translation = node.pos;
    }

    sim_data.kinetic_energy = total_kinetic_energy;
}

/// Update spring phyiscs
pub fn update_link_physics(
    time: Res<Time>, 
    mut links: Query<(& mut Link, &Handle<StandardMaterial>)>, 
    mut nodes: Query<(&mut Node, &mut Transform)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let delta_t = time.delta_seconds();
    for (mut link, mut material_handle) in links.iter_mut() {

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
        let force_dir = (node_to_pos - node_from_pos).normalize();
        let length = (node_to_pos - node_from_pos).length();
        let spring_displacement = length - link.orig_length;
        const DAMPING: f32 = 30.0; // 30 at damping and vel at 20 is pretty cool and div spring displament by 5
        // positive -> spring is expanded
        // negative -> spring is contracted

        // velocity of the spring is change of spring displacement over time. v = delta x / delta t
        let velocity = (spring_displacement - link.delta_spring_length_pre)/delta_t;
        let damping_force = -DAMPING * velocity;
        // let damping_force =0.0;
        let total_force = -1. * link.spring_const * spring_displacement + damping_force;
        // let force = -1. * link.spring_const * spring_displacement - (DAMPING * (node_to.0.vel - node_from.0.vel)); // THIS IS WRONG, works in sim but wrong physics wise
        // link.delta_spring_length_pre = spring_displacement/5.0; // results in more appealing simulations
        link.delta_spring_length_pre = spring_displacement/5.0;
        let from_force =  -0.5* total_force * force_dir;
        let to_force = -from_force;

        // https://github.com/bevyengine/bevy/discussions/8487
        let material = materials.get_mut(material_handle).unwrap();
        // Doing it by force the spring exerts
        // if let Some(force_color) = from_force.try_normalize() {
        //     let scaled_vec = force_color * 0.5 + Vec3::splat(0.5);
        //     material.base_color = Color::srgb(scaled_vec.x, scaled_vec.y, scaled_vec.z);
        // }
        // Doing it by the velocity. Apply the gain to get the colors to show more when velocity is low
        // let normalized_vel = f32::abs(velocity) / 255.0  * 40.0;
        // material.base_color = Color::srgb(normalized_vel, 0.0,0.0);
        

        let mut node_from_mut = match nodes.get_mut(link.from) {
            Ok(node) => node,
            Err(error) => panic!("Unable to get the node {error:?}"),
        };

        // this force is applied in the axis colinear from node 1 to node 2
        node_from_mut.0.sum_forces += from_force; //- to_force;

        let mut node_to_mut = match nodes.get_mut(link.to) {
            Ok(node) => node,
            Err(error) => panic!("Unable to get the node {error:?}"),
        };

        node_to_mut.0.sum_forces += to_force; //- from_force;
    }
}

/// Update the springs
pub fn update_spring(
    mut links: Query<(&Link, &mut Transform), Without<Node>>,
    nodes: Query<&Node>,
) {
    for (link, mut transform) in links.iter_mut() {
        // expect and unwrap not encouraged to use, see
        // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
        let node_to = nodes
            .get(link.to)
            .expect("The node should exist as no nodes are ever despawned.");
        let node_from = nodes
            .get(link.from)
            .expect("The node should exist as no nodes are ever despawned.");

        // update position and length of the spring
        // USE POS, NOT THE TRANSFORM OF THE SPRING
        // this is because the transform only updates every frame where as pos updates every fixedtimestep
        let dir = node_to.pos - node_from.pos;
        transform.translation = dir / 2.  + node_from.pos;
        transform.scale.z = dir.length() / link.orig_length;
        
        // rotate the spring so it aligns with the direction vector between the two nodes (node_to - node_from)
        // best attempt at explaining
        // This fixes my dir3::new_unchecked is not normalized error
        // it tries to get the main axis to look at main direction and secondary axis to look secondary direction.
        // in my case, I only care about main axis looking at main direction but this means there's infinite solutions
        // so main axis is Z. Z is the one I am scaling as well to get it to stretch between the two nodes.
        // I want Z to point to the vector from node_from to node_to or the dir vector
        // The secondary axis - just using Y, I assume X would work, I want to point to a vector orthogonal to the dir vector.
        // I get this vector by crossing the direction vector node_from vector.
        *transform = transform.aligned_by(Vec3::Z, dir, Vec3::Y, dir.cross(node_from.pos));
        // okay the cross thing is nice but i think that was causing a lot of them to spin which makes sense
        // because dir cross node_from trans could point in any outward direction really.
        // *transform = transform.aligned_by(Vec3::Z, dir, Vec3::Y, Vec3::splat(100.0));  //this seems to cause the sticks to glitch occasionally.
        // Update, because i dont care about secondary axis, applied what I learned from above to make looking_at work.
        // same principals apply, I want to look at the node named node_to and I want a vector orthogonal ot the dir vector.
        // *transform = transform.looking_at(node_to.pos,  dir.cross(node_from.pos)) // this also has the spinning problem

        // note that the following doesnt work but produces awesome results
        // *transform = transform.looking_to(node_to.pos,  dir.cross(node_from.pos))
    }
}
