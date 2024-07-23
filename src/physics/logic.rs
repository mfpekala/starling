use crate::prelude::*;

/// When moving `DynoTran`s that have a vel with mag greater than this number, the movement will
/// occur in steps of this length to resolve collisions for fast-moving objects.
const MAX_TRAN_STEP_LENGTH: f32 = 1.0;

/// Moves all dynos (both rot and tran) that are not statics, do not collide with statics, and have no triggers
fn move_uninteresting_dynos(
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
    mut rot_only_dynos: Query<
        (&DynoRot, &mut Transform),
        (
            Without<DynoTran>,
            Without<StaticKind>,
            Without<StaticReceiver>,
            Without<TriggerKind>,
        ),
    >,
    mut both_dynos: Query<
        (&DynoRot, &DynoTran, &mut Transform),
        (
            Without<StaticKind>,
            Without<StaticReceiver>,
            Without<TriggerKind>,
        ),
    >,
    mut tran_only_dynos: Query<
        (&DynoTran, &mut Transform),
        (
            Without<DynoRot>,
            Without<StaticKind>,
            Without<StaticReceiver>,
            Without<TriggerKind>,
        ),
    >,
) {
    let time_factor = time.delta_seconds() * bullet_time.factor();
    let apply_rotation = |dyno_rot: &DynoRot, tran: &mut Mut<Transform>| {
        tran.rotate_z(dyno_rot.rot);
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
        (Without<DynoTran>, With<StaticKind>, Without<StaticReceiver>),
    >,
    mut both_dynos: Query<
        (Option<&TriggerKind>, &DynoRot, &DynoTran, &mut Transform),
        (With<StaticKind>, Without<StaticReceiver>),
    >,
    mut tran_only_dynos: Query<
        (Option<&TriggerKind>, &DynoTran, &mut Transform),
        (Without<DynoRot>, With<StaticKind>, Without<StaticReceiver>),
    >,
) {
    let time_factor = time.delta_seconds() * bullet_time.factor();
    let apply_rotation = |dyno_rot: &DynoRot, tran: &mut Mut<Transform>| {
        tran.rotate_z(dyno_rot.rot);
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
        unimplemented!("Having both a rotation and a translation on a static is not yet supported");
    }
    for (triggers, dyno_tran, mut tran) in &mut tran_only_dynos {
        if triggers.is_some() {
            unimplemented!("Triggers on StaticKind (providers) is not yet supported");
        }
        apply_translation(dyno_tran, &mut tran);
    }
}

fn resolve_static_collisions(
    bounds: &Bounds,
    rx: &StaticReceiver,
    dyno_tran: &mut DynoTran,
    tran: &mut Transform,
    gtran_offset: Vec2,
    providers: &Query<(Entity, &Bounds, &StaticKind, &GlobalTransform)>,
) {
    for (provider_eid, provider_bounds, provider_kind, provider_gtran) in providers {
        let my_tran_n_angle = tran.tran_n_angle();
        let my_tran_n_angle = (my_tran_n_angle.0 + gtran_offset, my_tran_n_angle.1);
        let rhs_tran_n_angle = provider_gtran.tran_n_angle();
        let Some(mvmt) = bounds.get_shape().bounce_off(
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
        tran.translation += mvmt.extend(0.0);
        // TODO: It should only apply friction once per frame
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
        match (provider_kind, rx) {
            (_, StaticReceiver::Stop) => {
                dyno_tran.vel = Vec2::ZERO;
            }
            (_, StaticReceiver::Normal) => {
                dyno_tran.vel = bounce_with_friction(dyno_tran.vel, 0.2, 0.03);
            }
        }
    }
}

/// Moves all dynos (both rot and tran) that receive static collisions. Some may have triggers!
fn move_static_receiver_dynos(
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
    rot_only_dynos: Query<(&StaticReceiver, &DynoRot), (Without<DynoTran>,)>,
    both_dynos: Query<(&StaticReceiver, &DynoRot, &DynoTran)>,
    mut tran_only_dynos: Query<
        (
            &Bounds,
            &StaticReceiver,
            Option<&TriggerKind>,
            &mut DynoTran,
            &mut Transform,
            &GlobalTransform,
        ),
        (Without<DynoRot>, Without<StaticKind>),
    >,
    providers: Query<(Entity, &Bounds, &StaticKind, &GlobalTransform)>,
) {
    let time_factor = time.delta_seconds() * bullet_time.factor();
    if !rot_only_dynos.is_empty() {
        unimplemented!("DynoRot on StaticReceiver is not yet supported (single)");
    }
    if !both_dynos.is_empty() {
        unimplemented!("DynoRot on StaticReceiver is not yet supported (both)");
    }
    for (bounds, rx, trigger, mut dyno_tran, mut tran, gtran) in &mut tran_only_dynos {
        // TODO: Not sure if this offset is buggy, need to test it on something nested
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
                bounds,
                rx,
                &mut dyno_tran,
                &mut tran,
                gtran_offset,
                &providers,
            );
            // Update the loop stuff
            amount_moved += MAX_TRAN_STEP_LENGTH;
            total_to_move = total_to_move.min(dyno_tran.vel.length() * time_factor);
        }
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
            move_uninteresting_dynos,
            move_static_kind_dynos,
            move_static_receiver_dynos,
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
