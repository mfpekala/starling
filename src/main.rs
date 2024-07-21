use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use consts::WINDOW_HEIGHT_f32;

pub mod consts;
pub mod input;
pub mod menu;
pub mod state;

pub mod prelude {
    pub use super::consts::*;
    pub use super::input::*;
    pub use super::menu::*;
    pub use super::state::*;
}

fn main() {
    let mut app = App::new();
    // Bevy (or ecosystem) Plugins
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    title: "Starling".to_string(),
                    resolution: WindowResolution::new(consts::WINDOW_WIDTH_f32, WINDOW_HEIGHT_f32),
                    // mode: bevy::window::WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(WorldInspectorPlugin::new());
    // My plugins
    app.add_plugins(input::InputPlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(state::StatePlugin);
    app.run();
}
