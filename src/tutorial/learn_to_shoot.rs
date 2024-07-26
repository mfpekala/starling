use tutorial::setup_tutorial;

use super::HelpText;

use super::targets::*;
use crate::prelude::*;

#[derive(Component, Default, Reflect)]
struct LearnToShootData {
    alive_targets: HashSet<String>,
    can_finish_intro_challenge: bool,
    in_speed_challenge: bool,
    all_time_num_speed_targets_killed: u32,
    has_shown_two_birds_help: bool,
    // Maps target key to bullet that most recently killed it eid to show the joke
    killed_by: HashMap<String, u32>,
    has_shown_two_birds_joke: bool,
    help_text: String,
}

fn setup_learn_to_shoot(
    mut commands: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
    mut permanent_skills: ResMut<PermanentSkill>,
    mut ephemeral_skills: ResMut<EphemeralSkill>,
) {
    commands
        .spawn((Name::new("learn_to_fly_data"), LearnToShootData::default()))
        .set_parent(tutorial_root.eid());
    next_convo_state.set(ConvoState::TutorialBulletIntroStart);
    permanent_skills.force_set_num_launches(2);
    ephemeral_skills.start_attempt(&permanent_skills);
}

fn destroy_learn_to_shoot(data: Query<Entity, With<LearnToShootData>>, mut commands: Commands) {
    for eid in data.iter() {
        commands.entity(eid).despawn_recursive();
    }
}

macro_rules! spawn_target {
    ($c:expr, $root:expr, $x:expr, $y:expr, $k:expr, $respawn:expr) => {{
        $c.spawn(PracticeTargetBundle::new(
            Vec2::new($x as f32, $y as f32),
            $k,
            $respawn,
        ))
        .set_parent($root);
    }};
}

fn spawn_intro_challenge(
    mut c: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut data: Query<&mut LearnToShootData>,
    mut permanent_skills: ResMut<PermanentSkill>,
    mut ephemeral_skills: ResMut<EphemeralSkill>,
) {
    let r = tutorial_root.eid();
    // Commands, root_eid, x, y, key, respawn_after
    spawn_target!(c, r, -136, 71, "intro_1", None);
    spawn_target!(c, r, -78, 25, "intro_2", None);
    spawn_target!(c, r, 55, -3, "intro_3", None);
    spawn_target!(c, r, 131, -40, "intro_4", None);
    let mut data = data.single_mut();
    data.can_finish_intro_challenge = true;
    data.alive_targets = HashSet::from_iter([
        "intro_1".to_string(),
        "intro_2".to_string(),
        "intro_3".to_string(),
        "intro_4".to_string(),
    ]);
    data.help_text =
        "Drag and release right mouse to shoot.\nYou recharge 3 bullets every time you land."
            .into();
    permanent_skills.increase_num_bullets(3);
    ephemeral_skills.start_attempt(&permanent_skills);
}

fn spawn_speed_challenge(
    mut c: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut data: Query<&mut LearnToShootData>,
) {
    let r = tutorial_root.eid();
    // Commands, root_eid, x, y, radius, key
    let respawn_time = 2.0;
    spawn_target!(c, r, 35, 23, "speed_1", Some(respawn_time));
    spawn_target!(c, r, -54, -13, "speed_2", Some(respawn_time));
    spawn_target!(c, r, -114, 51, "speed_3", Some(respawn_time));
    spawn_target!(c, r, 100, -60, "speed_4", Some(respawn_time));
    let mut data = data.single_mut();
    data.alive_targets = HashSet::from_iter([
        "speed_1".to_string(),
        "speed_2".to_string(),
        "speed_3".to_string(),
        "speed_4".to_string(),
    ]);
    data.help_text = format!("Targets respawn after {respawn_time}s.\nGet them all!");
}

// fn spawn_challenge_end_fly_spot(mut c: Commands, tutorial_root: Res<TutorialRoot>) {
//     // NOTE: Same spot as the first one.
//     let r = tutorial_root.eid();
//     // Commands, root_eid, x, y, radius, key
//     spawn_fly_spot!(c, r, 127, 66, 12, "complete");
// }

fn update(
    mut data: Query<&mut LearnToShootData>,
    mut text: Query<&mut Text, With<HelpText>>,
    mut status_reader: EventReader<PracticeTargetStatus>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
    target_eids: Query<Entity, With<PracticeTarget>>,
    mut commands: Commands,
) {
    let mut data = data.single_mut();
    // Update the text
    let mut text = text.single_mut();
    text.sections[0].value = data.help_text.clone();
    // Update alive targets
    for update in status_reader.read() {
        if update.alive {
            data.alive_targets.insert(update.key.clone());
        } else {
            data.alive_targets.remove(&update.key);
            if data.in_speed_challenge {
                data.all_time_num_speed_targets_killed += 1;
            }
            if !data.has_shown_two_birds_joke {
                data.killed_by
                    .insert(update.key.clone(), update.bullet_index);
                let kb = data.killed_by.clone();
                let mut seen_entities = HashSet::new();
                for entity in kb.values() {
                    if !seen_entities.insert(entity) {
                        next_convo_state.set(ConvoState::TutorialBulletSpeedTwoBirdsJoke);
                        data.has_shown_two_birds_joke = true;
                    }
                }
            }
        }
    }

    if !data.in_speed_challenge {
        // Not in speed = in intro challenge
        if data.can_finish_intro_challenge && data.alive_targets.is_empty() {
            next_convo_state.set(ConvoState::TutorialBulletSpeedStart);
            data.in_speed_challenge = true;
        }
    } else {
        // In the speed challenge
        if data.alive_targets.len() >= 4
            && data.all_time_num_speed_targets_killed >= 7
            && !data.has_shown_two_birds_help
        {
            next_convo_state.set(ConvoState::TutorialBulletSpeedTwoBirdsHelp);
            data.has_shown_two_birds_help = true;
        }
        if data.alive_targets.len() == 0
            && !target_eids.is_empty()
            && data.all_time_num_speed_targets_killed >= 4
        {
            for eid in &target_eids {
                commands.entity(eid).despawn_recursive();
            }
            next_convo_state.set(ConvoState::TutorialBulletSpeedComplete);
        }
    }
}

fn transition_to_impossible_boss(
    mut next_transition_state: ResMut<NextState<MetaTransitionState>>,
) {
    next_transition_state.set(
        TransitionKind::FadeToBlack
            .to_meta_transition_state(1.0, TutorialState::ImpossibleBoss.to_meta_state()),
    )
}

pub(super) fn register_learn_to_shoot(app: &mut App) {
    app.register_type::<LearnToShootData>();
    super::targets::register_practice_targets(app);

    // High level setup
    app.add_systems(
        OnEnter(TutorialState::LearnToShoot.to_meta_state()),
        setup_learn_to_shoot.after(setup_tutorial),
    );
    app.add_systems(
        OnExit(TutorialState::LearnToShoot.to_meta_state()),
        destroy_learn_to_shoot,
    );
    app.add_systems(
        Update,
        update
            .run_if(in_state(TutorialState::LearnToShoot.to_meta_state()))
            .after(PhysicsSet),
    );

    // Parts of the tutorial
    app.add_systems(
        OnExit(ConvoState::TutorialBulletIntroStart),
        spawn_intro_challenge,
    );
    app.add_systems(
        OnExit(ConvoState::TutorialBulletSpeedStart),
        spawn_speed_challenge,
    );
    app.add_systems(
        OnExit(ConvoState::TutorialBulletSpeedComplete),
        transition_to_impossible_boss,
    );
}
