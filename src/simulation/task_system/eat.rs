use crate::prelude::*;

pub fn task_system_eat(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &mut PhysicalBody, &Position, Option<&Pathing>, Option<&Targeting>)>,
    food_query: Query<(Entity, &Position, &Item), With<WorkTarget>>,
    workmarkers: Query<(Entity, &ChildOf), With<WorkMarker>>,
) {
    let mut already_targeted = super::set_already_targetted(&query);
    for (entity, mut brain, mut physical_body, position, pathing, targeting) in query.iter_mut() {
        if pathing.is_some() { continue; }
        if brain.task != Some(Task::Eat) { continue; }

        let mut consumed = false;
        for (food_entity, food_position, _item) in food_query.iter() {
            if position.distance(food_position) <= 1 {
                if let Some(n) = physical_body.needs_food.as_mut() {
                    n.current = n.max;
                }
                commands.entity(food_entity).despawn_recursive();
                super::remove_x_markers(&mut commands, &workmarkers, food_entity);
                consumed = true;
                break;
            }
        }

        if consumed {
            commands.entity(entity).remove::<Targeting>();
            if brain.motivation == Some(Motivation::Hunger) || brain.motivation == Some(Motivation::Eat) {
                brain.remotivate();
            }
            continue;
        }

        let mut closest_distance = -1;
        let mut closest_entity = None;
        let mut closest_position = None;

        for (food_entity, food_position, _item) in food_query.iter() {
            if already_targeted.contains(&food_entity) { continue; }
            let distance = position.distance(food_position);
            if closest_distance == -1 || distance < closest_distance {
                closest_distance = distance;
                closest_entity = Some(food_entity);
                closest_position = Some(food_position);
            }
        }

        if let Some(closest_entity) = closest_entity {
            commands.entity(entity).insert(Targeting { target: closest_entity });
            commands.entity(entity).insert(Pathing { path: vec![], destination: *closest_position.unwrap() });
            already_targeted.push(closest_entity);
        } else {
            brain.remotivate();
        }
    }
}
