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
    mut query: Query<(&Position, &mut Visibility, Option<&mut Sprite>)>,
) {
    for (position, mut visibility, sprite) in query.iter_mut() {
        if position.z > current_z.z {
            *visibility = Visibility::Hidden;
        } else if position.z == current_z.z {
            *visibility = Visibility::Visible;
            if let Some(mut s) = sprite {
                s.color = Color::WHITE;
            }
        } else {
            // Below current level
            let diff = (current_z.z - position.z) as f32;
            if diff > 3.0 {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Visible;
                if let Some(mut s) = sprite {
                    // Dim the color based on depth
                    let dim = 1.0 - (diff * 0.25);
                    s.color = Color::srgb(dim, dim, dim);
                }
            }
        }
    }
}
