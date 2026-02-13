use crate::prelude::*;

pub struct MainMenusPlugin;

impl Plugin for MainMenusPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            OnEnter(GameState::MainMenu), 
            open_main_menu
        )
        .add_systems(
            Update,
            main_menu_button_system.run_if(in_state(GameState::MainMenu))
        )
        .add_systems(
            OnExit(GameState::MainMenu), 
            close_main_menu
        )
        ;
    }
}

pub fn main_menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        println!("Main Menu Button Interaction: {:?}", interaction);
        match *interaction {
            Interaction::Pressed => {
                println!("START GAME BUTTON PRESSED!");
                *color = BackgroundColor(Color::srgb(0.35, 0.75, 0.35));
                next_state.set(GameState::Initializing);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }
}

fn open_main_menu(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    _materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    font: Res<MyFont>,
) {
    // Center window
    let window = windows.single().expect("No primary window");
    let width = window.width();
    let height = window.height();
    let _window_size = Vec2::new(width, height);

    // Create main menu
    let text_font = TextFont { font: font.0.clone().into(), font_size: 18.0, ..default() };
    commands
        .spawn((
            Node { 
                flex_direction: FlexDirection::Column, 
                position_type: PositionType::Absolute, 
                justify_content: JustifyContent::Center, 
                align_items: AlignItems::Center, 
                width: Val::Percent(100.0), 
                height: Val::Percent(100.0), 
                ..Default::default() 
            }, 
            BackgroundColor(Color::srgba(0.15, 0.25, 0.15, 1.0)),
            MainMenuOverlay,
        ))
        .with_children(|parent| {
            parent.spawn((Text::new("WELCOME TO".to_string()), text_font.clone(), TextColor(Color::WHITE.into())));
            parent.spawn((Text::new("COLONY".to_string()), text_font.clone(), TextColor(Color::WHITE.into())));
            parent.spawn((Text::new("Get Started".to_string()), text_font.clone(), TextColor(Color::WHITE.into()), Node { margin: UiRect::all(Val::Px(20.0)), ..default() }));
            // Next insert a button
            parent.spawn((
                Button,
                Node { 
                    width: Val::Px(240.0), 
                    height: Val::Px(60.0), 
                    margin: UiRect::all(Val::Px(20.0)), 
                    justify_content: JustifyContent::Center, 
                    align_items: AlignItems::Center, 
                    border: UiRect::all(Val::Px(2.0)),
                    ..default() 
                }, 
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2))
            ))
            .with_children(|parent| {
                parent.spawn((Text::new("Start Game".to_string()), text_font.clone(), TextColor(Color::WHITE.into())));
            });
        })
        ;
}

pub fn close_main_menu(
    mut commands: Commands,
    mut query: Query<Entity, With<MainMenuOverlay>>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
