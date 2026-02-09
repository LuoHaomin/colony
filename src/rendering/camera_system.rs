use crate::prelude::*;
use crate::constants::TILE_SIZE;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera.run_if(in_state(GameState::InGame)));
    }
}

pub fn move_camera(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    for (mut transform, _camera) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let speed = 500.0;
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            velocity.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.x += 1.0;
        }
        
        let move_delta = velocity.normalize_or_zero() * speed * time.delta_secs();
        transform.translation += move_delta;
        
        // Zoom with Q/E
        if keyboard_input.pressed(KeyCode::KeyQ) {
            transform.translation.z += speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            transform.translation.z -= speed * time.delta_secs();
        }
    }
}
