pub mod colors {
    use bevy::color::Srgba;
    use bevy::prelude::Color;
    pub const BLUE: Color = Color::Srgba(Srgba::BLUE);
    pub const GREEN: Color = Color::Srgba(Srgba::GREEN);
    pub const RED: Color = Color::Srgba(Srgba::RED);
}

pub mod lattice {
    pub const DIM: u32 = 6;
    pub const STARTING_LINK_LEN: f32 = 3.;
    pub const NODE_RADIUS: f32 = 0.1;
    pub const LINK_RADIUS: f32 = 0.1;
}
