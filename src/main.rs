use macroquad::color::*;
use macroquad::prelude::*;

mod debug;
mod player;
use debug::DebugRenderer;
use player::*;

mod input;
use input::*;
mod gamestate;
mod math;
use gamestate::*;

//const PLAYER_ROTATION: f32 = 2.0;
const IS_DEBUG: bool = true;

#[macroquad::main("MyGame")]
async fn main() {
    let mut player = Player::new(screen_width() / 2.0, 120.0);
    let mut debug_renderer = DebugRenderer::new();
    let mut input_frame = InputFrame::new();

    loop {
        clear_background(BLACK);

        process_inputs(&mut input_frame);

        simulate(&mut player, &input_frame);

        render(&player, &mut debug_renderer);
        next_frame().await
    }
}

fn render(player: &Player, debug_renderer: &mut DebugRenderer) {
    render_player(player);
    if IS_DEBUG {
        render_debug_info(player, debug_renderer);
    }
    draw_text("Hello, taxi!", 20.0, 20.0, 30.0, WHITE);
    draw_text(
        "Arrow keys to move, click to teleport",
        20.0,
        50.0,
        20.0,
        WHITE,
    );
}

fn render_debug_info(player: &Player, debug_renderer: &mut DebugRenderer) {
    debug_renderer.reset();

    debug_renderer.add_text(&format!(
        "Center: ({:.1}, {:.1})",
        player.center.x, player.center.y
    ));
    //debug_renderer.add_text(&format!(
    //"Rotation this frame: {:.1}°",
    //player.curr_rotation
    //));
    //debug_renderer.add_text(&format!("Total Rotation: {:.1}°", player.rotation_deg));

    for (i, point) in player.points.iter().enumerate() {
        debug_renderer.add_text(&format!("Point {}: ({:.1}, {:.1})", i, point.x, point.y));
    }
}

fn render_player(player: &Player) {
    for i in 0..player.points.len() {
        let current = player.points[i];
        let next = player.points[(i + 1) % player.points.len()]; // Wrap around to first point
        draw_line(current.x, current.y, next.x, next.y, 2.0, YELLOW);
        draw_circle(current.x, current.y, 3.0, RED);
    }

    draw_circle(player.center.x, player.center.y, 3.0, RED);

    // Draw forward direction vector
    let forward_point = player.get_forward_point(50.0);
    draw_line(
        player.center.x,
        player.center.y,
        forward_point.x,
        forward_point.y,
        3.0,
        GREEN,
    );
    draw_circle(forward_point.x, forward_point.y, 5.0, GREEN);
}
