use crate::prelude::*;

pub struct NeedsPlugin;

impl Plugin for NeedsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            needs_status_system
            .run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_secs_f32(1.0))),
            metabolic_drain_system
            .run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_secs_f32(0.5))), // 2Hz metabolic tick
            photosynthesis_system
            .run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_secs_f32(0.5))),
        ).run_if(in_state(GameState::InGame)));
    }
}

pub fn photosynthesis_system(
    mut query: Query<(&Position, &mut PhysicalBody, &Genome)>,
    tile_env: Res<TileEnvHash>,
) {
    for (pos, mut body, genome) in query.iter_mut() {
        let phos_factor = 1.0 - genome.diet_type;
        if phos_factor <= 0.0 { continue; }

        if let Some(env) = tile_env.hash.get(pos) {
            let gain = env.fertility * 0.2 * phos_factor;
            body.energy_storage += gain;
            if body.energy_storage > body.energy_max {
                body.energy_storage = body.energy_max;
            }
        }
    }
}

pub fn metabolic_drain_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PhysicalBody, Option<&Genome>)>
) {
    for (entity, mut body, genome) in query.iter_mut() {
        // Base cost calculation
        let base_cost = 0.1; // Energy per metabolic tick
        let mut total_cost = base_cost;

        if let Some(g) = genome {
            // Cost scale with Size and Mobility
            total_cost *= g.size.powf(1.5) * 0.7 + g.mobility * 0.3 + 0.1;
        }

        body.energy_storage -= total_cost;

        // Death check
        if body.energy_storage <= 0.0 {
            body.energy_storage = 0.0;
            // Kill the entity
            // In the future, we convert it to debris/meat
            commands.entity(entity).despawn();
        }
        
        // Backwards compatibility sync for UI
        if body.energy_max > 0.0 {
            let percentage = body.energy_storage / body.energy_max;
            if let Some(need) = body.needs_food.as_mut() {
                need.current = percentage * need.max;
            }
        }
    }
}

pub fn needs_status_system(
    mut query: Query<&mut PhysicalBody>
) {
    for mut physical_body in query.iter_mut() {
        if let Some(needs_food) = physical_body.needs_food.as_mut() {
            needs_food.current -= needs_food.rate;
            if needs_food.current < 0.0 {
                needs_food.current = 0.0;
            }
        }
        if let Some(needs_entertainment) = physical_body.needs_entertainment.as_mut() {
            needs_entertainment.current -= needs_entertainment.rate;
            if needs_entertainment.current < 0.0 {
                needs_entertainment.current = 0.0;
            }
        }
        if let Some(needs_sleep) = physical_body.needs_sleep.as_mut() {
            needs_sleep.current -= needs_sleep.rate;
            if needs_sleep.current < 0.0 {
                needs_sleep.current = 0.0;
            }
        }
    }
}

#[derive(Message)]
pub struct FoodNotifEvent;
