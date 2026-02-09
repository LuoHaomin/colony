use crate::prelude::*;

pub fn keyboard_input(
    _commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    gamestate: Res<State<GameState>>,
    mut nextstate: ResMut<NextState<GameState>>,
    mut current_z: ResMut<CurrentDisplayZ>,
) {
    if input.just_pressed(KeyCode::Space) {
        // Pause or Unpause.
        match gamestate.get() {
            GameState::MainMenu => {
                nextstate.set(GameState::InGame);
            }
            GameState::InGame => {
                nextstate.set(GameState::Paused);
            }
            GameState::Paused => {
                nextstate.set(GameState::InGame);
            }
        }
    }

    if input.just_pressed(KeyCode::Comma) || input.just_pressed(KeyCode::KeyQ) {
        current_z.z = (current_z.z + 1).clamp(-3, 2);
    }
    if input.just_pressed(KeyCode::Period) || input.just_pressed(KeyCode::KeyE) {
        current_z.z = (current_z.z - 1).clamp(-3, 2);
    }

    for mut transform in camera.iter_mut() {
        let move_speed = 16.0;
        //transform.translation.x += 5.0;
        let mut next_position = transform.translation;
        if input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {//pressed(KeyCode::Up) || input.pressed(KeyCode::W) {
            next_position.y += move_speed;
        } else if input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            next_position.y -= move_speed;
        } else if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            next_position.x -= move_speed;
        } else if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            next_position.x += move_speed;
        }
        transform.translation = next_position;
        if (next_position.x >= -15.0) && (next_position.x < VIEWAREA_WIDTH as f32 * MAP_WIDTH as f32) && (next_position.y >= -15.0) && (next_position.y < VIEWAREA_HEIGHT as f32 * MAP_LENGTH as f32) {
            //transform.translation = next_position;
        }
        //transform.translation = next_position;
    }
}

pub fn scrollwheel_input(
    _commands: Commands,
    mut scroll_evr: MessageReader<MouseWheel>,
    //mut camera: Query<&mut Transform, With<Camera>>
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in camera.iter_mut() {
        let mut next_scale = transform.scale; // Vec3
        for ev in scroll_evr.read() {
            if ev.y > 0.0 {
                next_scale *= 1.1;
            } else if ev.y < 0.0 {
                next_scale *= 0.9;
            }
        }
        // Limit next_scale to between 0.5 and 1.5 (apply uniformly)
        let clamped = next_scale.x.min(1.5).max(0.5);
        transform.scale = Vec3::splat(clamped);
    }
    // for mut transform in camera.iter_mut() {
    //     let move_speed = 16.0;
    //     //transform.translation.x += 5.0;
    //     let mut next_position = transform.translation;
    //     for ev in scroll_evr.iter() {
    //         if ev.y > 0.0 {
    //             next_position.z += move_speed;
    //         } else if ev.y < 0.0 {
    //             next_position.z -= move_speed;
    //         }
    //     }
    //     transform.translation = next_position;
    // }
}
