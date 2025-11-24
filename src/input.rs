use crate::player::PlayerAction;
use macroquad::input::is_key_pressed;
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

// NOTE: this can be improved but leaving as is right now.
// Need to improve the logic to be smart about what actions its adding to the
// input frame (can't press left & right, etc.)
pub fn process_inputs(input_frame: &mut InputFrame) {
    input_frame.player_actions.clear();

    // Define key mappings
    let key_mappings = [
        (KeyCode::Up, PlayerAction::ShiftIntoDrive),
        (KeyCode::Down, PlayerAction::ShiftIntoReverse),
        (KeyCode::Space, PlayerAction::GasHeld),
        (KeyCode::Left, PlayerAction::TurnLeft),
        (KeyCode::Right, PlayerAction::TurnRight),
    ];

    // Process keyboard inputs
    for (key, action) in key_mappings {
        if is_key_down(key) {
            input_frame.player_actions.push(action);
        }
    }

    // For crazy dashing
    if is_key_pressed(KeyCode::Space) {
        input_frame.player_actions.push(PlayerAction::GasActivated);
    }

    // NOTE: leaving commented out bc its broken.
    // Need to update it to relocate relative to camera pos.
    //if is_mouse_button_pressed(MouseButton::Left) {
    //let (mouse_x, mouse_y) = mouse_position();
    //input_frame
    //.player_actions
    //.push(PlayerAction::Reposition(mouse_x, mouse_y));
    //}
}
