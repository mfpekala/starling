use rand::{thread_rng, Rng};

use crate::prelude::*;

#[derive(Component, Reflect)]
pub struct SimpGuide {
    mult: i32,
    speed: f32,
    prefer_future: f32,
}

#[derive(Component, Reflect)]
pub struct SimpHurtbox {
    health: u32,
    immune_to: HashSet<Entity>,
}

#[derive(Bundle)]
pub struct SimpBundle {
    name: Name,
    simp: SimpGuide,
    face_dyno: FaceDyno,
    physics: SimpGuidePhysicsBundle,
    multi: MultiAnimationManager,
    birthing: Birthing,
}
impl SimpBundle {
    const STATIC_RADIUS: f32 = 10.0;
    const TRIGGER_RADIUS: f32 = 8.0;
    const SPEED_RANGE: (f32, f32) = (30.0, 50.0);
    const MULT_RANGE: (i32, i32) = (-40, 40);
    const FUTURE_RANGE: (f32, f32) = (-0.5, 2.0);

    pub fn spawn(pos: Vec2, commands: &mut Commands, parent: Entity) {
        let mut rng = thread_rng();
        let speed = rng.gen_range(Self::SPEED_RANGE.0..Self::SPEED_RANGE.1);
        let mult = rng.gen_range(Self::MULT_RANGE.0..Self::MULT_RANGE.1);
        let fut = rng.gen_range(Self::FUTURE_RANGE.0..Self::FUTURE_RANGE.1);
        commands
            .spawn(Self {
                name: Name::new("simp_guide (body)"),
                simp: SimpGuide {
                    mult,
                    speed,
                    prefer_future: fut,
                },
                face_dyno: FaceDyno,
                physics: SimpGuidePhysicsBundle::new(pos, Self::STATIC_RADIUS),
                multi: multi!([
                    (
                        "core",
                        anim_man!({
                            birthing: {
                                path: "enemies/simp/simp_spawn.png",
                                size: (20, 20),
                                length: 6,
                                fps: 12.0,
                                next: "health3",
                            },
                            health3: {
                                path: "enemies/simp/simp_health3.png",
                                size: (20, 20),
                            },
                            health2: {
                                path: "enemies/simp/simp_health2.png",
                                size: (20, 20),
                            },
                            health1: {
                                path: "enemies/simp/simp_health1.png",
                                size: (20, 20),
                            },
                            death: {
                                path: "enemies/simp/simp_death.png",
                                size: (20, 20),
                                length: 4,
                                fps: 12.0,
                                next: "despawn",
                            }
                        })
                    ),
                    (
                        "light",
                        anim_man!({
                            spawn: {
                                path: "enemies/simp/simp_spawn_light.png",
                                size: (30, 30),
                                length: 6,
                                fps: 12.0,
                                next: "steady",
                            }
                            steady: {
                                path: "enemies/simp/simp_light.png",
                                size: (30, 30),
                            },
                        })
                        .with_render_layers(LightCamera::render_layers())
                    )
                ]),
                birthing: Birthing,
            })
            .with_children(|dad| {
                dad.spawn((
                    Name::new("simp_hurtbox"),
                    SimpHurtbox {
                        health: 3,
                        immune_to: default(),
                    },
                    SimpHurtboxPhysicsBundle::new(Self::TRIGGER_RADIUS),
                ));
            })
            .set_parent(parent);
    }
}

fn birth_simps(
    mut commands: Commands,
    birthing: Query<(Entity, &MultiAnimationManager, &SimpGuide), With<Birthing>>,
) {
    for (eid, multi, guide) in &birthing {
        if multi.manager("core").get_key().as_str() != "birthing" {
            commands.entity(eid).remove::<Birthing>();
            commands
                .entity(eid)
                .insert(StaticReceiver::from_kind(StaticReceiverKind::GoAround {
                    mult: guide.mult,
                }));
        }
    }
}

fn guide_simps(
    bird: Query<(&GlobalTransform, &DynoTran), With<Bird>>,
    mut simp_guides: Query<
        (&SimpGuide, &mut DynoTran, &GlobalTransform),
        (Without<Bird>, Without<Birthing>),
    >,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
) {
    let Ok((bird_gtran, bird_dyno_tran)) = bird.get_single() else {
        return;
    };
    let time_factor = time.delta_seconds() * bullet_time.factor();
    for (simp_guide, mut simp_dyno_tran, simp_body_gtran) in &mut simp_guides {
        let goal_bird_pos =
            bird_gtran.translation().truncate() + bird_dyno_tran.vel * simp_guide.prefer_future;
        let diff = goal_bird_pos - simp_body_gtran.translation().truncate();
        simp_dyno_tran.vel += diff.normalize_or_zero() * 100.0 * time_factor;
        simp_dyno_tran.vel = simp_dyno_tran.vel.clamp_length(0.0, simp_guide.speed);
    }
}

fn hurt_simps(
    mut simp_guides: Query<
        (&mut DynoTran, &mut MultiAnimationManager),
        (Without<AnyBullet>, Without<Birthing>),
    >,
    mut simp_hurtboxes: Query<
        (Entity, &mut SimpHurtbox, &TriggerReceiver, &Parent),
        Without<Dying>,
    >,
    collisions: Query<&TriggerCollisionRecord>,
    bullet_dyno_trans: Query<&DynoTran, With<AnyBullet>>,
    mut commands: Commands,
) {
    for (eid, mut hurtbox, rx, parent) in &mut simp_hurtboxes {
        let Ok((mut parent_dyno_tran, mut parent_multi)) = simp_guides.get_mut(parent.get()) else {
            // continue here so the filtre on guides is valid
            continue;
        };
        let mut new_immune_to = hurtbox.immune_to.clone();
        for cid in rx.collisions.iter() {
            let collision = collisions.get(*cid).unwrap();
            if collision.other_kind == TriggerKind::BulletGood {
                new_immune_to.insert(collision.other_eid);
                if !hurtbox.immune_to.contains(&collision.other_eid) {
                    hurtbox.immune_to.insert(collision.other_eid);
                    // Take damage!
                    hurtbox.health = hurtbox.health.saturating_sub(1);
                    let other_vel = bullet_dyno_trans.get(collision.other_eid).unwrap();
                    parent_dyno_tran.vel += other_vel.vel / 6.0;
                }
            }
        }
        hurtbox.immune_to = new_immune_to;
        if hurtbox.health > 0 {
            parent_multi
                .manager_mut("core")
                .set_key(format!("health{}", hurtbox.health).as_str(), &mut commands);
        } else {
            parent_multi
                .manager_mut("core")
                .set_key("death", &mut commands);
            parent_multi
                .manager_mut("light")
                .set_hidden(true, &mut commands);
            commands.entity(eid).insert(Dying);
        }
    }
}

pub(super) fn register_simps(app: &mut App) {
    app.register_type::<SimpGuide>();
    app.register_type::<SimpHurtbox>();

    app.add_systems(
        Update,
        (birth_simps, guide_simps, hurt_simps)
            .run_if(in_state(PhysicsState::Active))
            .after(PhysicsSet),
    );
}
