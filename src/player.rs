use bevy::prelude::*;
use crate::components::*;

// Left paddle attributes
const LPADDLE_START_POS: Vec3 = Vec3::new(-600.0, 0.0, 0.0);
const LPADDLE_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const LPADDLE_ID: i8 = 0;
// Key Binds
const LPADDLE_UP_KEY: KeyCode = KeyCode::W;
const LPADDLE_DOWN_KEY: KeyCode = KeyCode::S;

//Right paddle attributes
const RPADDLE_START_POS: Vec3 = Vec3::new(600.0, 0.0, 0.0);
const RPADDLE_COLOR: Color = Color::rgb(0.0, 0.7, 1.0);
const RPADDLE_ID: i8 = 1;
// Key Binds
const RPADDLE_UP_KEY: KeyCode = KeyCode::Up;
const RPADDLE_DOWN_KEY: KeyCode = KeyCode::Down;

// All paddle attributes
const PADDLE_SIZES: Vec2 = Vec2::new(30.0, 75.0);
const PADDLE_SPEED: f32 = 100.0;

// Player Plugin manages the player input and player paddles
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(FixedUpdate, input);
    }
}

// init function spawns both of the paddles with a Sprite Bundle & Paddle Component to identify them
fn init(mut commands: Commands) {
    // Spawns Left Side Paddle
    commands.spawn((SpriteBundle {
        sprite: Sprite { 
            color: LPADDLE_COLOR,
            custom_size: Some(PADDLE_SIZES),
            ..default()
        },
        transform: Transform::from_translation(LPADDLE_START_POS),
        ..default()
    }, 
    Paddle { id: LPADDLE_ID, speed: PADDLE_SPEED }));

    // Spawns Right Side Paddle
    commands.spawn((SpriteBundle {
        sprite: Sprite { 
            color: RPADDLE_COLOR,
            custom_size: Some(PADDLE_SIZES),
            ..default()
        },
        transform: Transform::from_translation(RPADDLE_START_POS),
        ..default()
    },
    Paddle { id: RPADDLE_ID, speed: PADDLE_SPEED }));

}

// Handles input: Arrow keys for blue/right paddle & WASD for left/red paddle
fn input(input: Res<Input<KeyCode>>, time: Res<Time>, mut paddle_pos: Query<(&mut Transform, &Paddle)>) {
    for (mut position, paddle) in &mut paddle_pos {

        if input.pressed(LPADDLE_UP_KEY) && paddle.id == LPADDLE_ID {
            position.translation.y += paddle.speed * time.delta_seconds(); 
        }

        if input.pressed(LPADDLE_DOWN_KEY) && paddle.id == LPADDLE_ID {
            position.translation.y -= paddle.speed * time.delta_seconds(); 
        }

        if input.pressed(RPADDLE_UP_KEY) && paddle.id == RPADDLE_ID {
            position.translation.y += paddle.speed * time.delta_seconds(); 
        }

        if input.pressed(RPADDLE_DOWN_KEY) && paddle.id == RPADDLE_ID {
            position.translation.y -= paddle.speed * time.delta_seconds(); 
        }
    }
}

