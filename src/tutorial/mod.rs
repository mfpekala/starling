use background::BackgroundKind;
use platforms::StickyPlatformBundle;

use crate::prelude::*;

/// Sets up the tutorial.
/// NOTE: For simplicity, you can't save game mid tutorial, i.e. we are assuming that
/// the state transitions will always go:
///   (non-tutorial) -> TutorialState::LearnFlight -> TutorialState::LearnShoot -> (non-tutorial)
/// This means setup_tutorial only happens OnEnter(MetaState::Tutorial(TutorialState::LearnFlight))
/// TODO: Can probably fix this with another computedstate (AnyTutorial) but not bothering rn
fn setup_tutorial(
    mut commands: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
) {
    next_convo_state.set(ConvoState::TutorialEggUnwrap);
    commands
        .spawn(StickyPlatformBundle::around_room())
        .set_parent(tutorial_root.eid());
    commands
        .spawn(StickyPlatformBundle::new(
            "chain1",
            Vec2::new(-90.0, -30.0),
            Shape::Circle { radius: 15.0 },
        ))
        .insert(DynoRot { rot: 2.0 })
        .set_parent(tutorial_root.eid());
    commands
        .spawn(StickyPlatformBundle::new(
            "chain2",
            Vec2::new(-10.0, 10.0),
            Shape::Circle { radius: 25.0 },
        ))
        .insert(DynoRot { rot: -4.0 })
        .set_parent(tutorial_root.eid());
    commands
        .spawn(StickyPlatformBundle::new(
            "chain3",
            Vec2::new(70.0, 35.0),
            Shape::Circle { radius: 15.0 },
        ))
        .insert(DynoRot { rot: 2.0 })
        .set_parent(tutorial_root.eid());
    commands
        .spawn(StickyPlatformBundle::new(
            "perch",
            Vec2::new(140.0, 45.0),
            Shape::Polygon {
                points: simple_rect(40.0, 10.0),
            },
        ))
        .set_parent(tutorial_root.eid());
    commands
        .spawn(BirdBundle::new(Vec2::new(-125.0, -80.0), Vec2::ZERO, 1, 1))
        .set_parent(tutorial_root.eid());
    BackgroundKind::Zenith.spawn(Vec2::ZERO, tutorial_root.eid(), &mut commands);
}

/// Sets up the tutorial.
/// NOTE: For simplicity, you can't save game mid tutorial, i.e. we are assuming that
/// the state transitions will always go:
///   (non-tutorial) -> TutorialState::LearnFlight -> TutorialState::LearnShoot -> (non-tutorial)
/// This means destroy_tutorial only happens OnExit(MetaState::Tutorial(TutorialState::LearnShoot))
fn destroy_tutorial(mut commands: Commands, tutorial_root: Res<TutorialRoot>) {
    commands.entity(tutorial_root.eid()).despawn_descendants();
}

pub(super) struct TutorialPlugin;
impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(TutorialState::LearnFlight.to_meta_state()),
            setup_tutorial,
        );
        app.add_systems(
            OnExit(TutorialState::LearnShooting.to_meta_state()),
            destroy_tutorial,
        );
    }
}
