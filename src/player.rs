use crate::constants::*;
use crate::math::Point;
use crate::math::rotate_around_point;
use std::f32::consts::PI;

/// Aka the taxi
pub struct Player {
    /// This is just a rectangle, but its treated as a polygon.
    /// This'll make collision detection easier later.
    pub points: [Point; 8],

    /// Center position of the Player (which is a rect)
    pub center: Point,

    /// In radians.
    rotation: f32,

    /// Normal vector pointed in the direction the player is facing.
    pub forward_normal: Point,

    pub velocity: Point,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        let center = Point::new(x, y);
        let points = create_player_vertices(&center);
        let forward_normal = Point::new(0.0, -1.0);
        let velocity = Point::new(0.0, 0.0);

        Player {
            points,
            center,
            forward_normal,
            rotation: 0.0,
            velocity,
        }
    }

    pub fn rotate(&mut self, player_action: PlayerAction, delta_time: f32) {
        let mut rotation_degrees = 0.0;
        match player_action {
            PlayerAction::TurnLeft => rotation_degrees = -PLAYER_ROTATION_SPEED * delta_time,
            PlayerAction::TurnRight => rotation_degrees = PLAYER_ROTATION_SPEED * delta_time,
            _ => (), // shouldn't happen
        }
        let rotation_radians = rotation_degrees * PI / 180.0;
        self.rotation += rotation_radians;

        // Update forward normal using total rotation, and update the vertices
        // with the rotation of this frame only.
        self.forward_normal.x = self.rotation.sin();
        self.forward_normal.y = -(self.rotation.cos());
        for vertex in self.points.iter_mut() {
            rotate_around_point(vertex, &self.center, rotation_radians);
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

    pub fn step_on_gas(&mut self, delta_time: f32) {
        //self.velocity.y += 10.0;
        //self.velocity.x += 10.0;
        if self.velocity.y < PLAYER_MAX_VELOCITY {
            self.velocity.y += GAS_VELOCITY;
        }
        if self.velocity.x < PLAYER_MAX_VELOCITY {
            self.velocity.x += GAS_VELOCITY;
        }
    }

    pub fn step_on_brake(&mut self, delta_time: f32) {
        //self.velocity.y += 10.0;
        //self.velocity.x += 10.0;
        if self.velocity.y > PLAYER_MAX_REVERSE_VELOCITY {
            self.velocity.y -= REVERSE_VELOCITY;
        }
        if self.velocity.x > PLAYER_MAX_REVERSE_VELOCITY {
            self.velocity.x -= REVERSE_VELOCITY;
        }
    }

    pub fn simulate(&mut self, delta_time: f32) {
        let drag = 700.0;

        // drag; slow car down when not accelerating.
        if self.velocity.y > 0.0 {
            self.velocity.y -= drag * delta_time;
        } else if self.velocity.y < 0.0 {
            self.velocity.y += drag * delta_time;
        }

        if self.velocity.x > 0.0 {
            self.velocity.x -= drag * delta_time;
        } else if self.velocity.x < 0.0 {
            self.velocity.x += drag * delta_time;
        }

        //if self.velocity.y.abs() < 25.0 {
        //self.velocity.y = 0.0;
        //}
        //if self.velocity.x.abs() < 25.0 {
        //self.velocity.x = 0.0;
        //}

        let dx = self.forward_normal.x * self.velocity.x;
        let dy = self.forward_normal.y * self.velocity.y;

        let x_vel = dx * delta_time;
        let y_vel = dy * delta_time;

        self.center.x += x_vel;
        self.center.y += y_vel;
        for point in &mut self.points {
            point.x += x_vel;
            point.y += y_vel;
        }
    }

    pub fn reposition(&mut self, x: f32, y: f32) {
        self.center.x = x;
        self.center.y = y;

        self.points = create_player_vertices(&self.center);
        self.forward_normal = Point::new(0.0, -1.0);
        self.rotation = 0.0;
    }

    // Get a point some distance ahead for drawing debug vector
    pub fn get_player_normal_vector_for_debug(&self, distance: f32) -> Point {
        let forward_vec = &self.forward_normal;
        Point::new(
            self.center.x + forward_vec.x * distance,
            self.center.y + forward_vec.y * distance,
        )
    }
}

fn create_player_vertices(center: &Point) -> [Point; 8] {
    use crate::constants::PLAYER_SHAPE;

    // Use configuration from constants module for easy editing
    let config = &PLAYER_SHAPE;

    let vertices = [
        // Top-left corner
        (center.x - config.half_width, center.y - config.half_height),
        // Inner top-left
        (
            center.x - config.half_width + config.inner_space_w,
            center.y - config.half_height - config.inner_space_h,
        ),
        // Inner top-right
        (
            center.x + config.half_width - config.inner_space_w,
            center.y - config.half_height - config.inner_space_h,
        ),
        // Top-right corner
        (center.x + config.half_width, center.y - config.half_height),
        // Bottom-right corner
        (center.x + config.half_width, center.y + config.half_height),
        // Inner bottom-right
        (
            center.x + config.half_width - config.inner_space_w,
            center.y + config.half_height + config.inner_space_h,
        ),
        // Inner bottom-left
        (
            center.x - config.half_width + config.inner_space_w,
            center.y + config.half_height + config.inner_space_h,
        ),
        // Bottom-left corner
        (center.x - config.half_width, center.y + config.half_height),
    ];

    // Convert tuples to Points
    vertices.map(|(x, y)| Point::new(x, y))
}

#[derive(Debug, Clone)]
pub enum PlayerAction {
    DriveForward,
    DriveBackward,
    TurnLeft,
    TurnRight,
    Reposition(f32, f32),
}
