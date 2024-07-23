use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use consts::WINDOW_HEIGHT_f32;

pub mod bird;
pub mod camera;
pub mod consts;
pub mod input;
pub mod math;
pub mod menu;
pub mod physics;
pub mod settings;
pub mod state;

pub mod prelude {
    pub use super::bird::*;
    #[allow(unused_imports)]
    pub use super::camera::*;
    pub use super::consts::*;
    pub use super::input::*;
    pub use super::math::*;
    #[allow(unused_imports)]
    pub use super::menu::*;
    pub use super::physics::*;
    pub use super::settings::*;
    pub use super::state::*;
    pub use bevy::prelude::*;
    pub use bevy::utils::HashMap;
    pub use bevy::utils::HashSet;
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
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(ClearColor(Color::WHITE))
    .add_plugins(WorldInspectorPlugin::new());
    // My plugins
    app.add_plugins(bird::BirdPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(settings::SettingsPlugin)
        .add_plugins(state::StatePlugin);
    app.run();
}
