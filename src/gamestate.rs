use crate::constants::CAMERA_SPEED;
use crate::input::InputFrame;
use crate::player::Player;
use crate::player::PlayerAction;
use crate::wall::Wall;
use macroquad::math::Rect;
use macroquad::time::get_time;

pub struct GameState {
    pub player: Player,
    pub wall: Wall // FIXME: temp lazy impl
}

impl GameState {
    pub fn new(player: Player) -> Self {
        // for testing
        let wall: Wall = Wall::new(player.center.x, player.center.y + 300.0);

        Self { player, wall }
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
    for player_action in input_frame.player_actions.iter() {
        match player_action {
            PlayerAction::GasHeld => {
                game_state.player.is_gas_held = true;
            }
            PlayerAction::GasActivated => {
                game_state.player.ticks_since_gas_was_activated = get_time();
            }
            PlayerAction::ShiftIntoDrive => {
                game_state.player.shift_into_drive();
            }
            PlayerAction::ShiftIntoReverse => {
                game_state.player.shift_into_reverse();
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
