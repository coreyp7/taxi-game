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

    pub fn update(&mut self, input_frame: &InputFrame) {
        // Update player based on input
        for player_action in input_frame.player_actions.iter() {
            match player_action {
                PlayerAction::DriveForward | PlayerAction::DriveBackward => {
                    self.player.drive(&player_action);
                }
                PlayerAction::TurnLeft => self.player.rotate(-PLAYER_ROTATION_SPEED),
                PlayerAction::TurnRight => self.player.rotate(PLAYER_ROTATION_SPEED),
                PlayerAction::Reposition(x, y) => self.player.reposition(*x, *y),
            }
        }
    }
}
