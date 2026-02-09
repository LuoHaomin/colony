use crate::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct TileHash {
    pub hash: HashMap<Position, TileType>,
}

#[derive(Resource, Default)]
pub struct SpriteSheet {
    pub handle: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

// For backward compatibility with initialization in load.rs
#[allow(non_snake_case)]
pub fn SpriteSheet(handle: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> SpriteSheet {
    SpriteSheet { handle, layout }
}

#[derive(Resource, Default)]
pub struct MyFont(pub Handle<Font>);

#[derive(Resource, Default)]
pub struct UniversalMeshAssets {
    pub meshes: HashMap<String, Handle<Mesh>>,
    pub materials: HashMap<String, Handle<StandardMaterial>>,
    pub cube: Handle<Mesh>,
    pub sphere: Handle<Mesh>,
    pub plane: Handle<Mesh>,
    pub capsule: Handle<Mesh>,
    pub cylinder: Handle<Mesh>,
    pub material_white: Handle<StandardMaterial>,
    pub material_red: Handle<StandardMaterial>,
    pub material_green: Handle<StandardMaterial>,
    pub material_blue: Handle<StandardMaterial>,
    pub material_brown: Handle<StandardMaterial>,
}

impl UniversalMeshAssets {
    pub fn get_mesh(&self, name: &str) -> Handle<Mesh> {
        self.meshes.get(name).cloned().unwrap_or_else(|| self.cube.clone())
    }
    pub fn get_material(&self, name: &str) -> Handle<StandardMaterial> {
        self.materials.get(name).cloned().unwrap_or_else(|| self.material_white.clone())
    }
}

#[derive(Resource, Default)]
pub struct SelectedObjectInformation {
    pub info: Vec<String>,
    pub entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct InfoPanelInformation {
    pub name: String,
    pub info: Vec<String>,
    pub needs: Vec<String>,
    pub attributes: Vec<String>,
    pub skills: Vec<String>,
    pub mouse_position: Option<Position>,
}

#[derive(Resource, Default)]
pub struct Dragging {
    pub dragging: bool,
    pub start_position: Option<Position>,
    pub looking_for: SelectableType,
    pub zone_type: ZoneType,
    pub item_type: ItemType,
}

#[derive(Resource, Default)]
pub struct GameSpeed {
    pub speed: f32,
}

#[derive(Resource, Default, Debug)]
pub struct Biome {
    pub name: String,
    pub tiles: Vec<TileType>,
    pub plants: Vec<ItemType>,
    pub objects: Vec<ItemType>,
    pub plant_scarcity: Vec<u32>,
    pub plant_overall_scarcity: u32,
    pub objects_scarcity: Vec<u32>,
    pub objects_overall_scarcity: u32,
}
