use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, EntityCountDiagnosticsPlugin},
    time::common_conditions::on_timer,
    prelude::*,
};
use std::time::Duration;

mod config; // this allows me to do use create::config::colors in other files in this folder
mod resources;
mod scene;
mod spring;

// https://docs.rs/smooth-bevy-cameras/0.11.0/smooth_bevy_cameras/
// https://github.com/bonsairobo/smooth-bevy-cameras/blob/main/examples/simple_unreal.rs
use smooth_bevy_cameras::controllers::unreal::UnrealCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;


//TODO: ADD THIS FPS OVERLAY https://bevyengine.org/examples/ui-user-interface/text/

fn main() {
    App::new()
        // plugins are pretty cool and enforces modular. if you don't want it, just remove it!
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
            LookTransformPlugin,
            UnrealCameraPlugin::default(),
        ))
        // ----------------------------------------------------------------------------
        // System enables diagnositcs for 10 seconds, then disables for 10 seconds. 
        // taken from example. https://bevyengine.org/examples/diagnostics/enabling-disabling-diagnostic/
        // .add_systems(
        //     Update,
        //     toggle.run_if(on_timer(Duration::from_secs_f32(10.0))), //interesting
        // )
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
            (resources::add_rng, spring::generate_lattice.after(resources::add_rng)),
        )
        // ----------------------------------------------------------------------------
        // not currently working
        // .add_systems(Update, scene::camera_reset_control)        
        // ----------------------------------------------------------------------------
        // This lets you run an update at some interval. Not sure how to make multiple of them
        // I think these just apply to FixedUpdate schedule.
        // Example: https://github.com/bevyengine/bevy/blob/latest/examples/time/time.rs
        // TODO: change spring module name to lattice
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(50)))
        // ----------------------------------------------------------------------------
        // Update the spring's loc via transforms.
        .add_systems(
            FixedUpdate, 
            (
                spring::update_nodes_state.before(spring::update_link_physics),
                spring::update_link_physics.before(spring::update_spring),
                spring::update_spring
            )
        )
        // ----------------------------------------------------------------------------
        // Run it
        .run();
}

// fn toggle(mut store: ResMut<DiagnosticsStore>) {
//     for diag in store.iter_mut() {
//         info!("toggling diagnostic {}", diag.path());
//         diag.is_enabled = !diag.is_enabled;
//     }
// }