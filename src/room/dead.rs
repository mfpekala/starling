use crate::prelude::*;

#[derive(Component, Default)]
struct RoomDeadData {
    time_since_egg_hit_ground: f32,
    has_hatched: bool,
}

fn setup_dead(mut commands: Commands, room_root: Res<RoomRoot>) {
    commands
        .spawn((Name::new("dead_data"), RoomDeadData::default()))
        .set_parent(room_root.eid());
}

fn drop_egg(mut commands: Commands, room_root: Res<RoomRoot>) {
    commands
        .spawn(EggChoice::new(Vec2::new(-110.0, 140.0)))
        .set_parent(room_root.eid());
}

fn spawn_upgrades(commands: &mut Commands, room_root: &RoomRoot) {
    let mut kinds = vec![];
    while kinds.len() < 2 {
        let kind = UpgradeKind::new(0.3, 0.1);
        if kinds.is_empty() || std::mem::discriminant(&kinds[0]) != std::mem::discriminant(&kind) {
            kinds.push(kind);
        }
    }
    UpgradeButtonBundle::spawn(
        1, // cursed
        Vec2::new(-80.0, 0.0),
        kinds[0],
        commands,
        room_root.eid(),
    );
    UpgradeButtonBundle::spawn(2, Vec2::new(80.0, 0.0), kinds[1], commands, room_root.eid());
}

fn update(
    mut falling_egg: Query<
        (
            Entity,
            &mut Transform,
            &mut MultiAnimationManager,
            Option<&Stuck>,
        ),
        With<Egg>,
    >,
    mut commands: Commands,
    mut data: Query<&mut RoomDeadData>,
    time: Res<Time>,
    upgrade_applied: Query<&UpgradeButton, With<UpgradeButtonApplied>>,
    mut next_transition_state: ResMut<NextState<MetaTransitionState>>,
    room_root: Res<RoomRoot>,
) {
    let mut data = data.single_mut();

    if let Ok((egg_id, mut egg_tran, mut multi, stuck)) = falling_egg.get_single_mut() {
        if egg_tran.translation.y.abs() < 50.0 && data.time_since_egg_hit_ground == 0.0 {
            // hacck
            commands
                .entity(egg_id)
                .insert(StaticReceiver::from_kind(StaticReceiverKind::Normal));
        }
        if egg_tran.translation.y < -400.0 {
            // Just in case
            egg_tran.translation.y = 0.0;
        }
        if stuck.is_some() {
            commands.entity(egg_id).remove::<StaticReceiver>();
            commands.entity(egg_id).remove::<DynoTran>();
            commands.entity(egg_id).remove::<Stuck>();
            data.time_since_egg_hit_ground = 0.1;
        }
        // Sketch
        if data.time_since_egg_hit_ground > 0.0 {
            data.time_since_egg_hit_ground += time.delta_seconds();
        }
        if data.time_since_egg_hit_ground > 0.7 {
            // This is like, unbelievably cursed code. Like 4 rounds of hacky changes.
            // I think I could remove it, but it works, so why would I?
            data.time_since_egg_hit_ground = -1.0;
            spawn_upgrades(&mut commands, &room_root);
        }
        if let Ok(upgrade_applied) = upgrade_applied.get_single() {
            // We've applied the upgrade! Yay!
            if !data.has_hatched {
                data.has_hatched = true;
                let multi_key = format!("core{}", upgrade_applied.ix);
                multi.manager_mut(&multi_key).set_key("bird", &mut commands);
                next_transition_state.set(
                    TransitionKind::FadeToBlack
                        .to_meta_transition_state(1.0, RoomState::Dead.next_room().to_meta_state()),
                );
            }
        }
    }
}

pub(super) fn register_dead(app: &mut App) {
    app.add_systems(
        OnEnter(RoomState::Dead.to_meta_state()),
        (setup_dead, drop_egg),
    );
    app.add_systems(
        Update,
        update
            .run_if(in_state(RoomState::Dead.to_meta_state()))
            .after(PhysicsSet),
    );
}
