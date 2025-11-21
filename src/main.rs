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
use gamestate::simulate;
use gamestate::*;
use input::*;
use player::Player;
use renderer::render;

#[macroquad::main("MyGame")]
async fn main() {
    let mut input_frame = InputFrame::new();
    // FIXME: have player instantiated in gamestate, shouldn't be here.
    let player = Player::new(screen_width() / 2.0, 120.0);
    let mut game_state = GameState::new(player);
    let mut debug_renderer = DebugRenderer::new();
    let mut camera = Rect::new(0.0, 0.0, screen_width(), screen_height());

    let mut delta_time = 0.0;
    loop {
        delta_time = get_frame_time();
        process_inputs(&mut input_frame);

        simulate(&input_frame, &mut game_state, &mut camera, delta_time);

        render(&game_state, &camera, &mut debug_renderer);

        next_frame().await
    }
}
