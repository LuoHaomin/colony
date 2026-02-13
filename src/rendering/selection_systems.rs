use crate::prelude::*;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_message::<SelectionEvent>()
        ;
    }
}

#[derive(Message)]
pub struct SelectionEvent {
    pub selected_position: Option<Position>,
    pub selected_type: SelectableType,
}
