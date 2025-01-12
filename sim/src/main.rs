use bevy::prelude::*;
use std::time::Duration;

mod scene;
mod spring;
use spring::Node;
use spring::Link;

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
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(20)))
        .add_systems(FixedUpdate, update_spring)
        // Run the app
        .run()
}



























fn update_spring(
    mut links: Query<(&mut Transform, &Link)>,
    mut nodes: Query<(&mut Transform), (With<Node>, Without<Link>)>,
    time: Res<Time>,
) {
    for (mut link_transform, link) in links.iter_mut() {
        // Create new positions
        let sin_position = 3.0* f32::sin(time.elapsed_seconds()) + 3.0;
        let new_node1_pos = Vec3::new(sin_position, 1.0, 0.0);
        let new_node2_pos = Vec3::new(0.0, 1.0, sin_position);

        // Update the springs' attached nodes with the new positions
        let mut node1 = nodes.get_mut(link.to).expect("Node should exist but doesn't");
        node1.translation = new_node1_pos;
        let mut node2 = nodes.get_mut(link.from).expect("Node should exist but doesn't");
        node2.translation = new_node2_pos;

        // Determine the link's new position, rotation, scale
        let direction = new_node1_pos - new_node2_pos;
        let res = direction.normalize() * (direction.length() / 2.0) + new_node2_pos;
        link_transform.translation = res;

        let current_spring_direction = link_transform.forward().xyz();
        link_transform.rotate(Quat::from_rotation_arc(current_spring_direction, direction.normalize()));
        link_transform.scale.z = direction.length() / link.orig_length;
    }
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


