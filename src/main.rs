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

fn print_point_status_system(
    query: Query<&Point, With<Territory>>
){
    for point in query.iter(){
        println!("point value: {}", point.0);
    }
}

fn main() {

    App::new()
        .add_startup_system(setup_scene.system())
        .add_system(nature_evolution_system.system())
        .add_system(print_point_status_system.system())
        .run();
}
