use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct BlockSize {
    w: f32,
    h: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Ball;

const SPRITE_SIZE: BlockSize = BlockSize{w:100., h:25.};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_startup_system(add_player)
    .add_startup_system(spawn_bricks)

    .add_system(move_player)
    .run();
}

fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {   
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() /2., 0.),
            ..default()
        }
    );   
}

fn add_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    
    commands.spawn(
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(SPRITE_SIZE.w, SPRITE_SIZE.h)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(window.width() / 2., SPRITE_SIZE.h, 0.)),
                ..default()
            },
            Player {}
        )
    );
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut direction = 0.0;
    
        let window = window_query.get_single().unwrap();
    
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction -= 1.0;
        }
    
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += 1.0;
        }
    
        player_transform.translation.x += direction * 500. * time.delta_seconds();

        let mut translation = player_transform.translation;
        
        let half_player_size = SPRITE_SIZE.w / 2.;

        let x_min = 0. + half_player_size;
        let x_max = window.width() - half_player_size;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        player_transform.translation = translation
    }
}

fn spawn_bricks(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let brick_size: BlockSize = BlockSize{ w: window.width() / 10., h: window.height() / 30. };

    for v_index in 1..4 {
        for h_index in 1..8 {
            commands.spawn(
                (
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::new(brick_size.w, brick_size.h)),
                            ..default()
                        },
                        transform: Transform::from_xyz(h_index as f32 * 150.,  window.height() - brick_size.h - (v_index as f32 * 80.), 0.),
                        ..default()
                    },
                    Brick{},
                )
            );
        }
    }
}

