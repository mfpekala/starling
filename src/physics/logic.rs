use crate::prelude::*;

/// When moving `DynoTran`s that have a vel with mag greater than this number, the movement will
/// occur in steps of this length to resolve collisions for fast-moving objects.
const MAX_TRAN_STEP_LENGTH: f32 = 1.0;

/// Resets all records (collisions + triggers (TODO: add trigger support here))
/// TODO: Do something more extensible. The collision records should exist in the world.
///       BUT there should be a fast way to get the collisions associated with a given Eid.
///       Why?
///           - This would enable an ergonomic system that iterates through all collisions to spawn collision sounds, particles, etc
///           - Without this, all of this kind of stuff^ has to iterate over all providers and receivers (bad), even though most of them will not be involved in a collision
///           - I could see a world where when there is an object we spawn a `CollisionRecord` and make these VecDeques the eids of htat spawned record
fn reset_collision_records(
    mut statics_provider_q: Query<&mut StaticProvider>,
    mut statics_receiver_q: Query<&mut StaticReceiver>,
) {
    for mut provider in statics_provider_q.iter_mut() {
        provider.collisions = VecDeque::new();
    }
    for mut receiver in statics_receiver_q.iter_mut() {
        receiver.collisions = VecDeque::new();
    }
}

/// Moves all dynos (both rot and tran) that are not statics, do not collide with statics, and have no triggers
fn move_uninteresting_dynos(
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
    mut rot_only_dynos: Query<
        (&DynoRot, &mut Transform),
        (
            Without<DynoTran>,
            Without<StaticProvider>,
            Without<StaticReceiver>,
            Without<TriggerKind>,
        ),
    >,
    mut both_dynos: Query<
        (&DynoRot, &DynoTran, &mut Transform),
        (
            Without<StaticProvider>,
            Without<StaticReceiver>,
            Without<TriggerKind>,
        ),
    >,
    mut tran_only_dynos: Query<
        (&DynoTran, &mut Transform),
        (
            Without<DynoRot>,
            Without<StaticProvider>,
            Without<StaticReceiver>,
            Without<TriggerKind>,
        ),
    >,
) {
    let time_factor = time.delta_seconds() * bullet_time.factor();
    let apply_rotation = |dyno_rot: &DynoRot, tran: &mut Mut<Transform>| {
        tran.rotate_z(dyno_rot.rot * time_factor);
    };
    let apply_translation = |dyno_tran: &DynoTran, tran: &mut Mut<Transform>| {
        tran.translation += (dyno_tran.vel * time_factor).extend(0.0);
    };
    for (dyno_rot, mut tran) in &mut rot_only_dynos {
        apply_rotation(dyno_rot, &mut tran);
    }
    for (dyno_rot, dyno_tran, mut tran) in &mut both_dynos {
        apply_rotation(dyno_rot, &mut tran);
        apply_translation(dyno_tran, &mut tran);
    }
    for (dyno_tran, mut tran) in &mut tran_only_dynos {
        apply_translation(dyno_tran, &mut tran);
    }
}

/// Moves all dynos (both rot and tran) that are statics. Some may have triggers!
fn move_static_kind_dynos(
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
    mut rot_only_dynos: Query<
        (Option<&TriggerKind>, &DynoRot, &mut Transform),
        (
            Without<DynoTran>,
            With<StaticProvider>,
            Without<StaticReceiver>,
        ),
    >,
    mut both_dynos: Query<
        (Option<&TriggerKind>, &DynoRot, &DynoTran, &mut Transform),
        (With<StaticProvider>, Without<StaticReceiver>),
    >,
    mut tran_only_dynos: Query<
        (Option<&TriggerKind>, &DynoTran, &mut Transform),
        (
            Without<DynoRot>,
            With<StaticProvider>,
            Without<StaticReceiver>,
        ),
    >,
) {
    let time_factor = time.delta_seconds() * bullet_time.factor();
    let apply_rotation = |dyno_rot: &DynoRot, tran: &mut Mut<Transform>| {
        tran.rotate_z(dyno_rot.rot * time_factor);
    };
    let apply_translation = |dyno_tran: &DynoTran, tran: &mut Mut<Transform>| {
        tran.translation += (dyno_tran.vel * time_factor).extend(0.0);
    };
    for (triggers, dyno_rot, mut tran) in &mut rot_only_dynos {
        if triggers.is_some() {
            unimplemented!("Triggers on StaticKind (providers) is not yet supported");
        }
        apply_rotation(dyno_rot, &mut tran);
    }
    for (triggers, dyno_rot, dyno_tran, mut tran) in &mut both_dynos {
        if triggers.is_some() {
            unimplemented!("Triggers on StaticKind (providers) is not yet supported");
        }
        apply_rotation(dyno_rot, &mut tran);
        apply_translation(dyno_tran, &mut tran);
    }
    for (triggers, dyno_tran, mut tran) in &mut tran_only_dynos {
        if triggers.is_some() {
            unimplemented!("Triggers on StaticKind (providers) is not yet supported");
        }
        apply_translation(dyno_tran, &mut tran);
    }
}

fn resolve_static_collisions(
    eid: Entity,
    bounds: &Bounds,
    rx: &mut StaticReceiver,
    dyno_tran: &mut DynoTran,
    tran: &mut Transform,
    gtran_offset: Vec2,
    providers: &mut Query<(Entity, &Bounds, &mut StaticProvider, &GlobalTransform)>,
    commands: &mut Commands,
) {
    for (provider_eid, provider_bounds, mut provider_data, provider_gtran) in providers {
        // Correct the global/local translation and see if there is a collision
        let my_tran_n_angle = tran.tran_n_angle();
        let my_tran_n_angle = (my_tran_n_angle.0 + gtran_offset, my_tran_n_angle.1);
        let rhs_tran_n_angle = provider_gtran.tran_n_angle();
        let Some((mvmt, cp)) = bounds.get_shape().bounce_off(
            my_tran_n_angle,
            (
                provider_bounds.get_shape(),
                rhs_tran_n_angle.0,
                rhs_tran_n_angle.1,
            ),
        ) else {
            // These things don't overlap, nothing to do
            continue;
        };

        // Add the collision records
        let receiver_record = StaticCollisionRecordReceiver {
            pos: cp,
            provider_eid,
            provider_kind: provider_data.kind,
        };
        rx.collisions.push_back(receiver_record);
        let provider_record = StaticCollisionRecordProvider {
            pos: cp,
            receiver_eid: eid,
            receiver_kind: rx.kind,
        };
        provider_data.collisions.push_back(provider_record);

        // Then actually move the objects out of each other and handle physics updates
        tran.translation += mvmt.extend(0.0);
        let bounce_with_friction = |vel: Vec2, springiness: f32, friction: f32| -> Vec2 {
            // TODO: All these normalize_or_zero's are probably a bit slow, fix later
            let old_perp = vel.dot(mvmt.normalize_or_zero()) * mvmt.normalize_or_zero();
            let old_par = vel - old_perp;
            let mut new_perp = old_perp * springiness;
            if new_perp.dot(mvmt) < 0.0 {
                new_perp *= -1.0;
            }
            let friction_mult =
                1.0 + vel.normalize_or_zero().dot(mvmt.normalize_or_zero()).abs() * 10.0;
            let new_par = old_par * (1.0 - (friction * friction_mult).min(1.0));
            new_perp + new_par
        };
        match (provider_data.kind, rx.kind) {
            (_, StaticReceiverKind::Stop) => {
                dyno_tran.vel = Vec2::ZERO;
            }
            (StaticProviderKind::Normal, StaticReceiverKind::Normal) => {
                dyno_tran.vel = bounce_with_friction(dyno_tran.vel, 0.2, 0.03);
            }
            (StaticProviderKind::Sticky, StaticReceiverKind::Normal) => {
                dyno_tran.vel = Vec2::ZERO;
                let stuck_marker = Stuck {
                    parent: provider_eid,
                    my_initial_angle: my_tran_n_angle.1,
                    parent_initial_angle: rhs_tran_n_angle.1,
                    initial_offset: tran.translation.truncate() + gtran_offset - rhs_tran_n_angle.0,
                };
                commands.entity(eid).insert(stuck_marker);
            }
        }
    }
}

/// Moves all dynos (both rot and tran) that receive static collisions and ARE NOT stuck. Some may have triggers!
fn move_unstuck_static_receiver_dynos(
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
    rot_only_dynos: Query<(&StaticReceiver, &DynoRot), (Without<DynoTran>,)>,
    both_dynos: Query<(&StaticReceiver, &DynoRot, &DynoTran)>,
    mut tran_only_dynos: Query<
        (
            Entity,
            &Bounds,
            &mut StaticReceiver,
            Option<&TriggerKind>,
            &mut DynoTran,
            &mut Transform,
            &GlobalTransform,
        ),
        (Without<DynoRot>, Without<StaticProvider>, Without<Stuck>),
    >,
    mut providers: Query<(Entity, &Bounds, &mut StaticProvider, &GlobalTransform)>,
    mut commands: Commands,
) {
    let time_factor = time.delta_seconds() * bullet_time.factor();
    if !rot_only_dynos.is_empty() {
        unimplemented!("DynoRot on StaticReceiver is not yet supported (single)");
    }
    if !both_dynos.is_empty() {
        unimplemented!("DynoRot on StaticReceiver is not yet supported (both)");
    }
    for (eid, bounds, mut rx, _trigger, mut dyno_tran, mut tran, gtran) in &mut tran_only_dynos {
        let gtran_offset = gtran.translation().truncate() - tran.translation.truncate();
        let mut amount_moved = 0.0;
        let mut total_to_move = dyno_tran.vel.length() * time_factor;
        while amount_moved < total_to_move {
            // TODO: This is hella inefficient but I just wanna get it working first
            let dir = dyno_tran.vel.normalize_or_zero();
            let mag =
                (dyno_tran.vel.length() * time_factor - amount_moved).min(MAX_TRAN_STEP_LENGTH);
            let moving = dir * mag;
            tran.translation += moving.extend(0.0);
            resolve_static_collisions(
                eid,
                bounds,
                &mut rx,
                &mut dyno_tran,
                &mut tran,
                gtran_offset,
                &mut providers,
                &mut commands,
            );
            // Update the loop stuff
            amount_moved += MAX_TRAN_STEP_LENGTH;
            total_to_move = total_to_move.min(dyno_tran.vel.length() * time_factor);
        }
    }
}

/// Moves all dynos (both rot and tran) that receive static collisions and ARE stuck. Some may have triggers!
fn move_stuck_static_receiver_dynos(
    mut tran_only_dynos: Query<
        (&Stuck, Option<&TriggerKind>, &mut DynoTran, &mut Transform),
        (
            With<Bounds>,
            With<StaticReceiver>,
            With<DynoTran>,
            Without<DynoRot>,
            Without<StaticProvider>,
        ),
    >,
    providers: Query<&GlobalTransform, (With<Bounds>, With<StaticProvider>)>,
) {
    for (stuck, _trigger, mut dyno_tran, mut tran) in &mut tran_only_dynos {
        let provider_gtran = providers.get(stuck.parent).unwrap();
        dyno_tran.vel = Vec2::ZERO;
        let (provider_tran, provider_angle) = provider_gtran.tran_n_angle();
        let angle_diff = provider_angle - stuck.parent_initial_angle;
        tran.set_angle(stuck.my_initial_angle + angle_diff);
        let rotated_pos = stuck.initial_offset.my_rotate(angle_diff);
        tran.translation.x = provider_tran.x + rotated_pos.x;
        tran.translation.y = provider_tran.y + rotated_pos.y;
    }
}

/// Moves all dynos (both rot and tran) that have triggers but no static interactions (kind or receive)
fn move_trigger_only_dynos() {}

/// Apply gravity to all entities that have `Gravity` and `DynoTran`
fn apply_gravity(
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
    mut dynos: Query<(&mut DynoTran, &Gravity)>,
) {
    let time_factor = time.delta_seconds() * bullet_time.factor();
    for (mut dyno, gravity) in &mut dynos {
        dyno.vel -= Vec2::Y * gravity.strength() * time_factor;
    }
}

pub(super) fn register_logic(app: &mut App) {
    app.add_systems(
        Update,
        (
            reset_collision_records,
            move_uninteresting_dynos,
            move_static_kind_dynos,
            move_unstuck_static_receiver_dynos,
            move_stuck_static_receiver_dynos,
            move_trigger_only_dynos,
            apply_gravity,
        )
            .chain()
            .in_set(PhysicsSet)
            // TODO: Once back on wifi google the proper way to do this
            .after(InputSet)
            .run_if(in_state(PhysicsState::Active)),
    );
}
