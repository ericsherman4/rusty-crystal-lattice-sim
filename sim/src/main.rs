use bevy::prelude::*;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod spring;

// https://docs.rs/smooth-bevy-cameras/0.11.0/smooth_bevy_cameras/
// https://github.com/bonsairobo/smooth-bevy-cameras/blob/main/examples/simple_unreal.rs
use smooth_bevy_cameras::controllers::unreal::{
    UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin,
};
use smooth_bevy_cameras::LookTransformPlugin;

fn main() {
    App::new()
        // plugins are pretty cool and enforces modular. if you don't want it, just remove it!
        .add_plugins((
            DefaultPlugins,
            LookTransformPlugin,
            UnrealCameraPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .run()
}

fn create_light(commands: &mut Commands) {
    // Light
    let point_light = PointLight {
        shadows_enabled: true,
        intensity: 1_300_000.0,
        range: 100.0,
        ..default()
    };

    let point_light_bundle = PointLightBundle {
        point_light: point_light,
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    };

    commands.spawn(point_light_bundle);
}

#[derive(EnumIter)]
enum Axis {
    X,
    Y,
    Z,
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

fn create_axis(
    direction: Axis,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> PbrBundle {
    const GIRTH: f32 = 0.2;
    const LENGTH: f32 = 5.;

    let cuboid_dim: Vec3;
    let adjusted_position: Vec3;
    let color: Color;

    match direction {
        Axis::X => {
            cuboid_dim = Vec3::new(LENGTH, GIRTH, GIRTH);
            adjusted_position = Vec3::new(LENGTH / 2., 0., 0.);
            color = Color::RED;
        }
        Axis::Y => {
            cuboid_dim = Vec3::new(GIRTH, LENGTH, GIRTH);
            adjusted_position = Vec3::new(0., LENGTH / 2., 0.);
            color = Color::GREEN;
        }
        Axis::Z => {
            cuboid_dim = Vec3::new(GIRTH, GIRTH, LENGTH);
            adjusted_position = Vec3::new(0., 0., LENGTH / 2.);
            color = Color::BLUE;
        }
    }

    PbrBundle {
        mesh: meshes.add(Cuboid::from_size(cuboid_dim)),
        material: materials.add(color),
        transform: Transform::from_translation(adjusted_position),
        ..default()
    }
}

fn create_coord_system(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Origin
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::default().mesh().uv(32, 18)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });

    for variant in Axis::iter() {
        commands.spawn(create_axis(variant, meshes, materials));
    }
}

fn create_cameras(commands: &mut Commands) {
    const STARTING_CAM_POS: Vec3 = Vec3::new(5., 8., 12.0);

    let bevy_camera = Camera3dBundle {
        projection: PerspectiveProjection { ..default() }.into(),
        // looking at is how to orient
        // y is up in bevy
        transform: Transform::from_translation(STARTING_CAM_POS).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    let unreal_camera = UnrealCameraBundle::new(
        UnrealCameraController::default(),
        Vec3::from(STARTING_CAM_POS),
        Vec3::new(0., 0., 0.),
        Vec3::Y,
    );

    commands
        .spawn((MyCamera, bevy_camera))
        .insert(unreal_camera);
}

// throw some basic shapes in a 3d environment
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_light(&mut commands);
    create_coord_system(&mut commands, &mut meshes, &mut materials);
    create_cameras(&mut commands);
    spring::create_spring(&mut commands, &mut meshes, &mut materials)

}

// this doesnt work idk lol
// not sure if the library gives us the ability to do this
// probably write own in the future and get rid of the unreal camera controls you dont want
fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<MyCamera>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for mut trans in &mut query {
            println!("reseting camera!");
            *trans = Transform::from_xyz(-2.5, 4.5, 9.0);
        }
    }
}

#[derive(Component)]
struct MyCamera;
