use bevy::prelude::*;
use super::{despawn_screen, DisplayQuality, GameState, Volume, TEXT_COLOR};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(game))
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
            )
            .add_system(nature_evolution_system.system());

    }
}

fn nature_evolution_system(
    mut query: Query<&mut Point, With<Territory>>
){
    for mut point in query.iter_mut() {
        point.0 += 1;
    }
}

// Tag component used to tag entities added on the game screen

#[derive(Component)]
struct OnGameScreen;

#[derive(Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        // First create a `NodeBundle` for centering what we want to display
        .spawn_bundle(NodeBundle {
            style: Style {
                // This will center the current node
                margin: Rect::all(Val::Auto),
                // This will display its children in a column, from top to bottom. Unlike
                // in Flexbox, Bevy origin is on bottom left, so the vertical axis is reversed
                flex_direction: FlexDirection::ColumnReverse,
                // `align_items` will align children on the cross axis. Here the main axis is
                // vertical (column), so the cross axis is horizontal. This will center the
                // children
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .insert(OnGameScreen)
}

// Tick the timer, and change state when finished

fn game(
    mut game_state: ResMut<State<GameState>>,
) {
    // game_state.set(GameState::Menu).unwrap();
}