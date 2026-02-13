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
    mut query: Query<(Entity, &mut Brain, &PhysicalBody, &Position, Option<&Genome>, Option<&Children>)>,
    targets: Query<(Entity, &Position, &MaterialProperties), Without<Brain>>,
) {
    for (entity, mut brain, physical_body, current_pos, genome, children) in query.iter_mut() {
        // If already busy with an action or task, skip
        if brain.action.is_some() || !brain.action_queue.is_empty() { continue; }
        if brain.task.is_some() { continue; }

        if !brain.task_queue.is_empty() {
            brain.task = Some(brain.task_queue.remove(0));
            continue;
        }
        
        let mut motivations = Vec::new();

        // Use default weights if no genome (legacy/static entities)
        let w_hunger = genome.map_or(1.0, |g| g.weight_hunger);
        let w_fatigue = genome.map_or(1.0, |g| g.weight_fatigue);
        let w_social = genome.map_or(1.0, |g| g.weight_social);

        // check if has any tool
        let has_tool = children.map_or(false, |c| !c.is_empty());

        // 1. Hunger Assessment
        let hunger_score = (1.0 - physical_body.energy_storage / physical_body.energy_max) * 100.0;
        if physical_body.energy_storage < physical_body.energy_max * 0.8 {
             motivations.push((Motivation::Hunger, hunger_score * 1.5 * w_hunger));
        }

        // TOOL GATHERING: If no tool, look for one
        if !has_tool {
            motivations.push((Motivation::Bored, 30.0)); // Boredom weight used as placeholder for "Tool search"
        }

        // 2. Fatigue (Sleep)
        if let Some(n) = &physical_body.needs_sleep {
            let energy_score = (1.0 - n.current / n.max) * 100.0;
            if n.current < n.low {
                motivations.push((Motivation::Tired, energy_score * 2.0 * w_fatigue));
            }
        }

        // 3. Social/Boredom
        if let Some(n) = &physical_body.needs_entertainment {
            let boredom_score = (1.0 - n.current / n.max) * 100.0;
            if n.current < n.low {
                motivations.push((Motivation::Bored, boredom_score * w_social));
            }
        }

        motivations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let best_motivation = motivations.first().map(|(m, _)| *m).unwrap_or(Motivation::Idle);
        brain.motivation = Some(best_motivation);

        match best_motivation {
            Motivation::Hunger => {
                // Look for nearby high energy density items
                let mut best_target: Option<(Entity, Position)> = None;
                let mut max_score = -1.0;

                for (t_entity, t_pos, t_material) in targets.iter() {
                    if t_material.energy_density > 0.1 {
                        let dist = current_pos.distance(t_pos) as f32;
                        let score = t_material.energy_density / (dist * 0.1 + 1.0);
                        if score > max_score {
                            max_score = score;
                            best_target = Some((t_entity, *t_pos));
                        }
                    }
                }

                if let Some((target_id, target_pos)) = best_target {
                    // Generate atomic sequence: Move -> Consume
                    brain.action_queue.push(AtomicAction::Move(target_pos));
                    brain.action_queue.push(AtomicAction::Consume(target_id));
                    info!("Entity {:?} decided to get food at {:?}", entity, target_pos);
                } else {
                    // No food found, meander
                    brain.action_queue.push(AtomicAction::Move(Position { 
                        x: current_pos.x + rand::rng().random_range(-5..6),
                        y: current_pos.y + rand::rng().random_range(-5..6),
                        z: current_pos.z 
                    }));
                }
            },
            Motivation::Bored if !has_tool => {
                // Look for tool (hard item without energy)
                let mut best_tool: Option<(Entity, Position)> = None;
                let mut max_score = -1.0;

                for (t_entity, t_pos, t_material) in targets.iter() {
                    if t_material.hardness > 2.0 && t_material.energy_density < 0.1 {
                        let dist = current_pos.distance(t_pos) as f32;
                        let score = t_material.hardness / (dist * 0.1 + 1.0);
                        if score > max_score {
                            max_score = score;
                            best_tool = Some((t_entity, *t_pos));
                        }
                    }
                }

                if let Some((target_id, target_pos)) = best_tool {
                    brain.action_queue.push(AtomicAction::Move(target_pos));
                    brain.action_queue.push(AtomicAction::Link(entity, target_id));
                    info!("Entity {:?} decided to pick up tool at {:?}", entity, target_pos);
                }
            },
            Motivation::Idle => {
                brain.action_queue.push(AtomicAction::Move(Position { 
                    x: current_pos.x + rand::rng().random_range(-3..4),
                    y: current_pos.y + rand::rng().random_range(-3..4),
                    z: current_pos.z 
                }));
            },
            _ => {
                // Fallback or generic higher-level assignment
                match best_motivation {
                    Motivation::Tired => brain.task = Some(Task::Maintain),
                    Motivation::Bored => brain.task = Some(Task::Social),
                    _ => brain.task = Some(Task::Idle),
                }
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
