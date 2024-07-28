use crate::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct PracticeTarget {
    pub key: String,
    pub respawn_after: Option<f32>,
    pub time_dead: Option<f32>,
}
#[derive(Bundle)]
pub struct PracticeTargetBundle {
    name: Name,
    practice_target: PracticeTarget,
    trigger: TutorialTriggerPhysicsBundle,
    multi: MultiAnimationManager,
}
impl PracticeTargetBundle {
    pub fn new(pos: Vec2, key: &str, respawn_after: Option<f32>) -> Self {
        let trigger = TutorialTriggerPhysicsBundle::new(pos, 8.0, key.to_string());
        Self {
            name: Name::new(format!("target_{key}")),
            practice_target: PracticeTarget {
                key: key.to_string(),
                respawn_after,
                time_dead: None,
            },
            trigger,
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        target: {
                            path: "tutorial/target.png",
                            size: (16, 16),
                        },
                        explode: {
                            path: "tutorial/target_break.png",
                            size: (20, 20),
                            length: 4,
                            next: "none",
                        },
                        none: {
                            path: "sprites/none.png",
                            size: (1, 1),
                        },
                    })
                ),
                (
                    "light",
                    anim_man!({
                        path: "tutorial/target_light.png",
                        size: (16, 16),
                    })
                    .with_render_layers(LightCamera::render_layers())
                ),
            ]),
        }
    }
}

#[derive(Event)]
pub(super) struct PracticeTargetStatus {
    pub bullet_index: u32,
    pub alive: bool,
    pub key: String,
}

fn update_practice_targets(
    mut practice_targets: Query<(
        Entity,
        &mut PracticeTarget,
        &TriggerReceiver,
        &mut MultiAnimationManager,
    )>,
    collisions: Query<&TriggerCollisionRecord>,
    mut status_writer: EventWriter<PracticeTargetStatus>,
    mut commands: Commands,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
) {
    for (eid, mut practice_target, triggers, mut multi) in &mut practice_targets {
        let respawn_after = practice_target.respawn_after.clone();
        let key = practice_target.key.clone();
        if let Some(time_dead) = practice_target.time_dead.as_mut() {
            // The target is dead
            let time_factor = time.delta_seconds() * bullet_time.factor();
            *time_dead += time_factor;
            match respawn_after {
                Some(amount) => {
                    if *time_dead >= amount {
                        status_writer.send(PracticeTargetStatus {
                            bullet_index: 0,
                            alive: true,
                            key: key.clone(),
                        });
                        practice_target.time_dead = None;
                        multi
                            .manager_mut("core")
                            .reset_key_with_points("target", &mut commands);
                        multi.manager_mut("light").set_hidden(false, &mut commands);
                    }
                }
                None => {
                    if multi.manager("core").get_key().as_str() == "none" {
                        commands.entity(eid).despawn_recursive();
                    }
                }
            }
        } else {
            // The target is alive
            let killed_by = triggers
                .collisions
                .iter()
                .find(|eid| collisions.get(**eid).unwrap().other_kind == TriggerKind::BulletGood);
            if let Some(killed_by) = killed_by {
                let collision = collisions.get(*killed_by).unwrap();
                status_writer.send(PracticeTargetStatus {
                    bullet_index: collision.other_eid.index(),
                    alive: false,
                    key: practice_target.key.clone(),
                });
                multi
                    .manager_mut("core")
                    .reset_key_with_points("explode", &mut commands);
                multi.manager_mut("light").set_hidden(true, &mut commands);
                practice_target.time_dead = Some(0.0);
                commands.spawn(SoundEffect::universal("sound_effects/fly_spot.ogg", 0.2));
            }
        }
    }
}

pub(super) fn register_practice_targets(app: &mut App) {
    app.add_event::<PracticeTargetStatus>();
    app.add_systems(
        Update,
        update_practice_targets
            .after(PhysicsSet)
            .run_if(in_state(TutorialState::LearnToShoot.to_meta_state())),
    );
}
