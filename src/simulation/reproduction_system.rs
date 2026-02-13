use crate::prelude::*;
use rand::Rng;
use std::time::Duration;
use bevy::time::common_conditions::on_timer;

pub struct ReproductionPlugin;

impl Plugin for ReproductionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, reproduction_system.run_if(on_timer(Duration::from_secs(5))));
    }
}

pub fn reproduction_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PhysicalBody, &Genome, &Generation, &mut ReproductionStatus, &Position, &ActorType)>,
    sprite_sheet: Res<SpriteSheet>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();
    let current_time = time.elapsed().as_secs_f32();

    for (_entity, mut body, genome, generation, mut status, pos, actor_type) in query.iter_mut() {
        // Only reproduce if energy is high enough (e.g., > 80% of max)
        if body.energy_storage > status.energy_threshold && current_time - (status.last_reproduction_tick as f32) > 100.0 {
            
            // Check for overcrowding (simplified: just random chance or neighbor check)
            if rng.random_bool(0.3) {
                // Costs 50% of energy to reproduce
                let energy_cost = body.energy_storage * 0.5;
                body.energy_storage -= energy_cost;
                status.last_reproduction_tick = current_time as u64;

                // Mutate genome
                let child_genome = mutate_genome(genome, genome.mutation_rate);
                let child_generation = generation.value + 1;

                // Spawn child nearby
                let child_pos = Position {
                    x: pos.x + rng.random_range(-1..=1),
                    y: pos.y + rng.random_range(-1..=1),
                    z: pos.z,
                };

                spawn_child(&mut commands, child_pos, &sprite_sheet, &child_genome, child_generation, *actor_type, energy_cost);
                
                info!("Generation {} reproduce child of Generation {}", generation.value, child_generation);
            }
        }
    }
}

fn mutate_genome(parent: &Genome, rate: f32) -> Genome {
    let mut rng = rand::rng();
    let mut child = parent.clone();

    let mut mutate = |val: &mut f32, min: f32, max: f32| {
        if rng.random_bool(rate as f64) {
            let offset = rng.random_range(-0.1..0.1);
            *val = (*val + offset).clamp(min, max);
        }
    };

    mutate(&mut child.size, 0.1, 5.0);
    mutate(&mut child.mobility, 0.1, 5.0);
    mutate(&mut child.sensory_range, 1.0, 50.0);
    mutate(&mut child.metabolic_efficiency, 0.1, 0.95);
    mutate(&mut child.diet_type, 0.0, 1.0);
    mutate(&mut child.aggression, 0.0, 1.0);
    mutate(&mut child.sociality, 0.0, 1.0);
    mutate(&mut child.mutation_rate, 0.01, 0.2);
    mutate(&mut child.weight_hunger, 0.1, 2.0);
    mutate(&mut child.weight_fatigue, 0.1, 2.0);
    mutate(&mut child.weight_social, 0.1, 2.0);

    child
}

fn spawn_child(
    commands: &mut Commands,
    position: Position,
    sprite_sheet: &Res<SpriteSheet>,
    genome: &Genome,
    generation: u32,
    actor_type: ActorType,
    initial_energy: f32,
) {
    let energy_max = 100.0 * genome.size;
    let physical_body = PhysicalBody {
        energy_max,
        energy_storage: initial_energy.min(energy_max),
        health: 100.0,
        ..default()
    };

    commands.spawn((
        Sprite {
            image: sprite_sheet.handle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: sprite_sheet.layout.clone(),
                index: actor_type.sprite_index(),
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(genome.size)).with_translation(position.to_transform().translation),
        position,
        genome.clone(),
        Generation { value: generation },
        ReproductionStatus {
            energy_threshold: energy_max * 0.8,
            last_reproduction_tick: 0,
        },
        MaterialProperties {
            mass: 1.0 * genome.size,
            hardness: 1.0,
            toughness: 1.0,
            energy_density: 1.0,
            conductivity: 1.0,
        },
        physical_body,
        actor_type,
        Brain::default(),
    ));
}
