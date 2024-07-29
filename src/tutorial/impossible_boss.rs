use rand::Rng;

use crate::prelude::*;

#[derive(Component, Default)]
struct ImpossibleBossData {
    has_spawned_first_simp: bool,
    has_killed_first_simp: bool,
    has_shown_take_damage: bool,
    has_shown_unleash: bool,
}

fn setup_impossible_boss(
    mut commands: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut permanent_skills: ResMut<PermanentSkill>,
    mut ephemeral_skills: ResMut<EphemeralSkill>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
    mut music_manager: ResMut<MusicManager>,
) {
    music_manager.fade_to_song(MusicKind::BossBattle);
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
            0,
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
    let data = ImpossibleBossData::default();
    commands
        .spawn((Name::new("imposible_boss_data"), data))
        .set_parent(tutorial_root.eid());
}

fn destroy_impossible_boss() {}

fn update_impossible_boss(
    simp_guides: Query<&SimpGuide>,
    mut commands: Commands,
    tutorial_root: Res<TutorialRoot>,
    bird: Query<&Bird>,
    mut data: Query<&mut ImpossibleBossData>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
    mut ephemeral_skills: ResMut<EphemeralSkill>,
    mut simp_spawner: Query<&mut EnemySpawner<SimpBundle>>,
) {
    let mut data = data.single_mut();
    let bird = bird.single();

    if !data.has_killed_first_simp {
        data.has_killed_first_simp = data.has_spawned_first_simp && simp_guides.is_empty();
    }

    if !data.has_spawned_first_simp {
        SimpBundle::spawn(Vec2::new(0.0, -20.0), &mut commands, tutorial_root.eid());
        data.has_spawned_first_simp = true;
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

    if !data.has_killed_first_simp {
        // Never take damage!!!
        ephemeral_skills.inc_current_health(3);
    } else {
        if !data.has_shown_unleash {
            next_convo_state.set(ConvoState::TutorialUnleashSimp);
            data.has_shown_unleash = true;
            let batch_size_range = 3..10;
            let mut batch_sizes = vec![];
            let mut unaccounted_for = 30;
            while unaccounted_for > 0 {
                let batch_size = rand::thread_rng().gen_range(batch_size_range.clone());
                let batch_size = batch_size.min(unaccounted_for);
                batch_sizes.push(batch_size);
                unaccounted_for -= batch_size;
            }
            let spawner_placements = vec![Vec2::new(-62.0, 62.0), Vec2::new(0.0, -50.0)];
            commands
                .spawn(EnemySpawnerBundle::<SimpBundle>::new(
                    spawner_placements,
                    batch_sizes,
                ))
                .set_parent(tutorial_root.eid());
            return;
        }

        if bird.get_taking_damage().is_some() && !data.has_shown_take_damage {
            next_convo_state.set(ConvoState::TutorialTakeDamage);
            data.has_shown_take_damage = true;
            return;
        }
    }

    // These spawners should be infinit
    for mut spawner in &mut simp_spawner {
        if spawner.batch_sizes.len() < 5 {
            spawner.batch_sizes.push(100);
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
