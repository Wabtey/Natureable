use bevy::{prelude::*};
use super::{GameState, Point, Territory};


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game)
            .with_system(setup)
            )
            .insert_resource(EvolveTimer(Timer::from_seconds(2.0, true)))
            .add_system(nature_evolution_system)
            .add_system(button_system);

    }
}


struct EvolveTimer(Timer);


fn nature_evolution_system(
    mut query: Query<&mut Point, With<Territory>>,
    time: Res<Time>, mut timer: ResMut<EvolveTimer>,
){
    if timer.0.tick(time.delta()).just_finished() {
        for mut point in query.iter_mut() {
            point.0 += 1;
        }
    }
    
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut point_query: Query<&Point, With<Territory>>
) {

    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        for point in point_query.iter_mut(){
            match *interaction {
                Interaction::Clicked => {
                    text.sections[0].value = point.0.to_string();
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    text.sections[0].value = "Upgrade".to_string();
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    text.sections[0].value = point.0.to_string();
                    *color = NORMAL_BUTTON.into();
                }
            }
        }               
    }
    
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Point",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..default()
            });
        });
}