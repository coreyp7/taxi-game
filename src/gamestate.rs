use crate::constants::PLAYER_ROTATION_SPEED;
use crate::input::InputFrame;
use crate::player::*;
use macroquad::math::Rect;

pub struct GameState {
    pub player: Player,
}

impl GameState {
    pub fn new(player: Player) -> Self {
        Self { player }
    }
}

pub fn update_game_state(
    input_frame: &InputFrame,
    game_state: &mut GameState,
    camera: &mut Rect,
    delta_time: f32,
) {
    // Update player based on input
    for player_action in input_frame.player_actions.iter() {
        match player_action {
            PlayerAction::DriveForward | PlayerAction::DriveBackward => {
                game_state.player.drive(&player_action, delta_time);
            }
            PlayerAction::TurnLeft => game_state.player.rotate(PlayerAction::TurnLeft, delta_time),
            PlayerAction::TurnRight => game_state
                .player
                .rotate(PlayerAction::TurnRight, delta_time),
            PlayerAction::Reposition(x, y) => game_state.player.reposition(*x, *y),
        }
    }

    // Update camera position.
}
