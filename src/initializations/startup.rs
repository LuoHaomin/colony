use bevy::prelude::*;
use crate::prelude::*;
use crate::simulation::unitgenerator_system::{spawn_unit_from_template, UnitTemplate};
use std::collections::HashMap;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (spawn_settlers, spawn_starting_stuff));
    }
}

pub fn spawn_settlers(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    mesh_assets: Res<UniversalMeshAssets>,
) {
    for i in 0..MAP_WIDTH {
        let x = i;
        let y = MAP_LENGTH / 2;
        let position = Position { x, y, z: 0 };
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
    let position = Position { x, y, z: 0 };
    spawn_unit_from_template(&mut commands, position, &sprite_sheet, &UnitTemplate::crab(), &mesh_assets);
}

pub fn spawn_starting_stuff(
    mut commands: Commands,
    biome: Res<Biome>,
    mesh_assets: Res<UniversalMeshAssets>,
) {
    let position = Position { x: MAP_WIDTH / 2, y: MAP_LENGTH / 2 + 3, z: 0 };
    
    commands
        .spawn((
            Mesh3d(mesh_assets.cube.clone()),
            MeshMaterial3d(mesh_assets.material_white.clone()),
            Transform::from_xyz(position.x as f32 * TILE_SIZE, position.y as f32 * TILE_SIZE, position.z as f32 * TILE_SIZE)
        ))
        .insert(position)
        .insert(SizeXYZ::cube(1.0))
        .insert(MonsterGenerator { monsters: vec![(UnitTemplate::rat(),1),(UnitTemplate::spider(),5),(UnitTemplate::cyclops(),1)] })
        .insert(position.to_transform());

    // GENERATE PLANTS
    let mut taken_positions: HashMap<Position, u8> = HashMap::new();
    let mut rng = rand::rng();

    for _ in 0..(MAP_WIDTH*MAP_LENGTH / 10) {
        let x = rng.random_range(1..MAP_WIDTH-1);
        let y = rng.random_range(1..MAP_LENGTH-1);
        let growth = rng.random_range(0.1..1.0);
        let position = Position { x, y, z: 0 };
        if taken_positions.contains_key(&position) { continue; }
        taken_positions.insert(position, 1);
        
        if biome.plants.is_empty() { continue; }
        let plant_type = biome.plants[rng.random_range(0..biome.plants.len())];

        let plant = commands
            .spawn((
                Mesh3d(mesh_assets.cylinder.clone()),
                MeshMaterial3d(mesh_assets.material_brown.clone()),
                Transform::from_xyz(position.x as f32 * TILE_SIZE, position.y as f32 * TILE_SIZE, TILE_SIZE * 0.3),
            ))
            .insert(position)
            .insert(Plant { growth, plant_type })
            .insert( Object { itemtype: plant_type, ..default() } )
            .with_children(|parent| {
                parent.spawn((
                    Mesh3d(mesh_assets.sphere.clone()),
                    MeshMaterial3d(mesh_assets.material_green.clone()),
                    Transform::from_xyz(0.0, 0.0, TILE_SIZE * 0.4),
                ));
            })
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
        let position = Position { x, y, z: 0 };
        if taken_positions.contains_key(&position) { continue; }
        taken_positions.insert(position, 1);
        
        if biome.objects.is_empty() { continue; }
        let object_type = biome.objects[rng.random_range(0..biome.objects.len())];

        let object = commands
            .spawn((
                Mesh3d(mesh_assets.cube.clone()),
                MeshMaterial3d(mesh_assets.material_white.clone()),
                Transform::from_xyz(position.x as f32 * TILE_SIZE, position.y as f32 * TILE_SIZE, TILE_SIZE * 0.1),
            ))
            .insert(position)
            .insert( Object { itemtype: object_type, ..default() } )
            .id();
        object_type.add_components(&mut commands, object);
    }
}

pub fn text_test(_commands: Commands) {}
