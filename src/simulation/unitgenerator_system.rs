use crate::prelude::*;
use rand::seq::SliceRandom;

pub struct UnitGeneratorPlugin;

impl Plugin for UnitGeneratorPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn spawn_unit_from_template(
    commands: &mut Commands,
    position: Position,
    _sprite_sheet: &Res<SpriteSheet>,
    template: &UnitTemplate,
    meshes: &UniversalMeshAssets,
) -> Entity {
    let mesh = meshes.capsule.clone();
    let material = match template.actor_type {
        ActorType::Spider | ActorType::Rat | ActorType::Cyclops => meshes.material_red.clone(),
        ActorType::Man | ActorType::Woman | ActorType::Elf | ActorType::Dwarf => meshes.material_blue.clone(),
        _ => meshes.material_white.clone(),
    };

    let entity = commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            position.to_transform(),
        ))
        .insert(position)
        .insert( PhysicalBody {
            needs_food: template.food_need.map(Into::into),
            needs_entertainment: template.entertainment_need.map(Into::into),
            needs_sleep: template.sleep_need.map(Into::into),
            index: 0,
            crisis: None,
            danger: None,
            injured: false,
            afflictions: template.afflictions.clone(),
            skillset: template.skillset.clone(),
            attributes: template.attributes.clone(),
        } )
        .insert( Brain {
            personality: template.personality.clone(),
            ..default()
        } )
        .id();
    for builder in &template.component_builders {
        builder(commands, entity);
    };
    entity
}

type ComponentBuilder = fn(&mut Commands, Entity);

#[derive(Clone)]
pub struct UnitTemplate {
    pub actor_type: ActorType,
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
            health: 100,
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
