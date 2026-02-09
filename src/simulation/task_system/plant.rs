use crate::prelude::*;

pub struct TaskPluginZone;

impl Plugin for TaskPluginZone {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, task_system_zone.run_if(in_state(GameState::InGame)));
    }
}

pub fn task_system_zone(
    mut commands: Commands,
    mut entities_that_might_plant: Query<(Entity, &mut Brain, &Position, Option<&Pathing>, Option<&Targeting>)>,
    targetables: Query<(Entity, &Position, &Zone)>,
    obstacles: Query<(Entity, &Position), Without<MapTile>>,
    _sprite_sheet: Res<SpriteSheet>,
    mesh_assets: Res<UniversalMeshAssets>,
) {
    let mut already_targeted = super::set_already_targetted(&entities_that_might_plant);
    'brains: for (entity, mut brain, position, pathing, targeting) in entities_that_might_plant.iter_mut() {
        if pathing.is_some() { continue; }
        if brain.task.is_none() { continue; }
        if ! brain.task.unwrap().is_zone_task() { continue; }
        let mut nearest_entity: Option<NearestEntity> = None;
        'targets: for (targetable_entity, targetable_position, zone) in targetables.iter() {
            if zone.zone_type == ZoneType::Farm && brain.task != Some(Task::Plant) { continue; }
            if zone.zone_type == ZoneType::Construction && brain.task != Some(Task::Construct) { continue; }
            if zone.zone_type == ZoneType::Storage && brain.task != Some(Task::Carrying) { continue; }
            
            let distance = position.distance(targetable_position);
            if distance <= 1 && targeting.is_some() && targeting.unwrap().target == targetable_entity {
                if brain.task == Some(Task::Plant) {
                    spawn_item(&mut commands, targetable_position, &mesh_assets, zone);
                    commands.entity(entity).insert(Targeting { target: targetable_entity });
                    commands.entity(entity).remove::<Targeting>();
                    continue 'brains;
                }
                if brain.task == Some(Task::Construct) {
                    spawn_building(&mut commands, targetable_position, &mesh_assets, zone);
                    commands.entity(entity).remove::<Targeting>();
                    continue 'brains;
                }
            }
            if already_targeted.contains(&targetable_entity) { continue; }
            for (_obstacle_entity, obstacle_position) in obstacles.iter() {
                if obstacle_position == targetable_position { continue 'targets; }
            }
            if nearest_entity.is_none() || distance < nearest_entity.as_ref().unwrap().distance {
                nearest_entity = Some(NearestEntity {
                    entity: targetable_entity,
                    distance,
                    position: *targetable_position,
                });
            }
        }
        if let Some(nearest) = nearest_entity {
            already_targeted.push(nearest.entity);
            commands.entity(entity).insert(Targeting { target: nearest.entity });
            let mut brain_task = brain.task.unwrap();
            brain_task = brain_task.get_steps();
            brain.task = Some(brain_task);
        }
    }
}

fn spawn_item(
    commands: &mut Commands,
    position: &Position,
    mesh_assets: &UniversalMeshAssets,
    zone: &Zone,
) {
    commands.spawn((
        Mesh3d(mesh_assets.cube.clone()),
        MeshMaterial3d(mesh_assets.material_white.clone()),
        Transform::from_xyz(position.x as f32 * TILE_SIZE, position.y as f32 * TILE_SIZE, TILE_SIZE * 0.1),
    ))
    .insert(*position)
    .insert(Plant { growth: 0.4, plant_type: zone.item_type })
    .insert( Object { itemtype: zone.item_type, ..default() } );
}

fn spawn_building(
    commands: &mut Commands,
    position: &Position,
    mesh_assets: &UniversalMeshAssets,
    zone: &Zone,
) {
    commands.spawn((
        Mesh3d(mesh_assets.cube.clone()),
        MeshMaterial3d(mesh_assets.material_white.clone()),
        Transform::from_xyz(position.x as f32 * TILE_SIZE, position.y as f32 * TILE_SIZE, TILE_SIZE * 0.1),
    ))
    .insert(*position)
    .insert(Object { itemtype: zone.item_type, under_construction: false, ..default() });
}

