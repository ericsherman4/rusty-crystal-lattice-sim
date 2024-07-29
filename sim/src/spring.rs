use bevy::{ecs::query, prelude::*};

#[derive(Component)]
pub struct Node
{
    pos: Vec3,
    vel: Vec3,
    sum_forces: Vec3,
    mesh: Mesh,
}

impl Node {
    fn new(id: u32,pos: Vec3, vel: Vec3, rad: f32) -> Self {
        Node {
            id,
            pos,
            vel,
            sum_forces : Vec3::ZERO,
            mesh: Sphere::new(rad).mesh().uv(32,18),
        }
    }
}

#[derive(Component)]
pub struct Link<'a>{
    // don't think we need a pos as the pos is determined by the 2 nodes.
    spring_const: f32,
    orig_length: f32,
    to_node: &'a Node,
    from_node: &'a Node,
    mesh: Mesh, 
}

impl<'a> Link<'a>{
    fn new(spring_const: f32, orig_length: f32, girth: f32, to_node_id : &'a Node, from_node_id: &'a Node ) -> Self {
        // function returns an instance of Link
        // When function names are the same as field names, don't need to type it twice.
        Link {
            spring_const,
            orig_length,
            to_node,
            from_node,

            // we shall use x as the length and then we will functions to rotate and move accordingly.
            mesh : Cuboid::new(orig_length, girth, girth).mesh(),
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


pub fn create_spring(
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    let sphere_rad:f32  = 0.3; 
    let pos_node1 = Vec3::new(0., 0., 5.); 
    let pos_node2 = Vec3::new(5., 0., 5.);
    let vel_node1 = Vec3::new(0.,0.0,0.); 
    let vel_node2 = Vec3::new(0.1,0.1,0.1);

    let node1 = Node::new(0,pos_node1, vel_node1, sphere_rad);
    let node2 = Node::new(1,pos_node2, vel_node2, sphere_rad);

    let link = Link::new(2., 5., 0.2, &node2, &node1);

    // i create a node and then give a reference to it to link but after this function, node is dropped. or rather
    // its ownership is transferred to bundled.

    // https://github.com/bevyengine/bevy/discussions/13309
    // https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html
    // in above link, components must always satisfy 'static trait bound.

    // ECS = entity component and system.
    // this creates an entity i.e object with the given components.
    // entities are just unique IDs or references that point to the components. 
    // see this video, think you might be going at this from an OOP perspective and not an ECS perspective. 
    // https://www.youtube.com/watch?v=B6ZFuYYZCSY
    // DATA in COMPONENTs, BEHAVIOR in SYSTEMs.
    // there is some kind of id function and maybe we just loop through and check the ID of the node.
    // seems super slow though but apparently the .get method has constant time complexity.
    // https://gamedev.stackexchange.com/questions/204007/in-bevy-ecs-what-is-a-good-way-to-have-entities-reference-each-other

    commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(node1.mesh.clone()), // not ideal
                material: materials.add(Color::YELLOW),
                transform: Transform::from_translation(node1.pos.clone()),
                ..default()
            },
            node1 
        )
    );

    commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(node2.mesh.clone()), 
                material: materials.add(Color::YELLOW),
                transform: Transform::from_translation(node2.pos.clone()),
                ..default()
            },
            node2
        )
    );

    commands.spawn(
        (
            PbrBundle { 
                mesh: meshes.add(link.mesh.clone()),
                material: materials.add(Color::YELLOW),
                transform: Transform::from_translation(node2.pos - node1.pos),
                ..default()
            },
            link
        )
    );


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