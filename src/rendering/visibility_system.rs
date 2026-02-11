use crate::prelude::*;

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_visibility,
        ));
    }
}

pub fn update_visibility(
    current_z: Res<CurrentDisplayZ>,
    mut query: Query<(&Position, &mut Visibility, Option<&mut Sprite>, Option<&EnvironmentalData>, Option<&MapTile>)>,
) {
    for (position, mut visibility, sprite, env_data, is_map_tile) in query.iter_mut() {
        if position.z > current_z.z {
            *visibility = Visibility::Hidden;
        } else if position.z == current_z.z {
            *visibility = Visibility::Visible;
            if let Some(mut s) = sprite {
                let mut base_color = Color::WHITE;
                
                // Environment Tint for MapTiles
                if is_map_tile.is_some() {
                    if let Some(env) = env_data {
                        // Blend fertility into green
                        let fertility = env.fertility.clamp(0.0, 1.0);
                        let temp = (env.temperature / 40.0 + 0.5).clamp(0.0, 1.0); // 0C to 40C scale
                        
                        base_color = Color::srgb(
                            1.0 - fertility * 0.5 + temp * 0.2, // R
                            1.0 + fertility * 0.2,             // G
                            1.0 - fertility * 0.5 - temp * 0.2  // B
                        );
                    }
                }
                s.color = base_color;
            }
        } else {
            // Below current level
            let diff = (current_z.z - position.z) as f32;
            if diff > 3.0 {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Visible;
                if let Some(mut s) = sprite {
                    let mut dim = 1.0 - (diff * 0.25);
                    
                    if is_map_tile.is_some() {
                        if let Some(env) = env_data {
                            let fertility = env.fertility.clamp(0.0, 1.0);
                            s.color = Color::srgb(
                                dim * (1.0 - fertility * 0.3),
                                dim * (1.0 + fertility * 0.1),
                                dim * (1.0 - fertility * 0.3)
                            );
                        } else {
                            s.color = Color::srgb(dim, dim, dim);
                        }
                    } else {
                        s.color = Color::srgb(dim, dim, dim);
                    }
                }
            }
        }
    }
}
