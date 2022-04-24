mod game;

use bevy::{prelude::*, winit::WinitSettings};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
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
struct Cell;

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
            .add_system(print_point_status_system);
    }
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(TestPrint)
        .add_startup_system(setup_scene)
        .add_state(GameState::Game)
        // this plugin will display a splash screen
        // .add_plugin(splash::SplashPlugin)
        .add_plugin(game::GamePlugin)

        .run();
}
