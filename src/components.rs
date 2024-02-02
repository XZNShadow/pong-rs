use bevy::prelude::*;

// Components to Identify the Entities
#[derive(Component)]
pub struct Paddle {
    pub id: i8,
    pub speed: f32
}

#[derive(Component)]
pub struct Ball {
    pub speed_x: f32,
    pub speed_y: f32
}

#[derive(Component)]
pub struct Wall {
    pub id: i8
}

#[derive(Component)]
pub struct Goal;
