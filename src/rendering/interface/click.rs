use crate::prelude::*;
use crate::rendering::selection_systems::SelectionEvent;

pub struct ClickPlugin;

impl Plugin for ClickPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_message::<ObjectFinderEvent>()
        .add_systems(
            Update,
            ( 
                mouse_click_input,
                mouse_drag_system,
                object_finder_system,
                mouse_move_system
            ).run_if(in_state(GameState::InGame).or(in_state(GameState::Paused)))
        )
        ;
    }
}

pub fn mouse_click_input(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut event: MessageWriter<ObjectFinderEvent>,
    mut dragging: ResMut<Dragging>,
    mut selection_event: MessageWriter<SelectionEvent>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let Some((camera, camera_transform)) = q_camera.iter().next() else { return; };
        let Some(window) = windows.iter().next() else { return; };

        if let Some(screen_pos) = window.cursor_position() {
            if let Some(position) = mouse_to_position(camera, camera_transform, screen_pos) {
                println!("Mouse click at screen {:?}, mapped to grid {:?}", screen_pos, position);
                event.write(ObjectFinderEvent { position });
                dragging.dragging = true;
                dragging.start_position = Some(position);
                selection_event.write(SelectionEvent {
                    selected_position: Some(position),
                    selected_type: dragging.looking_for.clone()
                });
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        dragging.dragging = false;
        dragging.start_position = None;
    }
}

pub fn mouse_drag_system(
    dragging: ResMut<Dragging>,
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut selection_event: MessageWriter<SelectionEvent>,
) {
    if !dragging.dragging { return; }
    let Some((camera, camera_transform)) = q_camera.iter().next() else { return; };
    let Some(window) = windows.iter().next() else { return; };

    if let Some(screen_pos) = window.cursor_position() {
        if let Some(current_position) = mouse_to_position(camera, camera_transform, screen_pos) {
            selection_event.write(SelectionEvent {
                selected_position: Some(current_position),
                selected_type: dragging.looking_for.clone()
            });
        }
    }
}

pub fn mouse_move_system(
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut info_panel: ResMut<InfoPanelInformation>,
) {
    let Some((camera, camera_transform)) = q_camera.iter().next() else { return; };
    let Some(window) = windows.iter().next() else { return; };

    if let Some(screen_pos) = window.cursor_position() {
        if let Some(position) = mouse_to_position(camera, camera_transform, screen_pos) {
            info_panel.mouse_position = Some(position);
        }
    }
}

pub fn mouse_to_position(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    screen_pos: Vec2,
) -> Option<Position> {
    // 2D Camera viewport to world
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) else { return None; };
    
    Some(Position {
        x: (world_pos.x / TILE_SIZE).round() as i32,
        y: (world_pos.y / TILE_SIZE).round() as i32,
        z: 0,
    })
}

pub fn object_finder_system(
    mut commands: Commands,
    mut event: MessageReader<ObjectFinderEvent>,
    mut selected_object: ResMut<SelectedObjectInformation>,
    objects: Query<(Entity, &Position, Option<&MapTile>)>,
    already_clicked: Query<Entity, With<ClickedOn>>,
    current_z: Res<CurrentDisplayZ>,
) {
    for e in event.read() {
        println!("Click detected at grid: {}, {}", e.position.x, e.position.y);
        
        // Clear previous selection
        for entity in already_clicked.iter() {
            commands.entity(entity).remove::<ClickedOn>();
        }

        let mut found_entity = None;
        let mut found_tile = None;

        // Find the "best" object at this coordinate.
        // We look for units on ANY layer first, then the specific layer tile.
        for (entity, position, is_tile) in objects.iter() {
            if position.x == e.position.x && position.y == e.position.y {
                if is_tile.is_some() {
                    // Only match tiles on current display layer
                    if position.z == current_z.z {
                        found_tile = Some(entity);
                    }
                } else {
                    // Match units if they are at or below current layer (visible)
                    if position.z <= current_z.z {
                        found_entity = Some(entity);
                    }
                }
            }
        }
        
        let target = found_entity.or(found_tile);
        if let Some(entity) = target {
            println!("Selected entity: {:?}", entity);
            selected_object.entity = Some(entity);
            commands.entity(entity).insert(ClickedOn);
        } else {
            selected_object.entity = None;
        }
    }
}

#[derive(Message)]
pub struct ObjectFinderEvent {
    pub position: Position,
}

