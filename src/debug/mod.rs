use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::prelude::*;

mod commands;
mod dphysics;
mod fps;
pub mod help_text;
mod mouse_pos;

pub use help_text::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
struct DebugState {
    pub show_commands: bool,
    pub show_fps: bool,
    pub show_physics_bounds: bool,
    pub show_mouse_pos: bool,
}
impl Default for DebugState {
    fn default() -> Self {
        Self {
            show_commands: false,
            show_fps: false,
            show_physics_bounds: false,
            show_mouse_pos: false,
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

fn set_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 4.0;
    config.render_layers = SpriteCamera::render_layers();
}

pub(super) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.insert_state(DebugState::default());
        app.add_systems(Startup, set_gizmo_config);

        commands::register_commands_debug(app);
        fps::register_fps_debug(app);
        dphysics::register_physics_debug(app);
        mouse_pos::register_mouse_pos_debug(app);
        // Actually useful for the game
        help_text::register_help_text(app);

        // Debug
        app.insert_resource(DebugInteractive(DebugState::default()));
        app.add_plugins(
            ResourceInspectorPlugin::<DebugInteractive>::new()
                .run_if(input_toggle_active(false, KeyCode::Tab)),
        );
        app.add_plugins(
            ResourceInspectorPlugin::<State<MetaState>>::new()
                .run_if(input_toggle_active(false, KeyCode::Tab)),
        );
        app.add_systems(Update, update_debug_state.run_if(in_state(AppMode::Dev)));
    }
}
