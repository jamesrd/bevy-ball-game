use bevy::prelude::*;

use self::styles::PAUSE_MENU_STYLE;

mod styles;

#[derive(Component)]
pub struct PauseMenu {}

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "PAUSED",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 64.0,
                            color: Color::WHITE,
                        },
                    )],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..default()
            });
        });
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}
