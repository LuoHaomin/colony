use bevy::prelude::*;
use crate::prelude::*;

#[derive(Component)]
pub struct Object {
    pub itemtype: ItemType,
    pub remaining_resources: Vec<(ItemType, u8)>,
    pub under_construction: bool,
}
impl Default for Object {
    fn default() -> Self {
        Object {
            itemtype: ItemType::Wall,
            remaining_resources: vec![],
            under_construction: false,
        }
    }
}

#[derive(Component)]
pub struct ItemReplacements {
    pub replacements: Vec<ItemType>,
}

#[derive(Component, PartialEq, Copy, Clone, Debug, Default)]
pub enum ItemType {
    #[default]
    Log,
    Stone,
    Wall,
    Statue,
    Tree,
    Bush,
    Weed,
    Food,
    Berry,
}

impl ItemType {
    pub fn material_properties(&self) -> MaterialProperties {
        match self {
            ItemType::Log => MaterialProperties { hardness: 2.0, toughness: 5.0, energy_density: 30.0, mass: 15.0, conductivity: 0.2 },
            ItemType::Stone | ItemType::Statue => MaterialProperties { hardness: 8.0, toughness: 20.0, energy_density: 0.0, mass: 100.0, conductivity: 0.1 },
            ItemType::Wall => MaterialProperties { hardness: 6.0, toughness: 15.0, energy_density: 0.0, mass: 50.0, conductivity: 0.1 },
            ItemType::Tree => MaterialProperties { hardness: 3.0, toughness: 10.0, energy_density: 20.0, mass: 200.0, conductivity: 0.2 },
            ItemType::Bush | ItemType::Berry | ItemType::Food => MaterialProperties { hardness: 0.1, toughness: 0.1, energy_density: 50.0, mass: 0.5, conductivity: 0.9 },
            ItemType::Weed => MaterialProperties { hardness: 0.01, toughness: 0.01, energy_density: 5.0, mass: 0.1, conductivity: 0.95 },
        }
    }

    pub fn sprite_index(&self) -> usize {
        match self {
            ItemType::Log => 94 * 64 + 30,
            ItemType::Stone => 51 * 64 + 8,
            ItemType::Wall => 6 * 64 + 32,
            ItemType::Statue => 19 * 64 + 19,
            ItemType::Tree => 20 * 64 + 14,
            ItemType::Bush => 67 * 64 + 57,
            ItemType::Berry => 67 * 64 + 57,
            ItemType::Weed => 67 * 64 + 57,
            ItemType::Food => 94 * 64 + 31,
        }
    }

    pub fn passable(&self) -> bool {
        match self {
            ItemType::Wall | ItemType::Statue | ItemType::Tree => false,
            _ => true,
        }
    }

    pub fn add_components(&self, _commands: &mut Commands, _entity: Entity) {
        // Physical properties are now handled via components at spawn time
    }
}

impl Object {
    pub fn passable(&self) -> bool {
        self.itemtype.passable()
    }   
}
