use bevy::prelude::*;

// Any place in the app that needs to react to input should use these events and resources
// I've never actually done this but this should _hopefully_ make it easy to plug in controller support

/// Pretty wide net, but should cover all input in menus, cutscenes, pause screens, convos...
#[derive(Event)]
pub enum NonGameInput {
    Continue,
}

// INTERNAL INPUT SYSTEM (ONLY USED IN THIS FILE)

/// Send any and all non-game input. Note the early returns, we only handle at most one
/// such input per frame
fn watch_non_game_input(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Enter) {
        commands.trigger(NonGameInput::Continue);
        return;
    }
}

pub(super) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NonGameInput>();

        app.add_systems(Update, watch_non_game_input);
    }
}
