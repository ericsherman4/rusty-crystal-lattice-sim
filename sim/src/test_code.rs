
// dead code, to be copied into main.rs


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

// maybe can make a custom mesh to solve this but this is easier
// https://www.christopherbiscardi.com/why-do-bevy-sprites-spawn-with-the-center-at-0-0
// fn set_cuboid_pos_by_end(target_position: Vec3, length_in_dir: f32, dir: CuboidOffsetDir) -> Vec3
// {
//     // By default, it sets it based on origin.
//     // Returns a modified transform that will place the cuboid correctly.
//     match dir {
//         CuboidOffsetDir::Y => Vec3::new(target_position.x, target_position.y + length_in_dir/2.0, target_position.z),
//         _ => Vec3::default(),
//     }
// }


// lets start by fixing node1 and only doing node2 
// F = -k(delta X (from nominal))
// a = F /m
// v = v_i + a * dt
// x = x_i + v * dt
// pub fn animate_spring(time: Res<Time>, mut query: Query<&mut Transform, With<Node>>) {
    

    
//     for mut transform in &mut query {

//     }

// }