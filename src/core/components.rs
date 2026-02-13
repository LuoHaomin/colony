use crate::prelude::*;


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Initializing,
    InGame,
    Paused,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MenuStates {
    #[default]
    Home,
    Tasks,
    Farm,
    Build,
    Zone,
}

impl MenuStates {
    pub fn to_index(&self) -> usize {
        match self {
            MenuStates::Home => 0,
            MenuStates::Tasks => 1,
            MenuStates::Farm => 2,
            MenuStates::Build => 3,
            MenuStates::Zone => 4,
        }
    }
}

#[derive(Resource, Default)]
pub struct MenuState {
    pub state: MenuStates,
}

#[derive(Component, PartialEq, Copy, Clone, Debug, Eq, Hash, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn distance(&self, other: &Self) -> i32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32).sqrt() as i32
    }
    pub fn to_transform(&self) -> Transform {
        // We use X, Y for the plane and Z for the layer depth.
        // In Bevy's default 2D/3D setup, usually (x, y) is the plane.
        // To make it look like Dwarf Fortress, we keep x, y as coordinates and 
        // use a small Z offset for actual rendering order if needed, 
        // but here Z is the layer depth.
        Transform::from_xyz(self.x as f32 * TILE_SIZE, self.y as f32 * TILE_SIZE, self.z as f32 * 0.1)
    }
    pub fn to_transform_layer(&self, layer: f32) -> Transform {
        Transform::from_xyz(self.x as f32 * TILE_SIZE, layer, self.y as f32 * TILE_SIZE)
    }
}

pub fn position_to_translation(x: i32, y: i32, z: i32) -> Transform {
    Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, z as f32 * TILE_SIZE)
}

#[derive(Component, Debug, Clone, PartialEq)]
pub enum TileType {
    Grass, Dirt, Water, Wall, WallGame, Gravel, Void
}

impl TileType {
    pub fn is_wall(&self) -> bool {
        matches!(self, TileType::Wall | TileType::WallGame)
    }
    pub fn get_texture_coords(&self) -> (u32, u32) {
        match self {
            TileType::Grass => (0, 0),
            TileType::Dirt => (1, 0),
            TileType::Water => (2, 0),
            TileType::Wall | TileType::WallGame => (3, 0),
            TileType::Gravel => (4, 0),
            TileType::Void => (0, 0),
        }
    }
    pub fn material_properties(&self) -> MaterialProperties {
        match self {
            TileType::Grass => MaterialProperties { hardness: 0.1, toughness: 0.2, energy_density: 0.5, mass: 1.0, conductivity: 0.8 },
            TileType::Dirt => MaterialProperties { hardness: 0.3, toughness: 0.5, energy_density: 0.1, mass: 1.5, conductivity: 0.4 },
            TileType::Water => MaterialProperties { hardness: 0.0, toughness: 0.0, energy_density: 0.0, mass: 1.0, conductivity: 0.9 },
            TileType::Wall | TileType::WallGame => MaterialProperties { hardness: 5.0, toughness: 10.0, energy_density: 0.0, mass: 10.0, conductivity: 0.1 },
            TileType::Gravel => MaterialProperties { hardness: 0.8, toughness: 0.3, energy_density: 0.0, mass: 1.8, conductivity: 0.6 },
            TileType::Void => MaterialProperties::default(),
        }
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct EnvironmentalData {
    pub temperature: f32,
    pub humidity: f32,
    pub fertility: f32,
}

#[derive(Resource, Default, PartialEq)]
pub enum VisualizationMode {
    #[default]
    Normal,
    Temperature,
    Humidity,
    Fertility,
}

#[derive(Component)]
pub struct MapTile;

#[derive(Component, Debug, Clone)]
pub struct SizeXYZ {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

impl SizeXYZ {
    pub fn flat(size: f32) -> Self {
        Self { width: size, height: size, depth: 0.1 }
    }
    pub fn cube(size: f32) -> Self {
        Self { width: size, height: size, depth: size }
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct MaterialProperties {
    pub mass: f32,
    pub hardness: f32,
    pub toughness: f32,
    pub energy_density: f32,
    pub conductivity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AtomicAction {
    Move(Position),
    ApplyForce(Entity, f32), // Target, Force magnitude
    Consume(Entity),        // Absorb energy
    Link(Entity, Entity),   // Attach two entities
    Scan,                   // Perceive surroundings
}

#[derive(Component, Debug, Clone, Default)]
pub struct Genome {
    // Physical
    pub size: f32,
    pub mobility: f32,
    pub sensory_range: f32,
    pub physical_strength: f32,
    // Metabolic
    pub metabolic_efficiency: f32,
    pub diet_type: f32, // 0.0=photosynthetic, 1.0=carnivorous
    pub thermal_tolerance: f32,
    // Behavioral
    pub aggression: f32,
    pub sociality: f32,
    pub mutation_rate: f32,
    // Motivation Weights (0.0 to 1.0)
    pub weight_hunger: f32,
    pub weight_fatigue: f32,
    pub weight_social: f32,
}

impl Genome {
    pub fn genetic_distance(&self, other: &Genome) -> f32 {
        let d = (self.size - other.size).powi(2) +
                (self.mobility - other.mobility).powi(4) +
                (self.metabolic_efficiency - other.metabolic_efficiency).powi(2) +
                (self.diet_type - other.diet_type).powi(2) +
                (self.thermal_tolerance - other.thermal_tolerance).powi(2) +
                (self.physical_strength - other.physical_strength).powi(2) +
                (self.aggression - other.aggression).powi(2) +
                (self.weight_hunger - other.weight_hunger).powi(2) +
                (self.weight_fatigue - other.weight_fatigue).powi(2);
        d.sqrt()
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct Generation {
    pub value: u32,
}

#[derive(Component, Debug, Clone, Default)]
pub struct ReproductionStatus {
    pub energy_threshold: f32,
    pub last_reproduction_tick: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Need {
    pub current: f32,
    pub max: f32,
    pub rate: f32,
    pub low: f32,
    pub normal: f32,
    pub high: f32,
}

#[derive(Component, Default, Clone)]
pub struct PhysicalBody {
    pub energy_storage: f32,
    pub energy_max: f32,
    pub health: f32,
    pub needs_food: Option<Need>,
    pub needs_sleep: Option<Need>,
    pub needs_entertainment: Option<Need>,
    pub index: u32,
    pub crisis: Option<String>,
    pub danger: Option<Danger>,
    pub injured: bool,
    pub afflictions: Vec<Affliction>,
    pub skillset: Skillset,
    pub attributes: Attributeset,
}

impl PhysicalBody {
    pub fn info_panel_needs(&self) -> Vec<String> {
        let mut info = Vec::new();
        if let Some(need) = &self.needs_food {
            info.push(format!("Hunger: {:.0}%", (1.0 - need.current / need.max) * 100.0));
        }
        if let Some(need) = &self.needs_sleep {
            info.push(format!("Energy: {:.0}%", (need.current / need.max) * 100.0));
        }
        if let Some(need) = &self.needs_entertainment {
            info.push(format!("Boredom: {:.0}%", (1.0 - need.current / need.max) * 100.0));
        }
        info
    }
    pub fn info_panel_attributes(&self) -> Vec<String> { vec![] }
    pub fn info_panel_skills(&self) -> Vec<String> { vec![] }
}

#[derive(Clone, Debug, Default)]
pub struct Affliction {
    pub location: AfflictionLocation,
    pub affliction_type: AfflictionType,
    pub duration: u32,
    pub severity: u32,
    pub worsening: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Skill {
    pub experience: i32,
    pub exp_lost: i32,
}

impl Skill {
    pub fn level(&self) -> i32 {
        (self.experience as f32).sqrt() as i32
    }
}

#[derive(Clone, Debug, Default)]
pub struct Skillset {
    pub animal_raising: Skill,
    pub brawling: Skill,
    pub construction: Skill,
    pub cooking: Skill,
    pub crafting: Skill,
    pub doctoring: Skill,
    pub farming: Skill,
    pub fishing: Skill,
    pub foraging: Skill,
    pub hunting: Skill,
    pub mining: Skill,
    pub social: Skill,
    pub woodcutting: Skill,
}

#[derive(Clone, Debug, Default)]
pub struct Attributeset {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Component, Default)]
pub struct Brain {
    pub task: Option<Task>,
    pub task_queue: Vec<Task>,
    pub action: Option<AtomicAction>,
    pub action_queue: Vec<AtomicAction>,
    pub personality: Vec<PersonalityTrait>,
    pub memory: Vec<Memory>,
    pub motivation: Option<Motivation>,
    pub order: Option<String>,
}

impl Brain {
    pub fn info_panel(&self) -> Vec<String> {
        let mut info = Vec::new();
        if let Some(task) = self.task {
            info.push(format!("Task: {:?}", task));
        }
        if !self.task_queue.is_empty() {
            info.push(format!("Queue: {} tasks", self.task_queue.len()));
        }
        if let Some(motivation) = self.motivation {
            info.push(format!("Motivation: {:?}", motivation));
        }
        info
    }
    pub fn remotivate(&mut self) {
        self.task = None;
        self.task_queue.clear();
        self.motivation = None;
    }
    pub fn get_next_personality_trait(&self) -> Option<PersonalityTrait> { None }
}

#[derive(Component, Default)]
pub struct Pathing {
    pub path: Vec<Position>,
    pub destination: Position,
    pub moving_target: bool,
    pub unreachable: bool,
}

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Targeting {
    pub target: Entity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Memory {
    Seen(Entity, Position),
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum PersonalityType {
    Human, Vicious, None, Territorial
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PersonalityTrait {
    Human, Vicious, None, Territorial, Creature
}

#[derive(Component, PartialEq, Copy, Clone, Debug, Default)]
pub enum Task {
    #[default]
    Idle,
    Crisis, Flee, Fight, Eat, Hospital, Sleep, Sleeping, Play, Order, Work, Personality, Meander,
    Doctor, Forage, Plant, Harvest, Mine, Chop, Construct, Hunt, Milk, Cook, Fish, Craft, Clean, Pickup, Carrying
}

impl Task {
    pub fn is_zone_task(&self) -> bool {
        matches!(self, Task::Plant | Task::Construct | Task::Carrying)
    }
    pub fn get_steps(&self) -> Self {
        *self
    }
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Motivation {
    Crisis, Rage, Order, Danger, Hunger, Thirst, Tired, Injured, Sick, Bored, Happy, Sad, Angry, Lonely, Love, Fear, Hate, Work, Personality, Meander, Idle,
    Eat, Hospital, Sleep, Play
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum ForageType {
    Once, Repeat
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum ActorType {
    Man, Woman, Elf, Dwarf, Spider, Rat, Cyclops, Monster, Crab
}

impl ActorType {
    pub fn sprite_index(&self) -> usize {
        match self {
            ActorType::Man => 31 + 0 * 64,
            ActorType::Woman => 31 + 1 * 64,
            ActorType::Elf => 31 + 2 * 64,
            ActorType::Dwarf => 31 + 3 * 64,
            ActorType::Spider => 28 + 5 * 64,
            ActorType::Rat => 28 + 6 * 64,
            ActorType::Cyclops => 28 + 7 * 64,
            ActorType::Monster => 28 + 8 * 64,
            ActorType::Crab => 28 + 9 * 64,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub enum SelectableType {
    #[default]
    Nothing, Carryable, Choppable, Constructable, Foragable, Harvestable, Huntable, Mineable, Unselecting, Unzoning, Zoning, Farm, Build, Tasks
}

#[derive(Component)]
pub struct WorkTarget;

#[derive(Component)]
pub struct Renderable {
    pub fg: Color,
    pub bg: Color,
}

#[derive(Component, Default)]
pub struct Bed;

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Foragable;

#[derive(Component)]
pub struct Choppable;

#[derive(Component)]
pub struct Harvestable;

#[derive(Component)]
pub struct Mineable;

#[derive(Component)]
pub struct Zone {
    pub zone_type: ZoneType,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ZoneType {
    #[default]
    Farm, Storage, Construction, Avoid
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Entity>,
}

#[derive(Component)]
pub struct Item {
    pub item_type: ItemType,
}

#[derive(Component)]
pub struct Highlighted;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct WorkMarker;

#[derive(Component, Default)]
pub struct Nest {
    pub position: Position,
}

#[derive(Component)]
pub struct Attackable;

#[derive(Component)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct StatusDisplay {
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum AfflictionLocation {
    #[default]
    Head, LeftArm, RightArm, LeftLeg, RightLeg, Torso, Bladder, Intestines, Genitals, Heart, Lungs, Brain, Stomach, Liver, Spleen, Kidneys
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum AfflictionType {
    #[default]
    Pain, Inflammation, Disease, Wound, BrokenBone, Cut, Frostbite, Infection
}

#[derive(Clone, Debug, PartialEq)]
pub struct Danger {
    pub danger_type: DangerType,
    pub danger_source: Option<Entity>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DangerType {
    Fire, Predator, Enemy, NaturalDisaster, Attacked
}

#[derive(Component)]
pub struct HasName {
    pub name: String,
}

#[derive(Component)]
pub struct ClickedOn;

#[derive(Component)]
pub struct Plant {
    pub growth: f32,
    pub plant_type: ItemType,
}

#[derive(Component)]
pub struct GiveMeAName;

#[derive(Component)]
pub struct MonsterGenerator {
    pub monsters: Vec<(crate::simulation::unitgenerator_system::UnitTemplate, u32)>,
}

impl MonsterGenerator {
    pub fn pick(&self) -> crate::simulation::unitgenerator_system::UnitTemplate {
        self.monsters[0].0.clone()
    }
}

#[derive(Component)]
pub struct Logs;

#[derive(Component)]
pub struct Attacked {
    pub attacker: Entity,
}

#[derive(Component, Default)]
pub struct TemporaryVisualElement {
    pub duration: f32,
}

#[derive(Component, Default)]
pub struct VisualFeedback {
    pub shake_timer: f32, // Time left to shake
    pub shake_intensity: f32,
    pub original_offset: Vec3,
}

#[derive(Component, Default)]
pub struct Dying;

#[derive(Component)]
pub struct GeneratedBy { pub entity: Entity }

#[derive(Component)]
pub struct MoveRandom;

#[derive(Component)]
pub struct MoveTowardsNearestAttackable;

#[derive(Component, Default)]
pub struct Food {
    pub spoilage: f32,
    pub spoilage_rate: f32,
}

#[derive(Component)]
pub struct Carryable;

#[derive(Component)]
pub struct InGameButton;

#[derive(Component)]
pub struct PauseOverlay;

#[derive(Component)]
pub struct MainMenuOverlay;

#[derive(Component)]
pub struct TextName;

#[derive(Component)]
pub struct IsName;

#[derive(Component, Default)]
pub struct HasNameShown;

#[derive(Component)]
pub struct SetNest;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum StrikeType {
    Hit,
    Miss,
}

impl StrikeType {
    pub fn sprite_index(&self) -> usize {
        match self {
            StrikeType::Hit => 0,
            StrikeType::Miss => 1,
        }
    }
}
