mod player;
mod components;
mod world;

use bevy::prelude::*;
use player::*;
use world::*;

fn main() {
    App::new()
    .add_systems(Startup, startup)
    .add_plugins(DefaultPlugins)
    .add_plugins(PlayerPlugin)
    .add_plugins(WorldPlugin)
    .run();
}

// Enables 2D Camera
fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
