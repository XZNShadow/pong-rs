use crate::components::*;
use bevy::{prelude::*, sprite::collide_aabb::collide};

// Wall Color
const WALL_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

// Vertical and Horizontal Wall Sizes
const VSIDE_WALL_SIZE: Vec3 = Vec3 {
    x: 20.0,
    y: 750.0,
    z: 0.0,
};
const HSIDE_WALL_SIZE: Vec3 = Vec3 {
    x: 1320.0,
    y: 20.0,
    z: 0.0,
};

// Wall Transforms
const RIGHT_WALL_POS: Vec3 = Vec3 {
    x: 650.0,
    y: 0.0,
    z: 0.0,
};
const LEFT_WALL_POS: Vec3 = Vec3 {
    x: -650.0,
    y: 0.0,
    z: 0.0,
};
const TOP_WALL_POS: Vec3 = Vec3 {
    x: 0.0,
    y: 370.0,
    z: 0.0,
};
const BOTTOM_WALL_POS: Vec3 = Vec3 {
    x: 0.0,
    y: -370.0,
    z: 0.0,
};

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

    // Left Wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform::from_translation(LEFT_WALL_POS).with_scale(VSIDE_WALL_SIZE),
            ..default()
        },
        Wall { id: 0 },
    ));

    // Right Wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform::from_translation(RIGHT_WALL_POS).with_scale(VSIDE_WALL_SIZE),
            ..default()
        },
        Wall { id: 1 },
    ));

    // Top Wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform::from_translation(TOP_WALL_POS).with_scale(HSIDE_WALL_SIZE),
            ..default()
        },
        Wall { id: 2 },
    ));

    // Bottom Wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform::from_translation(BOTTOM_WALL_POS).with_scale(HSIDE_WALL_SIZE),
            ..default()
        },
        Wall { id: 3 },
    ));
}

// Checks if entities collide
fn entity_collision(
    mut _commands: Commands,
    time: Res<Time>,
    mut paddles: Query<(&mut Transform, &Paddle), (With<Paddle>, Without<Wall>)>,
    mut walls: Query<(&Transform, &Wall), With<Wall>>,
    mut ball: Query<(&mut Transform, &mut Ball), (With<Ball>, Without<Wall>, Without<Paddle>)>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    //Ball Query
    let (mut ball_transform, mut ball) = ball.single_mut();

    // Iterates through walls to check for collision
    for (wall_transform, wall) in &mut walls {
        // Checks for Paddle & Wall Collision and stops the paddles from moving past the wall
        for (mut paddle_transform, paddle) in &mut paddles {
            if let Some(_collided) = collide(
                paddle_transform.translation,
                paddle_transform.scale.truncate(),
                wall_transform.translation,
                wall_transform.scale.truncate(),
            ) {
                if wall.id == 2 {
                    paddle_transform.translation.y =
                        paddle_transform.translation.y - paddle.speed * time.delta_seconds();
                } else if wall.id == 3 {
                    paddle_transform.translation.y =
                        paddle_transform.translation.y + paddle.speed * time.delta_seconds();
                }
            }
        }

        // Checks for Ball & Wall Collision and negates the ball speed to change direction
        if let Some(_collide) = collide(
            ball_transform.translation,
            ball_transform.scale.truncate(),
            wall_transform.translation,
            wall_transform.scale.truncate(),
        ) {
            if wall.id == 0 {
                // If collided with left wall then add point to the right player and reset the ball to the middle
                scoreboard.right_score += 1;
                ball_transform.translation.x = 0.0;
                ball_transform.translation.y = 0.0;
            } else if wall.id == 1 {
                // If collided with right wall then add point to the left player and reset the ball to the middle
                scoreboard.left_score += 1;
                ball_transform.translation.x = 0.0;
                ball_transform.translation.y = 0.0;
            } else {
                // Change the direction of the ball
                ball.speed_y = -ball.speed_y;
            }
        }
    }

    // Checks for Paddle & Ball Collision and negates the ball speed to change direction
    for (paddle_transform, paddle) in &mut paddles {
        if let Some(collide) = collide(
            paddle_transform.translation,
            paddle_transform.scale.truncate(),
            ball_transform.translation,
            ball_transform.scale.truncate(),
        ) {
            match collide {
                bevy::sprite::collide_aabb::Collision::Left => {
                    ball.speed_x = -ball.speed_x;
                }
                bevy::sprite::collide_aabb::Collision::Right => {
                    ball.speed_x = -ball.speed_x;
                }
                bevy::sprite::collide_aabb::Collision::Top => {
                    ball.speed_y = -ball.speed_y;

                    // To prevent the ball from entering the paddle
                    if paddle.direction == PaddleDirection::Up {
                        ball_transform.translation.y += 45.0;
                    }
                }
                bevy::sprite::collide_aabb::Collision::Bottom => {
                    ball.speed_y = -ball.speed_y;

                    // To prevent the ball from entering the paddle
                    if paddle.direction == PaddleDirection::Down {
                        ball_transform.translation.y -= 45.0;
                    }
                }
                bevy::sprite::collide_aabb::Collision::Inside => {
                    // Move the ball outside if it is inside the paddle
                    if paddle.id == 0 {
                        ball_transform.translation.x -= 30.0;
                    } else {
                        ball_transform.translation.x += 30.0;
                    }
                }
            }
        }
    }
}
