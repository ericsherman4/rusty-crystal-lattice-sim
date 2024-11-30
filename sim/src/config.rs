pub mod colors_config {
    use bevy::color::Srgba;
    use bevy::prelude::Color;
    pub const BLUE: Color = Color::Srgba(Srgba::BLUE);
    pub const GREEN: Color = Color::Srgba(Srgba::GREEN);
    pub const RED: Color = Color::Srgba(Srgba::RED);
    pub const NODE_COLOR: Color = Color::WHITE;
    pub const SPRING_COLOR: Color = BLUE;
}

pub mod lattice_config {
    pub const DIM: u32 = 3;
    pub const STARTING_LINK_LEN: f32 = 3.; // z component of cuboid gets this
    pub const NODE_RADIUS: f32 = 0.1/2.;
    pub const LINK_RADIUS: f32 = 0.1; //x and y component of cuboid get this
}

pub mod lights_config {
    use bevy::math::Vec3;

    pub const POS_COMPONENT: f32 = 10.0;
    pub const POS :Vec3 = Vec3::splat(POS_COMPONENT);
    pub const POS_2: Vec3 = Vec3::new(-7.2,-10.4,-8.8);
}

pub mod cam_config {
    use bevy::math::Vec3;
    pub const POS:Vec3 = Vec3::new(-5.,7.,-5.);
}

pub mod axis_config {
    pub const GIRTH : f32 = 0.2;
    pub const LENGTH: f32 = 5.;
    pub const HALF_LENGTH : f32 = LENGTH / 2.;
}

