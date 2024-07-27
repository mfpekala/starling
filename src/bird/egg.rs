use crate::prelude::*;

#[derive(Component, Reflect)]
pub struct Egg;

#[derive(Bundle)]
pub struct EggBundle {
    name: Name,
    egg: Egg,
    multi: MultiAnimationManager,
    spatial: SpatialBundle,
    gravity: Gravity,
    dyno_tran: DynoTran,
    bounds: Bounds,
}
impl EggBundle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            name: Name::new("egg"),
            egg: Egg,
            dyno_tran: DynoTran { vel: default() },
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        path: "lenny/egg.png",
                        size: (24, 24),
                    })
                ),
                (
                    "light",
                    anim_man!({
                        path: "lenny/spotlight.png",
                        size: (48, 48),
                    })
                    .with_render_layers(LightCamera::render_layers())
                    .with_scale(Vec2::ONE * 0.5),
                ),
            ]),
            spatial: spat_tran(pos.x, pos.y, ZIX_BIRD - 0.1),
            gravity: Gravity::Normal,
            bounds: Bounds::from_shape(Shape::Circle { radius: 10.0 }),
        }
    }
}
