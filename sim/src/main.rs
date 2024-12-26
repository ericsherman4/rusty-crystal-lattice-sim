use bevy::{
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin,
    }, prelude::*, time::common_conditions::{once_after_delay, repeating_after_delay}
};
use std::time::Duration;

mod config; // this allows me to do use crate::config::colors in other files in this folder
mod resources;
mod scene;
mod spring;

// https://docs.rs/smooth-bevy-cameras/0.11.0/smooth_bevy_cameras/
// https://github.com/bonsairobo/smooth-bevy-cameras/blob/main/examples/simple_unreal.rs
use smooth_bevy_cameras::controllers::unreal::UnrealCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;

//TODO: ADD THIS FPS OVERLAY https://bevyengine.org/examples/ui-user-interface/text/

const START_DELAY: u64 = 5;

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
            //TODO: check out https://docs.rs/bevy/0.14.2/bevy/render/diagnostic/struct.RenderDiagnosticsPlugin.html
        ))
        // ----------------------------------------------------------------------------
        // System enables diagnositcs for 10 seconds, then disables for 10 seconds.
        // taken from example. https://bevyengine.org/examples/diagnostics/enabling-disabling-diagnostic/
        // .add_systems(
        //     Update,
        //     toggle.run_if(on_timer(Duration::from_secs_f32(10.0))), //interesting
        // )
        // ----------------------------------------------------------------------------
        // Draw the initial scene and set background color
        .insert_resource(ClearColor(Srgba::hex("1f1f1f").unwrap().into()))
        // .insert_resource(AmbientLight {
        //     brightness: 100.0,
        //     ..default()
        // })
        // .insert_resource(ClearColor(Color::Srgba(Srgba::WHITE)))
        .add_systems(Startup, scene::setup)
        // ----------------------------------------------------------------------------
        // Draw the coordinate grid
        // TODO: making this a keyboard toggle would be useful
        // .add_systems(Startup, scene::draw_xyz)
        // ----------------------------------------------------------------------------
        // Generate a lattice structure
        .add_systems(
            Update,
            (resources::add_rng, spring::generate_lattice)
            .chain().run_if(once_after_delay(Duration::from_secs(START_DELAY)))
        )
        // ----------------------------------------------------------------------------
        // This lets you run an update at some interval. Not sure how to make multiple of them
        // I think these just apply to FixedUpdate schedule.
        // Example: https://github.com/bevyengine/bevy/blob/latest/examples/time/time.rs
        // TODO: change spring module name to lattice
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(5)))
        // ----------------------------------------------------------------------------
        // Update the spring's loc via transforms.
        // https://docs.rs/bevy/0.13.2/bevy/time/common_conditions/index.html
        .add_systems(
            FixedUpdate,
            (
                spring::update_nodes_state, 
                spring::update_link_physics,
                spring::update_spring
            ).chain().run_if(repeating_after_delay(Duration::from_secs(START_DELAY))),
        )

        // .add_systems(Update, scene::animate_ground)
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