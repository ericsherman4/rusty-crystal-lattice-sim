use bevy::prelude::*;

fn main() {
    App::new()
        // plugins are pretty cool and enforces modular. if you don't want it, just remove it!
        .add_plugins((DefaultPlugins, introduce::CustomHelloPlugin))
        .run()
}

mod introduce {

    // ECS system
    // system is a normal rust function
    // a component is rust structs that implement the Component trait
    // #[derive(Component)]
    // entities are a simple type containing a unique number
    // resources are globally unique data fo some kind. (for example, elasped time, renderers, assets ((sounds, meshes)))
    use super::*;
    pub struct CustomHelloPlugin;

    impl Plugin for CustomHelloPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(GreetTimer(Timer::from_seconds(2_f32, TimerMode::Repeating)))
                .add_systems(Startup, add_people)
                // systems runs in default whenever possible!
                // can add .chain() on the tuple if we want to enforce a particular order the functions run in
                .add_systems(
                    Startup,
                    (hello_world, (greet_people, greet_personless).chain()),
                )
                .add_systems(Update, clock_tick);
        }
    }

    // an example of a system (normal rust functions)
    fn hello_world() {
        println!("hello world");
    }

    fn add_people(mut commands: Commands) {
        commands.spawn((Person, Name(String::from("Bob"))));
        commands.spawn(Person); // no name
        commands.spawn(Name("I am not a person".to_string()));
    }

    // paremeters we pass in to a system function define what data the system runs on
    // runs on all entities with the person and the name component
    fn greet_people(query: Query<&Name, With<Person>>) {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }

    // you can also make mutable querts
    fn greet_personless(query: Query<&Name, Without<Person>>) {
        for name in &query {
            println!("personless person: {}", name.0);
        }
    }

    fn clock_tick(time: Res<Time>, mut timer: ResMut<GreetTimer>) {
        if timer.0.tick(time.delta()).just_finished() {
            println!("hi {}", time.delta().as_millis());
        }
    }

    // a component
    #[derive(Component)]
    struct Person;

    #[derive(Component)]
    struct Name(String);

    #[derive(Resource)]
    struct GreetTimer(Timer);
}
