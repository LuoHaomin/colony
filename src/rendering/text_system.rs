use crate::prelude::*;

pub fn text_system(
    mut commands: Commands,
    font: Res<MyFont>,
    object_info: Res<SelectedObjectInformation>,
    texts: Query<Entity, With<ObjectText>>,
) {
    for text in texts.iter() {
        commands.entity(text).despawn();
    }
    for (i, info) in object_info.info.iter().enumerate() {
        //println!("POS: {}", (i as f32 * 20.0) );
        commands.spawn((Text::new(info.to_string()), TextFont { font: font.0.clone().into(), font_size: 18.0, ..default() }, TextColor(Color::WHITE.into()), Node { position_type: PositionType::Absolute, bottom: Val::Px(45.0 + (i as f32 * 20.0)), left: Val::Px(15.0), ..default() }, ObjectText));
    }
}


pub fn text_update_system(
    mut query: Query<&mut Text, With<FpsText>>
) {
    for mut text in &mut query {
        *text = Text::new("ZZZZZZ".to_string());
        // if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        //     if let Some(value) = fps.smoothed() {
        //         // Update the value of the second section
        //         text.sections[1].value = format!("{value:.2}");
        //     }
        // }
    }
}


#[derive(Component)]
pub struct ObjectText;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

pub fn text_test(
    
) {
    
}
