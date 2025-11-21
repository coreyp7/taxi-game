use macroquad::math::Rect;

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

pub fn rotate_around_point(vertex: &mut Point, center_point: &Point, angle_radians: f32) {
    let x_origin: f32 = vertex.x - center_point.x;
    let y_origin: f32 = vertex.y - center_point.y;

    let x_rotated: f32 = x_origin * angle_radians.cos() - y_origin * angle_radians.sin();
    let y_rotated: f32 = x_origin * angle_radians.sin() + y_origin * angle_radians.cos();

    vertex.x = x_rotated + center_point.x;
    vertex.y = y_rotated + center_point.y;
}

pub fn convert_world_pos_to_camera_pos(world_pos: &Point, camera: &Rect) -> Point {
    Point::new(world_pos.x - camera.x, world_pos.y - camera.y)
}
