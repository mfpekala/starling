use std::f32::consts::PI;

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

#[derive(Component, Reflect)]
pub struct SpewWaiting {
    time_until_charge: Timer,
}
impl SpewWaiting {
    pub fn new() -> Self {
        Self {
            time_until_charge: Timer::from_seconds(
                thread_rng().gen_range(1.0..3.0),
                TimerMode::Once,
            ),
        }
    }
}

#[derive(Component, Reflect)]
pub struct SpewCharging {
    sound_played: bool,
}

#[derive(Bundle)]
pub struct SpewBundle {
    name: Name,
    simp: SpewGuide,
    physics: SpewGuidePhysicsBundle,
    birthing: Birthing,
}
impl SpewBundle {
    const STATIC_RADIUS: f32 = 18.0;
    const TRIGGER_RADIUS: f32 = 15.0;
    const SPEED_RANGE: (f32, f32) = (5.0, 15.0);
    const MULT_RANGE: (i32, i32) = (-10, 10);
    const FUTURE_RANGE: (f32, f32) = (-0.05, 0.1);
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
                    SpewWaiting::new(),
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
                                    next: "post_death_somehow",
                                },
                                post_death_somehow: {
                                    path: "sprites/none.png",
                                    size: (1, 1),
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
                                prelude: {
                                    path: "enemies/spew/spew_material_prelude.png",
                                    size: (6, 6),
                                    length: 14,
                                    fps: 12.0,
                                    next: "harmful",
                                },
                                harmful: {
                                    path: "enemies/spew/spew_material_harmful.png",
                                    size: (6, 6),
                                    length: 2,
                                    fps: 12.0,
                                    next: "fading",
                                },
                                fading: {
                                    path: "enemies/spew/spew_material_fading.png",
                                    size: (6, 6),
                                    length: 2,
                                    fps: 12.0,
                                    next: "inactive",
                                },
                            })
                            .with_offset(Vec3::Z * -0.5),
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

fn rotate_spews(
    bird: Query<(&GlobalTransform, &DynoTran), With<Bird>>,
    guides_q: Query<&SpewGuide>,
    mut spews_q: Query<(&mut Transform, &GlobalTransform, &Parent), With<SpewWaiting>>,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
) {
    let Ok((bird_gtran, bird_dyno_tran)) = bird.get_single() else {
        return;
    };
    let max_rot_this_frame = PI / 2.0 * time.delta_seconds() * bullet_time.factor();
    for (mut tran, gtran, parent) in &mut spews_q {
        let guide = guides_q.get(parent.get()).unwrap();
        let (my_gtran, my_angle) = gtran.tran_n_angle();
        let goal_bird_pos =
            bird_gtran.translation().truncate() + bird_dyno_tran.vel * guide.prefer_future;
        let diff = goal_bird_pos - my_gtran;
        let short_rot = shortest_rotation(my_angle - PI / 2.0, diff.to_angle());
        let rot = short_rot.signum() * short_rot.abs().clamp(0.0, max_rot_this_frame);
        tran.set_angle(my_angle + rot);
    }
}

fn update_waiting_spews(
    mut commands: Commands,
    mut spews_q: Query<(Entity, &mut MultiAnimationManager, &mut SpewWaiting)>,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
) {
    let time_factor = time.delta().mul_f32(bullet_time.factor());
    for (eid, mut multi, mut waiting) in &mut spews_q {
        waiting.time_until_charge.tick(time_factor);
        if waiting.time_until_charge.finished() {
            commands.entity(eid).remove::<SpewWaiting>();
            commands.entity(eid).insert(SpewCharging {
                sound_played: false,
            });
            multi
                .manager_mut("core")
                .reset_key_with_points("charging", &mut commands);
            multi
                .manager_mut("light")
                .reset_key_with_points("charging", &mut commands);
            let material_points = simple_rect(8.0, IDEAL_WIDTH_f32 * 2.0)
                .into_iter()
                .map(|p| p - Vec2::new(0.0, IDEAL_WIDTH_f32))
                .collect::<Vec<_>>();
            multi
                .manager_mut("material")
                .reset_key_with_points("prelude", &mut commands);
            multi
                .manager_mut("material")
                .reset_points(material_points, &mut commands);
        }
    }
}

fn update_charging_spews(
    mut bird: Query<(&mut Bird, &Bounds, &GlobalTransform)>,
    mut commands: Commands,
    mut spews_q: Query<(
        Entity,
        &mut MultiAnimationManager,
        &GlobalTransform,
        &mut SpewCharging,
    )>,
    mut skills: ResMut<EphemeralSkill>,
) {
    let Ok((mut bird, bird_bounds, bird_gtran)) = bird.get_single_mut() else {
        return;
    };
    for (eid, mut multi, spew_gtran, mut spew_charging) in &mut spews_q {
        if multi.manager("material").get_key().as_str() == "harmful" {
            if !spew_charging.sound_played {
                spew_charging.sound_played = true;
                commands.spawn(SoundEffect::universal("sound_effects/laser.ogg", 0.1));
            }
            if bird.taking_damage.is_some() {
                continue;
            }
            let harmful_shape = Shape::Polygon {
                points: multi.manager("material").get_points(),
            };
            let (hp1, hp2) = spew_gtran.tran_n_angle();
            if bird_bounds
                .get_shape()
                .bounce_off(bird_gtran.tran_n_angle(), (&harmful_shape, hp1, hp2))
                .is_some()
            {
                bird.taking_damage = Some(Timer::from_seconds(1.0, TimerMode::Once));
                skills.dec_current_health(1);
                commands.spawn(SoundEffect::universal(
                    "sound_effects/lenny_take_damage.ogg",
                    0.8,
                ));
            }
        } else if multi.manager("material").get_key().as_str() == "inactive" {
            // Done shooting, go back
            commands.entity(eid).remove::<SpewCharging>();
            commands.entity(eid).insert(SpewWaiting::new());
            multi
                .manager_mut("core")
                .reset_key_with_points("stable", &mut commands);
            multi
                .manager_mut("light")
                .reset_key_with_points("stable", &mut commands);
        }
    }
}

fn hurt_spews(
    mut spew_guides: Query<&mut DynoTran, (Without<AnyBullet>, Without<Birthing>, With<SpewGuide>)>,
    mut spew_hurtboxes: Query<
        (
            Entity,
            &mut SpewHurtbox,
            &TriggerReceiver,
            &Parent,
            &mut MultiAnimationManager,
        ),
        Without<Dying>,
    >,
    collisions: Query<&TriggerCollisionRecord>,
    bullet_dyno_trans: Query<&DynoTran, With<AnyBullet>>,
    mut commands: Commands,
    mut bird: Query<&mut Bird>,
) {
    for (eid, mut hurtbox, rx, parent, mut multi) in &mut spew_hurtboxes {
        let Ok(mut parent_dyno_tran) = spew_guides.get_mut(parent.get()) else {
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
            multi
                .manager_mut("damage")
                .reset_key_with_points(format!("health{}", hurtbox.health).as_str(), &mut commands);
        } else {
            multi
                .manager_mut("core")
                .reset_key_with_points("death", &mut commands);
            multi.manager_mut("light").set_hidden(true, &mut commands);
            multi.manager_mut("damage").set_hidden(true, &mut commands);
            multi
                .manager_mut("material")
                .set_hidden(true, &mut commands);
            commands.entity(eid).remove::<TriggerReceiver>();
            commands.entity(eid).remove::<SpewWaiting>();
            commands.entity(eid).remove::<SpewCharging>();
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

fn cursed_cleanup(
    mut commands: Commands,
    cursed: Query<(Entity, Option<&Children>), With<SpewGuide>>,
    more_cursed: Query<(Entity, &MultiAnimationManager, &Parent), With<SpewHurtbox>>,
) {
    for (eid, children) in &cursed {
        if children.is_none() || children.unwrap().is_empty() {
            commands.entity(eid).despawn_recursive();
        }
    }
    for (_eid, multi, parent) in &more_cursed {
        if multi.manager("core").get_key().as_str() == "post_death_somehow" {
            commands.entity(parent.get()).despawn_recursive();
        }
    }
}

pub(super) fn register_spews(app: &mut App) {
    app.register_type::<SpewGuide>();
    app.register_type::<SpewHurtbox>();

    app.add_systems(
        Update,
        (
            birth_spews,
            guide_spews,
            rotate_spews,
            update_waiting_spews,
            update_charging_spews,
            hurt_spews,
            cursed_cleanup,
        )
            .chain()
            .run_if(in_state(PhysicsState::Active))
            .after(PhysicsSet)
            .after(AnimationSet),
    );
}
