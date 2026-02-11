use crate::prelude::*;

#[derive(Component)]
pub struct MainHudRoot;

#[derive(Component)]
pub struct TopBar;

#[derive(Component)]
pub struct BottomBar;

#[derive(Component)]
pub struct InspectorPanel;

#[derive(Component)]
pub struct InspectorContent;

#[derive(Component)]
pub struct CommandButton {
    pub menu_state: Option<MenuStates>,
    pub selectable: Option<SelectableType>,
    pub zone: Option<ZoneType>,
    pub item: Option<ItemType>,
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            OnEnter(GameState::InGame),
            (initialize_game_ui, start_game_ui).chain()
        )
        .add_systems(
            Update,
            (
                game_ui_click.run_if(in_state(GameState::InGame)),
                hud_button_interaction.run_if(in_state(GameState::InGame)),
                update_hud_on_state_change.run_if(in_state(GameState::InGame)),
            )
        );
    }
}

pub fn initialize_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Root Node for HUD
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        MainHudRoot,
    )).with_children(|parent| {
        // TOP BAR
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
            TopBar,
        )).with_children(|top| {
            top.spawn(Text::new("Observation Ark - Colony v0.1.1"));
        });

        // BOTTOM BAR
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
            BottomBar,
        ));

        // INSPECTOR PANEL (Right side)
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(40.0),
                right: Val::Px(0.0),
                width: Val::Px(250.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
            InspectorPanel,
        )).with_children(|inspector| {
            inspector.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                InspectorContent,
            ));
        });
    });
}

pub fn update_hud_on_state_change(
    menu_state: Res<MenuState>,
    mut commands: Commands,
    bottom_bar: Query<Entity, With<BottomBar>>,
    button_query: Query<Entity, With<CommandButton>>,
    font: Res<MyFont>,
) {
    if menu_state.is_changed() {
        for entity in button_query.iter() {
            commands.entity(entity).despawn();
        }

        if let Some(bottom_bar_entity) = bottom_bar.iter().next() {
            let text_font = TextFont { font: font.0.clone(), font_size: 16.0, ..default() };
            
            // Build buttons based on menu_state.state
            let buttons = match menu_state.state {
                MenuStates::Home => vec![
                    ("TASKS", Some(MenuStates::Tasks), None, None, None),
                    ("FARM", Some(MenuStates::Farm), None, None, None),
                    ("BUILD", Some(MenuStates::Build), None, None, None),
                    ("ZONE", Some(MenuStates::Zone), None, None, None),
                ],
                MenuStates::Tasks => vec![
                    ("BACK", Some(MenuStates::Home), Some(SelectableType::Nothing), None, None),
                    ("CLEAR", None, Some(SelectableType::Unselecting), None, None),
                    ("CHOP", None, Some(SelectableType::Choppable), None, None),
                    ("FORAGE", None, Some(SelectableType::Foragable), None, None),
                    ("CARRY", None, Some(SelectableType::Carryable), None, None),
                    ("HUNT", None, Some(SelectableType::Huntable), None, None),
                    ("MINE", None, Some(SelectableType::Mineable), None, None),
                ],
                MenuStates::Farm => vec![
                    ("BACK", Some(MenuStates::Home), Some(SelectableType::Nothing), None, None),
                    ("CLEAR", None, Some(SelectableType::Unzoning), None, None),
                    ("CABBAGE", None, Some(SelectableType::Zoning), Some(ZoneType::Farm), Some(ItemType::Cabbage)),
                    ("PINE", None, Some(SelectableType::Zoning), Some(ZoneType::Farm), Some(ItemType::PineTree)),
                    ("OAK", None, Some(SelectableType::Zoning), Some(ZoneType::Farm), Some(ItemType::OakTree)),
                    ("CEDAR", None, Some(SelectableType::Zoning), Some(ZoneType::Farm), Some(ItemType::CedarTree)),
                ],
                MenuStates::Build => vec![
                    ("BACK", Some(MenuStates::Home), Some(SelectableType::Nothing), None, None),
                    ("CLEAR", None, Some(SelectableType::Unzoning), None, None),
                    ("WALL", None, Some(SelectableType::Zoning), Some(ZoneType::Construction), Some(ItemType::WallWood)),
                    // Add more later
                ],
                MenuStates::Zone => vec![
                    ("BACK", Some(MenuStates::Home), Some(SelectableType::Nothing), None, None),
                    ("STORAGE", None, Some(SelectableType::Zoning), Some(ZoneType::Storage), None),
                    ("AVOID", None, Some(SelectableType::Zoning), Some(ZoneType::Avoid), None),
                ],
            };

            commands.entity(bottom_bar_entity).with_children(|parent| {
                for (label, m_state, s_type, z_type, i_type) in buttons {
                    parent.spawn((
                        Button,
                        Node {
                            width: Val::Px(90.0),
                            height: Val::Px(60.0),
                            margin: UiRect::horizontal(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.3, 0.3, 0.5, 1.0)),
                        CommandButton {
                            menu_state: m_state,
                            selectable: s_type,
                            zone: z_type,
                            item: i_type,
                        },
                    )).with_children(|btn| {
                        btn.spawn((Text::new(label), text_font.clone()));
                    });
                }
            });
        }
    }
}

pub fn hud_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &CommandButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<MenuState>,
    mut dragging: ResMut<Dragging>,
) {
    for (interaction, button, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgba(0.5, 0.5, 0.8, 1.0));
                if let Some(ms) = button.menu_state {
                    menu_state.state = ms;
                }
                if let Some(st) = button.selectable {
                    dragging.looking_for = st;
                }
                if let Some(zt) = button.zone {
                    dragging.zone_type = zt;
                }
                if let Some(it) = button.item {
                    dragging.item_type = it;
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgba(0.4, 0.4, 0.7, 1.0));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgba(0.3, 0.3, 0.5, 1.0));
            }
        }
    }
}

// Keeping old systems for now to avoid breaking other logic, 
// but we'll disable their functionality or transition them.
pub fn start_game_ui(
    mut menu_state: ResMut<MenuState>,
) {
    // Just trigger a change to refresh the HUD
    menu_state.as_mut().state = MenuStates::Home;
}

pub fn game_ui_click() {}

