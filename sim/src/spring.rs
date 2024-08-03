use bevy::{prelude::*};
use rand::prelude::*;


//////////////////////////////////////////////////
/// NODE
//////////////////////////////////////////////////

#[derive(Component)]
pub struct Node
{
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
            sum_forces : Vec3::ZERO,
            mesh: Sphere::new(rad).mesh().uv(32,18),
        }
    }
}

//////////////////////////////////////////////////
/// LINK
//////////////////////////////////////////////////
#[derive(Component)]
pub struct Link{
    spring_const: f32,
    orig_length: f32,
    pub to : Entity,
    pub from: Entity,
    mesh: Mesh, 
}

impl Link{
    // from denotes from which node the link is connected and 
    // to denotes to which node the link is connected
    fn new(spring_const: f32, orig_length: f32, girth: f32, to: Entity, from:Entity ) -> Self {
        // function returns an instance of Link
        // When function names are the same as field names, don't need to type it twice.
        Link {
            spring_const,
            orig_length,
            to,
            from,
            // we shall use x as the length and then we will write functions to rotate and move accordingly.
            mesh : Cuboid::new(girth, girth, -orig_length).mesh(),
        }
    }
}

//////////////////////////////////////////////////
/// HELPER FUNCTIONS
//////////////////////////////////////////////////

fn get_rand_num(rng: &mut ThreadRng) -> f32 {
    rng.gen_range(-10.0..10.0)
}


//////////////////////////////////////////////////
/// SPRING SYSTEMS
//////////////////////////////////////////////////

pub fn create_spring(
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {

    let mut rng = rand::thread_rng();
    
    // some hardcoded parameters for now
    let sphere_rad:f32  = 0.3; 
    let node1_pos = Vec3::new(
        get_rand_num(&mut rng),
        get_rand_num(&mut rng),
        get_rand_num(&mut rng)
    ); 
    let node2_pos = Vec3::new(
        get_rand_num(&mut rng),
        get_rand_num(&mut rng),
        get_rand_num(&mut rng)
    );

    // Create a component
    let node1 = Node::new(node1_pos, Vec3::ZERO, sphere_rad);
    let node2 = Node::new(node2_pos, Vec3::ZERO, sphere_rad);

    let node1_ent = commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(node1.mesh.clone()), // not ideal
                material: materials.add(Color::YELLOW),
                transform: Transform::from_translation(node1.pos.clone()),
                ..default()
            },
            node1 
        )
    ).id();

    let node2_ent = commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(node2.mesh.clone()), 
                material: materials.add(Color::YELLOW),
                transform: Transform::from_translation(node2.pos.clone()),
                ..default()
            },
            node2
        )
    ).id();

    let length = (node1_pos - node2_pos).length();

    let link = Link::new(2., length, 0.2, node1_ent, node2_ent);

    
    
    // Link position is set by the center
    // need to figure out rotation
    // with the nodes in one axis, we can do node2 - node1 + node2
    
    let dir =  node1_pos - node2_pos;
    let length = dir.length();
    let res = dir.normalize()*(length/2.) + node2_pos;

    // let mut link_trans = Transform::from_translation(res);
    // link_trans.rotate(Quat::from_rotation_arc(Vec3::Z, dir));

    // get a rotation matrix to align to vectors. convert to quart. pass in quart.

    let mut link_trans = Transform::from_translation(res);
    // we let the forward axis by the axis that is aligned to the long direction of the rod
    // this is enforced by xyz vector we pass in when making the cuboid which is girth, girth, -length respectively
    link_trans.rotate(Quat::from_rotation_arc(link_trans.forward().xyz(), dir.normalize()));


    // let mat = Mat3 { x_axis: (), y_axis: (), z_axis: () }

    // link_trans.rotate(Quat::from_mat3(mat));




    
    
    println!("diff {:?}, pos1 {:?}, pos2 {:?}", dir, node1_pos, node2_pos);

    commands.spawn(
        (
            PbrBundle { 
                mesh: meshes.add(link.mesh.clone()),
                material: materials.add(Color::YELLOW),
                transform: link_trans,
                ..default()
            },
            link
        )
    );


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
    mut query : Query<(&mut Transform, &Link), Without<Node>>,
    mut nodes: Query<(Entity, &mut Node, &mut Transform),  Without<Link>>
) {

    let mut rng = rand::thread_rng();
    
    for (mut transform, link) in query.iter_mut() {
        let new_node1_pos = Vec3 {x: rng.gen_range(-10.0..10.0), y: rng.gen_range(-10.0..10.0), z: rng.gen_range(-10.0..10.0) };
        let new_node2_pos = Vec3 {x: rng.gen_range(-10.0..10.0), y: rng.gen_range(-10.0..10.0), z: rng.gen_range(-10.0..10.0) };

        let (_,_,mut node1) = nodes.get_mut(link.to).expect("help");
        node1.translation = new_node1_pos;

        let (_,_,mut node2) = nodes.get_mut(link.from).expect("help");
        node2.translation = new_node2_pos;

        let dir =  new_node1_pos - new_node2_pos;
        let length = dir.length();
        let res = dir.normalize()*(length/2.) + new_node2_pos;


        // this applies to link
        transform.translation =  res;
        let fwd = transform.forward().xyz();
        transform.rotate(Quat::from_rotation_arc(fwd, dir.normalize()));

    }
}


