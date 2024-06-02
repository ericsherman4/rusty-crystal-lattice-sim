use bevy::{ecs::query, prelude::*};

#[derive(Component)]
pub struct Link{
    to_node: &Node,
    from_node: &Node,
}

#[derive(Component)]
pub struct Node
{
    pos: Vec3,
    vel: Vec3,
    sum_forces: Vec3,
    
}

#[derive(Component)]
pub struct Static;



pub fn create_spring(
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<StandardMaterial>>
) {

    let sphere_rad:f32  = 0.3; 
    let link_girth: f32 = 0.2;
    let pos_node1 = Vec3::new(0., 0., 5.);
    let pos_node2 = Vec3::new(5., 0., 5.);
    let link = Vec3::new(2.5, 0., 5.); //placed at center

    let node1 = PbrBundle {
        mesh : meshes.add(Sphere::new(sphere_rad).mesh().uv(32,18)),
        material: materials.add(Color::YELLOW),
        transform: Transform::from_translation(pos_node1),
        ..default()
    };

    let node2 = PbrBundle {
        mesh : meshes.add(Sphere::new(sphere_rad).mesh().uv(32,18)),
        material: materials.add(Color::YELLOW),
        transform: Transform::from_translation(pos_node2),
        ..default()
    };

    let node1_data = Node {
        pos : pos_node1,
        vel: Vec3::new(0.,0.0,0.),
        sum_forces: Vec3::ZERO,
    };

    let node2_data = Node {
        pos : pos_node2,
        vel: Vec3::new(0.1,0.1,0.1),
        sum_forces: Vec3::ZERO,
    };

    // Create two nodes separated apart on the x axis
    commands.spawn((node1, node1_data, Static));
    commands.spawn((node2, node2_data));

    // create a skinny box between them
    let link = PbrBundle { 
        mesh: meshes.add(Cuboid::from_size(Vec3::new(5.0, link_girth, link_girth))),
        material: materials.add(Color::YELLOW),
        transform: Transform::from_translation(link),
        ..default()
    };

    commands.spawn((link, Link{to_node: &node1_data, from_node: &node2_data}));


}

// lets start by fixing node1 and only doing node2 
// F = -k(delta X (from nominal))
// a = F /m
// v = v_i + a * dt
// x = x_i + v * dt
pub fn animate_spring(time: Res<Time>, mut query: Query<&mut Transform, With<Node>>) {
    

    
    for mut transform in &mut query {

    }

}