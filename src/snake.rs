use bevy::prelude::*;

use crate::common;
use crate::food;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

// Tag Component of Snake Entity
pub struct SnakeHead {
    direction: Direction,
}

pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub struct SnakeSegment;

#[derive(Default)]
pub struct SnakeSegments(Vec<Entity>);

pub fn spawn_segment(
    mut commands: Commands,
    material: &Handle<ColorMaterial>,
    position: common::Position,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            material: (*material).clone(),
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(common::Size::square(0.65))
        .id() // 生成したSnakeSegmentのEntityを返す
}

pub fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut common::Position>,
    mut game_over_writer: EventWriter<common::GameOverEvent>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<common::Position>>();

        let mut head_pos = positions.get_mut(head_entity).unwrap();

        last_tail_position.0 = Some(*segment_positions.last().unwrap());

        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };

        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.clone().x as u32 >= common::ARENA_WIDTH
            || head_pos.clone().y as u32 >= common::ARENA_HEIGHT
        {
            game_over_writer.send(common::GameOverEvent);
        }

        if segment_positions.contains(&head_pos) {
            game_over_writer.send(common::GameOverEvent);
        }

        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    }
}

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &common::Position), With<food::Food>>,
    head_positions: Query<&common::Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub struct GrowthEvent;

#[derive(Default)]
pub struct LastTailPosition(Option<common::Position>);

pub fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
    materials: Res<common::Materials>,
) {
    if growth_reader.iter().next().is_some() {
        segments.0.push(spawn_segment(
            commands,
            &materials.segment_material,
            last_tail_position.0.unwrap(),
        ));
    }
}

pub fn spawn_snake(
    mut commands: Commands,
    materials: Res<common::Materials>,
    mut segments: ResMut<SnakeSegments>,
) {
    segments.0 = vec![
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.head_material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                ..Default::default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(common::Position { x: 3, y: 3 })
            .insert(common::Size::square(0.8))
            .id(), // SnakeHeadのEntityを返す
        spawn_segment(
            commands,
            &materials.segment_material,
            common::Position { x: 3, y: 2 },
        ), // Head以降のSnakeSegmentのEntityを返す
    ];
}