use crate::prelude::*;

pub struct TaskPlugin;

impl Plugin for TaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            task_bridge_system
        ).run_if(in_state(GameState::InGame)));
    }
}

/// A bridge system that converts high-level Tasks into the new Atomic Action sequences
fn task_bridge_system(
    mut query: Query<(Entity, &mut Brain, &Position)>,
) {
    for (_entity, mut brain, pos) in query.iter_mut() {
        if brain.task.is_none() || brain.action.is_some() || !brain.action_queue.is_empty() {
            continue;
        }

        let task = brain.task.unwrap();
        match task {
            Task::Maintain => {
                // For now, just a placeholder action
                brain.action_queue.push(AtomicAction::Scan); 
                brain.task = None;
            },
            Task::Social => {
                brain.action_queue.push(AtomicAction::Move(Position { 
                    x: pos.x + rand::rng().random_range(-2..3),
                    y: pos.y + rand::rng().random_range(-2..3), 
                    z: pos.z 
                }));
                brain.task = None;
            }
            _ => {
                brain.task = None;
            }
        }
    }
}

pub fn set_already_targetted(
    entities_that_might_target_things: &Query<(Entity, &mut Brain, &Position, Option<&Pathing>, Option<&Targeting>)>
) -> Vec<Entity> {
    entities_that_might_target_things
        .iter()
        .filter(|(_, _, _, _, targeting)| targeting.is_some())
        .map(|(_, _, _, _, targeting)| targeting.unwrap().target)
        .collect::<Vec<Entity>>()
}

pub fn remove_x_markers(
    commands: &mut Commands,
    workmarkers: &Query<(Entity, &ChildOf), With<WorkMarker>>,
    targetable_entity: Entity,
) {
    for (child, parent) in workmarkers.iter() {
        if parent.0 == targetable_entity {
            commands.entity(child).despawn();
        }
    }
}

