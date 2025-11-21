use crate::input::InputFrame;
use crate::player::*;

pub fn simulate(player: &mut Player, input_frame: &InputFrame) {
    for player_action in input_frame.player_actions.iter() {
        match player_action {
            PlayerAction::DriveForward | PlayerAction::DriveBackward => {
                player.drive(&player_action);
            }
            PlayerAction::TurnLeft => player.rotate(-1.2),
            PlayerAction::TurnRight => player.rotate(1.2),
            PlayerAction::Reposition(x, y) => player.reposition(*x, *y),
        }
    }
}
