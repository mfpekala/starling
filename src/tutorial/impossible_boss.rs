use crate::prelude::*;

fn setup_impossible_boss(
    mut commands: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut permanent_skills: ResMut<PermanentSkill>,
    mut ephemeral_skills: ResMut<EphemeralSkill>,
) {
    permanent_skills.force_set_num_launches(2);
    permanent_skills.force_set_num_bullets(3);
    permanent_skills.force_set_max_health(3);
    ephemeral_skills.start_attempt(&permanent_skills);
    commands.entity(tutorial_root.eid()).despawn_descendants();
    commands
        .spawn(StickyPlatformBundle::around_room())
        .set_parent(tutorial_root.eid());
    commands
        .spawn(BirdBundle::new(
            default(),
            default(),
            ephemeral_skills.get_num_launches(),
            ephemeral_skills.get_num_bullets(),
            ephemeral_skills.get_max_health(),
        ))
        .set_parent(tutorial_root.eid());
    SimpBundle::spawn(Vec2::new(0.0, -20.0), &mut commands, tutorial_root.eid());
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
    commands
        .spawn(GhostBundle::new(Vec2::new(143.0, 70.0), true))
        .set_parent(tutorial_root.eid());
}

fn destroy_impossible_boss() {}

fn update_impossible_boss(
    simp_guides: Query<&SimpGuide>,
    mut commands: Commands,
    tutorial_root: Res<TutorialRoot>,
) {
    if simp_guides.is_empty() {
        SimpBundle::spawn(Vec2::new(0.0, -30.0), &mut commands, tutorial_root.eid());
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
            .run_if(in_state(BirdAlive::Yes)),
    );
}
