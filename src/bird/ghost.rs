use crate::prelude::*;

#[derive(Component, Reflect)]
pub struct Ghost;

#[derive(Bundle)]
pub struct GhostBundle {
    name: Name,
    ghost: Ghost,
    multi: MultiAnimationManager,
    spatial: SpatialBundle,
    particles: SimpleParticleSpawner,
}
impl GhostBundle {
    pub fn new(pos: Vec2, flip_x: bool, green: bool) -> Self {
        Self {
            name: Name::new("ghost"),
            ghost: Ghost,
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        path: format!("ghost/fly{}.png", if green { "_green" } else {""}).as_str(),
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
            particles: SimpleParticleSpawner::new(
                Particle::new(default())
                    .with_colors(
                        Color::srgba_u8(255, 255, 255, 10),
                        Color::srgba_u8(255, 255, 255, 0),
                    )
                    .with_sizes(6.0, 4.0),
            ),
        }
    }
}
