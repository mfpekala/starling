use crate::prelude::*;

fn spawn_sky(commands: &mut ChildBuilder) {
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
}

#[derive(Debug)]
pub enum BackgroundKind {
    SkyOnly,
    Zenith,
    Forest,
}
impl BackgroundKind {
    pub fn spawn(&self, pos: Vec2, parent: Entity, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                Name::new(format!("background_{:?}", self)),
                spat_tran(pos.x, pos.y, 0.0),
            ))
            .set_parent(parent)
            .with_children(|commands| match self {
                BackgroundKind::SkyOnly => {
                    spawn_sky(commands);
                }
                BackgroundKind::Zenith => {
                    spawn_sky(commands);
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
                BackgroundKind::Forest => {
                    commands.spawn((
                        Name::new("sky"),
                        spat_tran(0.0, 0.0, 0.0),
                        multi!(anim_man!({
                            path: "debug/forest/background_sky_trees.png",
                            size: (320, 180),
                        })),
                    ));
                    commands.spawn((
                        Name::new("trees_far"),
                        spat_tran(0.0, 0.0, 1.0),
                        multi!(anim_man!({
                            path: "debug/forest/background_trees_far.png",
                            size: (320, 180),
                        })),
                    ));
                    commands.spawn((
                        Name::new("trees_far"),
                        spat_tran(0.0, 0.0, 2.0),
                        multi!(anim_man!({
                            path: "debug/forest/background_trees_mid.png",
                            size: (320, 180),
                        })),
                    ));
                    commands.spawn((
                        Name::new(""),
                        spat_tran(0.0, 0.0, 3.0),
                        multi!(anim_man!({
                            path: "debug/forest/background_trees_lightray.png",
                            size: (320, 180),
                        })),
                    ));
                    commands.spawn((
                        Name::new(""),
                        spat_tran(0.0, 0.0, 4.0),
                        multi!(anim_man!({
                            path: "debug/forest/background_trees_close.png",
                            size: (320, 180),
                        })),
                    ));
                }
            })
            .id()
    }
}
