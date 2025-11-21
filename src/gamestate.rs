use crate::constants::PLAYER_ROTATION_SPEED;
use crate::input::InputFrame;
use crate::player::*;

pub struct GameState {
    pub player: Player,
}

impl GameState {
    pub fn new(player: Player) -> Self {
        Self { player }
    }

    pub fn update(&mut self, input_frame: &InputFrame, delta_time: f32) {
        // Update player based on input
        for player_action in input_frame.player_actions.iter() {
            match player_action {
                PlayerAction::DriveForward | PlayerAction::DriveBackward => {
                    self.player.drive(&player_action, delta_time);
                }
                PlayerAction::TurnLeft => self.player.rotate(PlayerAction::TurnLeft, delta_time),
                PlayerAction::TurnRight => self.player.rotate(PlayerAction::TurnRight, delta_time),
                PlayerAction::Reposition(x, y) => self.player.reposition(*x, *y),
            }
        }
    }
}
