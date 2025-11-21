use crate::player::PlayerAction;
use macroquad::input::*;

//use macroquad:#[derive(Debug, Clone)]
pub struct InputFrame {
    pub player_actions: Vec<PlayerAction>,
}

impl InputFrame {
    pub fn new() -> Self {
        Self {
            player_actions: Vec::new(),
        }
    }
}

pub fn process_inputs(input_frame: &mut InputFrame) {
    input_frame.player_actions.clear();

    if is_key_down(KeyCode::Up) {
        input_frame.player_actions.push(PlayerAction::DriveForward);
    }

    if is_key_down(KeyCode::Down) {
        input_frame.player_actions.push(PlayerAction::DriveBackward);
    }

    if is_key_down(KeyCode::Left) {
        input_frame.player_actions.push(PlayerAction::TurnLeft);
    }

    if is_key_down(KeyCode::Right) {
        input_frame.player_actions.push(PlayerAction::TurnRight);
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        input_frame
            .player_actions
            .push(PlayerAction::Reposition(mouse_x, mouse_y));
    }
}
