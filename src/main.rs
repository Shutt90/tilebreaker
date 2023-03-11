use bevy::prelude::*;

struct Player {
    position: Position
}

struct Position {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn create_player(commands: Commands) {

}