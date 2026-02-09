use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
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

#[derive(Component, PartialEq, Copy, Clone, Debug, Serialize, Deserialize, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn distance(&self, other: &Self) -> i32 {
        ((self.x - other.x).pow(2) as f32 + (self.y - other.y).pow(2) as f32).sqrt() as i32
    }
    pub fn to_transform(&self) -> Transform {
        Transform::from_xyz(self.x as f32 * TILE_SIZE, self.y as f32 * TILE_SIZE, self.z as f32 * TILE_SIZE)
    }
}

pub fn position_to_translation(x: i32, y: i32, z: i32) -> Transform {
    Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, z as f32 * TILE_SIZE)
}

#[derive(Component, Debug, Clone, PartialEq)]
pub enum TileType {
    Grass, Dirt, Water, Wall, WallGame, Gravel
}

impl TileType {
    pub fn is_wall(&self) -> bool {
        matches!(self, TileType::Wall | TileType::WallGame)
    }
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

#[derive(Component, Default)]
pub struct PhysicalBody {
    pub needs_food: bool,
    pub needs_sleep: bool,
    pub needs_entertainment: bool,
    pub index: u32,
    pub crisis: Option<String>,
    pub danger: Option<DangerType>,
    pub injured: bool,
    pub afflictions: Vec<Affliction>,
    pub skillset: Skillset,
    pub attributes: Attributeset,
}

impl PhysicalBody {
    pub fn info_panel_needs(&self) -> Vec<String> { vec![] }
}

#[derive(Clone, Debug, Default)]
pub struct Affliction {
    pub location: AfflictionLocation,
    pub affliction_type: AfflictionType,
}

#[derive(Clone, Debug, Default)]
pub struct Skillset {}
#[derive(Clone, Debug, Default)]
pub struct Attributeset {}

#[derive(Component, Default)]
pub struct Brain {
    pub task: Option<Task>,
    pub personality: Vec<PersonalityTrait>,
    pub memory: Vec<Memory>,
    pub motivation: Option<Motivation>,
    pub order: Option<String>,
}

impl Brain {
    pub fn info_panel(&self) -> Vec<String> { vec![] }
}

#[derive(Component)]
pub struct Pathing {
    pub path: Vec<Position>,
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
    Crisis, Rage, Order, Danger, Hunger, Thirst, Tired, Injured, Sick, Bored, Happy, Sad, Angry, Lonely, Love, Fear, Hate, Work, Personality, Meander, Idle
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum ForageType {
    Once, Repeat
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum ActorType {
    Man, Woman, Elf, Dwarf, Spider, Rat, Cyclops, Monster, Crab
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum SelectableType {
    Carryable, Choppable, Constructable, Foragable, Harvestable, Huntable, Mineable, Nothing, Unselecting, Unzoning, Zoning, Farm, Build, Tasks
}

#[derive(Component)]
pub struct WorkTarget;

#[derive(Component)]
pub struct Renderable {
    pub fg: Color,
    pub bg: Color,
}

#[derive(Component)]
pub struct Bed;

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Default)]
pub struct Object {
    pub object_type: ObjectType,
    pub itemtype: ItemType,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ObjectType {
    #[default]
    Tree, Rock, BerryBush,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ItemType {
    #[default]
    Log, Stone, Berry, Cabbage, Carrot, PineTree, OakTree, CedarTree, WallWood, StatuePillar3, StatueCat, StatueDragon, Moss1, Moss2, LeafyDebris1, ThornBush, Weed, CactusRound
}

impl ItemType {
    pub fn is_forageable(&self) -> (Option<ItemType>, u32) { (None, 0) }
    pub fn is_choppable(&self) -> (Option<ItemType>, u32) { (None, 0) }
    pub fn add_components(&self, _commands: &mut Commands, _entity: Entity) {}
}

#[derive(Component)]
pub struct Highlighted;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct WorkMarker;

#[derive(Component)]
pub struct Nest;

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DangerType {
    Attacked, Fire
}
