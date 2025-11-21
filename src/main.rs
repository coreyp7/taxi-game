use macroquad::prelude::*;
use macroquad::time::{get_frame_time, get_time};

mod constants;
mod debug;
mod gamestate;
mod input;
mod math;
mod player;
mod renderer;

use debug::DebugRenderer;
use gamestate::*;
use input::*;
use player::Player;
use renderer::render;

#[macroquad::main("MyGame")]
async fn main() {
    let player = Player::new(screen_width() / 2.0, 120.0);
    let mut input_frame = InputFrame::new();
    let mut game_state = GameState::new(player);
    let mut debug_renderer = DebugRenderer::new();

    let mut delta_time = 0.0;
    loop {
        delta_time = get_frame_time();
        process_inputs(&mut input_frame);

        game_state.update(&input_frame, delta_time);

        render(&game_state, &mut debug_renderer);

        next_frame().await
    }
}
