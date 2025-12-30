use bevy::prelude::*;
use bevy_splash_kit::{SplashKitPlugin, Timeline};

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    Splash,
    MainMenu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(
            SplashKitPlugin::new(Timeline {
                fade_in: std::time::Duration::from_secs(1),
                hold: std::time::Duration::from_secs(2),
                fade_out: std::time::Duration::from_secs(1),
            })
            .then_enter(AppState::MainMenu),
        )
        .add_systems(OnEnter(AppState::Splash), spawn_splash_text)
        .add_systems(OnEnter(AppState::MainMenu), cleanup_splash_ui)
        .run();
}

#[derive(Component)]
struct SplashUi;

fn spawn_splash_text(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        SplashUi,
        children![(
            Text::new("Made with Bevy"),
            TextFont {
                font_size: 96.0,
                ..default()
            },
        )],
    ));
}

fn cleanup_splash_ui(mut commands: Commands, query: Query<Entity, With<SplashUi>>) {
    for entity in &query {
        commands.entity(entity).despawn_children();
        commands.entity(entity).despawn();
    }
}
