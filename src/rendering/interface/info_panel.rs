use crate::prelude::*;
use crate::rendering::interface::game_ui::InspectorContent;

pub struct InfoPanelPlugin;

impl Plugin for InfoPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_inspector, info_system));
    }
}

#[derive(Component)]
pub struct InspectorItem;

pub fn update_inspector(
    mut commands: Commands,
    font: Res<MyFont>,
    object_info: Res<InfoPanelInformation>,
    inspector_content: Query<Entity, With<InspectorContent>>,
    items_to_clear: Query<Entity, With<InspectorItem>>,
) {
    if object_info.is_changed() {
        // Clear old items
        for entity in items_to_clear.iter() {
            commands.entity(entity).despawn();
        }

        if let Some(content_entity) = inspector_content.iter().next() {
            commands.entity(content_entity).with_children(|parent| {
                // Header (Name)
                parent.spawn((
                    Text::new(object_info.name.clone()),
                    TextFont { font: font.0.clone(), font_size: 20.0, ..default() },
                    TextColor(Color::srgb(0.7, 1.0, 0.7)),
                    InspectorItem,
                ));

                // Separator
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(2.0),
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 1.0)),
                    InspectorItem,
                ));
                
                // Info details
                for line in &object_info.info {
                    parent.spawn((
                        Text::new(line.clone()),
                        TextFont { font: font.0.clone(), font_size: 14.0, ..default() },
                        TextColor(Color::WHITE),
                        InspectorItem,
                    ));
                }
            });
        }
    }
}

pub fn info_system(
    mut commands: Commands,
    mut people: Query<(Entity, &Position, &Brain, &PhysicalBody, Option<&HasName>), With<ClickedOn>>,
    mut info_panel: ResMut<InfoPanelInformation>,
) {
    if let Some((_entity, position, brain, physical_body, has_name)) = people.iter_mut().last() {
        info_panel.name = has_name.map(|h| h.name.clone()).unwrap_or_else(|| "Something".to_string());
        info_panel.info = vec![format!("Grid Pos: [{}, {}, {}]", position.x, position.y, position.z)];
        info_panel.info.extend(physical_body.info_panel_needs());
        info_panel.info.extend(brain.info_panel());
    }
    
    // Simple logic to keep only one clicked element
    let entities: Vec<Entity> = people.iter().map(|(e, ..)| e).collect();
    if entities.len() > 1 {
        for e in &entities[0..entities.len()-1] {
            commands.entity(*e).remove::<ClickedOn>();
        }
    }
}

#[derive(Component)]
pub struct ClickedOn;

#[derive(Component)]
pub struct HasName {
    pub name: String,
}
