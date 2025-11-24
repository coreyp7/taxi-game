pub const GAS_VELOCITY: f32 = 8.0;
pub const CRAZY_DASH_VELOCITY: f32 = 30.0;
pub const REVERSE_VELOCITY: f32 = 20.0;
pub const PLAYER_MAX_VELOCITY: f32 = 750.0;
//pub const PLAYER_MAX_VELOCITY: f32 = 500.0;
pub const PLAYER_MAX_REVERSE_VELOCITY: f32 = -350.0;
pub const PLAYER_ROTATION_SPEED: f32 = 125.0;
pub const CRAZY_DASH_LENGTH: f64 = 0.3;
pub const CRAZY_DASH_MAX_VELOCITY: f32 = 2300.0;

//pub const CAR_DRAG: f32 = 700.0;
pub const CAR_DRAG: f32 = 500.0;
//pub const CRAZY_DASH_DRAG: f32 = 600.0;

//pub const CAMERA_SPEED: f32 = 2.5;
pub const CAMERA_SPEED: f32 = 7.5;

/// Will render debug info if true.
pub const IS_DEBUG: bool = true;

// this is extra but was trying to improve the function where vertices are created
// TODO could revert this tbh
pub struct PlayerShapeConfig {
    pub half_width: f32,
    pub half_height: f32,
    pub inner_space_w: f32,
    pub inner_space_h: f32,
}

pub const PLAYER_SHAPE: PlayerShapeConfig = PlayerShapeConfig {
    half_width: 30.0,
    half_height: 40.0,
    inner_space_w: 10.0,
    inner_space_h: 15.0,
};
