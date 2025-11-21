use crate::constants::IS_DEBUG;
use crate::debug::{DebugRenderer, render_grid};
use crate::gamestate::GameState;
use crate::math::convert_world_pos_to_camera_pos;
use macroquad::prelude::*;

pub fn render(game_state: &GameState, camera: &Rect, debug_renderer: &mut DebugRenderer) {
    clear_background(BLACK);
    // Kept separate from debug info since it should be under everything else.
    if IS_DEBUG {
        render_grid(camera);
    }

    render_player(&game_state.player, camera);
    render_ui();

    if IS_DEBUG {
        render_debug_info(game_state, camera, debug_renderer);
    }
}

fn render_player(player: &crate::player::Player, camera: &Rect) {
    for i in 0..player.points.len() {
        let current = player.points[i];
        let next = player.points[(i + 1) % player.points.len()]; // Wrap around to first point

        let curr_vertex_camera_pos = convert_world_pos_to_camera_pos(&current, camera);
        let next_vertex_camera_pos = convert_world_pos_to_camera_pos(&next, camera);

        draw_line(
            curr_vertex_camera_pos.x,
            curr_vertex_camera_pos.y,
            next_vertex_camera_pos.x,
            next_vertex_camera_pos.y,
            2.0,
            YELLOW,
        );
        draw_circle(curr_vertex_camera_pos.x, curr_vertex_camera_pos.y, 3.0, RED);
    }

    let player_center_camera_pos = convert_world_pos_to_camera_pos(&player.center, camera);
    draw_circle(
        player_center_camera_pos.x,
        player_center_camera_pos.y,
        3.0,
        RED,
    );

    let player_normal_world_pos = player.get_player_normal_vector_for_debug(50.0);
    let player_normal = convert_world_pos_to_camera_pos(&player_normal_world_pos, camera);

    draw_line(
        player_center_camera_pos.x,
        player_center_camera_pos.y,
        player_normal.x,
        player_normal.y,
        3.0,
        GREEN,
    );
    draw_circle(player_normal.x, player_normal.y, 5.0, GREEN);
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

// TODO: move this into debug module, and then call it from main.
// Would that makes sense? Just wnat to be able to access delta time, etc
fn render_debug_info(game_state: &GameState, camera: &Rect, debug_renderer: &mut DebugRenderer) {
    debug_renderer.reset();

    debug_renderer.add_text(&format!(
        "Player Center: ({:.1}, {:.1})",
        game_state.player.center.x, game_state.player.center.y
    ));

    debug_renderer.add_text(&format!("Camera: ({:.1}, {:.1})", camera.x, camera.y));

    for (i, point) in game_state.player.points.iter().enumerate() {
        debug_renderer.add_text(&format!("Point {}: ({:.1}, {:.1})", i, point.x, point.y));
    }
}
