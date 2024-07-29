use crate::prelude::*;

#[derive(Bundle)]
pub struct StickyPlatformBundle {
    name: Name,
    physics: StickyPhysicsBundle,
    pub multi: MultiAnimationManager,
}
impl StickyPlatformBundle {
    pub fn new(name: &str, pos: Vec2, shape: Shape) -> StickyPlatformBundle {
        let multi_points = shape.to_anim_points();
        let anim_man = match &shape {
            Shape::Circle { radius } => anim_man!({
                path: "environment/log_circular.png",
                size: (70, 70),
            })
            .with_scale(Vec2::ONE * *radius / 31.0),
            Shape::Polygon { points } => {
                let bound = uvec2_bound(&points);
                if bound.x > bound.y {
                    anim_man!({
                        path: "environment/log_horizontal.png",
                        size: (64, 32),
                    })
                    .with_points(multi_points)
                } else {
                    anim_man!({
                        path: "environment/log_vertical.png",
                        size: (25, 64),
                    })
                    .with_points(multi_points)
                }
            }
        };
        Self {
            name: Name::new(format!("sticky_platform_{name}")),
            physics: StickyPhysicsBundle::new(pos, Bounds::from_shape(shape)),
            multi: multi!(anim_man),
        }
    }

    // Common configuration
    pub fn around_room() -> Self {
        let buffer_out = 5.0;
        let buffer_in = 5.0;
        let name = "around_room";
        let shape = Shape::Polygon {
            points: vec![
                // GO FORWARD
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 - buffer_out,
                    -IDEAL_HEIGHT_f32 / 2.0 - buffer_out,
                ),
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 - buffer_out,
                    IDEAL_HEIGHT_f32 / 2.0 + buffer_out,
                ),
                Vec2::new(
                    IDEAL_WIDTH_f32 / 2.0 + buffer_out,
                    IDEAL_HEIGHT_f32 / 2.0 + buffer_out,
                ),
                Vec2::new(
                    IDEAL_WIDTH_f32 / 2.0 + buffer_out,
                    -IDEAL_HEIGHT_f32 / 2.0 - buffer_out,
                ),
                // PAUSE
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 - buffer_out,
                    -IDEAL_HEIGHT_f32 / 2.0 - buffer_out,
                ),
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 + buffer_in,
                    -IDEAL_HEIGHT_f32 / 2.0 + buffer_in,
                ),
                // GO BACK
                Vec2::new(
                    IDEAL_WIDTH_f32 / 2.0 - buffer_in,
                    -IDEAL_HEIGHT_f32 / 2.0 + buffer_in,
                ),
                Vec2::new(
                    IDEAL_WIDTH_f32 / 2.0 - buffer_in,
                    IDEAL_HEIGHT_f32 / 2.0 - buffer_in,
                ),
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 + buffer_in,
                    IDEAL_HEIGHT_f32 / 2.0 - buffer_in,
                ),
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 + buffer_in,
                    -IDEAL_HEIGHT_f32 / 2.0 + buffer_in,
                ),
            ],
        };
        Self::new(name, Vec2::ZERO, shape)
    }
}

#[derive(Bundle)]
pub struct HardPlatformBundle {
    name: Name,
    physics: HardPhysicsBundle,
    pub multi: MultiAnimationManager,
}
impl HardPlatformBundle {
    pub fn new(name: &str, pos: Vec2, shape: Shape) -> HardPlatformBundle {
        let multi_points = shape.to_anim_points();
        Self {
            name: Name::new(format!("sticky_platform_{name}")),
            physics: HardPhysicsBundle::new(pos, Bounds::from_shape(shape)),
            multi: multi!(anim_man!({
                path: "debug/non_sticky_texture.png",
                size:  (36, 36),
            })
            .with_points(multi_points)),
        }
    }

    // Common configuration
    pub fn around_room() -> Self {
        let buffer_out = 5.0;
        let buffer_in = 5.0;
        let name = "around_room";
        let shape = Shape::Polygon {
            points: vec![
                // GO FORWARD
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 - buffer_out,
                    -IDEAL_HEIGHT_f32 / 2.0 - buffer_out,
                ),
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 - buffer_out,
                    IDEAL_HEIGHT_f32 / 2.0 + buffer_out,
                ),
                Vec2::new(
                    IDEAL_WIDTH_f32 / 2.0 + buffer_out,
                    IDEAL_HEIGHT_f32 / 2.0 + buffer_out,
                ),
                Vec2::new(
                    IDEAL_WIDTH_f32 / 2.0 + buffer_out,
                    -IDEAL_HEIGHT_f32 / 2.0 - buffer_out,
                ),
                // PAUSE
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 - buffer_out,
                    -IDEAL_HEIGHT_f32 / 2.0 - buffer_out,
                ),
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 + buffer_in,
                    -IDEAL_HEIGHT_f32 / 2.0 + buffer_in,
                ),
                // GO BACK
                Vec2::new(
                    IDEAL_WIDTH_f32 / 2.0 - buffer_in,
                    -IDEAL_HEIGHT_f32 / 2.0 + buffer_in,
                ),
                Vec2::new(
                    IDEAL_WIDTH_f32 / 2.0 - buffer_in,
                    IDEAL_HEIGHT_f32 / 2.0 - buffer_in,
                ),
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 + buffer_in,
                    IDEAL_HEIGHT_f32 / 2.0 - buffer_in,
                ),
                Vec2::new(
                    -IDEAL_WIDTH_f32 / 2.0 + buffer_in,
                    -IDEAL_HEIGHT_f32 / 2.0 + buffer_in,
                ),
            ],
        };
        Self::new(name, Vec2::ZERO, shape)
    }
}
