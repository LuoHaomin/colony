use crate::prelude::*;
use crate::selection_systems::SelectionEvent;

// Make plugin.
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
            ).run_if(in_state(GameState::InGame))
        )
        .insert_resource(Dragging { ..default() })
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
        let Ok((camera, camera_transform)) = q_camera.get_single() else { return; };
        let Ok(window) = windows.get_single() else { return; };

        if let Some(screen_pos) = window.cursor_position() {
            if let Some(position) = mouse_to_position(camera, camera_transform, screen_pos) {
                event.write(ObjectFinderEvent { position });
                dragging.dragging = true;
                dragging.start_position = Some(position);
                selection_event.write(SelectionEvent {
                    selected_position: Some(position),
                    selected_type: dragging.looking_for
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
    mut dragging: ResMut<Dragging>,
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut selection_event: MessageWriter<SelectionEvent>,
) {
    if !dragging.dragging { return; }
    let Ok((camera, camera_transform)) = q_camera.get_single() else { return; };
    let Ok(window) = windows.get_single() else { return; };

    if let Some(screen_pos) = window.cursor_position() {
        if let Some(current_position) = mouse_to_position(camera, camera_transform, screen_pos) {
            if dragging.start_position.is_none() {
                dragging.start_position = Some(current_position);
            }
            selection_event.write(SelectionEvent {
                selected_position: Some(current_position),
                selected_type: dragging.looking_for
            });
        }
    }
}

pub fn mouse_move_system(
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut info_panel: ResMut<InfoPanelInformation>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else { return; };
    let Ok(window) = windows.get_single() else { return; };

    if let Some(screen_pos) = window.cursor_position() {
        if let Some(position) = mouse_to_position(camera, camera_transform, screen_pos) {
            info_panel.mouse_position = Some(position);
        }
    }
}

pub fn object_finder_system(
    mut event: MessageReader<ObjectFinderEvent>,
    mut selected_object: ResMut<SelectedObjectInformation>,
    objects: Query<(Entity, &Position, &ActorType)>,
) {
    for e in event.read() {
        let mut found = false;
        for (entity, position, _actor_type) in objects.iter() {
            if position.x == e.position.x && position.y == e.position.y {
                selected_object.entity = Some(entity);
                found = true;
                break;
            }
        }
        if !found {
            selected_object.entity = None;
        }
    }
}

pub fn mouse_to_position(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    screen_pos: Vec2,
) -> Option<Position> {
    let ray = camera.viewport_to_world(camera_transform, screen_pos).ok()?;
    let origin = ray.origin;
    let direction = ray.direction;

    if direction.z.abs() < 1e-6 {
        return None;
    }

    let t = -origin.z / direction.z;
    if t < 0.0 {
        return None;
    }

    let intersection = origin + direction * t;
    Some(Position {
        x: (intersection.x / TILE_SIZE).round() as i32,
        y: (intersection.y / TILE_SIZE).round() as i32,
        z: 0,
    })
}

#[derive(Message)]
pub struct ObjectFinderEvent {
    pub position: Position,
}
