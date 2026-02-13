use crate::prelude::*;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, action_processor_system.run_if(in_state(GameState::InGame)));
    }
}

pub fn action_processor_system(
    mut commands: Commands,
    mut actors: Query<(Entity, &mut Brain, &mut Position, &mut Transform, Option<&Genome>)>,
    mut physics: Query<(Option<&mut PhysicalBody>, &MaterialProperties)>,
) {
    for (entity, mut brain, mut pos, mut transform, _genome) in actors.iter_mut() {
        if brain.action.is_none() && !brain.action_queue.is_empty() {
            brain.action = Some(brain.action_queue.remove(0));
        }

        if let Some(action) = brain.action {
            match action {
                AtomicAction::Move(dest) => {
                    let mut reached = true;
                    // Move 1 unit towards destination
                    if pos.x < dest.x { pos.x += 1; reached = false; }
                    else if pos.x > dest.x { pos.x -= 1; reached = false; }
                    if pos.y < dest.y { pos.y += 1; reached = false; }
                    else if pos.y > dest.y { pos.y -= 1; reached = false; }
                    if pos.z < dest.z { pos.z += 1; reached = false; }
                    else if pos.z > dest.z { pos.z -= 1; reached = false; }
                    
                    if reached {
                        brain.action = None;
                    }
                    *transform = pos.to_transform();
                },
                AtomicAction::ApplyForce(target_entity, force) => {
                    if let Ok((target_body, material)) = physics.get_mut(target_entity) {
                        let damage = (force - material.hardness).max(0.1);
                        
                        // Add visual feedback to target
                        commands.entity(target_entity).insert(VisualFeedback {
                            shake_timer: 0.2,
                            shake_intensity: 2.0,
                            ..default()
                        });

                        if let Some(mut b) = target_body {
                            // Target is biological/has health
                            b.health -= damage;
                        } else {
                            // Target is inanimate - maybe damage its mass/toughness?
                            // For now just despawn if force is enough
                            if damage > material.toughness {
                                commands.entity(target_entity).despawn();
                            }
                        }
                    }
                    brain.action = None; 
                },
                AtomicAction::Consume(target_entity) => {
                    // We need both actor's body and target's body
                    if let Ok([(Some(mut actor_body), _), (target_body_opt, target_material)]) = physics.get_many_mut([entity, target_entity]) {
                        let energy_to_take = 5.0; // Base value
                        let actual_energy = energy_to_take * target_material.energy_density;
                        
                        if let Some(mut b) = target_body_opt {
                            b.energy_storage -= energy_to_take;
                            if b.energy_storage <= 0.0 {
                                // Consumed it completely
                                commands.entity(target_entity).despawn();
                            }
                        } else {
                            // Consuming inanimate material
                            commands.entity(target_entity).despawn();
                        }
                        
                        actor_body.energy_storage = (actor_body.energy_storage + actual_energy).min(actor_body.energy_max);
                        info!("Actor {:?} consumed energy. New storage: {}", entity, actor_body.energy_storage);
                    }
                    brain.action = None;
                },
                AtomicAction::Link(_, _) => {
                    // Logic for carrying/combining
                    brain.action = None;
                },
                AtomicAction::Scan => {
                    // Memory populating logic
                    brain.action = None;
                }
            }
        }
    }
}
