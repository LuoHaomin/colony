use bevy::prelude::*;
use crate::prelude::*;
use crate::simulation::unitgenerator_system::{spawn_unit_from_template, UnitTemplate};
use std::collections::HashMap;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), (spawn_settlers, spawn_starting_stuff, finalize_initialization).chain());
    }
}

pub fn finalize_initialization(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::InGame);
}

pub fn spawn_settlers(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    mesh_assets: Res<UniversalMeshAssets>,
    tilehash: Res<TileHash>,
) {
    for i in 0..MAP_WIDTH {
        let x = i;
        let y = MAP_LENGTH / 2;

        let mut spawn_z = 0;
        for z in (-3..3).rev() {
            if let Some(tile) = tilehash.hash.get(&Position { x: x as i32, y: y as i32, z }) {
                if !tile.is_wall() {
                    spawn_z = z;
                    break;
                }
            }
        }

        let position = Position { x: x as i32, y: y as i32, z: spawn_z };
        if i == MAP_WIDTH / 2 {
            spawn_unit_from_template(&mut commands, position, &sprite_sheet, &UnitTemplate::elf(), &mesh_assets);
        }
        if i == MAP_WIDTH / 2 + 1 {
            spawn_unit_from_template(&mut commands, position, &sprite_sheet, &UnitTemplate::dwarf(), &mesh_assets);
        }
        if i == MAP_WIDTH / 2 + 2 {
            spawn_unit_from_template(&mut commands, position, &sprite_sheet, &UnitTemplate::human(), &mesh_assets);
        }
    }

    let x = MAP_WIDTH / 2;
    let y = MAP_LENGTH / 2 - 2;
    let mut spawn_z = 0;
    for z in (-3..3).rev() {
        if let Some(tile) = tilehash.hash.get(&Position { x: x as i32, y: y as i32, z }) {
            if !tile.is_wall() {
                spawn_z = z;
                break;
            }
        }
    }
    let position = Position { x: x as i32, y: y as i32, z: spawn_z };
    spawn_unit_from_template(&mut commands, position, &sprite_sheet, &UnitTemplate::crab(), &mesh_assets);
}

pub fn spawn_starting_stuff(
    mut commands: Commands,
    biome: Res<Biome>,
    mesh_assets: Res<UniversalMeshAssets>,
    sprite_sheet: Res<SpriteSheet>,
    tilehash: Res<TileHash>,
) {
    let x = MAP_WIDTH / 2;
    let y = MAP_LENGTH / 2 + 3;
    let mut spawn_z = 0;
    for z in (-3..3).rev() {
        if let Some(tile) = tilehash.hash.get(&Position { x: x as i32, y: y as i32, z }) {
            if !tile.is_wall() {
                spawn_z = z;
                break;
            }
        }
    }
    let position = Position { x: x as i32, y: y as i32, z: spawn_z };
    
    commands
        .spawn((
            Sprite {
                image: sprite_sheet.handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprite_sheet.layout.clone(),
                    index: ItemType::StatuePillar1.sprite_index(),
                }),
                ..default()
            },
            position.to_transform(),
        ))
        .insert(position)
        .insert(SizeXYZ::cube(1.0))
        .insert(MonsterGenerator { monsters: vec![(UnitTemplate::rat(),1),(UnitTemplate::spider(),5),(UnitTemplate::cyclops(),1)] })
        .insert(Visibility::default());

    // GENERATE PLANTS
    let mut taken_positions: HashMap<Position, u8> = HashMap::new();
    let mut rng = rand::rng();

    for _ in 0..(MAP_WIDTH*MAP_LENGTH / 10) {
        let x = rng.random_range(1..MAP_WIDTH-1);
        let y = rng.random_range(1..MAP_LENGTH-1);
        
        let mut spawn_z = 0;
        for z in (-3..3).rev() {
            if let Some(tile) = tilehash.hash.get(&Position { x: x as i32, y: y as i32, z }) {
                if !tile.is_wall() {
                    spawn_z = z;
                    break;
                }
            }
        }

        let growth = rng.random_range(0.1..1.0);
        let position = Position { x: x as i32, y: y as i32, z: spawn_z };
        if taken_positions.contains_key(&position) { continue; }
        taken_positions.insert(position, 1);
        
        if biome.plants.is_empty() { continue; }
        let plant_type = biome.plants[rng.random_range(0..biome.plants.len())];

        let plant = commands
            .spawn((
                Sprite {
                    image: sprite_sheet.handle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprite_sheet.layout.clone(),
                        index: plant_type.sprite_index(),
                    }),
                    ..default()
                },
                position.to_transform(),
            ))
            .insert(position)
            .insert(Plant { growth, plant_type })
            .insert( Object { itemtype: plant_type, ..default() } )
            .insert(Visibility::default())
            .id();
        
        if plant_type.is_forageable().0.is_some() && growth > 0.5 {
            commands.entity(plant).insert(Foragable);
        }
        if plant_type.is_choppable().0.is_some() && growth > 0.5 {
            commands.entity(plant).insert(Choppable);
        }
    }
    
    // Spawn Objects (Items)
    let scarcity = biome.objects_overall_scarcity.max(1) as i32;
    for _ in 0..(MAP_WIDTH*MAP_LENGTH / scarcity) {
        let x = rng.random_range(1..MAP_WIDTH-1) as i32;
        let y = rng.random_range(1..MAP_LENGTH-1) as i32;
        
        let mut spawn_z = 0;
        for z in (-3..3).rev() {
            if let Some(tile) = tilehash.hash.get(&Position { x, y, z }) {
                if !tile.is_wall() {
                    spawn_z = z;
                    break;
                }
            }
        }

        let position = Position { x, y, z: spawn_z };
        if taken_positions.contains_key(&position) { continue; }
        taken_positions.insert(position, 1);
        
        if biome.objects.is_empty() { continue; }
        let object_type = biome.objects[rng.random_range(0..biome.objects.len())];

        let object = commands
            .spawn((
                Sprite {
                    image: sprite_sheet.handle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprite_sheet.layout.clone(),
                        index: object_type.sprite_index(),
                    }),
                    ..default()
                },
                position.to_transform(),
            ))
            .insert(position)
            .insert( Object { itemtype: object_type, ..default() } )
            .insert(Visibility::default())
            .id();
        object_type.add_components(&mut commands, object);
    }
}

pub fn text_test(_commands: Commands) {}
