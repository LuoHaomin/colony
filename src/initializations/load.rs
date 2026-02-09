use crate::prelude::*;

pub fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the full spritesheet image
    let texture_handle: Handle<Image> = asset_server.load("AllSprites.png");
    // Build a grid layout for the spritesheet (cell size 32x32, 64 columns, 95 rows)
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 64, 95, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);
    // Store both the image handle and the layout handle in the SpriteSheet resource
    commands.insert_resource(SpriteSheet(texture_handle, layout_handle));
}

pub fn load_font(
    asset_server: Res<AssetServer>,
    mut font_handle: ResMut<MyFont>,
) {
    *font_handle = MyFont(asset_server.load("fonts/Helvetica.ttf"));
}

pub fn load_mesh_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(UniversalMeshAssets {
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

pub fn load_sfx(
    // mut commands: Commands,
    // asset_server: Res<AssetServer>
) {
    // commands.spawn((
    //     AudioBundle {
    //         source: asset_server.load("RPG Sound Pack/battle/swing.wav"),
    //         settings: PlaybackSettings::ONCE.paused().with_volume(bevy::audio::Volume::new_relative(0.5)),
    //     },
    //     SoundEffect,
    // ));
}

#[derive(Component)]
pub struct SoundEffect;
