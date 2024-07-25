use crate::prelude::*;

#[derive(Bundle)]
pub struct StickyPlatformBundle {
    name: Name,
    physics: StickyPhysicsBundle,
    multi: MultiAnimationManager,
}
impl StickyPlatformBundle {
    pub fn new(name: &str, pos: Vec2, shape: Shape) -> StickyPlatformBundle {
        let multi_points = shape.to_anim_points();
        Self {
            name: Name::new(format!("sticky_platform_{name}")),
            physics: StickyPhysicsBundle::new(pos, Bounds::from_shape(shape)),
            multi: multi!(anim_man!({
                path: "debug/texture.png",
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
