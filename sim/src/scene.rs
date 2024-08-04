use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use smooth_bevy_cameras::controllers::unreal::{UnrealCameraBundle, UnrealCameraController};

#[derive(EnumIter)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Component)]
pub struct MyCamera;

/// Setup scene / environment
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_light(&mut commands);
    create_cameras(&mut commands);
}

pub fn draw_xyz (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_coord_system(&mut commands, &mut meshes, &mut materials);
}

/// Handle keyboard input relating to the camera
pub fn camera_reset_control(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<MyCamera>>,
) {
    // this doesnt work idk lol
    // not sure if the library gives us the ability to do this
    // probably write own in the future and get rid of the unreal camera controls you dont want
    if keys.just_pressed(KeyCode::Space) {
        for mut trans in &mut query {
            println!("reseting camera!");
            *trans = Transform::from_xyz(-2.5, 4.5, 9.0);
        }
    }
}

/// Create the unreal engine camera object in the scene
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

/// Create an x,y,z axis in the scene
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

/// Create a coordinate system for reference when debugging
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

/// Create a light in the scene
fn create_light(commands: &mut Commands) {
    let pos: f32 = 3.0;

    // Light
    let point_light_bundle_1 = PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(pos, pos, pos),
        ..default()
    };

    let point_light_bundle_2 = PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-pos, -pos, -pos),
        ..default()
    };

    commands.spawn(point_light_bundle_1);
    commands.spawn(point_light_bundle_2);

    // commands.insert_resource(AmbientLight {
    //     color: Color::WHITE.into(),
    //     brightness:500.,
    // });
}
