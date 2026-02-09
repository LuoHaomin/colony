use crate::prelude::*;

pub fn task_system_chop(
    mut commands: Commands,
    mut entities_that_might_chop: Query<(Entity, &mut Brain, &Position, Option<&Pathing>, Option<&Targeting>)>,
    targetables: Query<(Entity, &Position, &Choppable)>,
) {
    let mut already_targeted = super::set_already_targetted(&entities_that_might_chop);
    for (entity, mut brain, position, pathing, targeting) in entities_that_might_chop.iter_mut() {
        if pathing.is_some() { continue; }
        if brain.task != Some(Task::Chop) { continue; }

        let mut chopped = false;
        for (targetable_entity, targetable_position, _choppable) in targetables.iter() {
            let distance = position.distance(targetable_position);
            if distance <= 1 && targeting.is_some() && targeting.unwrap().target == targetable_entity {
                commands.entity(targetable_entity).despawn();
                chopped = true;
                break;
            }
        }

        if chopped {
            commands.entity(entity).remove::<Targeting>();
            brain.remotivate();
            continue;
        }

        if targeting.is_some() { continue; }

        let mut nearest_entity: Option<(Entity, Position)> = None;
        let mut nearest_distance = i32::MAX;

        for (targetable_entity, targetable_position, _choppable) in targetables.iter() {
            if already_targeted.contains(&targetable_entity) { continue; }
            let distance = position.distance(targetable_position);
            if distance < nearest_distance {
                nearest_distance = distance;
                nearest_entity = Some((targetable_entity, *targetable_position));
            }
        }

        if let Some((targetable_entity, targetable_position)) = nearest_entity {
            already_targeted.push(targetable_entity);
            commands.entity(entity).insert(Targeting { target: targetable_entity });
            commands.entity(entity).insert(Pathing { 
                path: vec![], 
                destination: targetable_position,
                ..default()
            });
        }
    }
}
