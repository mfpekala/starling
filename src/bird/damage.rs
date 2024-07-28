use std::time::Duration;

use crate::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BirdTakeDamageSet;

fn take_simp_damage(
    mut birds: Query<(&mut Bird, &TriggerReceiver)>,
    collisions: Query<&TriggerCollisionRecord>,
    irrelevant_simps: Query<Entity, Or<(With<Birthing>, With<Dying>, With<Dead>)>>,
    mut commands: Commands,
) {
    for (mut bird, rx) in &mut birds {
        if bird.taking_damage.is_some() {
            return;
        }
        for cid in rx.collisions.iter() {
            let collision = collisions.get(*cid).unwrap();
            if collision.other_kind != TriggerKind::SimpBody {
                continue;
            }
            if irrelevant_simps.get(collision.other_eid).is_ok() {
                // The simp is either dying or not spawned
                continue;
            }
            bird.taking_damage = Some(Timer::from_seconds(1.0, TimerMode::Once));
            bird.health = bird.health.saturating_sub(1);
            commands.spawn(SoundEffect::universal(
                "sound_effects/lenny_take_damage.ogg",
                0.8,
            ));
            // Fuck it only take damage from simps at most once per frame
            // Wait this actually makes sense
            break;
        }
    }
}

fn update_animation(
    mut birds: Query<(&mut Bird, &mut MultiAnimationManager)>,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
    mut commands: Commands,
) {
    let time_factor = time.delta_seconds() * bullet_time.factor();
    for (mut bird, mut multi) in &mut birds {
        let is_taking_damage = match bird.taking_damage.as_mut() {
            Some(timer) => {
                timer.tick(Duration::from_secs_f32(time_factor));
                !timer.finished()
            }
            None => false,
        };
        if !is_taking_damage {
            bird.taking_damage = None;
        }
        multi.manager_mut("core").set_key(
            if is_taking_damage {
                "taking_damage"
            } else {
                "normal"
            },
            &mut commands,
        );
    }
}

pub(super) fn register_damage(app: &mut App) {
    app.add_systems(
        Update,
        (take_simp_damage, update_animation)
            .run_if(in_state(PhysicsState::Active))
            .run_if(in_state(BirdAlive::Yes))
            .in_set(BirdTakeDamageSet)
            .after(PhysicsSet),
    );
}
