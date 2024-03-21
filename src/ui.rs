use bevy::prelude::*;

use crate::components::Scoreboard;

// Text font size
const FONT_SIZE: f32 = 20.0;
// Text font padding
const TEXT_PADDING: Val = Val::Px(5.0);

// Text colors
const LTEXT_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const RTEXT_COLOR: Color = Color::rgb(0.0, 0.7, 1.0);

// UI plugin handles the UI creation & updates
pub struct UI;

impl Plugin for UI {
    fn build(&self, app: &mut App) {
        // Adds the scoreboard resource to the game
        app.insert_resource(Scoreboard {
            left_score: 0,
            right_score: 0,
        });
        app.add_systems(Startup, spawn);
        app.add_systems(Update, update);
    }
}

// Creates the UI entites on the screen
fn spawn(mut commands: Commands) {
    // Left player score counter
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: FONT_SIZE,
                    color: LTEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: FONT_SIZE,
                color: LTEXT_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: TEXT_PADDING,
            left: TEXT_PADDING,
            ..default()
        }),
    );

    // Right player score counter
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: FONT_SIZE,
                    color: RTEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: FONT_SIZE,
                color: RTEXT_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: TEXT_PADDING,
            right: TEXT_PADDING,
            ..default()
        }),
    );
}

// Updates the score periodically based on the scoreboard resource
fn update(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut first = true;
    query.for_each_mut(|mut text| {
        if first {
            text.sections[1].value = scoreboard.left_score.to_string();
            first = false;
        } else {
            text.sections[1].value = scoreboard.right_score.to_string();
        }
    });
}
