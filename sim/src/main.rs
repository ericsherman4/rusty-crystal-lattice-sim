use bevy::prelude::*;
use smooth_bevy_cameras::controllers::unreal::{UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin};
use smooth_bevy_cameras::{LookTransformPlugin};

fn main() {
    App::new()
        // plugins are pretty cool and enforces modular. if you don't want it, just remove it!
        .add_plugins((DefaultPlugins,LookTransformPlugin ,UnrealCameraPlugin::default()))
        .add_systems(Startup, setup)
        // .add_systems(Update, smooth_bevy_cameras::controllers::unreal::control_system)
        // .add_systems(Update, move_camera_system)
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

    // 
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

    // let bevy_camera = Camera3dBundle {
    //     projection: PerspectiveProjection {
    //         ..default()
    //     }.into(),
    //     // looking at is how to orient
    //     // y is up in bevy
    //     transform: Transform::from_xyz(-6.0, 10.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // };


    // let look_transform = LookTransformBundle {
    //     transform: LookTransform::new(Vec3::new(-6.0, 10.0, 9.0), Vec3::default(), Vec3::Y),
    //     smoother: Smoother::new(0.9), // Value between 0.0 and 1.0, higher is smoother.
    // };

    let unreal_camera = UnrealCameraBundle::new(
        UnrealCameraController::default(),
        Vec3::new(-2.0, 5.0, 5.0),
        Vec3::new(0., 0., 0.),
        Vec3::Y,
    ); 


    commands.spawn(Camera3dBundle::default()).insert(unreal_camera);

}

// fn move_camera_system(mut cameras: Query<&mut LookTransform>) {
//     // Later, another system will update the `Transform` and apply smoothing automatically.
//     for mut c in cameras.iter_mut() { c.target += Vec3::new(0.001, 0.001, 0.001); }
// }
