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
    mut clickable: Query<(Entity, &Position, Option<&PhysicalBody>, Option<&Brain>, Option<&HasName>, Option<&Genome>, Option<&Generation>, Option<&EnvironmentalData>), With<ClickedOn>>,
    mut info_panel: ResMut<InfoPanelInformation>,
) {
    if let Some((_entity, position, physical_body, brain, has_name, genome, generation, env_data)) = clickable.iter_mut().last() {
        info_panel.name = has_name.map(|h| h.name.clone()).unwrap_or_else(|| "Object".to_string());
        info_panel.info = vec![format!("Grid Pos: [{}, {}, {}]", position.x, position.y, position.z)];
        
        if let Some(pb) = physical_body {
            info_panel.info.extend(pb.info_panel_needs());
        }
        
        if let Some(b) = brain {
            info_panel.info.extend(b.info_panel());
        }

        if let Some(gen) = generation {
             info_panel.info.push(format!("Generation: {}", gen.value));
        }

        if let Some(g) = genome {
            info_panel.info.push(format!("-- Genome --"));
            info_panel.info.push(format!("Size: {:.2}", g.size));
            info_panel.info.push(format!("Mobility: {:.2}", g.mobility));
            info_panel.info.push(format!("Metab-Eff: {:.2}", g.metabolic_efficiency));
            info_panel.info.push(format!("Diet: {:.1} (0=Plant, 1=Meat)", g.diet_type));
        }

        if let Some(env) = env_data {
            info_panel.info.push(format!("-- Environment --"));
            info_panel.info.push(format!("Temp: {:.1} C", env.temperature));
            info_panel.info.push(format!("Fertility: {:.1}%", env.fertility * 100.0));
        }
    }
    
    // Simple logic to keep only one clicked element
    let entities: Vec<Entity> = clickable.iter().map(|(e, ..)| e).collect();
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
