pub mod colors_config {
    use bevy::color::Srgba;
    use bevy::prelude::Color;
    pub const BLUE: Color = Color::Srgba(Srgba::BLUE);
    pub const GREEN: Color = Color::Srgba(Srgba::GREEN);
    pub const RED: Color = Color::Srgba(Srgba::RED);
    pub const NODE_COLOR: Color = Color::WHITE;
    pub const SPRING_COLOR: Color = Color::BLACK;
}

pub mod lattice_config {
    use bevy::prelude::Visibility;

    pub const DIM: u32 = 6; // TODO: CAN YOU INTENTIONALLY PARALLEIZE THE QUERIES FOR THE UPDATE?
    pub const LINK_RADIUS: f32 = 0.05; //x and y component of cuboid get this
    pub const STARTING_LINK_LEN: f32 = 1.; // z component of cuboid gets this
    pub const LINK_VISIBILITY: Visibility = Visibility::Visible;

    pub const NODE_RADIUS: f32 = LINK_RADIUS / 2.0;
    pub const NODE_MASS: f32 = 5.0;

    pub const SPRING_CONST: f32 = 3.0;

    const START_VEL_ABS: f32 = 0.9;
    pub const START_VEL_MIN: f32 = -START_VEL_ABS;
    pub const START_VEL_MAX: f32 = START_VEL_ABS;
}

pub mod lights_config {
    use bevy::math::Vec3;

    pub const SPOT_LIGHT_SHADOWS: bool = false;

    pub const POS_COMPONENT: f32 = 10.0;
    pub const POS: Vec3 = Vec3::splat(POS_COMPONENT);
    pub const POS_2: Vec3 = Vec3::new(-7.2, -10.4, -8.8);
}

pub mod cam_config {
    use bevy::math::Vec3;
    pub const POS: Vec3 = Vec3::new(-10., 11., -10.0);
}

pub mod axis_config {
    pub const GIRTH: f32 = 0.05;
    pub const LENGTH: f32 = 2.;
    pub const HALF_LENGTH: f32 = LENGTH / 2.;
    pub const ORIGIN_SPHERE_RADIUS: f32 = GIRTH;
}
