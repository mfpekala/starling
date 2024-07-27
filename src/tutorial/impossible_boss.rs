use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

use crate::prelude::*;

#[derive(Component, Default)]
struct ImpossibleBossData {
    num_simps_spawned: u32,
    num_simps_killed: u32,
    has_shown_take_damage: bool,
    has_shown_unleash: bool,
    time_until_spawn: Timer,
}

fn setup_impossible_boss(
    mut commands: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut permanent_skills: ResMut<PermanentSkill>,
    mut ephemeral_skills: ResMut<EphemeralSkill>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
) {
    permanent_skills.force_set_num_launches(2);
    permanent_skills.force_set_num_bullets(3);
    permanent_skills.force_set_max_health(3);
    ephemeral_skills.start_attempt(&permanent_skills);
    commands.entity(tutorial_root.eid()).despawn_descendants();
    commands
        .spawn(HardPlatformBundle::around_room())
        .set_parent(tutorial_root.eid());
    commands
        .spawn(BirdBundle::new(
            Vec2::new(-90.0, 50.0),
            default(),
            ephemeral_skills.get_num_launches(),
            ephemeral_skills.get_num_bullets(),
            ephemeral_skills.get_max_health(),
        ))
        .set_parent(tutorial_root.eid());
    // SimpBundle::spawn(Vec2::new(0.0, -20.0), &mut commands, tutorial_root.eid());
    commands
        .spawn(StickyPlatformBundle::new(
            "chain1",
            Vec2::new(-80.0, -30.0),
            Shape::Circle { radius: 25.0 },
        ))
        .insert(DynoRot { rot: 4.0 })
        .set_parent(tutorial_root.eid());
    commands
        .spawn(StickyPlatformBundle::new(
            "chain2",
            Vec2::new(0.0, 30.0),
            Shape::Circle { radius: 25.0 },
        ))
        .insert(DynoRot { rot: -4.0 })
        .set_parent(tutorial_root.eid());
    commands
        .spawn(StickyPlatformBundle::new(
            "chain3",
            Vec2::new(80.0, -30.0),
            Shape::Circle { radius: 25.0 },
        ))
        .insert(DynoRot { rot: 4.0 })
        .set_parent(tutorial_root.eid());
    BackgroundKind::Zenith.spawn(Vec2::ZERO, tutorial_root.eid(), &mut commands);

    commands
        .spawn(GhostBundle::new(Vec2::new(143.0, 70.0), true, true))
        .set_parent(tutorial_root.eid());

    next_convo_state.set(ConvoState::TutorialIntroduceSimp);
    let mut data = ImpossibleBossData::default();
    commands
        .spawn((Name::new("imposible_boss_data"), data))
        .set_parent(tutorial_root.eid());
}

fn destroy_impossible_boss() {}

fn update_impossible_boss(
    simp_guides: Query<&SimpGuide>,
    mut commands: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut bird: Query<&mut Bird>,
    mut data: Query<&mut ImpossibleBossData>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
    ephemeral_skills: ResMut<EphemeralSkill>,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
) {
    let mut data = data.single_mut();
    let mut bird = bird.single_mut();

    // Sketch
    data.num_simps_killed = data.num_simps_spawned - simp_guides.iter().count() as u32;

    if data.num_simps_spawned == 0 {
        SimpBundle::spawn(Vec2::new(0.0, -20.0), &mut commands, tutorial_root.eid());
        data.num_simps_spawned += 1;
        commands
            .spawn(StickyPlatformBundle::new(
                "perch_floor",
                Vec2::new(140.0, 45.0),
                Shape::Polygon {
                    points: simple_rect(40.0, 10.0),
                },
            ))
            .set_parent(tutorial_root.eid());
        commands
            .spawn(StickyPlatformBundle::new(
                "perch_wall",
                Vec2::new(125.0, 65.0),
                Shape::Polygon {
                    points: simple_rect(10.0, 50.0),
                },
            ))
            .set_parent(tutorial_root.eid());
    }

    if data.num_simps_killed == 0 {
        if bird.get_health() < ephemeral_skills.get_max_health() {
            bird.set_health(ephemeral_skills.get_max_health());
        }
    } else {
        if !data.has_shown_unleash {
            next_convo_state.set(ConvoState::TutorialUnleashSimp);
            data.has_shown_unleash = true;
            return;
        }

        if bird.get_taking_damage().is_some() && !data.has_shown_take_damage {
            next_convo_state.set(ConvoState::TutorialTakeDamage);
            data.has_shown_take_damage = true;
            return;
        }

        let time_factor = time.delta_seconds() * bullet_time.factor();
        data.time_until_spawn
            .tick(Duration::from_secs_f32(time_factor));
        if data.time_until_spawn.finished() || simp_guides.iter().count() == 0 {
            let range = IDEAL_VEC_f32 - IDEAL_VEC_f32 * 0.5;
            let mut rng = thread_rng();
            let mut pos = Vec2::ZERO;
            pos.x = rng.gen::<f32>() * range.x * 0.8;
            pos.y = rng.gen::<f32>() * range.y * 0.8;
            SimpBundle::spawn(pos, &mut commands, tutorial_root.eid());
            data.time_until_spawn =
                Timer::from_seconds(15.0 / data.num_simps_killed as f32, TimerMode::Once);
            data.num_simps_spawned += 1;
        }
    }
}

pub(super) fn register_impossible_boss(app: &mut App) {
    app.add_systems(
        OnEnter(TutorialState::ImpossibleBoss.to_meta_state()),
        setup_impossible_boss,
    );
    app.add_systems(
        OnExit(TutorialState::ImpossibleBoss.to_meta_state()),
        destroy_impossible_boss,
    );
    app.add_systems(
        Update,
        update_impossible_boss
            .run_if(in_state(TutorialState::ImpossibleBoss.to_meta_state()))
            .run_if(in_state(BirdAlive::Yes))
            .after(BirdTakeDamageSet),
    );
}
