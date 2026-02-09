use crate::prelude::*;

pub fn names_system(
    mut commands: Commands,
    query: Query<(Entity, &HasName), Without<HasNameShown>>,
    asset_server: Res<AssetServer>
) {
    for (entity, has_name) in query.iter() {
        let font = asset_server.load("fonts/FiraSans-Medium.ttf");
        let child = commands.spawn((
            Text2d::new(has_name.name.clone()),
            TextFont {
                font: font.clone(),
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE.into()),
            TextName,
            Transform::from_xyz(0.0, 20.0, 10.0),
            IsName
        ))
        .id()
        ;
        commands.entity(entity).add_child(child);
        commands.entity(entity).insert(HasNameShown);
    }
}

pub fn update_unit_status_text(
    mut text_query: Query<(&mut Text2d, &ChildOf), With<TextName>>,
    brain_query: Query<(&HasName, &Brain)>,
) {
    for (mut text, parent) in text_query.iter_mut() {
        if let Ok((has_name, brain)) = brain_query.get(parent.0) {
            let task_text = match &brain.task {
                Some(task) => format!("{:?}", task),
                None => "Thinking...".to_string(),
            };
            text.0 = format!("{}\n[{}]", has_name.name, task_text);
        }
    }
}
