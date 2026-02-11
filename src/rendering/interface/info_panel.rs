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
    // Force update if changed
    if object_info.is_changed() {
        println!("INFO PANEL: Detected change. Updating UI with {} lines of info.", object_info.info.len());
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

                // Needs
                if !object_info.needs.is_empty() {
                    parent.spawn((
                        Text::new("-- Needs --"),
                        TextFont { font: font.0.clone(), font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 1.0)),
                        InspectorItem,
                    ));
                    for line in &object_info.needs {
                        parent.spawn((
                            Text::new(line.clone()),
                            TextFont { font: font.0.clone(), font_size: 14.0, ..default() },
                            TextColor(Color::WHITE),
                            InspectorItem,
                        ));
                    }
                }
            });
        }
    }
}

pub fn info_system(
    mut clickable: Query<(Entity, &Position, Option<&PhysicalBody>, Option<&Brain>, Option<&HasName>, Option<&Genome>, Option<&Generation>, Option<&EnvironmentalData>), With<ClickedOn>>,
    mut info_panel: ResMut<InfoPanelInformation>,
) {
    if let Some((_entity, position, physical_body, brain, has_name, genome, generation, env_data)) = clickable.iter_mut().last() {
        let name = has_name.map(|h| h.name.clone()).unwrap_or_else(|| "Object".to_string());
        let mut info = Vec::new();
        let mut needs = Vec::new();
        let mut attributes = Vec::new();
        let mut skills = Vec::new();
        
        info.push(format!("Grid Pos: [{}, {}, {}]", position.x, position.y, position.z));

        if let Some(pb) = physical_body {
            info.push(format!("Energy: {:.1}/{:.1}", pb.energy_storage, pb.energy_max));
            needs.extend(pb.info_panel_needs());
            attributes.extend(pb.info_panel_attributes());
            skills.extend(pb.info_panel_skills());
        }
        
        if let Some(b) = brain {
            info.extend(b.info_panel());
        }

        if let Some(gen) = generation {
             info.push(format!("Generation: {}", gen.value));
        }

        if let Some(g) = genome {
            info.push(format!("-- Genome --"));
            info.push(format!("Size: {:.2}", g.size));
            info.push(format!("Mobility: {:.2}", g.mobility));
            info.push(format!("Metabolic: {:.2}", g.metabolic_efficiency));
            info.push(format!("Diet: {:.2} (0=Light, 1=Meat)", g.diet_type));
            info.push(format!("Aggression: {:.2}", g.aggression));
        }

        if let Some(env) = env_data {
            info.push(format!("-- Environment --"));
            info.push(format!("Temp: {:.1} C", env.temperature));
            info.push(format!("Humidity: {:.1}%", env.humidity * 100.0));
            info.push(format!("Fertility: {:.1}%", env.fertility * 100.0));
        }

        // IMPORTANT: Bevy Resources mark themselves as changed if you just access them mutably.
        // We set the values here which will trigger is_changed() in update_inspector.
        info_panel.name = name;
        info_panel.info = info;
        info_panel.needs = needs;
        info_panel.attributes = attributes;
        info_panel.skills = skills;
    } else {
        // Clear panel if nothing is selected
        if info_panel.name != "Selection" {
            info_panel.name = "Selection".to_string();
            info_panel.info = vec!["Click something to inspect".to_string()];
            info_panel.needs = Vec::new();
            info_panel.attributes = Vec::new();
            info_panel.skills = Vec::new();
        }
    }
}
