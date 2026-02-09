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
    mut commands: Commands,
    mut plants: Query<(Entity, &mut Plant, &mut Transform, Option<&Foragable>, Option<&Choppable>)>,
) {
    for (entity, mut plant, mut transform, foragable, choppable) in plants.iter_mut() {
        if plant.growth < 1.0 {
            plant.growth += 0.05;
            transform.scale = Vec3::splat(plant.growth);
            if plant.growth >= 0.5 {
                if plant.plant_type.is_forageable().0.is_some() && foragable.is_none() {
                    commands.entity(entity).insert(Foragable);
                }
                if plant.plant_type.is_choppable().0.is_some() && choppable.is_none() {
                    commands.entity(entity).insert(Choppable);
                }
            }
        }
    }
}
