use macroquad::color::*;
use macroquad::prelude::*;

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
            font_size: 16.0,
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
