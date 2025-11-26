use crate::constants::*;
use crate::math::Point;
use crate::math::rotate_around_point;
use macroquad::time::get_time;
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

    pub shift_mode: ShiftMode,

    pub is_gas_held: bool,

    pub ticks_since_switching_into_drive: f64,

    pub ticks_since_gas_was_activated: f64,

    pub ticks_to_curr_crazy_dash_end: f64,

    pub time_between_drive_and_gas: f64,

    pub is_crazy_dashing: bool,

    pub drag: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShiftMode {
    DRIVE,
    REVERSE,
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
            shift_mode: ShiftMode::DRIVE,
            is_gas_held: false,
            ticks_since_switching_into_drive: 0.0,
            ticks_since_gas_was_activated: 0.0,
            ticks_to_curr_crazy_dash_end: 0.0,
            time_between_drive_and_gas: 0.0,
            is_crazy_dashing: false,
            drag: CAR_DEFAULT_DRAG,
        }
    }

    fn is_player_moving(&self) -> bool {
        let is_x_vel = self.velocity.x.abs() > 0.0;
        let is_y_vel = self.velocity.y.abs() > 0.0;
        is_x_vel && is_y_vel
    }

    pub fn rotate(&mut self, player_action: PlayerAction, delta_time: f32) {
        if !self.is_player_moving() {
            return;
        }

        let mut rotation_degrees = 0.0;

        // Allow sharper turn if we're moving faster.
        // So, multiply the rotation speed with velocity in some way.
        let turn_velocity_modifier = self.velocity.y / 100.0;

        match player_action {
            PlayerAction::TurnLeft => {
                rotation_degrees = -PLAYER_ROTATION_SPEED * turn_velocity_modifier * delta_time
            }
            PlayerAction::TurnRight => {
                rotation_degrees = PLAYER_ROTATION_SPEED * turn_velocity_modifier * delta_time
            }
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

    pub fn apply_gas(&mut self) {
        // If mid crazy dash, apply velocity with no limits.
        if self.is_crazy_dashing && self.velocity.y < CRAZY_DASH_MAX_VELOCITY {
            //self.velocity.x += GAS_VELOCITY;
            //self.velocity.y += GAS_VELOCITY;

            self.velocity.x += CRAZY_DASH_VELOCITY;
            self.velocity.y += CRAZY_DASH_VELOCITY;
            return;
        }
        self.is_crazy_dashing = false;
        match self.shift_mode {
            ShiftMode::DRIVE => {
                if self.velocity.y < PLAYER_MAX_VELOCITY {
                    self.velocity.y += GAS_VELOCITY;
                }
                if self.velocity.x < PLAYER_MAX_VELOCITY {
                    self.velocity.x += GAS_VELOCITY;
                }
            }
            ShiftMode::REVERSE => {
                if self.velocity.y > PLAYER_MAX_REVERSE_VELOCITY {
                    self.velocity.y -= REVERSE_VELOCITY;
                }
                if self.velocity.x > PLAYER_MAX_REVERSE_VELOCITY {
                    self.velocity.x -= REVERSE_VELOCITY;
                }
            }
        }
    }

    pub fn simulate(&mut self, delta_time: f32) {
        self.update_crazy_dash_status();

        //let mut drag = CAR_DEFAULT_DRAG;
        if self.is_crazy_dashing {
            self.drag = CRAZY_DASH_DRAG;
        }

        // apply drag to car when velocity > 0
        if self.velocity.y > 0.0 {
            self.velocity.y -= self.drag * delta_time;
        } else if self.velocity.y < 0.0 {
            self.velocity.y += self.drag * delta_time;
        }

        if self.velocity.x > 0.0 {
            self.velocity.x -= self.drag * delta_time;
        } else if self.velocity.x < 0.0 {
            self.velocity.x += self.drag * delta_time;
        }

        // Begin increasing the drag if less than the default (mid crazy dash).
        // (without this, the car very quickly slows down after a crazy dash)
        if self.drag < CAR_DEFAULT_DRAG {
            let mut new_drag = 1000.0 / self.velocity.y; // TODO: put in function

            if new_drag < 0.0 {
                new_drag = 0.1;
            }

            self.drag += new_drag;
            if self.drag > CAR_DEFAULT_DRAG {
                self.drag = CAR_DEFAULT_DRAG;
            }
        }

        // If velocity is near 0, just set to 0.
        // (Prevents buggy behavior at lower speeds)
        if self.velocity.y.abs() < 2.5 {
            self.velocity.y = 0.0;
        }
        if self.velocity.x.abs() < 2.5 {
            self.velocity.x = 0.0;
        }

        // Apply velocity if gas is held (either drive or reverse)
        if self.is_gas_held {
            self.apply_gas(); // TODO: this could be named better.
        }

        // Apply the velocity to each of the verticies of the car
        // in the direction the car is facing (forward_normal).
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

        // this will be set to true again before simulate is ran in the next frame.
        self.is_gas_held = false;
    }

    pub fn shift_into_drive(&mut self) {
        if self.shift_mode == ShiftMode::REVERSE {
            // delimiter cut; mark the time of the switch for use to detect
            // crazy dashes.
            self.ticks_since_switching_into_drive = get_time();
        }

        self.shift_mode = ShiftMode::DRIVE;
    }

    pub fn shift_into_reverse(&mut self) {
        self.shift_mode = ShiftMode::REVERSE;
    }

    //FIXME: this is broken rn. Transform x y from camera relative pos
    // to world pos.
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

    pub fn translate(&mut self, x: f32, y: f32) {
        self.center.x += x;
        self.center.y += y;

        for point in &mut self.points {
            point.x += x;
            point.y += y;
        }
    }

    fn update_crazy_dash_status(&mut self) {
        // If its time to end the crazy dash, end it and return early.
        if self.ticks_to_curr_crazy_dash_end > 0.0 && get_time() > self.ticks_to_curr_crazy_dash_end
        {
            self.is_crazy_dashing = false;
            self.ticks_to_curr_crazy_dash_end = -1.0;
            return;
        }

        let time_between_drive_and_gas =
            self.ticks_since_gas_was_activated - self.ticks_since_switching_into_drive;

        self.time_between_drive_and_gas = time_between_drive_and_gas; // Just for debug view

        //let activate_crazy_dash = (0.02..0.5).contains(&time_between_drive_and_gas);
        let activate_crazy_dash = CRAZY_DASH_INPUT_TIMING.contains(&time_between_drive_and_gas);

        if self.is_crazy_dashing || activate_crazy_dash {
            if activate_crazy_dash {
                self.ticks_to_curr_crazy_dash_end = get_time() + CRAZY_DASH_LENGTH;
                self.ticks_since_gas_was_activated = -1.0;
                self.ticks_since_switching_into_drive = -1.0;
            }
            self.is_crazy_dashing = true;
        } else {
            self.is_crazy_dashing = false;
        }
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
    GasHeld,
    GasActivated,
    ShiftIntoDrive,
    ShiftIntoReverse,
    TurnLeft,
    TurnRight,
    Reposition(f32, f32),
}
