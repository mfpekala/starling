use crate::prelude::*;

use super::DebugState;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
struct ShowMousePos;
impl ComputedStates for ShowMousePos {
    type SourceStates = (AppMode, DebugState);

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        let (app_mode, debug_state) = sources;
        if matches!(app_mode, AppMode::Prod) {
            return None;
        }
        if debug_state.show_mouse_pos {
            Some(Self)
        } else {
            None
        }
    }
}

fn show_fps(mouse_input: Res<MouseInput>) {
    println!(
        "mouse_pos: ({}, {})",
        mouse_input.get_world_pos().x,
        mouse_input.get_world_pos().y
    );
}

pub(super) fn register_mouse_pos_debug(app: &mut App) {
    app.add_computed_state::<ShowMousePos>();
    app.add_systems(Update, show_fps.run_if(in_state(ShowMousePos)));
}
