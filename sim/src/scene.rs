use crate::config::{axis_config, cam_config, colors_config, lattice_config, lights_config};
use bevy::{math::VectorSpace, prelude::*};
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_light(&mut commands, &mut config_store);
    create_cameras(&mut commands);
    create_ground(&mut commands,&mut meshes, &mut materials);
}

pub fn draw_xyz(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Origin
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Sphere {
                radius: axis_config::ORIGIN_SPHERE_RADIUS,
            }
            .mesh()
            .uv(32, 18),
        ),
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
fn create_ground(
    commands:&mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(200.0,200.0).subdivisions(10)),
            material: materials.add(Color::Srgba(Srgba::hex("1f1f1f").unwrap())),
            transform: Transform::from_xyz(0.0, -7.0, 0.0),
            ..default()
        }
    );
}

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

        // im not sure of the difference of setting here verses setting it with the unreal camera.
        // im not even sure i need both?? 
        transform: Transform::from_translation(STARTING_CAM_POS).looking_at(target, Vec3::Y),
        ..default()
    };

    let unreal_camera = UnrealCameraBundle::new(
        UnrealCameraController::default(),
        STARTING_CAM_POS,
        target,
        Vec3::Y,
    );

    commands
        .spawn((MyCamera, bevy_camera))
        .insert(unreal_camera);
}

//////////////////////////////////////////////////
/// LIGHTING
//////////////////////////////////////////////////

// #[derive(Component)]
// struct MyLight;

/// Create a light in the scene
fn create_light(commands: &mut Commands, gizmo_store: &mut ResMut<GizmoConfigStore>) {
    // Light


    let mut new_pos = Transform::from_translation(
        Vec3::splat(lattice_config::DIM as f32 * lattice_config::STARTING_LINK_LEN + lattice_config::STARTING_LINK_LEN*3.)
    ).looking_at(Vec3::new(5.0, 0.0, 5.0), Vec3::Y);
    // new_pos.translation.z += 5.0;
    // new_pos.translation.x += 5.0;

    // let mut new_pos = Transform::from_xyz(0.0,20.0,0.0).looking_at(Vec3::ZERO, Vec3::Y);


    //TODO: lights are confusing me, they are working backwards as i would expect them to.
    // like placing the light at all postivies values and spawning the camera there, looking at the cube
    // everything is in a shadow.
    let point_light_bundle_1 = SpotLightBundle {
        spot_light: SpotLight {
            shadows_enabled: true,
            shadow_depth_bias: 0.3,
            intensity: 20_000_000.,
            range: 50.,
            color: Color::Srgba(Srgba::WHITE),
            ..default()
        },
        // transform: Transform::from_translation(lights_config::POS),
        transform: Transform::from_translation(
            Vec3::splat(lattice_config::DIM as f32 * lattice_config::STARTING_LINK_LEN + lattice_config::STARTING_LINK_LEN*2.)
        ).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    // Second light
    let point_light_bundle_2 = SpotLightBundle {
        spot_light: SpotLight {
            shadows_enabled: true,
            shadow_depth_bias: 0.3,
            intensity: 20_000_000.,
            range: 50.,
            color: Color::Srgba(Srgba::WHITE),
            ..default()
        },
        // transform: Transform::from_translation(lights_config::POS_2),
        transform: Transform::from_translation(
            Vec3::splat(-1. * lattice_config::DIM as f32 * lattice_config::STARTING_LINK_LEN + lattice_config::STARTING_LINK_LEN*3.)
        ).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    let point_light_bundle_3 = SpotLightBundle {
        spot_light: SpotLight {
            shadows_enabled: true,
            shadow_depth_bias: 0.3,
            intensity: 20_000_000.,
            range: 50.,
            color: Color::Srgba(Srgba::WHITE),
            ..default()
        },
        // transform: Transform::from_translation(lights_config::POS_2),
        transform: new_pos,
        ..default()
    };


    let mut shadow_behind_light = Transform::from_translation(-1.0 * Vec3::splat(lattice_config::DIM as f32 * lattice_config::STARTING_LINK_LEN - 3.0));
    shadow_behind_light.translation.y = shadow_behind_light.translation.y*-1.0 + 6.0;
    shadow_behind_light.translation.x += 3.0;
    shadow_behind_light.look_at(Vec3::splat(lattice_config::DIM as f32 * lattice_config::STARTING_LINK_LEN)/2.0, Vec3::Y);

    let point_light_bundle_4 = SpotLightBundle {
        spot_light: SpotLight {
            shadows_enabled: true,
            shadow_depth_bias: 0.3,
            intensity: 20_000_000.,
            range: 50.,
            color: Color::Srgba(Srgba::WHITE),
            ..default()
        },
        // transform: Transform::from_translation(lights_config::POS_2),
        transform: shadow_behind_light,
        ..default()
    };

    // Light spawn
    commands.spawn(point_light_bundle_1);
    commands.spawn(point_light_bundle_2);
    commands.spawn(point_light_bundle_3);
    commands.spawn(point_light_bundle_4);

    // Gimzo config
    // let (_, light_config) = gizmo_store.config_mut::<LightGizmoConfigGroup>();
    // light_config.draw_all = true;
    // light_config.color = LightGizmoColor::Varied;
}
