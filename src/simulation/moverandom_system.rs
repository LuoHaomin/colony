use crate::prelude::*;

pub fn movement_random(
    mut entities: Query<(&mut Position, &mut Transform), (With <MoveRandom>, Without<TileType>)>,
    tile_types: Res<TileHash>,
) {
    for (mut position, mut transform) in entities.iter_mut() {
        let mut new_position = *position;
        let mut rng = rand::rng();
        let dir = rng.random_range(0..4);
        match dir {
            0 => new_position.y += 1,
            1 => new_position.y -= 1,
            2 => new_position.x -= 1,
            3 => new_position.x += 1,
            _ => {}
        }
        
        if let Some(tile_type) = tile_types.hash.get(&new_position) {
            if !tile_type.is_wall() {
                *position = new_position;
                *transform = position.to_transform();
            }
        }
    }
}

