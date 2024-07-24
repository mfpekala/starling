use crate::prelude::*;

use super::DebugState;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
struct ShowCommands;
impl ComputedStates for ShowCommands {
    type SourceStates = (AppMode, DebugState);

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        let (app_mode, debug_state) = sources;
        if matches!(app_mode, AppMode::Prod) {
            return None;
        }
        if debug_state.show_commands {
            Some(Self)
        } else {
            None
        }
    }
}

fn setup_debug_commands() {}

fn update_debug_commands() {}

fn destroy_debug_commands() {}

pub(super) fn register_commands_debug(app: &mut App) {
    app.add_computed_state::<ShowCommands>();
    app.add_systems(OnEnter(ShowCommands), setup_debug_commands);
    app.add_systems(Update, update_debug_commands.run_if(in_state(ShowCommands)));
    app.add_systems(OnExit(ShowCommands), destroy_debug_commands);
}
