use tutorial::setup_tutorial;

use super::HelpText;

use super::fly_spots::*;
use crate::prelude::*;

#[derive(Component, Default, Reflect)]
struct LearnToFlyData {
    ever_could_launch: bool,
    no_fly_since_launch: bool,
    time_since_last_warning: f32,
    time_since_last_launch: f32,
    has_passed_first_spot: bool,
    can_show_slow_mo_remark: bool,
    has_shown_slow_mo_remark: bool,
    is_at_least_three_fourths_done_with_challenge: bool,
    has_shown_exhaustion_warning: bool,
    help_text: String,
}

fn setup_learn_to_fly(mut commands: Commands, tutorial_root: Res<TutorialRoot>) {
    let mut data = LearnToFlyData::default();
    data.help_text = String::from("Press WASD, Space, or either mouse\nbutton to advance dialogue");
    commands
        .spawn((Name::new("learn_to_fly_data"), data))
        .set_parent(tutorial_root.eid());
}

fn destroy_learn_to_fly(
    data: Query<Entity, Or<(With<LearnToFlyData>, With<FlySpot>)>>,
    mut commands: Commands,
) {
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

fn spawn_first_fly_spot(
    mut c: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut data: Query<&mut LearnToFlyData>,
) {
    if let Ok(mut data) = data.get_single_mut() {
        // Wrap in an if because we might use our dev shortcut to skip LearnToFly
        data.help_text = "Use WASD or Arrow Keys to Fly".into();
        let r = tutorial_root.eid();
        // Commands, root_eid, x, y, radius, key
        spawn_fly_spot!(c, r, 127, 66, 12, "first");
    }
}

fn spawn_challenge_fly_spots(
    mut c: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut data: Query<&mut LearnToFlyData>,
    mut permanent_skills: ResMut<PermanentSkill>,
    mut ephemeral_skills: ResMut<EphemeralSkill>,
) {
    let r = tutorial_root.eid();
    // Commands, root_eid, x, y, radius, key
    spawn_fly_spot!(c, r, 35, 23, 8, "challenge_1");
    spawn_fly_spot!(c, r, -54, -13, 8, "challenge_2");
    spawn_fly_spot!(c, r, -114, 51, 6, "challenge_3");
    spawn_fly_spot!(c, r, 100, -60, 6, "challenge_4");
    let mut data = data.single_mut();
    data.help_text =
        "Drag and release left mouse to launch!\nTo recharge, fly into a sticky (pink) object."
            .into();
    permanent_skills.increase_num_launches(2);
    ephemeral_skills.start_attempt(&permanent_skills);
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
    mut text: Query<&mut Text, With<HelpText>>,
) {
    let mut data = data.single_mut();
    let bird = bird.single();
    data.ever_could_launch = bird.get_launches_left() > 0;
    // Update the text
    let mut text = text.single_mut();
    text.sections[0].value = data.help_text.clone();
    // Update the launch data
    if launches.read().next().is_some() {
        data.time_since_last_launch = 0.0;
        if data.ever_could_launch {
            data.no_fly_since_launch = true;
            data.can_show_slow_mo_remark = data.has_passed_first_spot;
        }
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
        && data.is_at_least_three_fourths_done_with_challenge
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
    mut data: Query<&mut LearnToFlyData>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Backspace) {
        next_convo_state.set(ConvoState::None);
        next_meta_state.set(TutorialState::LearnToShoot.to_meta_state());
        return;
    }
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
                data.help_text = String::new();
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
                    data.is_at_least_three_fourths_done_with_challenge = num_left <= 2;
                    if num_left <= 3 {
                        data.help_text = String::new();
                    }
                    commands.entity(eid).despawn_recursive();
                    if num_left <= 1 {
                        next_convo_state.set(ConvoState::TutorialLaunchChallengeCompleted);
                    }
                }
            }
            "complete" => {
                commands.entity(eid).despawn_recursive();
                next_meta_state.set(TutorialState::LearnToShoot.to_meta_state());
            }
            _ => panic!("bad flyspot"),
        }
    }
}

pub(super) fn register_learn_to_fly(app: &mut App) {
    app.register_type::<LearnToFlyData>();
    app.add_systems(
        OnEnter(TutorialState::LearnToFly.to_meta_state()),
        setup_learn_to_fly.after(setup_tutorial),
    );
    app.add_systems(
        OnExit(TutorialState::LearnToFly.to_meta_state()),
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
            .run_if(in_state(TutorialState::LearnToFly.to_meta_state()))
            .after(PhysicsSet),
    );
}
