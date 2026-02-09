use crate::prelude::*;

pub fn task_system_eat(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &mut PhysicalBody, &Position, Option<&Pathing>, Option<&Targeting>)>,
    food_query: Query<(Entity, &Position, &Item), With<WorkTarget>>,
) {
    let mut already_targeted = query.iter().filter_map(|(_, _, _, _, _, t)| t.map(|target| target.target)).collect::<Vec<_>>();
    for (entity, mut brain, mut physical_body, position, pathing, targeting) in query.iter_mut() {
        if pathing.is_some() { continue; }
        if brain.task != Some(Task::Eat) { continue; }

        let mut consumed = false;
        for (food_entity, food_position, _item) in food_query.iter() {
            let distance = position.distance(food_position);
            if distance <= 1 && targeting.is_some() && targeting.unwrap().target == food_entity {
                if let Some(needs_food) = &mut physical_body.needs_food {
                    needs_food.current = needs_food.max;
                }
                commands.entity(food_entity).despawn();
                consumed = true;
                break;
            }
        }

        if consumed {
            commands.entity(entity).remove::<Targeting>();
            brain.remotivate();
            continue;
        }

        if targeting.is_some() { continue; }

        let mut nearest_food: Option<(Entity, Position)> = None;
        let mut nearest_distance = i32::MAX;

        for (food_entity, food_position, _item) in food_query.iter() {
            if already_targeted.contains(&food_entity) { continue; }
            let distance = position.distance(food_position);
            if distance < nearest_distance {
                nearest_distance = distance;
                nearest_food = Some((food_entity, *food_position));
            }
        }

        if let Some((food_entity, food_position)) = nearest_food {
            already_targeted.push(food_entity);
            commands.entity(entity).insert(Targeting { target: food_entity });
            commands.entity(entity).insert(Pathing { 
                path: vec![], 
                destination: food_position,
                ..default()
            });
        }
    }
}
