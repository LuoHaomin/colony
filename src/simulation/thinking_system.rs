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
        if brain.task.is_some() { continue; }
        
        if brain.motivation.is_none() {
            // Check needs
            if let Some(n) = &physical_body.needs_food {
                if n.current < n.low {
                    brain.motivation = Some(Motivation::Hunger);
                }
            }
            if brain.motivation.is_none() {
                if let Some(n) = &physical_body.needs_sleep {
                    if n.current < n.low {
                        brain.motivation = Some(Motivation::Tired);
                    }
                }
            }
            if brain.motivation.is_none() {
                if let Some(n) = &physical_body.needs_entertainment {
                    if n.current < n.low {
                        brain.motivation = Some(Motivation::Bored);
                    }
                }
            }
            
            if brain.motivation.is_none() {
                brain.motivation = Some(Motivation::Idle);
            }
        }

        if let Some(m) = brain.motivation {
            match m {
                Motivation::Hunger => brain.task = Some(Task::Eat),
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
