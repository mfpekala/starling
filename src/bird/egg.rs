use crate::prelude::*;

#[derive(Component, Reflect)]
pub struct Egg;

#[derive(Bundle)]
pub struct EggChoice {
    name: Name,
    egg: Egg,
    multi: MultiAnimationManager,
    spatial: SpatialBundle,
    gravity: Gravity,
    dyno_tran: DynoTran,
    bounds: Bounds,
}
impl EggChoice {
    pub fn new(pos: Vec2) -> Self {
        Self {
            name: Name::new("egg"),
            egg: Egg,
            dyno_tran: DynoTran { vel: default() },
            multi: multi!([
                (
                    "core1",
                    anim_man!({
                        egg: {
                            path: "lenny/egg.png",
                            size: (24, 24),
                        },
                        bird: {
                            path: "lenny/fly.png",
                            size: (24, 24),
                            length: 3,
                            fps: 16.0,
                        },
                    })
                ),
                (
                    "light1",
                    anim_man!({
                        path: "lenny/spotlight.png",
                        size: (48, 48),
                    })
                    .with_render_layers(LightCamera::render_layers())
                    .with_scale(Vec2::new(2.0, 2.0))
                ),
                // THE NASTIEST HACK
                (
                    "core2",
                    anim_man!({
                        egg: {
                            path: "lenny/egg.png",
                            size: (24, 24),
                        },
                        bird: {
                            path: "lenny/fly.png",
                            size: (24, 24),
                            length: 3,
                            fps: 16.0,
                        },
                    })
                    .with_offset(Vec3::new(-pos.x * 2.0, 0.0, 0.0))
                ),
                (
                    "light2",
                    anim_man!({
                        path: "lenny/spotlight.png",
                        size: (48, 48),
                    })
                    .with_render_layers(LightCamera::render_layers())
                    .with_offset(Vec3::new(-pos.x * 2.0, 0.0, 0.0))
                    .with_scale(Vec2::new(2.0, 2.0))
                ),
            ]),
            spatial: spat_tran(pos.x, pos.y, ZIX_BIRD - 0.1),
            gravity: Gravity::Normal,
            bounds: Bounds::from_shape(Shape::Circle { radius: 10.0 }),
        }
    }
}
