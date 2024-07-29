use bevy::prelude::*;
use std::time::Duration;

// mod spring;
mod scene;

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

        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(3)))
        

        // Draw the initial scene
        .add_systems(Startup, scene::setup)

        // not currently working
        .add_systems(Update, scene::camera_reset_control) 

        // .add_systems(Update, insert_spring)

        // time example https://github.com/bevyengine/bevy/blob/latest/examples/time/time.rs
        .add_systems(FixedUpdate, test_ids)
        // .add_systems(Startup, test_ids)

        .add_systems(FixedUpdate, get_links)

        // Run it
        .run()
}

#[derive(Component)]
pub struct Node
{
    pos: Vec3
}

impl Node {
    fn new(pos: Vec3) -> Self
    {
        Self {
            pos
        }
    }
}

#[derive(Component)]
#[derive(Debug)]
pub struct Link
{
    to: u32,
    from: u32,
    to_ent: Entity,
    from_ent: Entity,
}

impl Link {
    fn new(to: u32, from: u32, to_ent: Entity, from_ent: Entity) -> Self {
        Self {
            to,
            from,
            to_ent,
            from_ent
        }
    }
}

pub fn test_ids(
    time: Res<Time>, 
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let node1 = commands.spawn((Node::new(Vec3 { x: (3.), y: (3.), z: (3.) }))).id();
    let node2 = commands.spawn((Node::new(Vec3 { x: (6.), y: (6.), z: (6.) }))).id();
    let node3 = commands.spawn(Node::new(Vec3::new(5.,5.,5.))).id();

    let link1 = commands.spawn((Link::new(node1.index(), node2.index(), node1, node2))).id();
    let link2 = commands.spawn((Link::new(node1.index(), node3.index(), node1, node3))).id();

    // println!(
    //     "Update: this is generic time clock, delta is {:?} and elapsed is {:?}. The nodes index is {} and {} and link is {}",
    //     time.delta(),
    //     time.elapsed(),
    //     node1.index(),
    //     node2.index(),
    //     link1.index(),
    // );

}

fn get_links(time: Res<Time>, linkq: Query<(Entity, &Link)>,mut nodes: Query<(Entity, &mut Node)>)
{
    println!("=======================");
    for (entity, link) in linkq.iter() {
        
        println!(
            "Changing link {:?}, to_pre is {:?}, from_pre is {:?}", 
            entity.index(),
            nodes.get(link.to_ent).expect("help").1.pos,
            nodes.get(link.from_ent).expect("help").1.pos,
        );

        let (_,mut node1) = nodes.get_mut(link.to_ent).expect("help");
        node1.pos = Vec3::new(1., 1., 1.);

        let (_,mut node2) = nodes.get_mut(link.from_ent).expect("help");
        node2.pos = Vec3::new(1., 1., 1.);

        println!(
            "Changing link {:?}, to_post is {:?}, from_post is {:?}", 
            entity.index(),
            nodes.get(link.to_ent).expect("help").1.pos,
            nodes.get(link.from_ent).expect("help").1.pos,
        );
        
    }
    println!("=======================");
}


// Insert a spring into the scene
// fn insert_spring(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     spring::create_spring(&mut commands, &mut meshes, &mut materials);
// }
