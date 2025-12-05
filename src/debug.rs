use crate::math::{Point, convert_world_pos_to_camera_pos};
use macroquad::color::*;
use macroquad::prelude::*;

/// this is really extra but it works
#[derive(Clone, Copy)]
pub struct DebugState {
    pub show_text: bool,
    pub show_constants: bool,
    pub show_grid: bool,
    pub show_crazy_dash_indicator: bool,
    pub show_state_history: bool,
}

impl DebugState {
    pub fn new() -> Self {
        Self {
            show_text: true,
            show_constants: true,
            show_grid: true,
            show_crazy_dash_indicator: true,
            show_state_history: true,
        }
    }

    pub fn toggle_text(&mut self) {
        self.show_text = !self.show_text;
    }

    pub fn toggle_constants(&mut self) {
        self.show_constants = !self.show_constants;
    }

    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    pub fn toggle_crazy_dash_indicator(&mut self) {
        self.show_crazy_dash_indicator = !self.show_crazy_dash_indicator;
    }

    pub fn toggle_state_history(&mut self) {
        self.show_state_history = !self.show_state_history;
    }
}

/// Just a convenient function/struct I can call when I need to add
/// different debug text to the window easily.
pub struct DebugRenderer {
    pub debug_state: DebugState,
    // top left display
    current_line: usize,
    font_size: f32,
    line_height: f32,
    start_y: f32,
    x: f32,
    color: Color,
    // top right constant display
    current_constant_line: usize,
    constant_font_size: f32,
    constant_line_height: f32,
    constant_start_y: f32,
    constant_color: Color,
}

impl DebugRenderer {
    pub fn new() -> Self {
        DebugRenderer {
            debug_state: DebugState::new(),
            current_line: 0,
            font_size: 26.0,
            line_height: 20.0,
            start_y: 80.0,
            x: 20.0,
            color: GREEN,
            current_constant_line: 0,
            constant_font_size: 15.0,
            constant_line_height: 13.5,
            constant_start_y: 10.0,
            constant_color: YELLOW,
        }
    }

    pub fn reset(&mut self) {
        self.current_line = 0;
        self.current_constant_line = 0;
    }

    pub fn toggle_text(&mut self) {
        self.debug_state.toggle_text();
    }

    pub fn toggle_constants(&mut self) {
        self.debug_state.toggle_constants();
    }

    pub fn toggle_grid(&mut self) {
        self.debug_state.toggle_grid();
    }

    pub fn toggle_crazy_dash_indicator(&mut self) {
        self.debug_state.toggle_crazy_dash_indicator();
    }

    pub fn add_text(&mut self, text: &str) {
        if !self.debug_state.show_text {
            return;
        }

        draw_text(
            text,
            self.x,
            self.start_y + self.line_height * self.current_line as f32,
            self.font_size,
            self.color,
        );
        self.current_line += 1;
    }

    pub fn add_constant(&mut self, text: &str) {
        if !self.debug_state.show_constants {
            return;
        }

        // Measure the text to get its actual width for proper right alignment
        let text_dimensions = measure_text(text, None, self.constant_font_size as u16, 1.0);
        let text_width = text_dimensions.width;

        // Right margin from the edge of the screen
        let right_margin = 20.0;
        // Calculate x position for right alignment
        let x_pos = screen_width() - text_width - right_margin;

        draw_text(
            text,
            x_pos,
            self.constant_start_y + self.constant_line_height * self.current_constant_line as f32,
            self.constant_font_size,
            self.constant_color,
        );
        self.current_constant_line += 1;
    }

    pub fn add_state_history_text(&mut self, text: &str) {
        if !self.debug_state.show_state_history {
            return;
        }
        // Measure the text to get its actual width for proper right alignment
        let text_dimensions = measure_text(text, None, self.constant_font_size as u16, 1.0);
        let text_width = text_dimensions.width;
        let right_margin = 20.0;
        let x_pos = screen_width() - text_width - right_margin;
        draw_text(
            text,
            x_pos,
            self.constant_start_y + self.constant_line_height * self.current_constant_line as f32,
            self.constant_font_size,
            self.constant_color,
        );
        self.current_constant_line += 1;
    }

    pub fn display_state_history(&mut self, player: &crate::player::Player) {
        if !self.debug_state.show_state_history {
            return;
        }
        self.add_state_history_text("[5] Player State History (latest first):");
        for (i, (state, timestamp)) in player.state_history.iter().enumerate() {
            self.add_state_history_text(&format!("{}: {:?} @ {:.2}s", i + 1, state, timestamp));
        }
    }
}

pub fn render_grid(camera: &Rect) {
    let grid_size = 100.0;

    // Get start/end of camera rect
    let start_x = (camera.x / grid_size).floor() * grid_size;
    let start_y = (camera.y / grid_size).floor() * grid_size;
    let end_x = start_x + camera.w + grid_size;
    let end_y = start_y + camera.h + grid_size;

    // Draw vertical grid lines
    let mut world_x = start_x;
    while world_x <= end_x {
        let line_top = Point::new(world_x, camera.y);
        let line_bottom = Point::new(world_x, camera.y + camera.h);

        let camera_top = convert_world_pos_to_camera_pos(&line_top, camera);
        let camera_bottom = convert_world_pos_to_camera_pos(&line_bottom, camera);

        // Ignore if line isn't in camera view
        if camera_top.x >= 0.0 && camera_top.x <= camera.w {
            draw_line(
                camera_top.x,
                camera_top.y,
                camera_bottom.x,
                camera_bottom.y,
                1.0,
                Color::from_rgba(40, 40, 40, 255),
            );
        }
        world_x += grid_size;
    }

    // Draw horizontal grid lines
    let mut world_y = start_y;
    while world_y <= end_y {
        let line_left = Point::new(camera.x, world_y);
        let line_right = Point::new(camera.x + camera.w, world_y);

        let screen_left = convert_world_pos_to_camera_pos(&line_left, camera);
        let screen_right = convert_world_pos_to_camera_pos(&line_right, camera);

        // Ignore if line isn't in camera view
        if screen_left.y >= 0.0 && screen_left.y <= camera.h {
            draw_line(
                screen_left.x,
                screen_left.y,
                screen_right.x,
                screen_right.y,
                1.0,
                Color::from_rgba(40, 40, 40, 255),
            );
        }
        world_y += grid_size;
    }

    let origin_world = Point::new(0.0, 0.0);
    let origin_camera_pos = convert_world_pos_to_camera_pos(&origin_world, camera);

    // Only draw origin if in camera view
    if origin_camera_pos.x >= -20.0
        && origin_camera_pos.x <= camera.w + 20.0
        && origin_camera_pos.y >= -20.0
        && origin_camera_pos.y <= camera.h + 20.0
    {
        draw_circle(origin_camera_pos.x, origin_camera_pos.y, 10.0, RED);
        draw_text(
            "(0,0)",
            origin_camera_pos.x + 15.0,
            origin_camera_pos.y + 5.0,
            20.0,
            WHITE,
        );
    }
}
