use bevy::{prelude::*, window::PrimaryWindow};

struct BlockSize {
    w: f32,
    h: f32,
}

#[derive(Resource)]
pub struct WinSize {
    w: f32,
    h: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ball;

impl Player {}

const SPRITE_SIZE: BlockSize = BlockSize{w:100., h:25.};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Tilebreaker".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_startup_system(setup)
    .add_system(add_player_get_window)
    .add_system(move_player)
    .run();
}

fn setup(
    mut commands: Commands,
) {   
    commands.spawn(Camera2dBundle::default());   
}

fn add_player_get_window(mut commands: Commands, window_query: Query<&Window>) {
    let Some(window) = window_query.get_single().ok() else { return };
    let (win_w, win_h) = (window.width(), window.height());

	commands.insert_resource(WinSize { w: win_w, h: win_h });
    
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(SPRITE_SIZE.w, SPRITE_SIZE.h)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., -win_h / 2. + SPRITE_SIZE.h, 0.)),
        ..default()
    });
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>, window_query: Query<&Window>) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    let Some(window) = window_query.get_single().ok() else { return };
    let win_w = window.width();

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction += 1.0;
    }

    let new_paddle_position = player_transform.translation.x + direction * 500.;

    let left_bound = win_w;
    let right_bound = -win_w;

    player_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}