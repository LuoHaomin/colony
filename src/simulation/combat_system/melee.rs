use crate::prelude::*;

pub fn combat_system_melee(
    mut commands: Commands,
    mut entities_that_might_fight: Query<(Entity, &mut Brain, &mut PhysicalBody, &Position, Option<&mut Pathing>, Option<&Targeting>, Option<&Genome>)>,
    attackables: Query<(Entity, &Position, Option<&Genome>), With<Brain>>,
    sprite_sheet: Res<SpriteSheet>,
) {
    for (e, mut brain, mut physical_body, position, pathing, targeting, genome) in entities_that_might_fight.iter_mut() {
        if brain.task != Some(Task::Fight) && brain.task != Some(Task::Hunt) { continue; }
        if let Some(targeting) = targeting {
            let mut entity_found = false;
            for (entity, target_position, _target_genome) in attackables.iter() {
                if entity == targeting.target {
                    entity_found = true;
                    if position.distance(target_position) <= 1 {
                        let sprite = Sprite::from_atlas_image(
                            sprite_sheet.handle.clone(),
                            TextureAtlas { layout: sprite_sheet.layout.clone(), index: StrikeType::Hit.sprite_index() },
                        );
                        commands.spawn((sprite, Transform::from_xyz(target_position.x as f32 * TILE_SIZE, target_position.y as f32 * TILE_SIZE, target_position.z as f32 * TILE_SIZE)))
                            .insert(target_position.clone())
                            .insert(target_position.to_transform_layer(1.1))
                            .insert( TemporaryVisualElement { duration: 0.2 } );
                        commands.entity(entity).insert(Attacked { attacker: e });
                        if pathing.is_some() { commands.entity(e).remove::<Pathing>(); }
                    } else {
                        // Try to follow/hunt the entity.
                        if pathing.is_none() {
                            commands.entity(e).insert( Pathing { path: vec![], destination: *target_position, ..default() });
                        } else {
                            let mut path = pathing.unwrap();
                            // path.destination = *target_position;
                            path.moving_target = true;
                            //path.path = vec![];
                        }
                    }
                    break;
                }
            }
            if !entity_found {
                commands.entity(e).remove::<Targeting>();
                brain.remotivate();
            }
        } else {
            // Find a target.
            // Humans might find a target based on if they're hunting or defending.
            // Animals might find a target based on if they're hungry or defending.
            // For now just find the nearest physical body and make that the target.
            if let Some(danger) = &physical_body.danger {
                if danger.danger_type == DangerType::Attacked {
                    // If we're being attacked, we should attack back.
                    // Error: What happens after you win the fight? Or if the attacker no longer exists?
                    if let Some(danger_source) = danger.danger_source {
                        // check if attackables contains danger_source
                        let mut danger_source_found = false;
                        for (entity, _target_position, _target_genome) in attackables.iter() {
                            if entity == danger_source {
                                danger_source_found = true;
                                break;
                            }
                        }
                        if !danger_source_found {
                            // The danger source no longer exists. We should stop attacking.
                            brain.remotivate();
                            physical_body.danger = None;
                            continue;
                        }
                        commands.entity(e).insert(Targeting { target: danger_source });
                        continue;
                    }
                }
            }
            let mut closest_distance = 9999;
            let mut closest_target = None;
            let mut closest_position = None;
            
            let sensory_range = genome.map(|g| g.sensory_range).unwrap_or(15.0) as i32;

            for (attackable, attackable_position, target_genome) in attackables.iter() {
                if attackable == e { continue; }
                
                let distance = position.distance(attackable_position);
                if distance > sensory_range { continue; }

                // Hunting logic vs Fighting logic
                if brain.task == Some(Task::Hunt) {
                    if let (Some(g1), Some(g2)) = (genome, target_genome) {
                        // Don't hunt things that are genetically too close (same species)
                        if g1.genetic_distance(g2) < 0.2 { continue; }
                    }
                }

                if distance < closest_distance {
                    closest_distance = distance;
                    closest_target = Some(attackable);
                    closest_position = Some(attackable_position);
                }
            }
            if let Some(closest_target) = closest_target {
                commands.entity(e).insert(Targeting { target: closest_target });
                let target_position = closest_position.unwrap();
                commands.entity(e).insert( Pathing { path: vec![], destination: *target_position, ..default() });
            } else {
                // Nothing to attack. Now what?
                brain.remotivate();
            }
        }
    }
}

fn do_melee_damage(
    commands: &mut Commands,
    attacker_entity: Option<Entity>,
    attacked_entity: Entity,
    attacker_body: &mut PhysicalBody,
    attacker_genome: Option<&Genome>,
    body2: &mut PhysicalBody,
    _asset_server: &Res<AssetServer>
) {
    let damage = (
        1 +
        (attacker_body.attributes.strength - body2.attributes.constitution).max(0).min(20) +
        (attacker_body.skillset.brawling.level()).max(0).min(20)
    ) as f32;
    body2.health -= damage;
    if body2.health <= 0.0 {
        // Predation energy gain
        if let Some(genome) = attacker_genome {
            // Carnivores gain energy from kills
            let energy_gain = (body2.energy_max * 0.5 + body2.energy_storage) * genome.diet_type;
            attacker_body.energy_storage = (attacker_body.energy_storage + energy_gain).min(attacker_body.energy_max);
            info!("Predator {:?} consumed target for {} energy", attacker_entity, energy_gain);
        }

        commands.entity(attacked_entity).despawn();
    }
    body2.danger = Some(Danger {
        danger_type: DangerType::Attacked,
        danger_source: attacker_entity,
    });
}

pub fn attacked_entities_system(
    mut commands: Commands,
    attacked_query: Query<(Entity, &Attacked), With<Attacked>>,
    mut physical_bodies: Query<(Entity, &mut PhysicalBody, Option<&Genome>)>,
    asset_server: Res<AssetServer>
) {
    for (attacked_entity, attack_info) in attacked_query.iter() {
        commands.entity(attacked_entity).remove::<Attacked>();
        
        // Use unsafe split or two-pass to get both bodies
        let mut attacker_data: Option<(Entity, PhysicalBody, Option<Genome>)> = None;
        for (entity, body, genome) in physical_bodies.iter() {
            if entity == attack_info.attacker {
                attacker_data = Some((entity, body.clone(), genome.cloned()));
                break;
            }
        }

        if let Some((a_entity, mut a_body, a_genome)) = attacker_data {
            if let Ok((_v_entity, mut v_body, _v_genome)) = physical_bodies.get_mut(attacked_entity) {
                do_melee_damage(&mut commands, Some(a_entity), attacked_entity, &mut a_body, a_genome.as_ref(), &mut v_body, &asset_server);
                
                // Write back attacker body changes (energy gain)
                if let Ok((_, mut entry_body, _)) = physical_bodies.get_mut(a_entity) {
                    *entry_body = a_body;
                }
            }
        }
    }
}

pub fn temporary_visual_elements_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TemporaryVisualElement)>,
) {
    let delta_seconds = time.delta_secs();
    for (entity, mut tve) in query.iter_mut() {
        tve.duration -= delta_seconds;
        if tve.duration <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

