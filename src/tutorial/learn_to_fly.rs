use super::fly_spots::*;
use crate::prelude::*;

#[derive(Component, Default, Reflect)]
struct LearnToFlyData {
    no_fly_since_launch: bool,
    time_since_last_warning: f32,
    time_since_last_launch: f32,
    has_passed_first_spot: bool,
    can_show_slow_mo_remark: bool,
    has_shown_slow_mo_remark: bool,
    is_at_least_halfway_done_with_challenge: bool,
    has_shown_exhaustion_warning: bool,
}

fn setup_learn_to_fly(mut commands: Commands, tutorial_root: Res<TutorialRoot>) {
    commands
        .spawn((Name::new("learn_to_fly_data"), LearnToFlyData::default()))
        .set_parent(tutorial_root.eid());
}

fn destroy_learn_to_fly(data: Query<Entity, With<LearnToFlyData>>, mut commands: Commands) {
    for eid in data.iter() {
        commands.entity(eid).despawn_recursive();
    }
}

macro_rules! spawn_fly_spot {
    ($c:expr, $root:expr, $x:expr, $y:expr, $rad:expr, $k: expr) => {{
        $c.spawn(FlySpotBundle::new(
            Vec2::new($x as f32, $y as f32),
            $rad as f32,
            $k,
        ))
        .set_parent($root);
    }};
}

fn spawn_first_fly_spot(mut c: Commands, tutorial_root: Res<TutorialRoot>) {
    let r = tutorial_root.eid();
    // Commands, root_eid, x, y, radius, key
    spawn_fly_spot!(c, r, 127, 66, 12, "first");
}

fn spawn_challenge_fly_spots(mut c: Commands, tutorial_root: Res<TutorialRoot>) {
    let r = tutorial_root.eid();
    // Commands, root_eid, x, y, radius, key
    spawn_fly_spot!(c, r, 35, 23, 8, "challenge_1");
    spawn_fly_spot!(c, r, -54, -13, 8, "challenge_2");
    spawn_fly_spot!(c, r, -114, 51, 6, "challenge_3");
    spawn_fly_spot!(c, r, 100, -60, 6, "challenge_4");
}

fn spawn_challenge_end_fly_spot(mut c: Commands, tutorial_root: Res<TutorialRoot>) {
    // NOTE: Same spot as the first one.
    let r = tutorial_root.eid();
    // Commands, root_eid, x, y, radius, key
    spawn_fly_spot!(c, r, 127, 66, 12, "complete");
}

fn update_data(
    mut data: Query<&mut LearnToFlyData>,
    mut launches: EventReader<Launch>,
    bird: Query<&Bird>,
    mvmt: Res<MovementInput>,
    time: Res<Time>,
    convo_state: Res<State<ConvoState>>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
) {
    let mut data = data.single_mut();
    let bird = bird.single();
    // Update the launch data
    if launches.read().next().is_some() {
        data.no_fly_since_launch = true;
        data.time_since_last_launch = 0.0;
        data.can_show_slow_mo_remark = data.has_passed_first_spot;
    }
    // Track the no flight warning
    data.no_fly_since_launch = data.no_fly_since_launch && mvmt.get_dir().length_squared() < 0.1;
    if convo_state.get() == &ConvoState::None {
        data.time_since_last_warning += time.delta_seconds();
        data.time_since_last_launch += time.delta_seconds();
    }
    // Show the slow motion remark once when it makes sense
    if convo_state.get() == &ConvoState::None
        && data.can_show_slow_mo_remark
        && !data.has_shown_slow_mo_remark
        && data.time_since_last_launch > 0.4
    {
        next_convo_state.set(ConvoState::TutorialLaunchSlowMotionRemark);
        data.has_shown_slow_mo_remark = true;
    }
    // Show the exhaustion warning once it makes sense
    if convo_state.get() == &ConvoState::None
        && data.is_at_least_halfway_done_with_challenge
        && bird.get_launches_left() < 2
        && data.time_since_last_launch > 0.2
        && !data.has_shown_exhaustion_warning
    {
        next_convo_state.set(ConvoState::TutorialLaunchExhaustedWarning);
        data.has_shown_exhaustion_warning = true;
    }
}

fn update_fly_spots(
    mut commands: Commands,
    fly_spots: Query<(Entity, &FlySpot, &TriggerReceiver)>,
    collisions: Query<&TriggerCollisionRecord>,
    convo_state: Res<State<ConvoState>>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
    mut data: Query<&mut LearnToFlyData>,
) {
    let mut data = data.single_mut();
    let num_left = fly_spots.iter().count();
    for (eid, fly_spot, trigger) in &fly_spots {
        if trigger.collisions.iter().all(|tid| {
            collisions
                .get(*tid)
                .ok()
                .map(|thing| thing.other_kind != TriggerKind::Bird)
                .unwrap_or(false)
        }) {
            continue;
        }
        match fly_spot.key.as_str() {
            "first" => {
                commands.entity(eid).despawn_recursive();
                next_convo_state.set(ConvoState::TutorialLaunchChallengeStart);
                data.has_passed_first_spot = true;
            }
            "challenge_1" | "challenge_2" | "challenge_3" | "challenge_4" => {
                if !data.no_fly_since_launch {
                    // Don't spam
                    if data.time_since_last_warning > 3.0 && convo_state.get() == &ConvoState::None
                    {
                        next_convo_state.set(ConvoState::TutorialLaunchFlightWarning);
                        data.time_since_last_warning = 0.0;
                    }
                } else {
                    data.is_at_least_halfway_done_with_challenge = num_left <= 3;
                    commands.entity(eid).despawn_recursive();
                    if num_left <= 1 {
                        next_convo_state.set(ConvoState::TutorialLaunchChallengeCompleted);
                    }
                }
            }
            _ => panic!("bad flyspot"),
        }
    }
}

pub(super) fn register_learn_to_fly(app: &mut App) {
    app.register_type::<LearnToFlyData>();
    app.add_systems(
        OnEnter(TutorialState::LearnFlight.to_meta_state()),
        setup_learn_to_fly,
    );
    app.add_systems(
        OnExit(TutorialState::LearnFlight.to_meta_state()),
        destroy_learn_to_fly,
    );
    app.add_systems(OnExit(ConvoState::TutorialEggUnwrap), spawn_first_fly_spot);
    app.add_systems(
        OnExit(ConvoState::TutorialLaunchChallengeStart),
        spawn_challenge_fly_spots,
    );
    app.add_systems(
        OnExit(ConvoState::TutorialLaunchChallengeCompleted),
        spawn_challenge_end_fly_spot,
    );
    app.add_systems(
        Update,
        (update_data, update_fly_spots)
            .run_if(in_state(TutorialState::LearnFlight.to_meta_state()))
            .after(PhysicsSet),
    );
}
