use crate::prelude::*;
use bevy::prelude::*;

fn setup_studio() {
    println!("setup_studio");
}

fn destroy_studio() {
    println!("destroy_studio");
}

fn handle_continue(
    trigger: Trigger<NonGameInput>,
    meta_state: Res<State<MetaState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
) {
    let Some(menu_state) = meta_state.get_menu_state() else {
        // We're not in a menu, don't do anything
        return;
    };
    match trigger.event() {
        NonGameInput::Continue => {
            let new_state = match menu_state {
                MenuState::Studio => MenuState::Title.to_meta_state(),
                MenuState::Title => MenuState::Studio.to_meta_state(),
            };
            next_meta_state.set(new_state);
        }
    }
}

pub(super) struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.world_mut().observe(handle_continue);
        app.add_systems(OnEnter(MetaState::Menu(MenuState::Studio)), setup_studio);
        app.add_systems(OnExit(MetaState::Menu(MenuState::Studio)), destroy_studio);
    }
}
