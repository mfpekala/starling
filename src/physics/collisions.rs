use crate::prelude::*;

use super::CorePhysicsSet;

#[derive(Component)]
pub struct AnyBullet;

pub(super) fn handle_bullet_collisions(
    mut bullets: Query<
        (
            &mut MultiAnimationManager,
            &StaticReceiver,
            &TriggerReceiver,
        ),
        With<AnyBullet>,
    >,
    mut commands: Commands,
) {
    for (mut multi, static_receiver, trigger_receiver) in &mut bullets {
        if static_receiver.collisions.is_empty() {
            continue;
        }
        match trigger_receiver.kind {
            TriggerKind::BulletGood => {
                if multi.manager("core").get_key().as_str() != "solid" {
                    // We've already started doing stuff to this bullet
                    continue;
                }
                multi.manager_mut("core").reset_key_with_points("explode", &mut commands);
            }
            _ => panic!("Unsupported handle_bullet_collisions kind. How did AnyBullet end up on this component?")
        }
    }
}

pub(super) fn register_collisions(app: &mut App) {
    app.add_systems(
        Update,
        handle_bullet_collisions
            .run_if(in_state(PhysicsState::Active))
            .in_set(PhysicsSet)
            .after(CorePhysicsSet),
    );
}
