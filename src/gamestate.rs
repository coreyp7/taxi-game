use crate::constants::CAMERA_SPEED;
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

pub fn simulate(
    input_frame: &InputFrame,
    game_state: &mut GameState,
    camera: &mut Rect,
    delta_time: f32,
) {
    simulate_player(input_frame, game_state, delta_time);

    update_camera_pos(camera, &game_state.player, delta_time);
}

fn simulate_player(input_frame: &InputFrame, game_state: &mut GameState, delta_time: f32) {
    // Update player based on input
    for player_action in input_frame.player_actions.iter() {
        match player_action {
            PlayerAction::DriveForward | PlayerAction::DriveBackward => {
                game_state.player.apply_force(&player_action, delta_time);
            }
            PlayerAction::TurnLeft => game_state.player.rotate(PlayerAction::TurnLeft, delta_time),
            PlayerAction::TurnRight => game_state
                .player
                .rotate(PlayerAction::TurnRight, delta_time),
            PlayerAction::Reposition(x, y) => game_state.player.reposition(*x, *y),
        }
    }

    game_state.player.simulate(delta_time);
}

fn update_camera_pos(camera: &mut Rect, player: &Player, delta_time: f32) {
    let target_camera_x = player.center.x - camera.w / 2.0;
    let target_camera_y = player.center.y - camera.h / 2.0;

    // Calculate the difference between current and target camera position
    let dx = target_camera_x - camera.x;
    let dy = target_camera_y - camera.y;

    // Move camera towards target position with smooth interpolation
    camera.x += dx * CAMERA_SPEED * delta_time;
    camera.y += dy * CAMERA_SPEED * delta_time;
}
