
use crate::math::Point;

pub struct Wall {
    pub points: [Point; 6],
    pub center: Point,
    pub rotation: f32,
}

impl Wall {
    pub fn new(x: f32, y: f32) -> Self {
        let center = Point::new(x, y);
        let points = create_wall_vertices(&center);
        let rotation = 0.0;

        Self {
            points,
            center,
            rotation,
        }
    }
}

// currently hexagon for collision
fn create_wall_vertices(center: &Point) -> [Point; 6] {
    let size = 100.0; // Radius of the hexagon
    let angle_step = std::f32::consts::PI / 3.0; // 60 degrees in radians

    [
        Point::new(
            center.x + size * (0.0 * angle_step).cos(),
            center.y + size * (0.0 * angle_step).sin(),
        ),
        Point::new(
            center.x + size * (1.0 * angle_step).cos(),
            center.y + size * (1.0 * angle_step).sin(),
        ),
        Point::new(
            center.x + size * (2.0 * angle_step).cos(),
            center.y + size * (2.0 * angle_step).sin(),
        ),
        Point::new(
            center.x + size * (3.0 * angle_step).cos(),
            center.y + size * (3.0 * angle_step).sin(),
        ),
        Point::new(
            center.x + size * (4.0 * angle_step).cos(),
            center.y + size * (4.0 * angle_step).sin(),
        ),
        Point::new(
            center.x + size * (5.0 * angle_step).cos(),
            center.y + size * (5.0 * angle_step).sin(),
        ),
    ]
}
