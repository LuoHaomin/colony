use crate::prelude::*;

pub struct SeasonsPlugin;

impl Plugin for SeasonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate,
            seasons_system
            .run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_secs_f32(5.0)))
            .run_if(in_state(GameState::InGame))
        );
    }
}

pub fn seasons_system(
    _commands: Commands,
    mut plants: Query<(&mut Plant, &mut Transform)>,
) {
    for (mut plant, mut transform) in plants.iter_mut() {
        if plant.growth < 1.0 {
            plant.growth += 0.05;
            transform.scale = Vec3::splat(plant.growth);
        }
    }
}
