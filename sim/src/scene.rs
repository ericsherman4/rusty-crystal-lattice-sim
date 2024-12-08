use crate::config::{axis_config, cam_config, colors_config, lattice_config, lights_config};
use bevy::prelude::*;
use smooth_bevy_cameras::controllers::unreal::{UnrealCameraBundle, UnrealCameraController};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

//////////////////////////////////////////////////
/// SCENE, ENVIRONMENT, SETUP
//////////////////////////////////////////////////

#[derive(EnumIter)]
enum Axis {
    X,
    Y,
    Z,
}

/// Setup scene / environment
pub fn setup(
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
) {
    create_light(&mut commands, &mut config_store);
    create_cameras(&mut commands);
    // create_ground(&mut commands,&mut meshes, &mut materials);
}

pub fn draw_xyz(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Origin
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere{radius: axis_config::ORIGIN_SPHERE_RADIUS}.mesh().uv(32, 18)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });

    for variant in Axis::iter() {
        commands.spawn(create_axis(variant, &mut meshes, &mut materials));
    }
}

/// Create an x,y,z axis in the scene
fn create_axis(
    direction: Axis,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> PbrBundle {
    let cuboid_dim: Vec3;
    let adjusted_position: Vec3;
    let color: Color;

    const LENGTH: f32 = axis_config::HALF_LENGTH;
    const GIRTH: f32 = axis_config::GIRTH;

    match direction {
        Axis::X => {
            cuboid_dim = Vec3::new(LENGTH, GIRTH, GIRTH);
            adjusted_position = Vec3::new(LENGTH / 2., 0., 0.);
            color = colors_config::RED;
        }
        Axis::Y => {
            cuboid_dim = Vec3::new(GIRTH, LENGTH, GIRTH);
            adjusted_position = Vec3::new(0., LENGTH / 2., 0.);
            color = colors_config::GREEN
        }
        Axis::Z => {
            cuboid_dim = Vec3::new(GIRTH, GIRTH, LENGTH);
            adjusted_position = Vec3::new(0., 0., LENGTH / 2.);
            color = colors_config::BLUE;
        }
    }

    PbrBundle {
        mesh: meshes.add(Cuboid::from_size(cuboid_dim)),
        material: materials.add(color),
        transform: Transform::from_translation(adjusted_position),
        ..default()
    }
}

/// Create a ground to help with lighting
// fn create_ground(
//     commands:&mut Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn((
//         Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
//         MeshMaterial3d(materials.add(Color::from(SILVER))),
//     ));
// }

//////////////////////////////////////////////////
/// CAMERAS
//////////////////////////////////////////////////

#[derive(Component)]
struct MyCamera;

/// Create the unreal engine camera object in the scene
fn create_cameras(commands: &mut Commands) {
    const STARTING_CAM_POS: Vec3 = cam_config::POS;

    let target = Vec3::splat(lattice_config::STARTING_LINK_LEN * lattice_config::DIM as f32 / 2.);
    println!("Camera is pointing at {}", target);
    let bevy_camera = Camera3dBundle {
        projection: PerspectiveProjection { ..default() }.into(),
        // looking at is how to orient
        // y is up in bevy

        // TODO: fix this, camera is not looking at it's target cause when you move forward, i fly
        // right through the 0,0,0 origin. irrespective of what height the camera ia at.
        transform: Transform::from_translation(STARTING_CAM_POS).looking_at(target, Vec3::Y),
        ..default()
    };

    let unreal_camera = UnrealCameraBundle::new(
        UnrealCameraController::default(),
        STARTING_CAM_POS,
        Vec3::new(0., 0., 0.),
        Vec3::Y,
    );

    commands
        .spawn((MyCamera, bevy_camera))
        .insert(unreal_camera);
}

//////////////////////////////////////////////////
/// LIGHTING
//////////////////////////////////////////////////

/// Create a light in the scene
fn create_light(commands: &mut Commands, gizmo_store: &mut ResMut<GizmoConfigStore>) {
    // Light
    //TODO: lights are confusing me, they are working backwards as i would expect them to.
    // like placing the light at all postivies values and spawning the camera there, looking at the cube
    // everything is in a shadow.
    let point_light_bundle_1 = PointLightBundle {
        point_light: PointLight {
            // shadows_enabled: true,
            // shadow_depth_bias: 0.2,
            intensity: 10_000_000.,
            range: 300.,
            ..default()
        },
        transform: Transform::from_translation(lights_config::POS),
        ..default()
    };

    // Second light
    let point_light_bundle_2 = PointLightBundle {
        point_light: PointLight {
            // shadows_enabled: true,
            intensity: 10_000_000.,
            range: 300.,
            ..default()
        },
        transform: Transform::from_translation(lights_config::POS_2),
        ..default()
    };

    // Light spawn
    commands.spawn(point_light_bundle_1);
    commands.spawn(point_light_bundle_2);

    // Gimzo config
    let (_, light_config) = gizmo_store.config_mut::<LightGizmoConfigGroup>();
    light_config.draw_all = true;
    light_config.color = LightGizmoColor::Varied;
}
