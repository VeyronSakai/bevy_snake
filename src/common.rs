use bevy::prelude::*;

pub const ARENA_WIDTH: u32 = 10;
pub const ARENA_HEIGHT: u32 = 10;

// Resource materials of the snake
pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
    pub segment_material: Handle<ColorMaterial>,
    pub food_material: Handle<ColorMaterial>,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x.clone(),
            height: x.clone(),
        }
    }
}

pub struct GameOverEvent;

