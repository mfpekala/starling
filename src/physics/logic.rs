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
    gtran: &GlobalTransform,
    providers: &Query<(Entity, &Bounds, &StaticKind, &GlobalTransform)>,
) {
    for (provider_eid, provider_bounds, provider_kind, provider_gtran) in providers {
        let rhs_tran_n_angle = provider_gtran.tran_n_angle();
        let Some(mvmt) = bounds.get_shape().bounce_off(
            gtran.tran_n_angle(),
            (
                provider_bounds.get_shape(),
                rhs_tran_n_angle.0,
                rhs_tran_n_angle.1,
            ),
        ) else {
            // These things don't overlap, nothing to do
            continue;
        };
        dyno_tran.vel = Vec2::ZERO;
        tran.translation += mvmt.extend(0.0);
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
        let mut amount_moved = 0.0;
        let mut total_to_move = dyno_tran.vel.length() * time_factor;
        while amount_moved < total_to_move {
            // TODO: This is hella inefficient but I just wanna get it working first
            let dir = dyno_tran.vel.normalize_or_zero();
            let mag =
                (dyno_tran.vel.length() * time_factor - amount_moved).min(MAX_TRAN_STEP_LENGTH);
            let moving = dir * mag;
            tran.translation += moving.extend(0.0);
            resolve_static_collisions(bounds, rx, &mut dyno_tran, &mut tran, gtran, &providers);
            // Update the loop stuff
            amount_moved += mag;
            total_to_move = total_to_move.min(dyno_tran.vel.length() * time_factor);
        }
        println!("amount_moved: {amount_moved}, tf: {time_factor}");
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
            .run_if(in_state(PhysicsState::Active)),
    );
}
