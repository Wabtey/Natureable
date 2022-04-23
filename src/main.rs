mod game;

use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // Splash,
    Menu,
    Game,
}


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



fn setup_scene(
    mut commands: Commands
){
    commands
        .spawn()
        .insert(Player)
        .insert(Territory)
        .insert(Point(0));
    commands.spawn_bundle(UiCameraBundle::default());
    
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
        .add_state(GameState::Splash)
        // this plugin will display a splash screen
        // .add_plugin(splash::SplashPlugin)
        .add_plugin(game::GamePlugin)

        .run();
}
