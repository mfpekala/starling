use crate::prelude::*;

#[derive(Component, Reflect)]
pub struct Ghost;

#[derive(Bundle)]
pub struct GhostBundle {
    name: Name,
    multi: MultiAnimationManager,
    spatial: SpatialBundle,
}
impl GhostBundle {
    pub fn new(pos: Vec2, flip_x: bool) -> Self {
        Self {
            name: Name::new("ghost"),
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        path: "ghost/fly.png",
                        size: (24, 24),
                        length: 3,
                        fps: 12.0,
                    })
                    .with_flip_x(flip_x)
                ),
                (
                    "light",
                    anim_man!({
                        path: "lenny/spotlight.png",
                        size: (48, 48),
                    })
                    .with_render_layers(LightCamera::render_layers()),
                ),
            ]),
            spatial: spat_tran(pos.x, pos.y, ZIX_BIRD - 0.1),
        }
    }
}
