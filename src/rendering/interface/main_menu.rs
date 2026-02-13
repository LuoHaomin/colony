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
            OnExit(GameState::MainMenu), 
            close_main_menu
        )
        ;
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
                width: Val::Vw(100.0), 
                height: Val::Vh(100.0), 
                ..Default::default() 
            }, 
            BackgroundColor(Color::srgba(0.15, 0.25, 0.15, 1.0))
        ))
        .insert(MainMenuOverlay)
        .with_children(|parent| {
            parent.spawn((Text::new("WELCOME TO".to_string()), text_font.clone(), TextColor(Color::WHITE.into())));
            parent.spawn((Text::new("COLONY".to_string()), text_font.clone(), TextColor(Color::WHITE.into())));
            parent.spawn((Text::new("Get Started".to_string()), text_font.clone(), TextColor(Color::WHITE.into()), Node { margin: UiRect::all(Val::Px(20.0)), ..default() }));
            // Next insert a button
            parent.spawn((
                Button,
                Node { 
                    width: Val::Px(200.0), 
                    height: Val::Px(50.0), 
                    margin: UiRect::all(Val::Px(20.0)), 
                    justify_content: JustifyContent::Center, 
                    align_items: AlignItems::Center, 
                    ..default() 
                }, 
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
            ))
            .with_children(|parent| {
                parent.spawn((Text::new("Start Game".to_string()), text_font.clone(), TextColor(Color::WHITE.into())));
            });


        })
        // .spawn(TextComponents {
        //     text: Text {
        //         value: "Press Space to Start".to_string(),
        //         font: asset_server.load("fonts/Roboto-Bold.ttf"),
        //         style: TextStyle {
        //             font_size: 30.0,
        //             color: Color::rgb(0.9, 0.9, 0.9),
        //             ..Default::default()
        //         },
        //     },
        //     style: Style {
        //         size: Size::new(Val::Px(window_size.x), Val::Px(window_size.y)),
        //         position_type: PositionType::Absolute,
        //         position: Rect {
        //             left: Val::Px(0.0),
        //             top: Val::Px(0.0),
        //             ..Default::default()
        //         },
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // })
        ;
}

pub fn close_main_menu(
    mut commands: Commands,
    _state: Res<State<GameState>>,
    mut query: Query<Entity, With<MainMenuOverlay>>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
    // state.set(GameState::InGame).unwrap();
}
