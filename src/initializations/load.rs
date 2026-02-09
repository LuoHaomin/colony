use crate::prelude::*;
use std::collections::HashMap;

pub fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("AllSprites.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 64, 95, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);
    commands.insert_resource(SpriteSheet { handle: texture_handle, layout: layout_handle });
}

pub fn load_font(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.insert_resource(MyFont(asset_server.load("fonts/FiraSans-Bold.ttf")));
}

pub fn load_mesh_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(UniversalMeshAssets {
        meshes: HashMap::new(),
        materials: HashMap::new(),
        cube: meshes.add(Cuboid::from_size(Vec3::splat(TILE_SIZE))),
        sphere: meshes.add(Sphere::new(TILE_SIZE * 0.5)),
        plane: meshes.add(Plane3d::default().mesh().size(TILE_SIZE, TILE_SIZE)),
        capsule: meshes.add(Capsule3d::new(TILE_SIZE * 0.4, TILE_SIZE * 0.8)),
        cylinder: meshes.add(Cylinder::new(TILE_SIZE * 0.2, TILE_SIZE * 1.0)),
        material_white: materials.add(Color::WHITE),
        material_red: materials.add(Color::srgb(0.8, 0.2, 0.2)),
        material_green: materials.add(Color::srgb(0.2, 0.8, 0.2)),
        material_blue: materials.add(Color::srgb(0.2, 0.2, 0.8)),
        material_brown: materials.add(Color::srgb(0.4, 0.2, 0.1)),
    });
}

pub fn load_sfx() {}

#[derive(Component)]
pub struct SoundEffect;
