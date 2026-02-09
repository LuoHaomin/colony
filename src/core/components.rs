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

#[derive(Component, PartialEq, Copy, Clone, Debug, Serialize, Deserialize, Eq, Hash, Default)]
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
        Transform::from_xyz(self.x as f32 * TILE_SIZE, self.z as f32 * TILE_SIZE, self.y as f32 * TILE_SIZE)
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

impl Skill {
    pub fn level(&self) -> i32 {
        (self.experience as f32).sqrt() as i32
    }
}

#[derive(Component, Default)]
pub struct Pathing {
    pub path: Vec<Position>,
    pub destination: Position,
    pub moving_target: bool,
    pub unreachable: bool,
}

#[derive(Component, Clone, PartialEq, Debug, Default)]
pub enum SelectableType {
    #[default]
    Nothing, Carryable, Choppable, Constructable, Foragable, Harvestable, Huntable, Mineable, Unselecting, Unzoning, Zoning, Farm, Build, Tasks
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
pub struct MonsterGenerator {
    pub monsters: Vec<(crate::simulation::unitgenerator_system::UnitTemplate, u32)>,
}

impl MonsterGenerator {
    pub fn pick(&self) -> crate::simulation::unitgenerator_system::UnitTemplate {
        self.monsters[0].0.clone()
    }
}
