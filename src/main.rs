mod game;

use bevy::{
    prelude::*,
    //winit::WinitSettings
};
use bevy_vox::*;
use bevy_flycam::*;

#[bevy_main]
fn main() {

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        //.insert_resource(WinitSettings::desktop_app())        
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(VoxPlugin)
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0, // default: 12.0
        })
        .add_plugin(TestPrint)
        .add_plugin(game::GamePlugin)
        .add_startup_system(setup_scene)
        .add_state(GameState::Game)
        // this plugin will display a splash screen
        // .add_plugin(splash::SplashPlugin)

        .run();
}


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // Splash,
    // Menu,
    Game,
}

#[derive(Default)]
struct Game {
    board: Vec<Vec<Cell>>,
    player: Player,
    score: Point,
    wave : i32,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

#[derive(Default, Component)]
struct Player;

#[derive(Default, Component)]
struct Point(i32);

#[derive(Component)]
struct Territory;

#[derive(Component)]
struct Cell;

#[derive(Component)]
struct Character;



fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands
        .spawn()
        .insert(Player)
        .insert(Territory)
        .insert(Point(0));
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_scene(asset_server.load("SimField.vox"));    
    commands
        // light
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 6.0, 25.0)),
            ..Default::default()
        });
    commands
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 6.0, 2.0)),
            ..Default::default()
        });
    commands
        // camera
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -100.0, 100.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        });
        // 0 -100 50
        // 0 0 50
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



