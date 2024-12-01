use bevy::prelude::*;
use resources::add_rng;
use std::time::Duration;

mod config; // this allows me to do use create::config::colors in other files in this folder
mod resources;
mod scene;
mod spring;

// https://docs.rs/smooth-bevy-cameras/0.11.0/smooth_bevy_cameras/
// https://github.com/bonsairobo/smooth-bevy-cameras/blob/main/examples/simple_unreal.rs
use smooth_bevy_cameras::controllers::unreal::UnrealCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;

fn main() {
    App::new()
        // plugins are pretty cool and enforces modular. if you don't want it, just remove it!
        .add_plugins((
            DefaultPlugins,
            LookTransformPlugin,
            UnrealCameraPlugin::default(),
        ))
        // ----------------------------------------------------------------------------
        // This lets you run an update at some interval. Not sure how to make multiple of them
        // I think these just apply to FixedUpdate schedule.
        // Example: https://github.com/bevyengine/bevy/blob/latest/examples/time/time.rs
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(300)))
        // ----------------------------------------------------------------------------
        // Draw the initial scene
        .add_systems(Startup, scene::setup)
        // ----------------------------------------------------------------------------
        // Draw the coordinate grid
        // TODO: making this a keyboard toggle would be useful
        .add_systems(Startup, scene::draw_xyz)
        // ----------------------------------------------------------------------------
        // Generate a lattice structure
        .add_systems(
            Startup,
            (resources::add_rng, spring::generate_lattice.after(add_rng)),
        )
        // ----------------------------------------------------------------------------
        // not currently working
        // .add_systems(Update, scene::camera_reset_control)
        // ----------------------------------------------------------------------------
        // Insert a spring into the scene
        // .add_systems(Startup, spring::insert_spring)
        // ----------------------------------------------------------------------------
        // Update the spring's loc via transforms.
        .add_systems(FixedUpdate, spring::update_spring)
        // ----------------------------------------------------------------------------
        // Run it
        .run();
}
