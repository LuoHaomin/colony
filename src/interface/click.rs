use crate::prelude::*;
use crate::selection_systems::SelectionEvent;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::prelude::Parent;

// Make plugin.
pub struct ClickPlugin;

impl Plugin for ClickPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ObjectFinderEvent>()
        .add_systems(
            Update,
            ( mouse_click_input.run_if(in_state(GameState::InGame)),
            mouse_drag_system.run_if(in_state(GameState::InGame)),
            object_finder_system.run_if(in_state(GameState::InGame)),
            mouse_move_system.run_if(in_state(GameState::InGame)) )
        )
        // .add_system(
        //     mouse_click_input
        //     .run_if(in_state(GameState::InGame))
        // )
        // .add_system(
        //     mouse_drag_system
        //     .run_if(in_state(GameState::InGame))
        // )
        // .add_system(
        //     object_finder_system
        //     .run_if(in_state(GameState::InGame))
        // )
        // .add_system(
        //     mouse_move_system
        //     .run_if(in_state(GameState::InGame))
        // )
        // .add_system_set(
        //     SystemSet::on_update(GameState::InGame)
        //         .with_system(mouse_click_input),
        // )
        // .add_system_set(
        //     SystemSet::on_update(GameState::InGame)
        //         .with_system(mouse_drag_system),
        // )
        // .add_system_set(
        //     SystemSet::on_update(GameState::InGame)
        //         .with_system(object_finder_system),
        // )
        // .add_system(mouse_move_system)
        .insert_resource(Dragging { ..default() })
        ;
    }
}

pub fn mouse_click_input(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&mut Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut event: EventWriter<ObjectFinderEvent>,
    mut dragging: ResMut<Dragging>,
    mut selection_event: EventWriter<SelectionEvent>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single().expect("No camera");
        let window = windows.single().expect("No primary window");
        let mut position = None;
        let wc = window.cursor_position();
        if let Some(wc) = wc {
            let y = window.height() - wc.y;
            // ?, Chop, Wand, Arrow, Leaf, Legs
            if y < 32.0 {
                if (0..32).contains(&(wc.x as i32)) {
                    dragging.looking_for = SelectableType::Foragable;
                }
                if (32..64).contains(&(wc.x as i32)) {
                    dragging.looking_for = SelectableType::Choppable;
                }
                if (64..96).contains(&(wc.x as i32)) {
                    dragging.looking_for = SelectableType::Unselecting;
                }
                if (96..128).contains(&(wc.x as i32)) {
                    dragging.looking_for = SelectableType::Zoning;
                }
                if (128..160).contains(&(wc.x as i32)) {
                    dragging.looking_for = SelectableType::Unzoning;
                }
                if (160..192).contains(&(wc.x as i32)) {
                    dragging.looking_for = SelectableType::Unselecting;
                }
                if (192..224).contains(&(wc.x as i32)) {
                    dragging.looking_for = SelectableType::Unselecting;
                }
                return;
            }
            if y < 164.0 { return; }
        }
            if let Some(screen_pos) = window.cursor_position() {
                position = Some(mouse_to_position(camera, camera_transform, window, screen_pos));
            }
        if let Some(position) = position {
            event.send(ObjectFinderEvent { position });
            dragging.dragging = true;
            dragging.start_position = Some(position);
        }
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        // Maybe we send an event here? Or we iterate here through all "Highlighted" and mark them "Selected"?
        dragging.dragging = false;
        selection_event.send(SelectionEvent);
    }
}

pub fn mouse_drag_system(
    mut commands: Commands,
    windows: Query<&mut Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    dragging: Res<Dragging>,
    positions: Query<(Entity, &Position, Option<&Highlighted>)>,
    // highlights: Query<(Entity, &Position), With<Highlighted>>,
    highlightboxes: Query<(Entity, &Parent), With<HighlightBox>>
) {
    // Yes, this runs all the time.
    // But it's not a problem, because it's a no-op if we're not dragging.
    if !dragging.dragging { return; }
    if dragging.start_position.is_none() { return; }
    let start_position = dragging.start_position.unwrap();
    let (camera, camera_transform) = q_camera.single().expect("No camera");
    let window = windows.single().expect("No primary window");
    let mut end_position = None;
    if let Some(screen_pos) = window.cursor_position() {
        end_position = Some(mouse_to_position(camera, camera_transform, window, screen_pos));
    }
    if end_position.is_none() { return; }
    let end_position = end_position.unwrap();
    // Now just take all objects with a position that matches and mark them as "Highlighted".
    // Somehow only allow the types I want to be highlighted. Foragable. Unit. Choppable. Food. Storable.
    for (entity, pos, highlighted) in positions.iter() {
        if (start_position.x.min(end_position.x) <= pos.x) && (pos.x <= start_position.x.max(end_position.x) && (start_position.y.min(end_position.y) <= pos.y) && (pos.y <= start_position.y.max(end_position.y))) {
            if highlighted.is_some() { continue; }
            let highlight_box = commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.2),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, pos.z as f32 + 0.1),//pos.x as f32, pos.y as f32, pos.z as f32 + 0.1),
                ..default()
            }).insert(HighlightBox).id();
            commands.entity(entity).insert(Highlighted );
            commands.entity(entity).add_child(highlight_box);
        } else {
            if highlighted.is_none() { continue; }
            commands.entity(entity).remove::<Highlighted>();
            for (highlight_box, parent) in highlightboxes.iter() {
                if parent.get() == entity {
                    commands.entity(highlight_box).despawn();
                }
            }
        }
    }
}

pub fn mouse_move_system(
    windows: Query<&mut Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    // dragging: Res<Dragging>, Use to only highlight a specific type in the future??
    positions: Query<(Entity, &Position, Option<&Brain>, Option<&Food>, Option<&Plant>)>,
    mut object_info: ResMut<SelectedObjectInformation>,
) {
    let (camera, camera_transform) = q_camera.single().expect("No camera");
    let window = windows.single().expect("No primary window");
    let mut pos = None;
    if let Some(screen_pos) = window.cursor_position() {
        pos = Some(mouse_to_position(camera, camera_transform, window, screen_pos));
    }
    if pos.is_none() { return; }
    let pos = pos.unwrap();
    // Append info for each object to the SelectedObjectInfo.
    object_info.info = vec![];
    for (_e, p, b, f, plant) in positions.iter() {
        if (p.x == pos.x) && (p.y == pos.y) {
            if let Some(f) = f {
                object_info.info.push(f.hover_note());
            }
            if let Some(plant) = plant {
                object_info.info.push(plant.hover_note());
            }
            if let Some(brain) = b {
                if let Some(task) = brain.task {
                    object_info.info.push(format!("Task: {:?}", task));
                }
                if let Some(motivation) = brain.motivation {
                    object_info.info.push(format!("Motivation: {:?}", motivation));
                }
            }
        }
    }
}

pub fn object_finder_system(
    mut commands: Commands,
    mut event: EventReader<ObjectFinderEvent>,
    mut people: Query<(Entity, &Position, &mut Brain, Option<&PhysicalBody>, Option<&ClickedOn>)>,
) {
    for event in event.iter() {
        for (entity, position, _brain, physical_body, clickedon) in people.iter_mut() {
            if clickedon.is_some() {
                commands.entity(entity).remove::<ClickedOn>();
                continue;
            }
            if position == &event.position {
                if physical_body.is_some() {
                    commands.entity(entity).insert(ClickedOn);
                }
            }
        }
    }
}

#[derive(Event)]
pub struct ObjectFinderEvent {
    pub position: Position
}

fn mouse_to_position(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window: &Window,
    screen_pos: Vec2,
) -> Position {
    // Simplified mapping: convert screen coordinates directly to tile coordinates.
    // Note: this is an approximation that assumes a 1:1 mapping between screen pixels and world units.
    let window_size_y = window.height();
    let world_x = screen_pos.x;
    let world_y = window_size_y - screen_pos.y;
    Position { x: (world_x / TILE_SIZE) as i32, y: (world_y / TILE_SIZE) as i32, z: 0 }
}