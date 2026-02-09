mod prelude;
pub use crate::prelude::*;

mod core;
mod simulation;
mod rendering;
mod initializations;

use crate::core::*;
use crate::simulation::*;
use crate::rendering::*;
use crate::initializations::*;
use crate::rendering::camera_system::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
             initializations::biome::BiomePlugin,
             initializations::startup::StartupPlugin,
             CameraPlugin,
        ))
        .add_systems(
            PreStartup, (
                initializations::load::load_sprites, 
                initializations::load::load_font, 
                initializations::load::load_sfx,
                initializations::load::load_mesh_assets
            )
        )
        .insert_resource(SelectedObjectInformation::default())
        .insert_resource(InfoPanelInformation::default())
        .init_resource::<TileHash>()
        .init_resource::<Dragging>()
        .init_resource::<GameSpeed>()
        .init_resource::<UniversalMeshAssets>()
        .init_resource::<SpriteSheet>()
        .init_resource::<MyFont>()
        .insert_resource(MenuState {
            state: MenuStates::Home,
        })
        .add_systems(
            Startup, (
                initializations::map::generate_map, 
                setup_camera, 
                rendering::text_system::text_test, 
                initializations::window_system::set_window_title, 
                initializations::window_system::set_window_maximized
            )
        )
        .init_state::<GameState>()
        .add_plugins((
            rendering::interface::MainMenusPlugin, 
            rendering::button_system::ButtonPlugin,
            rendering::selection_systems::SelectionPlugin,
            simulation::monstergenerator_system::MonsterGeneratorPlugin,
            simulation::movetoward_system::MovementPlugin,
            simulation::seasons::SeasonsPlugin,
            simulation::needs::NeedsPlugin,
            rendering::interface::GameUiPlugin,
            rendering::interface::InfoPanelPlugin,
            simulation::thinking_system::ThinkingPlugin,
            simulation::task_system::TaskPlugin,
            simulation::combat_system::CombatPlugin,
            simulation::spoilage_system::SpoilagePlugin,
            rendering::interface::ClickPlugin
        ))
        .add_systems(
            FixedUpdate, (
                simulation::moverandom_system::movement_random
                    .run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_secs_f32(0.1))),
                remove_bad_positions,
                simulation::namegiving_system::namegiving_system,
                simulation::movetoward_system::movement_toward_attackable,
                simulation::nest::nest_system,
            ).run_if(in_state(GameState::InGame))
        )
        .add_systems(
            Update, (
                rendering::statusdisplay_system::status_display_system
                    .run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_secs_f32(0.5))),
                rendering::text_system::text_system,
                rendering::names_system::names_system,
                rendering::text_system::text_update_system,
                rendering::interface::keyboard_input,
                rendering::interface::scrollwheel_input,
            ).run_if(in_state(GameState::InGame))
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(TILE_SIZE * 19.0, TILE_SIZE * 11.0, 500.0)
            .looking_at(Vec3::new(TILE_SIZE * 19.0, TILE_SIZE * 11.0, 0.0), Vec3::Y),
    ));
    
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(100.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
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
