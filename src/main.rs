mod ball;
mod components;
mod player;
mod ui;
mod world;

use ball::*;
use bevy::prelude::*;
use player::*;
use ui::*;
use world::*;

fn main() {
    App::new()
        .add_systems(Startup, startup)
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(UI)
        .run();
}

// Enables 2D Camera
fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
