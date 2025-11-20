use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

pub struct Player {
    pub points: [Point; 4],
    pub center: Point,
    pub rotation_deg: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        let center = Point::new(x, y);

        let half_width = 30.0;
        let half_height = 50.0;

        Player {
            points: [
                Point::new(center.x - half_width, center.y - half_height), // top-left
                Point::new(center.x + half_width, center.y - half_height), // top-right
                Point::new(center.x + half_width, center.y + half_height), // bottom-right
                Point::new(center.x - half_width, center.y + half_height), // bottom-left
            ],
            center,
            rotation_deg: 0.0,
        }
    }

    pub fn rotate(&mut self, rotation_degrees: f32) {
        // Add the rotation to current rotation
        self.rotation_deg += rotation_degrees;

        // Convert to radians
        let theta = self.rotation_deg * PI / 180.0;

        for vertex in self.points.iter_mut() {
            let x_origin: f32 = vertex.x - self.center.x;
            let y_origin: f32 = vertex.y - self.center.y;

            let x_rotated: f32 = x_origin * theta.cos() - y_origin * theta.sin();
            let y_rotated: f32 = x_origin * theta.sin() + y_origin * theta.cos();

            vertex.x = x_rotated + self.center.x;
            vertex.y = y_rotated + self.center.y;
        }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.center.x += x;
        self.center.y += y;

        for point in &mut self.points {
            point.x += x;
            point.y += y;
        }
    }
}
