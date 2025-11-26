use crate::constants::{
    CAMERA_SPEED, CAR_DEFAULT_DRAG, CRAZY_DASH_LENGTH, CRAZY_DASH_MAX_VELOCITY,
    CRAZY_DASH_VELOCITY, GAS_VELOCITY, IS_DEBUG, PLAYER_MAX_REVERSE_VELOCITY, PLAYER_MAX_VELOCITY,
    PLAYER_ROTATION_SPEED, REVERSE_VELOCITY,
};
use crate::debug::{DebugRenderer, render_grid};
use crate::gamestate::GameState;
use crate::math::convert_world_pos_to_camera_pos;
use crate::player::ShiftMode;
use macroquad::prelude::*;

pub fn render(game_state: &GameState, camera: &Rect, debug_renderer: &mut DebugRenderer) {
    clear_background(BLACK);

    // Kept separate from debug info since it should be under everything else.
    if IS_DEBUG {
        render_grid(camera);
    }

    render_player(&game_state.player, camera);
    render_ui(game_state);

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

        let mut outline_color = YELLOW;
        if player.is_crazy_dashing {
            outline_color = BLUE;
        }

        draw_line(
            curr_vertex_camera_pos.x,
            curr_vertex_camera_pos.y,
            next_vertex_camera_pos.x,
            next_vertex_camera_pos.y,
            2.0,
            outline_color,
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

fn render_ui(game_state: &GameState) {
    draw_text("Hello, taxi!", 20.0, 20.0, 30.0, WHITE);
    draw_text(
        "Arrow keys to move, click to teleport",
        20.0,
        50.0,
        20.0,
        WHITE,
    );

    render_gear_indicator(game_state);
}

fn render_gear_indicator(game_state: &GameState) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let indicator_size = 60.0;
    let margin = 20.0;
    let base_x = screen_width - indicator_size - margin;
    let base_y = screen_height - (indicator_size * 2.0) - margin;

    let shift_mode = game_state.player.shift_mode;

    let drive_color = match shift_mode {
        ShiftMode::DRIVE => Color::new(0.0, 0.8, 0.0, 1.0), // Bright green when active
        ShiftMode::REVERSE => Color::new(0.0, 0.4, 0.0, 1.0), // Dark green when inactive
    };

    let reverse_color = match shift_mode {
        ShiftMode::REVERSE => Color::new(1.0, 0.0, 0.0, 1.0), // Bright red when active
        ShiftMode::DRIVE => Color::new(0.4, 0.0, 0.0, 1.0),   // Dark red when inactive
    };

    draw_rectangle(base_x, base_y, indicator_size, indicator_size, drive_color);
    draw_rectangle_lines(base_x, base_y, indicator_size, indicator_size, 2.0, WHITE);
    draw_text("D", base_x + 18.0, base_y + 35.0, 40.0, WHITE);

    let reverse_y = base_y + indicator_size + 5.0;
    draw_rectangle(
        base_x,
        reverse_y,
        indicator_size,
        indicator_size,
        reverse_color,
    );
    draw_rectangle_lines(
        base_x,
        reverse_y,
        indicator_size,
        indicator_size,
        2.0,
        WHITE,
    );
    draw_text("R", base_x + 18.0, reverse_y + 35.0, 40.0, WHITE);
}

// TODO: move this into debug module, and then call it from main.
// Would that makes sense? Just wnat to be able to access delta time, etc
fn render_debug_info(game_state: &GameState, camera: &Rect, debug_renderer: &mut DebugRenderer) {
    debug_renderer.reset();

    // Add constants to top-right corner
    debug_renderer.add_constant(&format!("GAS_VELOCITY: {}", GAS_VELOCITY));
    debug_renderer.add_constant(&format!("CRAZY_DASH_VELOCITY: {}", CRAZY_DASH_VELOCITY));
    debug_renderer.add_constant(&format!("REVERSE_VELOCITY: {}", REVERSE_VELOCITY));
    debug_renderer.add_constant(&format!("DASH_LENGTH: {}", CRAZY_DASH_LENGTH));
    debug_renderer.add_constant(&format!("DASH_MAX_VELOCITY: {}", CRAZY_DASH_MAX_VELOCITY));
    debug_renderer.add_constant(&format!("MAX_VELOCITY: {}", PLAYER_MAX_VELOCITY));
    debug_renderer.add_constant(&format!(
        "MAX_REV_VELOCITY: {}",
        PLAYER_MAX_REVERSE_VELOCITY
    ));
    debug_renderer.add_constant(&format!("ROTATION_SPEED: {}", PLAYER_ROTATION_SPEED));
    debug_renderer.add_constant(&format!("CAR_DEFAULT_DRAG: {}", CAR_DEFAULT_DRAG));
    debug_renderer.add_constant(&format!("CAMERA_SPEED: {}", CAMERA_SPEED));

    debug_renderer.add_text(&format!(
        "Player Center: ({:.1}, {:.1})",
        game_state.player.center.x, game_state.player.center.y
    ));

    //debug_renderer.add_text(&format!("Camera: ({:.1}, {:.1})", camera.x, camera.y));

    //for (i, point) in game_state.player.points.iter().enumerate() {
    //debug_renderer.add_text(&format!("Point {}: ({:.1}, {:.1})", i, point.x, point.y));
    //}

    debug_renderer.add_text(&format!(
        "player velocity: ({:.2}, {:.2})",
        game_state.player.velocity.x, game_state.player.velocity.y
    ));

    debug_renderer.add_text(&format!(
        "ticks since switching to drive: {:.2}",
        game_state.player.ticks_since_switching_into_drive
    ));

    debug_renderer.add_text(&format!(
        "ticks since activating gas: {:.2}",
        game_state.player.ticks_since_gas_was_activated
    ));

    debug_renderer.add_text(&format!(
        "crazy dash end time: {:.2}",
        game_state.player.ticks_to_curr_crazy_dash_end
    ));

    debug_renderer.add_text(&format!(
        "time between drive and gas: {:.2}",
        game_state.player.time_between_drive_and_gas
    ));

    debug_renderer.add_text(&format!(
        "are we crazy dashing?: {}",
        game_state.player.is_crazy_dashing
    ));

    debug_renderer.add_text(&format!("drag: {:.2}", game_state.player.drag));

    debug_renderer.add_text(&format!("time: {:.2}", get_time()));

    // Crazy dash visual indicator - flashing blue square
    if game_state.player.is_crazy_dashing && debug_renderer.debug_state.show_crazy_dash_indicator {
        //let flash_intensity = ((get_time() * 10.0).sin() * 0.5 + 0.5) as f32; // Flash between 0 and 1
        let blue_color = Color::new(0.0, 0.5, 1.0, 1.0);

        let indicator_size = 30.0;
        let margin = 20.0;
        draw_rectangle(
            margin,
            screen_height() - indicator_size - margin,
            indicator_size,
            indicator_size,
            blue_color,
        );
        draw_rectangle_lines(
            margin,
            screen_height() - indicator_size - margin,
            indicator_size,
            indicator_size,
            2.0,
            WHITE,
        );
    }
}
