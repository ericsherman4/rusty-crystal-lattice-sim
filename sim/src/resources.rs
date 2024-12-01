use bevy::prelude::{Commands, Resource};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

//////////////////////////////////////////////////
/// RNG
//////////////////////////////////////////////////

#[derive(Resource)]
pub struct RandomSource(pub ChaCha8Rng);

pub fn add_rng(mut commands: Commands) {
    // https://rust-random.github.io/book/guide-rngs.html
    // TODO: current deterministic but change to get varying results.
    let keyboard_mashing: u64 = 459347051375372;
    let seeded_rng = ChaCha8Rng::seed_from_u64(keyboard_mashing);
    commands.insert_resource(RandomSource(seeded_rng));
    println!("Successfully added RNG");
}
