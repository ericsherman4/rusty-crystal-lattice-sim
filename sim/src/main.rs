use bevy::prelude::*;
use std::time::Duration;

mod scene;
mod spring;

// https://docs.rs/smooth-bevy-cameras/0.11.0/smooth_bevy_cameras/
// https://github.com/bonsairobo/smooth-bevy-cameras/blob/main/examples/simple_unreal.rs
use smooth_bevy_cameras::controllers::unreal::UnrealCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;











fn main() {
    App::new()
        // Add plugins
        .add_plugins((
            DefaultPlugins,
            LookTransformPlugin,
            UnrealCameraPlugin::default(),
        ))
        // Setup camera, lighting, and insert a spring
        .add_systems(Startup, (scene::setup, insert_spring))
        // Update the spring every 1500ms
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(1500)))
        .add_systems(FixedUpdate, update_spring)
        // Run the app
        .run()
}

















/// Insert a spring into the scene
fn insert_spring(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spring::create_spring(&mut commands, &mut meshes, &mut materials);
    println!("Time is {}", time.elapsed().as_secs())
}

use spring::Node;
use spring::Link;








fn update_spring(
    mut links: Query<(&mut Transform, &Link)>,
    mut nodes: Query<(&mut Transform), (With<Node>, Without<Link>)>,
) {
    let mut rng = rand::thread_rng();

    for (mut link_transform, link) in links.iter_mut() {
        let new_node1_pos = spring::create_random_vector(&mut rng);
        let new_node2_pos = spring::create_random_vector(&mut rng);

        let mut node1 = nodes.get_mut(link.to).expect("Node should exist but doesn't");
        node1.translation = new_node1_pos;

        let mut node2 = nodes.get_mut(link.from).expect("Node should exist but doesn't");
        node2.translation = new_node2_pos;

        let dir = new_node1_pos - new_node2_pos;
        let res = dir.normalize() * (dir.length() / 2.0) + new_node2_pos;
        link_transform.translation = res;

        let fwd = link_transform.forward().xyz();
        link_transform.rotate(Quat::from_rotation_arc(fwd, dir.normalize()));
    }
}


