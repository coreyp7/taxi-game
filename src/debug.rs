use crate::math::{Point, convert_world_pos_to_camera_pos};
use macroquad::color::*;
use macroquad::prelude::*;

/// Just a convenient function/struct I can call when I need to add
/// different debug text to the window easily.
pub struct DebugRenderer {
    current_line: usize,
    font_size: f32,
    line_height: f32,
    start_y: f32,
    x: f32,
    color: Color,
}

impl DebugRenderer {
    pub fn new() -> Self {
        DebugRenderer {
            current_line: 0,
            font_size: 26.0,
            line_height: 20.0,
            start_y: 80.0,
            x: 20.0,
            color: GREEN,
        }
    }

    pub fn reset(&mut self) {
        self.current_line = 0;
    }

    pub fn add_text(&mut self, text: &str) {
        draw_text(
            text,
            self.x,
            self.start_y + self.line_height * self.current_line as f32,
            self.font_size,
            self.color,
        );
        self.current_line += 1;
    }
}

pub fn render_grid(camera: &Rect) {
    let grid_size = 100.0;

    // Get start/end of camera box
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
