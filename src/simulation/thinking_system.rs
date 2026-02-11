use crate::prelude::*;

pub struct ThinkingPlugin;

impl Plugin for ThinkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            thinking_system,
            remotivate_system
        ).run_if(in_state(GameState::InGame)));
    }
}

pub fn thinking_system(
    mut query: Query<(Entity, &mut Brain, &PhysicalBody)>,
) {
    for (_entity, mut brain, physical_body) in query.iter_mut() {
        // If we have a task and it's not finished, keep at it.
        // In the future, we might interrupt tasks if a higher priority motivation emerges.
        if brain.task.is_some() { continue; }

        // If task is finished but queue is not empty, pop next task
        if !brain.task_queue.is_empty() {
            brain.task = Some(brain.task_queue.remove(0));
            continue;
        }
        
        // Evaluate motivations based on needs with weights
        let mut motivations = Vec::new();

        // Hunger: Priority increases as current decreases
        if let Some(n) = &physical_body.needs_food {
            let hunger_score = (1.0 - n.current / n.max) * 100.0;
            if n.current < n.low {
                motivations.push((Motivation::Hunger, hunger_score * 1.5));
            } else if n.current < n.normal {
                motivations.push((Motivation::Hunger, hunger_score * 0.5));
            }
        }

        // Tired: Priority increases as current decreases
        if let Some(n) = &physical_body.needs_sleep {
            let energy_score = (1.0 - n.current / n.max) * 100.0;
            if n.current < n.low {
                motivations.push((Motivation::Tired, energy_score * 2.0));
            } else if n.current < n.normal {
                motivations.push((Motivation::Tired, energy_score * 0.3));
            }
        }

        // Boredom
        if let Some(n) = &physical_body.needs_entertainment {
            let boredom_score = (1.0 - n.current / n.max) * 100.0;
            if n.current < n.low {
                motivations.push((Motivation::Bored, boredom_score));
            }
        }

        // Pick highest score motivation
        motivations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        if let Some((best_m, _)) = motivations.first() {
            brain.motivation = Some(*best_m);
        } else {
            brain.motivation = Some(Motivation::Idle);
        }

        if let Some(m) = brain.motivation {
            match m {
                Motivation::Hunger => {
                    // Complex task sequence example:
                    // In a real refactor, "Eat" would pop a sequence like [FindFood, MoveTo, Consume]
                    brain.task = Some(Task::Eat);
                },
                Motivation::Tired => brain.task = Some(Task::Sleep),
                Motivation::Bored => brain.task = Some(Task::Play),
                Motivation::Idle => brain.task = Some(Task::Meander),
                _ => brain.task = Some(Task::Idle),
            }
        }
    }
}

pub fn remotivate_system(mut query: Query<&mut Brain>) {
    for mut brain in query.iter_mut() {
        // Randomly remotivate
        if rand::rng().random_bool(0.1) {
            brain.remotivate();
        }
    }
}
