use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Point(i32);

#[derive(Component)]
struct Territory;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Character;

fn nature_evolution_system(
    mut query: Query<&mut Point, With<Territory>>
){
    for mut point in query.iter_mut() {
        point.0 += 1;
    }
}

fn setup_scene(
    mut commands: Commands
){
    commands
        .spawn()
        .insert(Player)
        .insert(Territory)
        .insert(Point(0));
    
}

struct PrintTimer(Timer);

fn print_point_status_system(
    time: Res<Time>, mut timer: ResMut<PrintTimer>,
    query: Query<&Point, With<Territory>>
){
    if timer.0.tick(time.delta()).just_finished() {
        for point in query.iter(){
            println!("point value: {}", point.0);
        }
    }
    
}

pub struct TestPrint;
impl Plugin for TestPrint {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrintTimer(Timer::from_seconds(2.0, true)))
            .add_system(print_point_status_system.system());
    }
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TestPrint)
        .add_startup_system(setup_scene.system())
        .add_system(nature_evolution_system.system())
        .run();
}
