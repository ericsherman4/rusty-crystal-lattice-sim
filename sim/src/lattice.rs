use bevy::{
    color,
    gizmos::config,
    prelude::*,
    time::common_conditions::{on_timer, once_after_delay, repeating_after_delay},
};
use std::time::Duration;

mod components;
mod lattice_gen;
use crate::config::{colors_config, lattice_config};
use components::{Link, Node, Static};
use lattice_gen::{create_all_nodes, generate_lattice, generate_lattice_animated, LatticeGen, RandomSourcePlugin, LatticeGenAnim};

//-------------------------------------------------------
// STRUCTS
//-------------------------------------------------------

#[derive(Resource)]
pub struct SimulationData {
    kinetic_energy: f32,
    center_of_mass: Transform,
}

pub struct LatticePlugin;

//-------------------------------------------------------
// IMPLEMENTATIONS
//-------------------------------------------------------

impl Default for SimulationData {
    fn default() -> Self {
        SimulationData {
            kinetic_energy: 0.0,
            center_of_mass: Transform::from_translation(Vec3::ZERO),
        }
    }
}

impl Plugin for LatticePlugin {
    fn build(&self, app: &mut App) {
        const LATTICE_START_DELAY: Duration = Duration::from_millis(1_000);

        // Inserts the rng to generate the lattice
        app.add_plugins(RandomSourcePlugin);

        app.insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(10)));
        app.insert_resource(SimulationData::default());
        app.insert_resource(LatticeGen::new(lattice_config::DIM));
        app.insert_resource(LatticeGenAnim::new());

        app.add_systems(Update, rotate_around_center);

        if false {
            app.add_systems(
                Update,
                (create_all_nodes, generate_lattice)
                    .chain()
                    .run_if(once_after_delay(LATTICE_START_DELAY)),
            );
        }
        else {
            app.add_systems(
                Update, create_all_nodes.run_if(once_after_delay(LATTICE_START_DELAY)),
            );
            app.add_systems(Update, generate_lattice_animated
                .after(create_all_nodes)
                // .run_if(on_timer(Duration::from_millis(1)))
                .run_if(repeating_after_delay(LATTICE_START_DELAY)));
        }


        app.add_systems(
            FixedUpdate,
            (
                // update_nodes_state, 
                // update_link_physics, 
                update_spring
            ).chain().run_if(repeating_after_delay(LATTICE_START_DELAY)),
        );

        app.add_systems(Update, update_center_of_mass);
        // app.add_systems(
        //     Update,
        //     print_kinetic_energy.run_if(on_timer(Duration::from_secs_f32(0.5))),
        // );
    }
}

/// Rotate the camera around the center of the cube.
/// Must unlock the unreal engine camera for it to work.
fn rotate_around_center(
    data: Res<SimulationData>,
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut camera = camera.single_mut();

    let secondary_axis_rot_rate = 0.0;
    let primary_axis_rot_rate = 0.3 * time.delta_seconds();

    if keyboard.pressed(KeyCode::ArrowRight) {
        let rot = Quat::from_euler(
            EulerRot::YXZ,
            primary_axis_rot_rate,
            secondary_axis_rot_rate,
            secondary_axis_rot_rate,
        );
        camera.rotate_around(data.center_of_mass.translation, rot);
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        let rot = Quat::from_euler(
            EulerRot::YXZ,
            -primary_axis_rot_rate,
            -secondary_axis_rot_rate,
            -secondary_axis_rot_rate,
        );
        camera.rotate_around(data.center_of_mass.translation, rot);
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        let rot = Quat::from_axis_angle(camera.right().into(), -primary_axis_rot_rate);
        camera.rotate_around(data.center_of_mass.translation, rot);
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        let rot = Quat::from_axis_angle(camera.right().into(), primary_axis_rot_rate);
        camera.rotate_around(data.center_of_mass.translation, rot);
    }
}

//-------------------------------------------------------
// SPRING SYSTEMS
//-------------------------------------------------------

/// Update the center of mass of the cube in simulation data
/// Used for camera rotations
pub fn update_center_of_mass(mut data: ResMut<SimulationData>, nodes: Query<&Node>) {
    let mut accum = Vec3::ZERO;
    let mut count = 0;

    for node in nodes.iter() {
        count += 1;
        accum += node.pos;
    }

    // Guard against not yet having any nodes spawned
    if count == 0 {
        return;
    }

    data.center_of_mass.translation = accum / count as f32;
    // println!("initial COM is {}", data.center_of_mass.translation);
}

/// Print the kinetic energy of the cube
pub fn print_kinetic_energy(sim_data: Res<SimulationData>) {
    println!("{}", sim_data.kinetic_energy);
}

/// Update the state of the nodes and their positions using the calculated force
pub fn update_nodes_state(
    time: Res<Time>,
    mut nodes: Query<(&mut Node, &mut Transform), Without<Static>>,
    mut sim_data: ResMut<SimulationData>,
) {
    let delta_t = time.delta_seconds();
    // println!("Elasped time is {}", delta_t);

    let mut total_kinetic_energy = 0.0;

    for (mut node, mut transform) in nodes.iter_mut() {
        // update vel and pos
        let acc = node.sum_forces / node.mass;
        node.vel = node.vel + acc * delta_t;
        node.pos = node.pos + node.vel * delta_t;

        // zero out sum forces
        node.sum_forces = Vec3::ZERO;

        // calculate the total kinetic energy
        let velocity_magnitude = node.vel.length();
        total_kinetic_energy += 0.5 * node.mass * velocity_magnitude * velocity_magnitude;

        // Update the nodes mesh transform
        transform.translation = node.pos;
    }

    sim_data.kinetic_energy = total_kinetic_energy;
}

/// Update spring physics and sum up forces on each node
pub fn update_link_physics(
    time: Res<Time>,
    mut links: Query<(&mut Link, &Handle<StandardMaterial>)>,
    mut nodes: Query<(&mut Node, &mut Transform)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let delta_t = time.delta_seconds();
    const DAMPING: f32 = 5.0; // 30 at damping and vel at 20 is pretty cool and div spring displament by 5

    for (mut link, mut material_handle) in links.iter_mut() {

        let node_from = nodes.get(link.from).unwrap();
        let node_to = nodes.get(link.to).unwrap();

        let node_to_pos = node_to.0.pos;
        let node_from_pos = node_from.0.pos;
        let force_dir = (node_to_pos - node_from_pos).normalize();
        let length = (node_to_pos - node_from_pos).length();
        let spring_displacement = length - link.orig_length;

        // velocity of the spring is change of spring displacement over time. v = delta x / delta t
        let velocity = (spring_displacement - link.delta_spring_length_pre) / delta_t;
        let damping_force = -DAMPING * velocity;
        // let damping_force =0.0;
        let total_force = -1. * link.spring_const * spring_displacement + damping_force;
        // let force = -1. * link.spring_const * spring_displacement - (DAMPING * (node_to.0.vel - node_from.0.vel)); // THIS IS WRONG, works in sim but wrong physics wise
        // link.delta_spring_length_pre = spring_displacement/5.0; // results in more appealing simulations
        link.delta_spring_length_pre = spring_displacement / 5.0;
        let from_force = -0.5 * total_force * force_dir;
        let to_force = -from_force;

        // https://github.com/bevyengine/bevy/discussions/8487
        let material = materials.get_mut(material_handle).unwrap();
        // Doing it by force the spring exerts
        // if let Some(force_color) = from_force.try_normalize() {
        //     let scaled_vec = force_color * 0.5 + Vec3::splat(0.5);
        //     material.base_color = Color::srgb(scaled_vec.x, scaled_vec.y, scaled_vec.z);
        // }
        // Doing it by the velocity. Apply the gain to get the colors to show more when velocity is low
        // let normalized_vel = f32::abs(velocity) / 255.0  * 40.0;
        // material.base_color = Color::srgb(normalized_vel, 0.0,0.0);

        // this force is applied in the axis colinear from node 1 to node 2
        nodes.get_mut(link.from).unwrap().0.sum_forces += from_force;
        nodes.get_mut(link.to).unwrap().0.sum_forces += to_force;
    }
}

/// Update the spring transforms
pub fn update_spring(
    mut links: Query<(&Link, &mut Transform), Without<Node>>,
    nodes: Query<&Node>,
) {
    for (link, mut transform) in links.iter_mut() {
        // expect and unwrap not encouraged to use, see
        // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
        let node_to = nodes
            .get(link.to)
            .expect("The node should exist as no nodes are ever despawned.");
        let node_from = nodes
            .get(link.from)
            .expect("The node should exist as no nodes are ever despawned.");

        // update position and length of the spring
        // USE POS, NOT THE TRANSFORM OF THE SPRING
        // this is because the transform only updates every frame where as pos updates every fixedtimestep
        let dir = node_to.pos - node_from.pos;
        transform.translation = dir / 2. + node_from.pos;
        transform.scale.z = dir.length() / link.orig_length;

        // rotate the spring so it aligns with the direction vector between the two nodes (node_to - node_from)
        // best attempt at explaining
        // This fixes my dir3::new_unchecked is not normalized error
        // it tries to get the main axis to look at main direction and secondary axis to look secondary direction.
        // in my case, I only care about main axis looking at main direction but this means there's infinite solutions
        // so main axis is Z. Z is the one I am scaling as well to get it to stretch between the two nodes.
        // I want Z to point to the vector from node_from to node_to or the dir vector
        // The secondary axis - just using Y, I assume X would work, I want to point to a vector orthogonal to the dir vector.
        // I get this vector by crossing the direction vector node_from vector.
        *transform = transform.aligned_by(Vec3::Z, dir, Vec3::Y, dir.cross(node_from.pos));
        // okay the cross thing is nice but i think that was causing a lot of them to spin which makes sense
        // because dir cross node_from trans could point in any outward direction really.
        // *transform = transform.aligned_by(Vec3::Z, dir, Vec3::Y, Vec3::splat(100.0));  //this seems to cause the sticks to glitch occasionally.
        // Update, because i dont care about secondary axis, applied what I learned from above to make looking_at work.
        // same principals apply, I want to look at the node named node_to and I want a vector orthogonal ot the dir vector.
        // *transform = transform.looking_at(node_to.pos,  dir.cross(node_from.pos)) // this also has the spinning problem

        // note that the following doesnt work but produces awesome results
        // *transform = transform.looking_to(node_to.pos,  dir.cross(node_from.pos))
    }
}
