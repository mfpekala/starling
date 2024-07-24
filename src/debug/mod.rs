use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::prelude::*;

mod commands;
mod fps;
mod physics;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
struct DebugState {
    pub show_commands: bool,
    pub show_fps: bool,
    pub show_physics_bounds: bool,
}
impl Default for DebugState {
    fn default() -> Self {
        Self {
            show_commands: true,
            show_fps: false,
            show_physics_bounds: true,
        }
    }
}

#[derive(Resource, Reflect)]
struct DebugInteractive(DebugState);
fn update_debug_state(
    interactive_state: Res<DebugInteractive>,
    debug_state: Res<State<DebugState>>,
    mut next_debug_state: ResMut<NextState<DebugState>>,
) {
    if &interactive_state.0 != debug_state.get() {
        next_debug_state.set(interactive_state.0.clone());
    }
}

pub(super) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.insert_state(DebugState::default());
        commands::register_commands_debug(app);
        fps::register_fps_debug(app);
        physics::register_physics_debug(app);

        // Debug
        app.insert_resource(DebugInteractive(DebugState::default()));
        app.add_plugins(ResourceInspectorPlugin::<DebugInteractive>::new());
        app.add_systems(Update, update_debug_state.run_if(in_state(AppMode::Dev)));
    }
}
