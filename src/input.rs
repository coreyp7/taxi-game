use crate::player::PlayerAction;
use crate::debug::DebugRenderer;
use macroquad::input::is_key_pressed;
use macroquad::input::*;

#[derive(Debug, Clone)]
pub enum DebugAction {
    ToggleText,
    ToggleConstants,
    ToggleGrid,
    ToggleCrazyDashIndicator,
}

//use macroquad:#[derive(Debug, Clone)]
pub struct InputFrame {
    pub player_actions: Vec<PlayerAction>,
    pub debug_actions: Vec<DebugAction>,
}

impl InputFrame {
    pub fn new() -> Self {
        Self {
            player_actions: Vec::new(),
            debug_actions: Vec::new(),
        }
    }
}

// NOTE: this can be improved but leaving as is right now.
// Need to improve the logic to be smart about what actions its adding to the
// input frame (can't press left & right, etc.)
pub fn process_inputs(input_frame: &mut InputFrame) {
    input_frame.player_actions.clear();
    input_frame.debug_actions.clear();

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

    // Specifically for crazy dashing
    if is_key_pressed(KeyCode::Space) {
        input_frame.player_actions.push(PlayerAction::GasActivated);
    }

    // convenient toggles for debug info
    let debug_key_mappings = [
        (KeyCode::Key1, DebugAction::ToggleText),
        (KeyCode::Key2, DebugAction::ToggleConstants),
        (KeyCode::Key3, DebugAction::ToggleCrazyDashIndicator),
        (KeyCode::Key4, DebugAction::ToggleGrid),
    ];

    for (key, action) in debug_key_mappings {
        if is_key_pressed(key) {
            input_frame.debug_actions.push(action);
        }
    }
}

pub fn process_debug_inputs(input_frame: &InputFrame, debug_renderer: &mut DebugRenderer) {
    // Process debug toggles
    for debug_action in input_frame.debug_actions.iter() {
        match debug_action {
            DebugAction::ToggleText => debug_renderer.toggle_text(),
            DebugAction::ToggleConstants => debug_renderer.toggle_constants(),
            DebugAction::ToggleGrid => debug_renderer.toggle_grid(),
            DebugAction::ToggleCrazyDashIndicator => {
                debug_renderer.toggle_crazy_dash_indicator()
            }
        }
    }
}
