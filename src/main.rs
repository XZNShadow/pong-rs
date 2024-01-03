mod player;
mod components;

use bevy::prelude::*;
use player::*;

fn main() {
    App::new()
    .add_systems(Startup, startup)
    .add_plugins(DefaultPlugins)
    .add_plugins(PlayerPlugin)
    .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
