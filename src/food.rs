use bevy::prelude::*;
use rand::prelude::random;

use crate::common;

pub struct Food;

pub fn food_spawner(
    mut commands: Commands,
    materials: Res<common::Materials>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.food_material.clone(),
            ..Default::default()
        })
        .insert(Food)
        .insert(common::Position {
            x: (random::<f32>() * common::ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * common::ARENA_HEIGHT as f32) as i32,
        })
        .insert(common::Size::square(0.8));
}