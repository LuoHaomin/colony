use crate::prelude::*;

pub struct InfoPanelPlugin;

impl Plugin for InfoPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (show_info_panel, info_system));
    }
}

pub fn show_info_panel(
    mut commands: Commands,
    font: Res<MyFont>,
    object_info: Res<InfoPanelInformation>,
    texts: Query<Entity, With<InfoPanelText>>,
) {
    for text in texts.iter() {
        commands.entity(text).despawn();
    }
    // Simple header
    commands.spawn((
        Text::new(object_info.name.clone()),
        TextFont { font: font.0.clone(), font_size: 24.0, ..default() },
        TextColor(Color::WHITE),
        Node { position_type: PositionType::Absolute, top: Val::Px(15.0), left: Val::Px(15.0), ..default() },
        InfoPanelText,
    ));
}

pub fn info_system(
    mut commands: Commands,
    mut people: Query<(Entity, &Position, &Brain, &PhysicalBody, Option<&HasName>), With<ClickedOn>>,
    mut info_panel: ResMut<InfoPanelInformation>,
) {
    if let Some((_entity, position, brain, physical_body, has_name)) = people.iter_mut().last() {
        info_panel.name = has_name.map(|h| h.name.clone()).unwrap_or_default();
        info_panel.info = vec![format!("Position: {}, {}", position.x, position.y)];
        info_panel.info.extend(physical_body.info_panel_needs());
        info_panel.info.extend(brain.info_panel());
    }
    
    // Clear old clicks
    let entities: Vec<Entity> = people.iter().map(|(e, ..)| e).collect();
    if entities.len() > 1 {
        for e in &entities[0..entities.len()-1] {
            commands.entity(*e).remove::<ClickedOn>();
        }
    }
}

#[derive(Component)]
pub struct InfoPanelText;

#[derive(Component)]
pub struct ClickedOn;

#[derive(Component)]
pub struct HasName {
    pub name: String,
}
