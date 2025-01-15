use bevy::{
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin,
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
    time::common_conditions::{on_timer, once_after_delay, repeating_after_delay},
};
use lattice::LatticePlugin;

// https://docs.rs/smooth-bevy-cameras/0.11.0/smooth_bevy_cameras/
// https://github.com/bonsairobo/smooth-bevy-cameras/blob/main/examples/simple_unreal.rs
use smooth_bevy_cameras::controllers::unreal::UnrealCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;

//TODO: ADD THIS FPS OVERLAY https://bevyengine.org/examples/ui-user-interface/text/

mod config;
mod lattice;
mod scene;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LookTransformPlugin,
            UnrealCameraPlugin::default(),
            LatticePlugin,
            // FrameTimeDiagnosticsPlugin,
            // EntityCountDiagnosticsPlugin::default(),
            // LogDiagnosticsPlugin::default(),
            //TODO: check out https://docs.rs/bevy/0.14.2/bevy/render/diagnostic/struct.RenderDiagnosticsPlugin.html
        ))
        // .insert_resource(ClearColor(Srgba::hex("3b4a56").unwrap().into())), color for video
        .insert_resource(ClearColor(Srgba::hex("000000").unwrap().into()))
        .add_systems(Startup, scene::setup)
        // no stopping user from running draw_xyz
        .add_systems(
            Update,
            scene::draw_xyz.run_if(input_just_pressed(KeyCode::KeyO)),
        )
        .add_systems(
            Update,
            scene::lock_camera.run_if(input_just_pressed(KeyCode::KeyL)),
        )
        // .add_systems(Update, scene::animate_ground)
        .run();
}

// driver code
// System enables diagnositcs for 10 seconds, then disables for 10 seconds.
// taken from example. https://bevyengine.org/examples/diagnostics/enabling-disabling-diagnostic/
// .add_systems(
//     Update,
//     toggle.run_if(on_timer(Duration::from_secs_f32(10.0))), //interesting
// )

// fn toggle(mut store: ResMut<DiagnosticsStore>) {
//     for diag in store.iter_mut() {
//         info!("toggling diagnostic {}", diag.path());
//         diag.is_enabled = !diag.is_enabled;
//     }
// }
