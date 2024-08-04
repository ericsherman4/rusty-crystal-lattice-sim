use bevy::prelude::*;
use rand::prelude::*;

//////////////////////////////////////////////////
/// NODE
//////////////////////////////////////////////////

#[derive(Component)]
pub struct Node {
    pos: Vec3,
    vel: Vec3,
    sum_forces: Vec3,
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

//////////////////////////////////////////////////
/// LINK
//////////////////////////////////////////////////

#[derive(Component)]
pub struct Link {
    spring_const: f32,
    orig_length: f32,
    pub to: Entity,
    pub from: Entity,
    mesh: Mesh,
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

    // cant do this cause when i call this function and pass the result.mesh() to the add function,
    // link ownership is transferred and then I can't use link after. Don't know how to fix.
    fn create_cuboid(self, girth: f32) -> Cuboid {
        Cuboid::new(girth, girth, -self.orig_length)
    }
}

//////////////////////////////////////////////////
/// LATTICE
//////////////////////////////////////////////////

const LATTICE_DIM: u32 = 1;
const LATTICE_STARTING_LINK_LEN: f32 = 3.;
const LATTICE_NODE_RADIUS: f32 = 0.1;
const LATTICE_LINK_RADIUS: f32 = 0.1;


//////////////////////////////////////////////////
/// HELPER FUNCTIONS
//////////////////////////////////////////////////

/// Get a random float number
fn get_rand_num(rng: &mut ThreadRng) -> f32 {
    rng.gen_range(-10.0..10.0)
}

/// Get number of nodes in a lattice given dim. 
/// Dim is the number of "1x1x1 cubes" on one side of the cube lattice.
fn calc_num_lattice_nodes(dim: u32) -> u32{ 
    assert!(dim > 0);
    match u32::checked_pow(dim+1,3) {
        None => panic!("overflow while calculating number of lattice nodes"),
        Some(val) => val
    }
}

fn calc_num_lattice_links(dim: u32) -> u32 {
    assert!(dim > 0);
    match u32::checked_mul(3*dim*(dim + 1), 3*dim+1) {
        None => panic!("overflow while calculating number of lattice nodes"),
        Some(val) => val
    }
}

struct LatticeNodes{
    dim: u32,
    data: Vec<Entity>
}

impl LatticeNodes {
    
    // TODO: 
    // we know at most a lattice node should be used 18 times.
    // if it connected any more than 18..i.e cases where u spawn two links between nodes,
    // then we just say no link. so maybe keep track of a counter for each node, and if its over 18,
    // dont make a link

    fn new(dim: u32) -> Self {
        Self {
            dim,
            data: Vec::with_capacity(dim as usize),
        }
    }

    // this has fucked me up a lot.
    // if you pass in self, then when u call this function, ownership will be transferred?
    // so u need the arg to be reference or mutable reference
    // fn get(&self,x:i32, y:i32, z:i32) -> Entity {
    // you can unpack a struct into components and pass it in as argument
    fn get(&self, UVec3 {x, y, z}: UVec3) -> Entity {
        assert!(x <= (self.dim + 1));
        assert!(y <= (self.dim + 1));
        assert!(z <= (self.dim + 1));
        println!("{} {} {}", x,y,z);
        self.data[(z*self.dim*self.dim + y*self.dim + x) as usize]
    }

    fn add(&mut self, node: Entity) {
        self.data.push(node);
    }
}


//////////////////////////////////////////////////
/// SPRING SYSTEMS
//////////////////////////////////////////////////

pub fn generate_lattice(    
    mut commands: Commands,
    mut meshes:  ResMut<Assets<Mesh>>,
    mut materials:  ResMut<Assets<StandardMaterial>>,
) {
    let num_nodes = calc_num_lattice_nodes(LATTICE_DIM);
    let num_links = calc_num_lattice_links(LATTICE_DIM);
    let nodes_dim = LATTICE_DIM +1;

    // have a 3d array of nodes? 
    // let nodes = [Node::new(Vec3::ZERO, Vec3::ZERO, 1.0); calc_num_lattice_nodes(dim)];
    // let nodes: [Node; num_nodes] = from_fn(|_| Node::new(Vec3::ZERO, Vec3::ZERO, LATTICE_RAD));

    let mut node_data = LatticeNodes::new(LATTICE_DIM);

    // fill out and spawn all the nodes
    for z in 0..nodes_dim {
        for y in 0..nodes_dim {
            for x in 0..nodes_dim {
                // TODO: provide randomized vel.
                // TODO: fix this stupid mesh clone thing
                let starting_pos = Vec3::new(
                    x as f32* LATTICE_STARTING_LINK_LEN, 
                    y as f32 * LATTICE_STARTING_LINK_LEN, 
                    z as f32 * LATTICE_STARTING_LINK_LEN,
                );
                let node1 = Node::new(starting_pos, Vec3::ZERO, LATTICE_NODE_RADIUS);
                node_data.add(
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(node1.mesh.clone()), // not ideal
                            material: materials.add(Color::YELLOW),
                            transform: Transform::from_translation(node1.pos.clone()),
                            ..default()
                        },
                        node1,
                    ))
                    .id()
                );
            }
        }
    }

    let dir_arr = [
        IVec3::new(1, 0, 0),
        IVec3::new(-1, 0, 0),
        IVec3::new(0, 1, 0),
        IVec3::new(0, -1, 0),
        IVec3::new(0, 0, 1),
        IVec3::new(0, 0, -1),
        IVec3::new(1, 1, 0),
        IVec3::new(1, -1, 0),
        IVec3::new(-1, 1, 0),
        IVec3::new(-1, -1, 0),
        IVec3::new(0, 1, 1),
        IVec3::new(0, -1, 1),
        IVec3::new(0, 1, -1),
        IVec3::new(0, -1, -1),
        IVec3::new(1, 0, 1),
        IVec3::new(1, 0, -1),
        IVec3::new(-1, 0, 1),
        IVec3::new(-1, 0, -1),
    ];

    let mut counter: u32 = 0;
    // fill out and spawn all links
    for z in 0..nodes_dim {
        for y in 0..nodes_dim {
            for x in 0..nodes_dim { 
                // need to check 18 directions 
                for dir in dir_arr {
                    
                    // TODO: this u32 and i32 conversions are messy
                    let to_node_pos = IVec3 {
                        x: x as i32 + dir.x,
                        y: y as i32 + dir.y,
                        z: z as i32 + dir.z,
                    };

                    if to_node_pos.x < 0 || to_node_pos.x >= nodes_dim as i32 
                        || to_node_pos.y < 0 || to_node_pos.y >= nodes_dim as i32 
                        || to_node_pos.z < 0 || to_node_pos.z >= nodes_dim as i32
                    {
                        continue
                    }

                    // TODO: broken because I am connecting links twice.
                    println!("getting node {:?}. input dir is {:?}", to_node_pos, dir);
                    let to_node = node_data.get(to_node_pos.as_uvec3());
                    let from_node = node_data.get(UVec3 {x, y, z});

                    let link = Link::new(2., LATTICE_STARTING_LINK_LEN, LATTICE_LINK_RADIUS, to_node, from_node);
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(link.mesh.clone()),
                            material: materials.add(Color::YELLOW),
                            transform: Transform::from_xyz(
                                x as f32* LATTICE_STARTING_LINK_LEN, 
                                y as f32 * LATTICE_STARTING_LINK_LEN, 
                                z as f32 * LATTICE_STARTING_LINK_LEN,
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
    //TODO: put back
    // assert_eq!(counter, num_links);


}

/// Create a spring
pub fn create_spring(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    // some hardcoded parameters for now
    let sphere_rad: f32 = 0.3;
    let node1_pos = Vec3::new(
        get_rand_num(&mut rng),
        get_rand_num(&mut rng),
        get_rand_num(&mut rng),
    );
    let node2_pos = Vec3::new(
        get_rand_num(&mut rng),
        get_rand_num(&mut rng),
        get_rand_num(&mut rng),
    );

    // Create a component
    let node1 = Node::new(node1_pos, Vec3::ZERO, sphere_rad);
    let node2 = Node::new(node2_pos, Vec3::ZERO, sphere_rad);

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

    let length = (node1_pos - node2_pos).length();

    let link = Link::new(2., length, 0.2, node1_ent, node2_ent);

    // Link position is set by the center
    // need to figure out rotation
    // with the nodes in one axis, we can do node2 - node1 + node2

    let dir = node1_pos - node2_pos;
    let length = dir.length();
    let res = dir.normalize() * (length / 2.) + node2_pos;

    // let mut link_trans = Transform::from_translation(res);
    // link_trans.rotate(Quat::from_rotation_arc(Vec3::Z, dir));

    // get a rotation matrix to align to vectors. convert to quart. pass in quart.

    let mut link_trans = Transform::from_translation(res);
    // we let the forward axis by the axis that is aligned to the long direction of the rod
    // this is enforced by xyz vector we pass in when making the cuboid which is girth, girth, -length respectively
    link_trans.rotate(Quat::from_rotation_arc(
        link_trans.forward().xyz(),
        dir.normalize(),
    ));

    // let mat = Mat3 { x_axis: (), y_axis: (), z_axis: () }

    // link_trans.rotate(Quat::from_mat3(mat));

    println!("diff {:?}, pos1 {:?}, pos2 {:?}", dir, node1_pos, node2_pos);

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

/// Insert a spring into the scene
pub fn insert_spring(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_spring(&mut commands, &mut meshes, &mut materials);
    println!("Time is {}", time.elapsed().as_secs())
}

/// Update the springs position
pub fn update_spring(
    mut query: Query<(&mut Transform, &Link), Without<Node>>,
    mut nodes: Query<(Entity, &mut Node, &mut Transform), Without<Link>>,
) {
    let mut rng = rand::thread_rng();

    for (mut transform, link) in query.iter_mut() {
        // let new_node1_pos = Vec3 {
        //     x: rng.gen_range(-10.0..10.0),
        //     y: rng.gen_range(-10.0..10.0),
        //     z: rng.gen_range(-10.0..10.0),
        // };
        // let new_node2_pos = Vec3 {
        //     x: rng.gen_range(-10.0..10.0),
        //     y: rng.gen_range(-10.0..10.0),
        //     z: rng.gen_range(-10.0..10.0),
        // };

        // let (_, _, mut node1) = nodes.get_mut(link.to).expect("help");
        // node1.translation = new_node1_pos;

        // let (_, _, mut node2) = nodes.get_mut(link.from).expect("help");
        // node2.translation = new_node2_pos;

        // let dir = new_node1_pos - new_node2_pos;
        // let length = dir.length();
        // let res = dir.normalize() * (length / 2.) + new_node2_pos;

        let (_, _, node1) = nodes.get(link.to).expect("help");

        let (_, _, node2) = nodes.get(link.from).expect("help");

        let dir = node1.translation - node2.translation;
        let length = dir.length();
        let res = dir.normalize() * (length / 2.) + node2.translation;

        // this applies to link
        transform.translation = res;
        let fwd = transform.forward().xyz();
        transform.rotate(Quat::from_rotation_arc(fwd, dir.normalize()));
    }
}
