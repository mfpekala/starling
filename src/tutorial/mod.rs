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
