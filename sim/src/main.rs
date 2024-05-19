use bevy::prelude::*;

// https://docs.rs/smooth-bevy-cameras/0.11.0/smooth_bevy_cameras/
// https://github.com/bonsairobo/smooth-bevy-cameras/blob/main/examples/simple_unreal.rs
use smooth_bevy_cameras::LookTransformPlugin;
use smooth_bevy_cameras::controllers::unreal::{UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin};

fn main() {
    App::new()
        // plugins are pretty cool and enforces modular. if you don't want it, just remove it!
        .add_plugins((DefaultPlugins,LookTransformPlugin ,UnrealCameraPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update,keyboard_input)
        .run()
}


// throw some basic shapes in a 3d environment
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sphere

    // Light
    let point_light = PointLight {
        shadows_enabled: true,
        intensity: 2_000_000.0,
        range: 100.0,
        ..default()
    };

    let point_light_bundle = PointLightBundle {
        point_light: point_light,
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    };

    commands.spawn(point_light_bundle);


    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::RED),
        transform: Transform::from_xyz(2.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..default()
    });

    let bevy_camera = Camera3dBundle {
        projection: PerspectiveProjection {
            ..default()
        }.into(),
        // looking at is how to orient
        // y is up in bevy
        transform: Transform::from_xyz(-6.0, 10.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    let unreal_camera = UnrealCameraBundle::new(
        UnrealCameraController::default(),
        Vec3::new(-6.0, 10.0, 9.0),
        Vec3::new(0., 0., 0.),
        Vec3::Y,
    ); 

    commands.spawn((MyCamera,bevy_camera)).insert(unreal_camera);

}


// this doesnt work idk lol
// not sure if the library gives us the ability to do this
// probably write own in the future and get rid of the unreal camera controls you dont want
fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<MyCamera>>) {
    if keys.just_pressed(KeyCode::Space){
        for mut trans in &mut query {
            println!("reseting camera!");
            *trans = Transform::from_xyz(-6.0, 10.0, 9.0);
        }
    }
}

#[derive(Component)]
struct MyCamera;