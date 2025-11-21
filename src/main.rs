use macroquad::prelude::*;

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

    loop {
        process_inputs(&mut input_frame);

        game_state.update(&input_frame);

        render(&game_state, &mut debug_renderer);
        next_frame().await
    }
}
