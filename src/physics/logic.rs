use crate::prelude::*;

/// When moving `DynoTran`s that have a vel with mag greater than this number, the movement will
/// occur in steps of this length to resolve collisions for fast-moving objects.
const MAX_TRAN_STEP_LENGTH: f32 = 2.0;

/// Resets all records (collisions + triggers). Happens during PreUpdate
fn reset_collision_records(
    mut statics_provider_q: Query<&mut StaticProvider>,
    mut statics_receiver_q: Query<&mut StaticReceiver>,
    collision_root: Res<CollisionRoot>,
    mut commands: Commands,
) {
    for mut provider in statics_provider_q.iter_mut() {
        provider.collisions = VecDeque::new();
    }
    for mut receiver in statics_receiver_q.iter_mut() {
        receiver.collisions = VecDeque::new();
    }
    commands.entity(collision_root.eid()).despawn_descendants();
}

/// Enforces current limitations in the physics system by panicking if I ever fuck up.
fn enforce_invariants(
    provider_and_receiver: Query<Entity, (With<StaticProvider>, With<StaticReceiver>)>,
    trigger_on_static: Query<Entity, (With<TriggerReceiver>, With<StaticProvider>)>,
    no_bounds: Query<
        Entity,
        (
            Or<(
                With<StaticProvider>,
                With<StaticReceiver>,
                With<TriggerReceiver>,
            )>,
            Without<Bounds>,
        ),
    >,
    no_gtran: Query<
        Entity,
        (
            Or<(
                With<StaticProvider>,
                With<StaticReceiver>,
                With<TriggerReceiver>,
            )>,
            Without<GlobalTransform>,
        ),
    >,
    no_dyno_tran_on_static_receiver: Query<Entity, (With<StaticReceiver>, Without<DynoTran>)>,
    dyno_rot_on_static_receiver: Query<Entity, (With<StaticReceiver>, With<DynoRot>)>,
) {
    if !provider_and_receiver.is_empty() {
        panic!("An entity cannot be both a static provider and a static receiver");
    }
    if !trigger_on_static.is_empty() {
        panic!("Trigger receivers on static providers are not yet supported");
    }
    if !no_bounds.is_empty() {
        panic!("No bounds on a static/trigger");
    }
    if !no_gtran.is_empty() {
        panic!("No global transform on a static/trigger");
    }
    if !no_dyno_tran_on_static_receiver.is_empty() {
        panic!("No dynotran on static receiver (how is it supposed to move?)");
    }
    if !dyno_rot_on_static_receiver.is_empty() {
        panic!("Cannot yet put a dynoRot on a staticreceiver, sorry");
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
            Without<TriggerReceiver>,
        ),
    >,
    mut both_dynos: Query<
        (&DynoRot, &DynoTran, &mut Transform),
        (
            Without<StaticProvider>,
            Without<StaticReceiver>,
            Without<TriggerReceiver>,
        ),
    >,
    mut tran_only_dynos: Query<
        (&DynoTran, &mut Transform),
        (
            Without<DynoRot>,
            Without<StaticProvider>,
            Without<StaticReceiver>,
            Without<TriggerReceiver>,
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

/// Moves all dynos (both rot and tran) that are static providers.
/// NOTE: Trigger support does not yet exist on static providers, i.e. these entities cannot have triggers.
fn move_static_provider_dynos(
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
    mut rot_only_dynos: Query<
        (&DynoRot, &mut Transform),
        (Without<DynoTran>, With<StaticProvider>),
    >,
    mut both_dynos: Query<(&DynoRot, &DynoTran, &mut Transform), With<StaticProvider>>,
    mut tran_only_dynos: Query<
        (Option<&TriggerReceiver>, &DynoTran, &mut Transform),
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
    for (dyno_rot, mut tran) in &mut rot_only_dynos {
        apply_rotation(dyno_rot, &mut tran);
    }
    for (dyno_rot, dyno_tran, mut tran) in &mut both_dynos {
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

/// A helper function to resolve static collisions for a single entity. This will do the work of pushing the
/// entity given by eid outside of other entities it's colliding with
fn resolve_static_collisions(
    eid: Entity,
    bounds: &Bounds,
    rx: &mut StaticReceiver,
    dyno_tran: &mut DynoTran,
    tran: &mut Transform,
    gtran_offset: Vec2,
    providers: &mut Query<(Entity, &Bounds, &mut StaticProvider, &GlobalTransform)>,
    commands: &mut Commands,
    collision_root: &CollisionRoot,
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

        // Create a collision record
        let collision_record = StaticCollisionRecord {
            pos: cp,
            provider_eid,
            provider_kind: provider_data.kind,
            receiver_eid: eid,
            receiver_kind: rx.kind,
        };
        let collision_eid = commands
            .spawn(StaticCollisionBundle::new(collision_record))
            .set_parent(collision_root.eid())
            .id();
        rx.collisions.push_back(collision_eid);
        provider_data.collisions.push_back(collision_eid);

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

/// Resolves trigger collisions. Note that the data is broken up into multiple queries to allow for
/// proper handling in the parent systems.
///
/// I actually believe this has a slight bug. It always uses global transform, which is static all frame.
/// I.e. if bullet a moves and then bullet b goes it will still be checking against bullet a old pos.
/// Ehh probably fine
fn resolve_trigger_collisions(
    eid: Entity,
    bounds: &Bounds,
    rx: &mut TriggerReceiver,
    gtran: &Transform,
    shared_data: &Query<(Entity, &Bounds, &GlobalTransform)>,
    trigger_data: &mut Query<(Entity, &mut TriggerReceiver)>,
    commands: &mut Commands,
    collision_root: &CollisionRoot,
    dup_set: &mut HashSet<(Entity, Entity)>,
) {
    for (other_eid, mut other_rx) in trigger_data {
        if other_eid == eid {
            // You can't collide with your own trigger, idiot
            continue;
        }
        let my_tran_n_angle = gtran.tran_n_angle();
        let (_, other_bounds, other_gtran) = shared_data.get(other_eid).unwrap();
        let rhs_tran_n_angle = other_gtran.tran_n_angle();
        let Some((_, cp)) = bounds.get_shape().bounce_off(
            my_tran_n_angle,
            (
                other_bounds.get_shape(),
                rhs_tran_n_angle.0,
                rhs_tran_n_angle.1,
            ),
        ) else {
            // These things don't overlap, nothing to do
            continue;
        };
        // Create collision records (NOTE: It's symmetric, one for each, and we don't dup)
        if !dup_set.contains(&(eid, other_eid)) {
            let my_collision_record = TriggerCollisionRecord {
                pos: cp,
                other_eid,
                other_kind: other_rx.kind.clone(),
            };
            let my_collision_eid = commands
                .spawn(TriggerCollisionBundle::new(my_collision_record))
                .set_parent(collision_root.eid())
                .id();
            rx.collisions.push_back(my_collision_eid);
        }
        if !dup_set.contains(&(other_eid, eid)) {
            let other_collision_record = TriggerCollisionRecord {
                pos: cp,
                other_eid: eid,
                other_kind: rx.kind.clone(),
            };
            let other_collision_eid = commands
                .spawn(TriggerCollisionBundle::new(other_collision_record))
                .set_parent(collision_root.eid())
                .id();
            other_rx.collisions.push_back(other_collision_eid);
        }
    }
}

/// Handles moving all unstuck dynos that have _either_ a staticreceiver or a triggerreceiver
fn move_unstuck_static_or_trigger_receivers(
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
    relevant_eids: Query<
        Entity,
        (
            Or<(With<StaticReceiver>, With<TriggerReceiver>)>,
            Without<Stuck>,
            Or<(With<DynoTran>, With<DynoRot>)>,
        ),
    >,
    shared_data: Query<(Entity, &Bounds, &GlobalTransform)>,
    mut dyno_data: Query<
        (
            Entity,
            Option<&mut DynoTran>,
            Option<&mut DynoRot>,
            &mut Transform,
        ),
        Or<(With<DynoTran>, With<DynoRot>)>,
    >,
    mut static_data: Query<(Entity, &mut StaticReceiver), Without<Stuck>>,
    mut trigger_data: Query<(Entity, &mut TriggerReceiver)>,
    mut static_providers: Query<(Entity, &Bounds, &mut StaticProvider, &GlobalTransform)>,
    mut commands: Commands,
    collision_root: Res<CollisionRoot>,
) {
    let time_factor = time.delta_seconds() * bullet_time.factor();
    for eid in &relevant_eids {
        // Shared data (immutable)
        let (_, my_bounds, my_gtran) = shared_data.get(eid).unwrap();
        let my_bounds = my_bounds.clone();
        let my_gtran = my_gtran.clone();

        // Mutable dyno data (need to mutate and then assign at end)
        let (_, my_dyno_tran, my_dyno_rot, my_tran) = dyno_data.get(eid).unwrap();
        let mut my_dyno_tran = my_dyno_tran.map(|inner| inner.clone());
        let mut my_dyno_rot = my_dyno_rot.map(|inner| inner.clone());
        let mut my_tran = my_tran.clone();
        let my_gtran_offset = my_gtran.translation().truncate() - my_tran.translation.truncate();

        // Mutable static data (need to mutate and then assign at end)
        let mut my_static = static_data.get(eid).ok().map(|inner| inner.1.clone());

        // Mutable trigger data (need to mutate and then assign at end)
        let mut my_trigger = trigger_data.get(eid).ok().map(|inner| inner.1.clone());
        let mut dup_set = HashSet::<(Entity, Entity)>::new();

        // If we have rotational movement, rotate first
        if let Some(my_dyno_rot) = my_dyno_rot.as_mut() {
            my_tran.rotate_z(my_dyno_rot.rot * time_factor);
        }

        // If we have translational movement, inch along
        if let Some(mut my_dyno_tran) = my_dyno_tran.as_mut() {
            let mut amount_moved = 0.0;
            let mut total_to_move = my_dyno_tran.vel.length() * time_factor;
            while amount_moved < total_to_move {
                // TODO: This is hella inefficient but I just wanna get it working first
                let dir = my_dyno_tran.vel.normalize_or_zero();
                let mag = (my_dyno_tran.vel.length() * time_factor - amount_moved)
                    .min(MAX_TRAN_STEP_LENGTH);
                let moving = dir * mag;
                my_tran.translation += moving.extend(0.0);
                if let Some(mut my_static_rx) = my_static.as_mut() {
                    resolve_static_collisions(
                        eid,
                        &my_bounds,
                        &mut my_static_rx,
                        &mut my_dyno_tran,
                        &mut my_tran,
                        my_gtran_offset,
                        &mut static_providers,
                        &mut commands,
                        &collision_root,
                    );
                }
                if let Some(my_trigger_rx) = my_trigger.as_mut() {
                    // Basically because GlobalTransform doesn't update mid-system we need to do this shenanigans
                    let mut mid_step_gtran = my_tran.clone();
                    mid_step_gtran.translation += my_gtran_offset.extend(0.0);
                    resolve_trigger_collisions(
                        eid,
                        &my_bounds,
                        my_trigger_rx,
                        &mid_step_gtran,
                        &shared_data,
                        &mut trigger_data,
                        &mut commands,
                        &collision_root,
                        &mut dup_set,
                    );
                }
                // Update the loop stuff
                amount_moved += MAX_TRAN_STEP_LENGTH;
                total_to_move = total_to_move.min(my_dyno_tran.vel.length() * time_factor);
            }
        } else {
            // We're not translating, resolve triggers once to be sure;
            if let Some(my_trigger_rx) = my_trigger.as_mut() {
                // Basically because GlobalTransform doesn't update mid-system we need to do this shenanigans
                let mut mid_step_gtran = my_tran.clone();
                mid_step_gtran.translation += my_gtran_offset.extend(0.0);
                resolve_trigger_collisions(
                    eid,
                    &my_bounds,
                    my_trigger_rx,
                    &mid_step_gtran,
                    &shared_data,
                    &mut trigger_data,
                    &mut commands,
                    &collision_root,
                    &mut dup_set,
                );
            }
        }

        let (_, reset_dyno_tran, reset_dyno_rot, mut reset_tran) = dyno_data.get_mut(eid).unwrap();
        if let Some(mut reset_dyno_tran) = reset_dyno_tran {
            *reset_dyno_tran = my_dyno_tran.unwrap();
        }
        if let Some(mut reset_dyno_rot) = reset_dyno_rot {
            *reset_dyno_rot = my_dyno_rot.unwrap();
        }
        *reset_tran = my_tran;

        let reset_rx = static_data.get_mut(eid).ok().map(|inner| inner.1);
        if let Some(mut reset_rx) = reset_rx {
            *reset_rx = my_static.unwrap();
        }

        let reset_rx = trigger_data.get_mut(eid).ok().map(|inner| inner.1);
        if let Some(mut reset_rx) = reset_rx {
            *reset_rx = my_trigger.unwrap();
        }
    }
}

/// Moves all dynos (both rot and tran) that receive static collisions and ARE stuck. Some may have triggers!
fn move_stuck_static_receiver_dynos(
    mut tran_only_dynos: Query<
        (
            &Stuck,
            Option<&TriggerReceiver>,
            &mut DynoTran,
            &mut Transform,
        ),
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
    // Reset collisions during preupdate
    app.add_systems(
        PreUpdate,
        reset_collision_records
            .in_set(PhysicsSet)
            .run_if(in_state(PhysicsState::Active)),
    );
    // Enforce invariants during update when in dev mode
    app.add_systems(
        Update,
        enforce_invariants
            .in_set(PhysicsSet)
            .run_if(in_state(PhysicsState::Active))
            .run_if(in_state(AppMode::Dev)),
    );
    // Physics yay!
    app.add_systems(
        Update,
        (
            move_uninteresting_dynos,
            move_static_provider_dynos,
            move_unstuck_static_or_trigger_receivers,
            move_stuck_static_receiver_dynos,
            move_trigger_only_dynos,
            apply_gravity,
        )
            .chain()
            .in_set(PhysicsSet)
            .after(InputSet)
            .run_if(in_state(PhysicsState::Active)),
    );
}
