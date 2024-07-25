use bevy::prelude::*;

pub const IDEAL_WIDTH: u32 = 320;
pub const IDEAL_HEIGHT: u32 = 180;
pub const IDEAL_VEC: UVec2 = UVec2::new(IDEAL_WIDTH, IDEAL_HEIGHT);

#[allow(nonstandard_style)]
pub const IDEAL_WIDTH_f32: f32 = IDEAL_WIDTH as f32;
#[allow(nonstandard_style)]
pub const IDEAL_HEIGHT_f32: f32 = IDEAL_HEIGHT as f32;
#[allow(nonstandard_style)]
pub const IDEAL_VEC_f32: Vec2 = Vec2::new(IDEAL_WIDTH_f32, IDEAL_HEIGHT_f32);

/// How much bigger is the window then our ideal?
pub const IDEAL_GROWTH: u32 = 4;
/// How much bigger is the window then our ideal? (float)
#[allow(nonstandard_style)]
pub const IDEAL_GROWTH_f32: f32 = 4.0;

pub const WINDOW_WIDTH: u32 = IDEAL_WIDTH * IDEAL_GROWTH;
pub const WINDOW_HEIGHT: u32 = IDEAL_HEIGHT * IDEAL_GROWTH;
pub const WINDOW_VEC: UVec2 = UVec2::new(WINDOW_WIDTH, WINDOW_HEIGHT);

#[allow(nonstandard_style)]
pub const WINDOW_WIDTH_f32: f32 = WINDOW_WIDTH as f32;
#[allow(nonstandard_style)]
pub const WINDOW_HEIGHT_f32: f32 = WINDOW_HEIGHT as f32;
#[allow(nonstandard_style)]
pub const WINDOW_VEC_f32: Vec2 = Vec2::new(WINDOW_WIDTH_f32, WINDOW_HEIGHT_f32);

// Helpful to have all the z-indices here for rendor order shenanigans
// Remember, each of these zix only effect interactions within the same layer, between layers, order is determined
pub const ZIX_BACKGROUND: f32 = 0.0;
pub const ZIX_BIRD: f32 = 5.0;
pub const ZIX_BULLET: f32 = 4.0;
pub const ZIX_DEBUG: f32 = 350.0;
pub const ZIX_MENU: f32 = 300.0;
pub const ZIX_PAUSE: f32 = 200.0;
pub const ZIX_STICKY: f32 = 3.0;
pub const ZIX_TRANSITION: f32 = 500.0;
pub const ZIX_MIN: f32 = -600.0; // Anything further back than this gets culled by the camera(s)
pub const ZIX_MAX: f32 = 600.0; // Anything further forward than this gets culled by the camera(s)

pub const DEFAULT_ANIMATION_FPS: f32 = 24.0;
