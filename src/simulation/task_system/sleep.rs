use crate::prelude::*;

pub fn task_system_sleep(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &mut PhysicalBody, &Position, Option<&Pathing>, Option<&Targeting>)>,
    bed_query: Query<(Entity, &Position), With<Bed>>,
) {
    for (entity, mut brain, mut physical_body, position, pathing, _targeting) in query.iter_mut() {
        if pathing.is_some() { continue; }
        if brain.task != Some(Task::Sleep) { continue; }

        let mut in_bed = false;
        for (_bed_entity, bed_position) in bed_query.iter() {
            if position.distance(bed_position) <= 1 {
                in_bed = true;
                break;
            }
        }

        if in_bed {
            if let Some(n) = physical_body.needs_sleep.as_mut() {
                n.current += n.rate * 2.0;
                if n.current >= n.max {
                    n.current = n.max;
                    brain.remotivate();
                }
            }
            continue;
        }

        let mut closest_distance = -1;
        let mut closest_position = None;
        for (_bed_entity, bed_position) in bed_query.iter() {
            let distance = position.distance(bed_position);
            if closest_distance == -1 || distance < closest_distance {
                closest_distance = distance;
                closest_position = Some(bed_position);
            }
        }

        if let Some(bed_pos) = closest_position {
            commands.entity(entity).insert(Pathing { path: vec![], destination: *bed_pos });
        } else {
            brain.task = Some(Task::Sleeping); // Just sleep where you are
        }
    }
}
