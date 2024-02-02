use bevy::{prelude::*, sprite::collide_aabb::collide};
use crate::components::*;

// Wall Color
const WALL_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

// Vertical and Horizontal Wall Sizes
const VSIDE_WALL_SIZE: Vec3 = Vec3 { x: 20.0, y: 750.0, z:0.0 };
const HSIDE_WALL_SIZE: Vec3 = Vec3 { x: 1320.0, y: 20.0, z: 0.0 };

// Wall Transforms
const RIGHT_WALL_POS: Vec3 = Vec3 { x: 650.0, y: 0.0, z: 0.0 };
const LEFT_WALL_POS: Vec3 = Vec3 { x: -650.0, y: 0.0, z: 0.0 };
const TOP_WALL_POS: Vec3 = Vec3 { x: 0.0, y: 370.0, z: 0.0 };
const BOTTOM_WALL_POS: Vec3 = Vec3 { x: 0.0, y: -370.0, z: 0.0 };

// World Plugin spawns in environment entities and manages collision detects
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(FixedUpdate, entity_collision);
    }
}

// Spawns Walls
fn spawn(mut commands: Commands) {
    // Side Walls
    // Right Wall
    commands.spawn((SpriteBundle{
        sprite: Sprite {
            color: WALL_COLOR,
            ..default()
        },
        transform: Transform::from_translation(RIGHT_WALL_POS).with_scale(VSIDE_WALL_SIZE),
        ..default()
    }, 
    Wall { id: 0 }));

    // Left Wall
    commands.spawn((SpriteBundle{
        sprite: Sprite {
            color: WALL_COLOR,
            ..default()
        },
        transform: Transform::from_translation(LEFT_WALL_POS).with_scale(VSIDE_WALL_SIZE),
        ..default()
    },
    Wall { id: 1 }));

    // Top Wall
    commands.spawn((SpriteBundle{
        sprite: Sprite {
            color: WALL_COLOR,
            ..default()
        },
        transform: Transform::from_translation(TOP_WALL_POS).with_scale(HSIDE_WALL_SIZE),
        ..default()
    },
    Wall { id: 2 }));

    // Bottom Wall
    commands.spawn((SpriteBundle{
        sprite: Sprite {
            color: WALL_COLOR,
            ..default()
        },
        transform: Transform::from_translation(BOTTOM_WALL_POS).with_scale(HSIDE_WALL_SIZE),
        ..default()
    },
    Wall { id: 3 }));

}

// Checks if entities collide
fn entity_collision(mut _commands: Commands, time: Res<Time>, mut paddles: Query<(&mut Transform, &Paddle), (With<Paddle>, Without<Wall>)>, mut walls: Query<(&Transform, &Wall), With<Wall>>, mut ball: Query<(&Transform, &mut Ball), (With<Ball>, Without<Wall>, Without<Paddle>)>) {
    //Ball Query
    let (ball_transform, mut ball) = ball.single_mut();

    // Iterates through walls to check for collision
    for (wall_transform, wall) in &mut walls {

        // Checks for Paddle & Wall Collision and stops the paddles from moving past the wall
        for (mut paddle_transform, paddle) in &mut paddles {

            if let Some(_collided) = collide(paddle_transform.translation, paddle_transform.scale.truncate(), wall_transform.translation, wall_transform.scale.truncate()) {
                if wall.id == 2 {
                    paddle_transform.translation.y = paddle_transform.translation.y - paddle.speed * time.delta_seconds();
                } else if wall.id == 3 {
                    paddle_transform.translation.y = paddle_transform.translation.y + paddle.speed * time.delta_seconds();
                }
            }
        }

        // Checks for Ball & Wall Collision and negates the ball speed to change direction 
        if let Some(_collide) = collide(ball_transform.translation, ball_transform.scale.truncate(), wall_transform.translation, wall_transform.scale.truncate()) {
            if wall.id == 0 || wall.id == 1 {
                ball.speed_x = -ball.speed_x;
            } else {
                ball.speed_y = -ball.speed_y;
            }
        }
    }

    // Checks for Paddle & Ball Collision and negates the ball speed to change direction 
    for (paddle_transform, _paddle) in &mut paddles {
        if let Some(_collide) = collide(paddle_transform.translation, paddle_transform.scale.truncate(), ball_transform.translation, ball_transform.scale.truncate()) {
            ball.speed_x = -ball.speed_x;
        }
    }

}