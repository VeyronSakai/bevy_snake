mod snake;
mod common;
mod food;

use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::core::FixedTimestep;

fn main() {
    App::build()
        // ウインドウの生成
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // Snakeが大きくなった際に発火されるイベント
        .add_event::<snake::GrowthEvent>()
        // Snakeが壁にぶつかったり自分自身にぶつかった際に発火されるイベント
        .add_event::<common::GameOverEvent>()
        // 初期化処理。StartUp Stageで実行される。
        .add_startup_system(setup.system())
        // Snakeを生成する。setupで生成されたmaterialを使う必要があるのでStartUpの直後に実行される別のStageを追加する
        .add_startup_stage("game_setup", SystemStage::single(snake::spawn_snake.system()))
        .add_system(
            snake::snake_movement_input
                .system()
                .label(snake::SnakeMovement::Input)
                .before(snake::SnakeMovement::Movement)
        )
        // 複数のSystemにlabelなどを付けたい場合はadd_system_setを使う
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.50))
                .with_system(
                    snake::snake_movement
                        .system()
                        .label(snake::SnakeMovement::Movement)
                )
                .with_system(
                    snake::snake_eating
                        .system()
                        .label(snake::SnakeMovement::Eating)
                        .after(snake::SnakeMovement::Movement)
                )
                .with_system(
                    snake::snake_growth
                        .system()
                        .label(snake::SnakeMovement::Growth)
                        .after(snake::SnakeMovement::Eating),
                )
        )
        // PostUpdateで実行したいので、add_system_setではなく、add_system_set_to_stageを使う
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation.system())
                .with_system(size_scaling.system()),
        )
        // 新しいStageを作って、そこで実行する
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food::food_spawner.system())
        )
        .add_system(game_over.system().after(snake::SnakeMovement::Movement))
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // カメラを生成する
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(common::Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        segment_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    });
    // 背景の色を黒くする
    commands.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)));
    // Snakeの体をResource化
    commands.insert_resource(snake::SnakeSegments::default());
    commands.insert_resource(snake::LastTailPosition::default());
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&common::Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / common::ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / common::ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&common::Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game.clone() * bound_window.clone() - (bound_window.clone() / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, common::ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, common::ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<common::GameOverEvent>,
    materials: Res<common::Materials>,
    segments_res: ResMut<snake::SnakeSegments>,
    food: Query<Entity, With<food::Food>>,
    segments: Query<Entity, With<snake::SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        snake::spawn_snake(commands, materials, segments_res);
    }
}