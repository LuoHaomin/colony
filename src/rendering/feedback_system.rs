use crate::prelude::*;

pub struct FeedbackPlugin;

impl Plugin for FeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            visual_feedback_system,
        ).run_if(in_state(GameState::InGame)));
    }
}

pub fn visual_feedback_system(
    time: Res<Time>,
    mut query: Query<(&mut VisualFeedback, &mut Transform)>,
) {
    for (mut feedback, mut transform) in query.iter_mut() {
        if feedback.shake_timer > 0.0 {
            feedback.shake_timer -= time.delta_secs();
            
            // Apply random shake offset
            let offset = Vec3::new(
                (rand::rng().random_range(-1.0..1.0)) * feedback.shake_intensity,
                (rand::rng().random_range(-1.0..1.0)) * feedback.shake_intensity,
                0.0
            );
            transform.translation += offset;
            
            // Clean up if finished
            if feedback.shake_timer <= 0.0 {
                feedback.shake_intensity = 0.0;
            }
        }
    }
}
