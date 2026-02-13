use crate::prelude::*;
use rand::seq::SliceRandom;

pub struct UnitGeneratorPlugin;

impl Plugin for UnitGeneratorPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn spawn_unit_from_template(
    commands: &mut Commands,
    position: Position,
    sprite_sheet: &Res<SpriteSheet>,
    template: &UnitTemplate,
    _meshes: &UniversalMeshAssets,
) -> Entity {
    let mut physical_body = PhysicalBody {
        needs_food: template.food_need.map(Into::into),
        needs_entertainment: template.entertainment_need.map(Into::into),
        needs_sleep: template.sleep_need.map(Into::into),
        energy_max: 100.0,
        energy_storage: 90.0,
        health: 100.0,
        index: 0,
        crisis: None,
        danger: None,
        injured: false,
        afflictions: template.afflictions.clone(),
        skillset: template.skillset.clone(),
        attributes: template.attributes.clone(),
    };

    let entity = commands
        .spawn((
            Sprite {
                image: sprite_sheet.handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprite_sheet.layout.clone(),
                    index: template.actor_type.sprite_index(),
                }),
                ..default()
            },
            position.to_transform(),
            MaterialProperties {
                mass: 1.0,
                hardness: 1.0,
                toughness: 1.0,
                energy_density: 1.0,
                conductivity: 1.0,
            },
        ))
        .insert(position)
        .id();

    if let Some(genome) = &template.genome {
        commands.entity(entity).insert(genome.clone());
        physical_body.energy_max *= genome.size;
        physical_body.energy_storage = physical_body.energy_max * 0.9;
        
        // Scale sprite size
        commands.entity(entity).insert(Transform::from_scale(Vec3::splat(genome.size)));

        // Add generation and reproduction status
        commands.entity(entity).insert(Generation { value: 0 });
        commands.entity(entity).insert(ReproductionStatus {
            energy_threshold: physical_body.energy_max * 0.8,
            last_reproduction_tick: 0,
        });
    }

    commands.entity(entity).insert(physical_body);
    commands.entity(entity).insert( Brain {
        personality: template.personality.clone(),
        ..default()
    } );
    for builder in &template.component_builders {
        builder(commands, entity);
    };
    entity
}

type ComponentBuilder = fn(&mut Commands, Entity);

#[derive(Clone)]
pub struct UnitTemplate {
    pub actor_type: ActorType,
    pub genome: Option<Genome>,
    pub food_need: Option<NeedExample>,
    pub entertainment_need: Option<NeedExample>,
    pub sleep_need: Option<NeedExample>,
    pub personality: Vec<PersonalityTrait>,
    pub skillset: Skillset,
    pub attributes: Attributeset,
    pub afflictions: Vec<Affliction>,
    pub component_builders: Vec<ComponentBuilder>,
}

#[derive(Copy, Clone)]
pub struct NeedExample {
    pub current: f32,
    pub max: f32,
    pub rate: f32,
    pub low: f32,
    pub normal: f32,
    pub high: f32,
    pub variance: f32,
}

impl From<NeedExample> for Need {
    fn from(example: NeedExample) -> Self {
        let mut rng = rand::rng();
        let v = example.variance;
        Self {
            current: example.current + rng.random_range(-v..v),
            max: example.max + rng.random_range(-v..v),
            rate: example.rate + rng.random_range(-v..v),
            low: example.low + rng.random_range(-v..v),
            normal: example.normal + rng.random_range(-v..v),
            high: example.high + rng.random_range(-v..v),
        }
    }
}

impl UnitTemplate {
    pub fn human() -> Self {
        Self {
            actor_type: ActorType::Man,
            genome: Some(Genome { 
                size: 1.0, mobility: 1.0, sensory_range: 15.0, physical_strength: 1.0,
                metabolic_efficiency: 0.8, diet_type: 0.5, thermal_tolerance: 15.0,
                sociality: 0.8, aggression: 0.2, mutation_rate: 0.05,
                weight_hunger: 1.0, weight_fatigue: 1.0, weight_social: 1.0
            }),
            food_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            entertainment_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            sleep_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            personality: vec![PersonalityTrait::Human],
            skillset: Self::random_skillset_humanoid(),
            attributes: Self::random_attributeset_humanoid(),
            afflictions: Self::random_afflictions_humanoid(),
            component_builders: vec![
                |commands, entity| { commands.entity(entity).insert(GiveMeAName); },
            ],
        }
    }
    pub fn elf() -> Self {
        Self {
            actor_type: ActorType::Elf,
            genome: Some(Genome { 
                size: 0.9, mobility: 1.2, sensory_range: 20.0, physical_strength: 1.0,
                metabolic_efficiency: 0.7, diet_type: 0.3, thermal_tolerance: 15.0,
                sociality: 0.6, aggression: 0.1, mutation_rate: 0.05,
                weight_hunger: 0.8, weight_fatigue: 1.2, weight_social: 1.5
            }),
            food_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            entertainment_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            sleep_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            personality: vec![PersonalityTrait::Human],
            skillset: Self::random_skillset_humanoid(),
            attributes: Self::random_attributeset_humanoid(),
            afflictions: Self::random_afflictions_humanoid(),
            component_builders: vec![
                |commands, entity| { commands.entity(entity).insert(GiveMeAName); },
            ],
        }
    }
    pub fn dwarf() -> Self {
        Self {
            actor_type: ActorType::Dwarf,
            genome: Some(Genome { 
                size: 0.8, mobility: 0.8, sensory_range: 12.0, physical_strength: 1.5,
                metabolic_efficiency: 0.9, diet_type: 0.6, thermal_tolerance: 20.0,
                sociality: 0.9, aggression: 0.4, mutation_rate: 0.05,
                weight_hunger: 1.5, weight_fatigue: 0.8, weight_social: 0.5
            }),
            food_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            entertainment_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            sleep_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            personality: vec![PersonalityTrait::Human],
            skillset: Self::random_skillset_humanoid(),
            attributes: Self::random_attributeset_humanoid(),
            afflictions: Self::random_afflictions_humanoid(),
            component_builders: vec![
                |commands, entity| { commands.entity(entity).insert(GiveMeAName); },
            ],
        }
    }
    pub fn rat() -> Self {
        Self {
            actor_type: ActorType::Rat,
            genome: Some(Genome { 
                size: 0.3, mobility: 1.5, sensory_range: 8.0, physical_strength: 0.5,
                metabolic_efficiency: 0.5, diet_type: 0.8, thermal_tolerance: 10.0,
                sociality: 0.4, aggression: 0.1, mutation_rate: 0.1,
                weight_hunger: 2.0, weight_fatigue: 1.0, weight_social: 0.1
            }),
            food_need: None,
            entertainment_need: None,
            sleep_need: None,
            personality: vec![PersonalityTrait::Creature, PersonalityTrait::Territorial],
            afflictions: vec![],
            skillset: Skillset::default(),
            attributes: Attributeset::default(),
            component_builders: vec![
                |commands, entity| { commands.entity(entity).insert(HasName { name: "Rat".to_string() }); },
                |commands, entity| { commands.entity(entity).insert(SetNest); }
            ],
        }
    }
    pub fn spider() -> Self {
        Self {
            actor_type: ActorType::Spider,
            genome: Some(Genome { 
                size: 0.5, mobility: 1.8, sensory_range: 6.0, physical_strength: 0.8,
                metabolic_efficiency: 0.4, diet_type: 1.0, thermal_tolerance: 15.0,
                sociality: 0.1, aggression: 0.6, mutation_rate: 0.1,
                weight_hunger: 1.8, weight_fatigue: 0.5, weight_social: 0.0
            }),
            food_need: None,
            entertainment_need: None,
            sleep_need: None,
            personality: vec![PersonalityTrait::Creature, PersonalityTrait::Territorial],
            afflictions: vec![],
            skillset: Skillset::default(),
            attributes: Attributeset::default(),
            component_builders: vec![
                |commands, entity| { commands.entity(entity).insert(HasName { name: "Spider".to_string() }); },
                |commands, entity| { commands.entity(entity).insert(SetNest); }
            ],
        }
    }
    pub fn crab() -> Self {
        Self {
            actor_type: ActorType::Crab,
            genome: Some(Genome { 
                size: 0.4, mobility: 0.7, sensory_range: 10.0, physical_strength: 1.2,
                metabolic_efficiency: 0.9, diet_type: 0.4, thermal_tolerance: 5.0,
                sociality: 0.2, aggression: 0.1, mutation_rate: 0.05,
                weight_hunger: 0.5, weight_fatigue: 1.5, weight_social: 0.1
            }),
            food_need: None,
            entertainment_need: None,
            sleep_need: None,
            personality: vec![PersonalityTrait::Creature],
            afflictions: vec![],
            skillset: Skillset::default(),
            attributes: Attributeset::default(),
            component_builders: vec![
                |commands, entity| {
                    commands.entity(entity).insert(HasName { name: "Crab".to_string() });
                },
            ],
        }
    }
    pub fn cyclops() -> Self {
        Self {
            actor_type: ActorType::Cyclops,
            genome: Some(Genome { 
                size: 2.5, mobility: 0.6, sensory_range: 15.0, physical_strength: 5.0,
                metabolic_efficiency: 0.6, diet_type: 0.9, thermal_tolerance: 25.0,
                sociality: 0.1, aggression: 0.9, mutation_rate: 0.02,
                weight_hunger: 2.5, weight_fatigue: 0.5, weight_social: 0.0
            }),
            food_need: None,
            entertainment_need: None,
            sleep_need: None,
            personality: vec![PersonalityTrait::Creature, PersonalityTrait::Vicious],
            afflictions: vec![],
            skillset: Skillset::default(),
            attributes: Attributeset::default(),
            component_builders: vec![
                |commands, entity| {
                    commands.entity(entity).insert(HasName { name: "Cyclops".to_string() });
                },
            ],
        }
    }
    pub fn random_afflictions_humanoid() -> Vec<Affliction> {
        let mut rng = rand::rng();
        let mut afflictions = vec![];
        if rng.random_bool(0.3) {
            afflictions.push(Affliction {
                location: AfflictionLocation::Head,
                affliction_type: AfflictionType::Pain,
                duration: 0,
                severity: 1,
                worsening: false,
            });
        }
        afflictions
    }
    pub fn random_skillset_humanoid() -> Skillset {
        let mut rng = rand::rng();
        let ranges = [500..700, 500..700, 300..400];
        let mut values: Vec<i32> = ranges.iter().map(|range| rng.random_range(range.clone())).collect();
        Skillset {
            animal_raising: Skill { experience: values.pop().unwrap_or(100), exp_lost: 0 },
            brawling: Skill { experience: values.pop().unwrap_or(100), exp_lost: 0 },
            construction: Skill { experience: values.pop().unwrap_or(100), exp_lost: 0 },
            cooking: Skill { experience: 100, exp_lost: 0 },
            crafting: Skill { experience: 100, exp_lost: 0 },
            doctoring: Skill { experience: 100, exp_lost: 0 },
            farming: Skill { experience: 100, exp_lost: 0 },
            fishing: Skill { experience: 100, exp_lost: 0 },
            foraging: Skill { experience: 100, exp_lost: 0 },
            hunting: Skill { experience: 100, exp_lost: 0 },
            mining: Skill { experience: 100, exp_lost: 0 },
            social: Skill { experience: 100, exp_lost: 0 },
            woodcutting: Skill { experience: 100, exp_lost: 0 },
        }
    }
    pub fn random_attributeset_humanoid() -> Attributeset {
        Attributeset {
            strength: 3,
            dexterity: 3,
            constitution: 3,
            intelligence: 3,
            wisdom: 3,
            charisma: 4,
        }
    }
}

pub fn text_test() {}
