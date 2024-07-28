use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod animation;
pub mod bird;
pub mod camera;
pub mod consts;
pub mod convo;
pub mod debug;
pub mod enemies;
pub mod environment;
pub mod input;
pub mod macros;
pub mod math;
pub mod menu;
pub mod physics;
pub mod room;
pub mod roots;
pub mod settings;
pub mod sound;
pub mod state;
pub mod tutorial;

use consts::*;

pub mod prelude {
    pub use super::animation::*;
    pub use super::bird::*;
    #[allow(unused_imports)]
    pub use super::camera::*;
    pub use super::consts::*;
    pub use super::convo::*;
    #[allow(unused_imports)]
    pub use super::enemies::*;
    pub use super::environment::*;
    pub use super::input::*;
    pub use super::macros::*;
    pub use super::math::*;
    #[allow(unused_imports)]
    pub use super::menu::*;
    pub use super::physics::*;
    pub use super::roots::*;
    pub use super::settings::*;
    pub use super::sound::*;
    pub use super::state::*;
    pub use super::tutorial;
    pub use bevy::color::palettes::tailwind;
    pub use bevy::input::common_conditions::input_toggle_active;
    pub use bevy::prelude::*;
    pub use bevy::render::view::RenderLayers;
    pub use bevy::utils::HashMap;
    pub use bevy::utils::HashSet;
    pub use rand::{thread_rng, Rng};
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::VecDeque;
    pub use std::ops::Range;
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
                    resolution: WindowResolution::new(WINDOW_WIDTH_f32, WINDOW_HEIGHT_f32),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Tab)));
    // My plugins
    app.add_plugins(animation::AnimationPlugin)
        .add_plugins(bird::BirdPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(convo::ConvoPlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(enemies::EnemiesPlugin)
        .add_plugins(environment::EnvironmentPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(room::RoomPlugin)
        .add_plugins(roots::RootPlugin)
        .add_plugins(settings::SettingsPlugin)
        .add_plugins(sound::SoundPlugin)
        .add_plugins(state::StatePlugin)
        .add_plugins(tutorial::TutorialPlugin);
    app.run();
}
