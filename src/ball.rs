use bevy::{prelude::*, sprite::*};

use crate::components::Ball;

// Ball Plugin manages the ball spawning and movement
pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(FixedUpdate, ball_movement);
    }
}

// Spawn function spawns in ball with Ball component
fn spawn (mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Ball
    ));
}

fn ball_movement(mut ball: Query<&mut Transform, With<Ball>>) {
    let ball = ball.single_mut();
}