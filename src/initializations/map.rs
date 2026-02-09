use crate::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), generate_map);
    }
}

pub fn generate_map(
    mut commands: Commands,
    biome: Res<Biome>,
    mut tiletypes: ResMut<TileHash>,
    mesh_assets: Res<UniversalMeshAssets>,
) {
    let wall_mesh = mesh_assets.cube.clone();
    let wall_material = mesh_assets.material_white.clone();
    let floor_mesh = mesh_assets.plane.clone();
    let floor_material = mesh_assets.material_green.clone();

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_LENGTH {
            let tyle_type = if x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_LENGTH - 1 {
                TileType::WallGame
            } else {
                biome.tiles.choose(&mut rand::thread_rng()).unwrap_or(&TileType::Grass).clone()
            };

            let is_wall = tyle_type.is_wall();
            let mut transform = position_to_translation(x, y, 0);

            let entity = commands.spawn((
                MapTile,
                Position { x, y, z: 0 },
                tyle_type.clone(),
                SizeXYZ::flat(TILE_SIZE),
            )).id();

            if is_wall {
                commands.entity(entity).insert((
                    Mesh3d(wall_mesh.clone()),
                    MeshMaterial3d(wall_material.clone()),
                    transform,
                ));
            } else {
                // Plane is horizontal by default, we want it on z=0 facing up (y+) or facing camera (z+)
                // In Bevy 3D, z=0 is a vertical plane. XY is the screen.
                // Our units move on XY plane. So z=0 floor should be fine?
                // Actually, Bevy standard is XZ floor, Y up.
                // But our legacy code uses (x, y) for 2D. 
                // Let's keep (x, y) on a plane and z as height.
                // So floor is a plane on z=0.
                transform.rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
                commands.entity(entity).insert((
                    Mesh3d(floor_mesh.clone()),
                    MeshMaterial3d(floor_material.clone()),
                    transform,
                ));
            }

            tiletypes.hash.insert( Position { x, y, z: 0 }, tyle_type);
        }
    }
}
