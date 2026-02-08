use super::prelude::*;
use bevy::text::{Text2dBundle, TextStyle, Text};

pub fn names_system(
    mut commands: Commands,
    query: Query<(Entity, &HasName), Without<HasNameShown>>,
    asset_server: Res<AssetServer>
) {
    for (entity, has_name) in query.iter() {
        //if (has_name.with_children)
        //has_name.name = "Bob".to_string();
        let font = asset_server.load("fonts/FiraSans-Medium.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 18.0,
            color: Color::WHITE,
        };
        let child = commands.spawn((
            Text2dBundle {
                text: Text::new(has_name.name.clone()),
                ..default()
            },
            TextName
        ))
        .insert(Transform::from_xyz(0.0, 30.0, 100.0))
        .insert(IsName)
        .id()
        ;
        commands.entity(entity).add_child(child);
        //commands.entity(entity).remove::<HasName>();
        commands.entity(entity).insert(HasNameShown);
    }
}