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

    /// Soely for debugging.
    pub curr_rotation: f32,

    /// Normal vector pointed in the direction the player is facing.
    pub forward_normal: Point,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        let center = Point::new(x, y);
        let points = Self::create_points_from_center(&center);
        let forward_normal = Point::new(0.0, -1.0);

        Player {
            points,
            center,
            rotation_deg: 0.0,
            curr_rotation: 0.0,
            forward_normal,
        }
    }

    fn create_points_from_center(center: &Point) -> [Point; 4] {
        let half_width = 30.0;
        let half_height = 50.0;

        [
            Point::new(center.x - half_width, center.y - half_height), // top-left
            Point::new(center.x + half_width, center.y - half_height), // top-right
            Point::new(center.x + half_width, center.y + half_height), // bottom-right
            Point::new(center.x - half_width, center.y + half_height), // bottom-left
        ]
    }

    pub fn rotate(&mut self, rotation_degrees: f32) {
        self.curr_rotation = rotation_degrees;
        self.rotation_deg += rotation_degrees;

        // Update forward normal using total rotation
        let total_angle_radians = self.rotation_deg * PI / 180.0;
        self.forward_normal.x = total_angle_radians.sin();
        self.forward_normal.y = -total_angle_radians.cos();

        // Rotate points by only the incremental rotation
        let incremental_angle_radians = rotation_degrees * PI / 180.0;
        for vertex in self.points.iter_mut() {
            rotate_around_point(vertex, &self.center, incremental_angle_radians);
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

    // Get the forward direction as a unit vector for debugging
    pub fn get_forward_vector(&self) -> Point {
        let theta = self.rotation_deg * PI / 180.0;
        Point::new(theta.sin(), -theta.cos())
    }

    // Get a point some distance ahead for drawing debug vector
    pub fn get_forward_point(&self, distance: f32) -> Point {
        let forward = self.get_forward_vector();
        Point::new(
            self.center.x + forward.x * distance,
            self.center.y + forward.y * distance,
        )
    }

    pub fn drive(&mut self, distance: f32) {
        //let forward = self.get_forward_vector();
        let dx = self.forward_normal.x * distance;
        let dy = self.forward_normal.y * distance;
        self.translate(dx, dy);
    }

    /*
    pub fn move_backward(&mut self, distance: f32) {
        self.move_forward(distance);
    }
    */

    pub fn reposition(&mut self, x: f32, y: f32) {
        self.center.x = x;
        self.center.y = y;

        // Recalculate all points based on the new center
        self.points = Self::create_points_from_center(&self.center);
        self.forward_normal = Point::new(0.0, -1.0);
    }
}

// TODO: move this into an appropriate module.
fn rotate_around_point(vertex: &mut Point, center_point: &Point, angle_radians: f32) {
    let x_origin: f32 = vertex.x - center_point.x;
    let y_origin: f32 = vertex.y - center_point.y;

    let x_rotated: f32 = x_origin * angle_radians.cos() - y_origin * angle_radians.sin();
    let y_rotated: f32 = x_origin * angle_radians.sin() + y_origin * angle_radians.cos();

    vertex.x = x_rotated + center_point.x;
    vertex.y = y_rotated + center_point.y;
}
