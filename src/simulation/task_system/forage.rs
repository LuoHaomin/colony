use crate::prelude::*;

pub fn task_system_forage(
    mut commands: Commands,
    mut entities_that_might_forage: Query<(Entity, &mut Brain, &Position, Option<&Pathing>, Option<&Targeting>)>,
    mut targets: Query<(Entity, &Position, &Foragable, &mut Plant), With<WorkTarget>>,
    workmarkers: Query<(Entity, &ChildOf), With<WorkMarker>>,
    mesh_assets: Res<UniversalMeshAssets>,
) {
    let mut already_targeted = super::set_already_targetted(&entities_that_might_forage);
    'outer: for (entity, mut brain, position, pathing, targeting) in entities_that_might_forage.iter_mut() {
        if pathing.is_some() { continue; }
        if brain.task != Some(Task::Forage) { continue; }

        let mut shortest_distance = -1;
        let mut closest_entity = None;
        let mut closest_position = None;

        for (targetable_entity, targetable_position, _, mut plant) in targets.iter_mut() {
            let distance = position.distance(targetable_position);
            if distance <= 1 && targeting.is_some() && targeting.unwrap().target == targetable_entity {
                commands.entity(entity).remove::<Targeting>();
                super::remove_x_markers(&mut commands, &workmarkers, targetable_entity);
                spawn_berries(&mut commands, targetable_entity, targetable_position, &mesh_assets, &mut plant);
                continue 'outer;
            }
            if already_targeted.contains(&targetable_entity) { continue; }

            if shortest_distance == -1 || distance < shortest_distance {
                shortest_distance = distance;
                closest_entity = Some(targetable_entity);
                closest_position = Some(targetable_position);
            }
        }

        if let Some(closest_entity) = closest_entity {
            commands.entity(entity).insert(Targeting { target: closest_entity });
            commands.entity(entity).insert(Pathing { path: vec![], destination: *closest_position.unwrap() });
            already_targeted.push(closest_entity);
        } else {
            brain.remotivate();
        }
    }
}

fn spawn_berries(
    commands: &mut Commands,
    _targetable_entity: Entity,
    targetable_position: &Position,
    mesh_assets: &Res<UniversalMeshAssets>,
    plant: &mut Plant,
) {
    plant.growth = 0.5;
    commands.spawn((
        Mesh3d(mesh_assets.sphere.clone()),
        MeshMaterial3d(mesh_assets.material_red.clone()),
        targetable_position.to_transform(),
        *targetable_position,
        Item { item_type: ItemType::Berry },
    ));
}
