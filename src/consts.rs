use bevy::prelude::*;

pub const WINDOW_WIDTH: u32 = 320 * 4;
pub const WINDOW_HEIGHT: u32 = 180 * 4;
pub const WINDOW_VEC: UVec2 = UVec2::new(WINDOW_HEIGHT, WINDOW_HEIGHT);

#[allow(nonstandard_style)]
pub const WINDOW_WIDTH_f32: f32 = WINDOW_WIDTH as f32;
#[allow(nonstandard_style)]
pub const WINDOW_HEIGHT_f32: f32 = WINDOW_HEIGHT as f32;
#[allow(nonstandard_style)]
pub const WINDOW_VEC_f32: Vec2 = Vec2::new(WINDOW_WIDTH_f32, WINDOW_HEIGHT_f32);
