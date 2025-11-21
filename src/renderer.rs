use crate::debug::DebugRenderer;
use crate::gamestate::GameState;
use macroquad::prelude::*;

const IS_DEBUG: bool = true;

pub fn render(game_state: &GameState, debug_renderer: &mut DebugRenderer) {
    clear_background(BLACK);

    render_player(&game_state.player);
    render_ui();

    if IS_DEBUG {
        render_debug_info(game_state, debug_renderer);
    }
}

fn render_player(player: &crate::player::Player) {
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

fn render_ui() {
    draw_text("Hello, taxi!", 20.0, 20.0, 30.0, WHITE);
    draw_text(
        "Arrow keys to move, click to teleport",
        20.0,
        50.0,
        20.0,
        WHITE,
    );
}

fn render_debug_info(game_state: &GameState, debug_renderer: &mut DebugRenderer) {
    debug_renderer.reset();

    debug_renderer.add_text(&format!(
        "Center: ({:.1}, {:.1})",
        game_state.player.center.x, game_state.player.center.y
    ));

    for (i, point) in game_state.player.points.iter().enumerate() {
        debug_renderer.add_text(&format!("Point {}: ({:.1}, {:.1})", i, point.x, point.y));
    }
}
