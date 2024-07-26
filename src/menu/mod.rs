use crate::prelude::*;

mod studio;
mod title;

fn handle_continue(
    trigger: Trigger<NonGameInput>,
    meta_state: Res<State<MetaState>>,
    transition_state: Res<State<MetaTransitionState>>,
    mut next_transition_state: ResMut<NextState<MetaTransitionState>>,
) {
    let Some(menu_state) = meta_state.get_menu_state() else {
        // We're not in a menu, don't do anything
        return;
    };
    if !matches!(transition_state.get(), MetaTransitionState::Stable) {
        // There's already a transition happening, wait
        return;
    }
    match trigger.event() {
        NonGameInput::Continue => {
            let new_state = match menu_state {
                MenuState::Studio => MenuState::Title.to_meta_state(),
                MenuState::Title => TutorialState::LearnToFly.to_meta_state(),
            };
            next_transition_state
                .set(TransitionKind::FadeToBlack.to_meta_transition_state(1.0, new_state));
        }
    }
}

pub(super) struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.world_mut().observe(handle_continue);
        app.add_systems(
            OnEnter(MetaState::Menu(MenuState::Studio)),
            studio::setup_studio,
        );
        app.add_systems(
            OnExit(MetaState::Menu(MenuState::Studio)),
            studio::destroy_studio,
        );
    }
}
