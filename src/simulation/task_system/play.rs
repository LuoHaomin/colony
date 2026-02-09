use crate::prelude::*;

pub fn task_system_play(
    mut query: Query<(Entity, &mut Brain, &mut PhysicalBody)>,
) {
    for (_entity, mut brain, mut physical_body) in query.iter_mut() {
        if brain.task != Some(Task::Play) { continue; }
        if let Some(n) = physical_body.needs_entertainment.as_mut() {
            n.current += n.rate * 2.0;
            if n.current >= n.max {
                n.current = n.max;
                brain.remotivate();
            }
        }
    }
}
