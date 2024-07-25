use crate::prelude::*;

#[derive(Debug)]
pub enum BackgroundKind {
    Zenith,
}
impl BackgroundKind {
    pub fn spawn(&self, pos: Vec2, parent: Entity, commands: &mut Commands) {
        commands
            .spawn((
                Name::new(format!("background_{:?}", self)),
                spat_tran(pos.x, pos.y, 0.0),
            ))
            .set_parent(parent)
            .with_children(|commands| match self {
                BackgroundKind::Zenith => {
                    commands.spawn((
                        Name::new("sky"),
                        spat_tran(0.0, 0.0, 0.0),
                        multi!(anim_man!({
                            path: "debug/background_sky.png",
                            size: (IDEAL_WIDTH, IDEAL_HEIGHT),
                        })
                        .with_render_layers(BgSpriteCamera::render_layers())),
                    ));
                    commands.spawn((
                        Name::new("far_clouds"),
                        spat_tran(0.0, 0.0, 1.0),
                        multi!(anim_man!({
                            path: "debug/background_clouds_far.png",
                            size: (IDEAL_WIDTH, IDEAL_HEIGHT),
                        })
                        .with_render_layers(BgSpriteCamera::render_layers())
                        .with_scroll(Vec2::new(0.002, 0.0))),
                    ));
                    commands.spawn((
                        Name::new("close_clouds"),
                        spat_tran(0.0, 0.0, 2.0),
                        multi!(anim_man!({
                            path: "debug/background_clouds_close.png",
                            size: (IDEAL_WIDTH, IDEAL_HEIGHT),
                        })
                        .with_render_layers(BgSpriteCamera::render_layers())
                        .with_scroll(Vec2::new(0.01, 0.0))),
                    ));
                    commands.spawn((
                        Name::new("mountains"),
                        spat_tran(0.0, 0.0, 3.0),
                        multi!(anim_man!({
                            path: "debug/background.png",
                            size: (IDEAL_WIDTH, IDEAL_HEIGHT),
                        })
                        .with_render_layers(BgSpriteCamera::render_layers())),
                    ));
                }
            });
    }
}
