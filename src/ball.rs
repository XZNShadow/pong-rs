use bevy::{prelude::*, sprite::*};

use crate::components::Ball;

const BALL_XSPEED:f32 = 100.0;
const BALL_YSPEED:f32 = 100.0;
const BALL_START_POS: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
const BALL_RADIUS: f32 = 10.0;

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
            mesh: meshes.add(shape::Circle::new(1.0).into()).into(),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(BALL_START_POS).with_scale(Vec2::splat(BALL_RADIUS).extend(1.0)),
            ..default()
        },
        Ball { speed_x: BALL_XSPEED, speed_y: BALL_YSPEED}
    ));
}

fn ball_movement(time: Res<Time>, mut ball: Query<(&mut Transform, &Ball), With<Ball>>) {
    // Gets the ball Query
    let (mut transform, ball) = ball.single_mut();

    // Moves the ball based on the speed
    transform.translation.x += ball.speed_x *  time.delta_seconds();
    transform.translation.y += ball.speed_y *  time.delta_seconds();
}