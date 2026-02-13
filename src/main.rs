mod prelude;
pub use crate::prelude::*;

mod core;
mod simulation;
mod rendering;
mod initializations;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CurrentDisplayZ { z: 0 })
        .add_plugins((
             initializations::BiomePlugin,
             initializations::StartupPlugin,
             rendering::CameraPlugin,
        ))
        .add_systems(
            PreStartup, (
                initializations::load_sprites, 
                initializations::load_font, 
                initializations::load_sfx,
                initializations::load_mesh_assets
            )
        )
        .insert_resource(SelectedObjectInformation::default())
        .insert_resource(InfoPanelInformation::default())
        .init_resource::<TileHash>()
        .init_resource::<TileEnvHash>()
        .init_resource::<Dragging>()
        .init_resource::<GameSpeed>()
        .init_resource::<CurrentDisplayZ>()
        .init_resource::<VisualizationMode>()
        .init_resource::<UniversalMeshAssets>()
        .init_resource::<SpriteSheet>()
        .init_resource::<MyFont>()
        .insert_resource(MenuState {
            state: MenuStates::Home,
        })
        .add_systems(
            Startup, (
                initializations::generate_map, 
                setup_camera, 
                rendering::text_test, 
                initializations::set_window_title, 
                initializations::set_window_maximized
            )
        )
        .init_state::<GameState>()
        .add_plugins((
            rendering::MainMenusPlugin, 
            rendering::SelectionPlugin,
            simulation::monstergenerator_system::MonsterGeneratorPlugin,
            simulation::seasons::SeasonsPlugin,
            simulation::needs::NeedsPlugin,
            rendering::GameUiPlugin,
        ))
        .add_plugins((
            rendering::InfoPanelPlugin,
            rendering::FeedbackPlugin,
            simulation::thinking_system::ThinkingPlugin,
            simulation::action_system::ActionPlugin,
            simulation::task_system::TaskPlugin,
            simulation::reproduction_system::ReproductionPlugin,
            simulation::spoilage_system::SpoilagePlugin,
            rendering::ClickPlugin,
            rendering::VisibilityPlugin
        ))
        .add_systems(
            FixedUpdate, (
                remove_bad_positions,
                simulation::namegiving_system::namegiving_system,
            ).run_if(in_state(GameState::InGame))
        )
        .add_systems(
            Update, (
                rendering::status_display_system
                    .run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_secs_f32(0.5))),
                rendering::text_system,
                rendering::names_system,
                rendering::update_unit_status_text,
                rendering::text_update_system,
                rendering::scrollwheel_input,
            ).run_if(in_state(GameState::InGame))
        )
        .add_systems(Update, rendering::keyboard_input)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(TILE_SIZE * 19.0, TILE_SIZE * 11.0, 100.0),
    ));
}

fn remove_bad_positions(
    mut commands: Commands,
    query: Query<(Entity, &Position), Without<MapTile>>,
    tiletypes: Res<TileHash>,
) {
    for (entity, position) in query.iter() {
        if tiletypes.hash.contains_key(position) {
            if tiletypes.hash.get(position).unwrap().is_wall() {
                commands.entity(entity).despawn();
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}
