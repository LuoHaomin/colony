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
    sprite_sheet: Res<SpriteSheet>,
) {
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_LENGTH {
            // Simple hill generation
            let hill_height = ((x as f32 * 0.1).sin() * (y as f32 * 0.1).cos() * 2.0).round() as i32;
            
            for z in -3..3 {
                let is_border = x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_LENGTH - 1;
                
                let tyle_type = if is_border {
                    if z <= hill_height { TileType::WallGame } else { TileType::Void }
                } else {
                    if z < hill_height {
                        TileType::Dirt
                    } else if z == hill_height {
                        biome.tiles.choose(&mut rand::rng()).unwrap_or(&TileType::Grass).clone()
                    } else {
                        TileType::Void
                    }
                };

                if tyle_type == TileType::Void { continue; }
                
                let (tx, ty) = tyle_type.get_texture_coords();
                let position = Position { x, y, z };
                
                commands.spawn((
                    MapTile,
                    position,
                    tyle_type.clone(),
                    SizeXYZ::flat(TILE_SIZE),
                    Sprite {
                        image: sprite_sheet.handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: sprite_sheet.layout.clone(),
                            index: (tx + ty * 64) as usize,
                        }),
                        ..default()
                    },
                    position.to_transform(),
                    Visibility::default(),
                ));

                tiletypes.hash.insert(position, tyle_type);
            }
        }
    }
}

