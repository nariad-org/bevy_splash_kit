use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, display_title)
        .run();
}

fn display_title(mut commands: Commands) {
    commands.spawn(Camera2d);

    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        ..default()
    };

    let title = (
        Text::new("Made with Bevy"),
        TextFont {
            font_size: 130.0,
            ..default()
        },
    );

    commands.spawn((container, children![title]));
}
