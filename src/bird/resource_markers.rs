use crate::prelude::*;

#[derive(Component, Debug, PartialEq, Eq)]
enum ResourceMarkerKind {
    Launch,
    Fire,
}
impl ResourceMarkerKind {
    fn to_multi(&self) -> MultiAnimationManager {
        let icon_size = 14;
        let empty_size = 7;
        let scale_down = 0.5;
        match self {
            Self::Launch => {
                multi!([
                    (
                        "core",
                        anim_man!({
                            on: {
                                path: "drag_markers/feather_full.png",
                                size: (icon_size, icon_size),
                            },
                            off: {
                                path: "drag_markers/resource_empty.png",
                                size: (empty_size, empty_size),
                            },
                        })
                        .with_scale(Vec2::ONE * scale_down)
                    ),
                    (
                        "light",
                        anim_man!({
                            on: {
                                path: "drag_markers/feather_full_light.png",
                                size: (icon_size, icon_size),
                            },
                            off: {
                                path: "drag_markers/resource_empty.png",
                                size: (empty_size, empty_size),
                            }
                        })
                        .with_scale(Vec2::ONE * scale_down)
                        .with_render_layers(LightCamera::render_layers())
                    ),
                ])
            }
            Self::Fire => {
                multi!([
                    (
                        "core",
                        anim_man!({
                            on: {
                                path: "drag_markers/bullet_resource_full.png",
                                size: (icon_size, icon_size),
                            },
                            off: {
                                path: "drag_markers/resource_empty.png",
                                size: (empty_size, empty_size),
                            },
                        })
                        .with_scale(Vec2::ONE * scale_down)
                    ),
                    (
                        "light",
                        anim_man!({
                            on: {
                                path: "drag_markers/bullet_resource_full_light.png",
                                size: (icon_size, icon_size),
                            },
                            off: {
                                path: "drag_markers/resource_empty.png",
                                size: (empty_size, empty_size),
                            }
                        })
                        .with_scale(Vec2::ONE * scale_down)
                        .with_render_layers(LightCamera::render_layers())
                    ),
                ])
            }
        }
    }
}

#[derive(Component, Default)]
struct SketchyChildMap {
    map: HashMap<u32, Entity>,
}

#[derive(Bundle)]
struct ResourceMarkerBundle {
    name: Name,
    kind: ResourceMarkerKind,
    spatial: SpatialBundle,
    map: SketchyChildMap,
}
impl ResourceMarkerBundle {
    fn new(kind: ResourceMarkerKind, offset: Vec2) -> Self {
        Self {
            name: Name::new(format!("resource_marker_{kind:?}")),
            kind,
            spatial: spat_tran(offset.x, offset.y, 0.0),
            map: default(),
        }
    }
}

#[derive(Component)]
struct ResourceMarkerChild;

#[derive(Bundle)]
struct ResourceMarkerChildBundle {
    name: Name,
    marker: ResourceMarkerChild,
    multi: MultiAnimationManager,
    spatial: SpatialBundle,
}
impl ResourceMarkerChildBundle {
    fn new(ix: u32, total: u32, multi: MultiAnimationManager) -> Self {
        let halfway = (total - 1) as f32 / 2.0;
        let offset = (ix as f32 - halfway) * 7.0;
        Self {
            name: Name::new(format!("item_{ix}")),
            marker: ResourceMarkerChild,
            multi,
            spatial: spat_tran(offset, 0.0, 0.0),
        }
    }
}

fn update_resource_markers(
    mut commands: Commands,
    permanent_skills: Res<PermanentSkill>,
    bird: Query<(Entity, &Bird)>,
    mut parents: Query<(Entity, &ResourceMarkerKind, &mut SketchyChildMap)>,
    mut anims: Query<&mut MultiAnimationManager, With<ResourceMarkerChild>>,
) {
    let Ok((bid, bird)) = bird.get_single() else {
        return;
    };

    // So sketch...
    // (Spawns in the markers)
    if parents.iter().len() != 2 {
        for (eid, _, _) in parents.iter() {
            if let Some(commands) = commands.get_entity(eid) {
                commands.despawn_recursive();
            }
        }
        commands
            .spawn(ResourceMarkerBundle::new(
                ResourceMarkerKind::Launch,
                Vec2::new(0.0, -11.0),
            ))
            .set_parent(bid);
        commands
            .spawn(ResourceMarkerBundle::new(
                ResourceMarkerKind::Fire,
                Vec2::new(0.0, 12.0),
            ))
            .set_parent(bid);
        return;
    }

    for (eid, kind, mut smap) in &mut parents {
        let (total, left) = if kind == &ResourceMarkerKind::Launch {
            (
                permanent_skills.get_num_launches(),
                bird.get_launches_left(),
            )
        } else {
            (permanent_skills.get_num_bullets(), bird.get_bullets_left())
        };
        if total != smap.map.len() as u32 {
            // We do not have the right number of things showing
            // Clear all the children, spawn them in and reset the map, then return
            // The animations will be fixed next frame
            smap.map.clear();
            commands.entity(eid).despawn_descendants();
            commands.entity(eid).with_children(|me| {
                // Rust no
                for ix in 0..total {
                    let id = me
                        .spawn(ResourceMarkerChildBundle::new(ix, total, kind.to_multi()))
                        .id();
                    smap.map.insert(ix, id);
                }
            });
        } else {
            for ix in 0..total {
                let eid = smap.map[&ix];
                let showing = ix < left;
                let key = if showing { "on" } else { "off" };
                let Ok(mut multi) = anims.get_mut(eid) else {
                    return;
                };
                multi
                    .manager_mut("core")
                    .reset_key_with_points(key, &mut commands);
                multi
                    .manager_mut("light")
                    .reset_key_with_points(key, &mut commands);
            }
        }
    }
}

pub(super) fn register_resource_markers(app: &mut App) {
    app.add_systems(Update, update_resource_markers);
}
