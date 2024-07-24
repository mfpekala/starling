use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

use crate::prelude::*;

use super::DebugState;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
struct ShowFps;
impl ComputedStates for ShowFps {
    type SourceStates = (AppMode, DebugState);

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        let (app_mode, debug_state) = sources;
        if matches!(app_mode, AppMode::Prod) {
            return None;
        }
        if debug_state.show_fps {
            Some(Self)
        } else {
            None
        }
    }
}

fn show_fps(diagnostics_store: Res<DiagnosticsStore>) {
    let test = diagnostics_store
        .get_measurement(&FrameTimeDiagnosticsPlugin::FPS)
        .map(|thing| thing.value)
        .unwrap_or(-1.0);
    println!("fps: {test}");
}

pub(super) fn register_fps_debug(app: &mut App) {
    app.add_computed_state::<ShowFps>();
    app.add_systems(Update, show_fps.run_if(in_state(ShowFps)));
}
