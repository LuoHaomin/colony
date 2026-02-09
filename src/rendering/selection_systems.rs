use crate::prelude::*;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_message::<SelectionEvent>()
        .add_systems(Update, (
            select_foragables,
        ))
        ;
    }
}

#[derive(Message)]
pub struct SelectionEvent {
    pub selected_position: Option<Position>,
    pub selected_type: SelectableType,
}

pub fn select_foragables(
    mut commands: Commands,
    mut selection_reader: MessageReader<SelectionEvent>,
    mut query: Query<(Entity, Option<&Foragable>), With<Highlighted>>,
) {
    for event in selection_reader.read() {
        // Logic for selection
    }
}
