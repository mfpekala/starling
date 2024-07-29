use rand::{thread_rng, Rng};

use crate::prelude::*;

#[derive(Component, Reflect)]
pub struct SpewGuide {
    mult: i32,
    speed: f32,
    prefer_future: f32,
}

#[derive(Component, Reflect)]
pub struct SpewHurtbox {
    health: u32,
    immune_to: HashSet<Entity>,
}

#[derive(Bundle)]
pub struct SpewBundle {
    name: Name,
    simp: SpewGuide,
    physics: SpewGuidePhysicsBundle,
    birthing: Birthing,
}
impl SpewBundle {
    const STATIC_RADIUS: f32 = 16.0;
    const TRIGGER_RADIUS: f32 = 13.0;
    const SPEED_RANGE: (f32, f32) = (5.0, 15.0);
    const MULT_RANGE: (i32, i32) = (-10, 10);
    const FUTURE_RANGE: (f32, f32) = (-0.5, 2.0);
}
impl EnemyBundle for SpewBundle {
    type CountComponent = SpewGuide;

    fn spawn(pos: Vec2, commands: &mut Commands, parent: Entity) {
        let mut rng = thread_rng();
        let speed = rng.gen_range(Self::SPEED_RANGE.0..Self::SPEED_RANGE.1);
        let mult = rng.gen_range(Self::MULT_RANGE.0..Self::MULT_RANGE.1);
        let fut = rng.gen_range(Self::FUTURE_RANGE.0..Self::FUTURE_RANGE.1);
        commands.spawn(SoundEffect::universal("sound_effects/simp_spawn.ogg", 0.4));
        commands
            .spawn(Self {
                name: Name::new("spew_guide (body)"),
                simp: SpewGuide {
                    mult,
                    speed,
                    prefer_future: fut,
                },
                physics: SpewGuidePhysicsBundle::new(pos, Self::STATIC_RADIUS),
                birthing: Birthing,
            })
            .with_children(|dad| {
                dad.spawn((
                    Name::new("spew_hurtbox"),
                    SpewHurtbox {
                        health: 3,
                        immune_to: default(),
                    },
                    SpewHurtboxPhysicsBundle::new(Self::TRIGGER_RADIUS),
                    Birthing,
                    multi!([
                        (
                            "core",
                            anim_man!({
                                birthing: {
                                    path: "enemies/spew/spew_spawn.png",
                                    size: (30, 30),
                                    length: 5,
                                    fps: 12.0,
                                    next: "stable",
                                },
                                stable: {
                                    path: "enemies/spew/spew_stable.png",
                                    size: (30, 30),
                                },
                                charging: {
                                    path: "enemies/spew/spew_charging.png",
                                    size: (30, 30),
                                },
                                death: {
                                    path: "enemies/spew/spew_death.png",
                                    size: (30, 30),
                                    length: 4,
                                    fps: 6.0,
                                    next: "despawn",
                                }
                            })
                        ),
                        (
                            "damage",
                            anim_man!({
                                health3: {
                                    path: "sprites/none.png",
                                    size: (1, 1),
                                },
                                health2: {
                                    path: "enemies/spew/spew_hurt1.png",
                                    size: (30, 30),
                                },
                                health1: {
                                    path: "enemies/spew/spew_hurt2.png",
                                    size: (30, 30),
                                }
                            })
                            .with_offset(Vec3::Z * 0.5)
                        ),
                        (
                            "light",
                            anim_man!({
                                stable: {
                                    path: "enemies/spew/spew_light_stable.png",
                                    size: (64, 64),
                                }
                                charging: {
                                    path: "enemies/spew/spew_light_charging.png",
                                    size: (64, 64),
                                },
                            })
                            .with_render_layers(LightCamera::render_layers())
                            .with_hidden(true),
                        ),
                        (
                            "material",
                            anim_man!({
                                inactive: {
                                    path: "sprites/none.png",
                                    size: (1, 1),
                                },
                                active: {
                                    path: "enemies/spew/spew_material.png",
                                    size: (6, 6),
                                    length: 18,
                                    next: "inactive",
                                },
                            })
                        )
                    ]),
                ));
            })
            .set_parent(parent);
    }
}

fn birth_spews(
    mut commands: Commands,
    mut birthing: Query<
        (Entity, &mut MultiAnimationManager, &SpewHurtbox, &Parent),
        With<Birthing>,
    >,
    rents: Query<&SpewGuide>,
) {
    for (eid, mut multi, _hurtbox, parent) in &mut birthing {
        if multi.manager("core").get_key().as_str() != "birthing" {
            multi.manager_mut("light").set_hidden(false, &mut commands);
            commands.entity(eid).remove::<Birthing>();
            let guide = rents.get(parent.get()).unwrap();
            commands
                .entity(parent.get())
                .insert(StaticReceiver::from_kind(StaticReceiverKind::GoAround {
                    mult: guide.mult,
                }));
            commands.entity(parent.get()).remove::<Birthing>();
        }
    }
}

fn guide_spews(
    bird: Query<(&GlobalTransform, &DynoTran), With<Bird>>,
    mut spew_guides: Query<
        (&SpewGuide, &mut DynoTran, &GlobalTransform),
        (Without<Bird>, Without<Birthing>),
    >,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
) {
    let Ok((bird_gtran, bird_dyno_tran)) = bird.get_single() else {
        return;
    };
    let time_factor = time.delta_seconds() * bullet_time.factor();
    for (spew_guide, mut spew_dyno_tran, spew_body_gtran) in &mut spew_guides {
        let goal_bird_pos =
            bird_gtran.translation().truncate() + bird_dyno_tran.vel * spew_guide.prefer_future;
        let diff = goal_bird_pos - spew_body_gtran.translation().truncate();
        spew_dyno_tran.vel += diff.normalize_or_zero() * 100.0 * time_factor;
        spew_dyno_tran.vel = spew_dyno_tran.vel.clamp_length(0.0, spew_guide.speed);
    }
}

fn hurt_spews(
    mut spew_guides: Query<
        (&mut DynoTran, &mut MultiAnimationManager),
        (Without<AnyBullet>, Without<Birthing>, With<SpewGuide>),
    >,
    mut spew_hurtboxes: Query<
        (Entity, &mut SpewHurtbox, &TriggerReceiver, &Parent),
        Without<Dying>,
    >,
    collisions: Query<&TriggerCollisionRecord>,
    bullet_dyno_trans: Query<&DynoTran, With<AnyBullet>>,
    mut commands: Commands,
    mut bird: Query<&mut Bird>,
) {
    for (eid, mut hurtbox, rx, parent) in &mut spew_hurtboxes {
        let Ok((mut parent_dyno_tran, mut parent_multi)) = spew_guides.get_mut(parent.get()) else {
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
                    commands.spawn(SoundEffect::universal(
                        "sound_effects/spew_take_damage.ogg",
                        0.2,
                    ));
                }
            }
        }
        hurtbox.immune_to = new_immune_to;
        if hurtbox.health > 0 {
            parent_multi
                .manager_mut("damage")
                .set_key(format!("health{}", hurtbox.health).as_str(), &mut commands);
        } else {
            parent_multi
                .manager_mut("core")
                .set_key("death", &mut commands);
            parent_multi
                .manager_mut("light")
                .set_hidden(true, &mut commands);
            parent_multi
                .manager_mut("material")
                .set_hidden(true, &mut commands);
            commands.entity(eid).insert(Dying {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
                dont_despawn: false,
            });
            commands.spawn(SoundEffect::universal("sound_effects/spew_death1.ogg", 0.1));
            // Ahh if this weren't a jam I'd do something nicer here maybe but idk, this just feels clunky
            bird.get_single_mut()
                .and_then(|mut bird| {
                    bird.dec_kills_left(1);
                    Ok(())
                })
                .ok();
        }
    }
}

pub(super) fn register_spews(app: &mut App) {
    app.register_type::<SpewGuide>();
    app.register_type::<SpewHurtbox>();

    app.add_systems(
        Update,
        (birth_spews, guide_spews, hurt_spews)
            .run_if(in_state(PhysicsState::Active))
            .after(PhysicsSet),
    );
}
