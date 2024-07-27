use background::BackgroundKind;
use platforms::StickyPlatformBundle;

use crate::prelude::*;

fn setup_impossible_boss(
    mut commands: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut permanent_skills: ResMut<PermanentSkill>,
    mut ephemeral_skills: ResMut<EphemeralSkill>,
) {
    permanent_skills.force_set_num_launches(2);
    permanent_skills.force_set_num_bullets(3);
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
        ))
        .set_parent(tutorial_root.eid());
    SimpBundle::spawn(Vec2::ZERO, &mut commands, tutorial_root.eid());
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
}

fn destroy_impossible_boss() {}

pub(super) fn register_impossible_boss(app: &mut App) {
    app.add_systems(
        OnEnter(TutorialState::ImpossibleBoss.to_meta_state()),
        setup_impossible_boss,
    );
    app.add_systems(
        OnExit(TutorialState::ImpossibleBoss.to_meta_state()),
        destroy_impossible_boss,
    );
}
