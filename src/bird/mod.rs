#[allow(unused)]
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::prelude::*;
pub mod dragging;
pub mod flight;
pub mod ghost;
mod resource_markers;
pub mod skill_tree;

pub use skill_tree::*;

/// The component that marks the bird entity (protagonist)
/// There should only ever be one of these
#[derive(Component, Debug, Clone, Reflect)]
pub struct Bird {
    launches_left: u32,
    bullets_left: u32,
}
impl Bird {
    pub fn get_launches_left(&self) -> u32 {
        self.launches_left
    }

    pub fn get_bullets_left(&self) -> u32 {
        self.bullets_left
    }
}

#[derive(Bundle)]
pub struct BirdBundle {
    name: Name,
    bird: Bird,
    face_dyno: FaceDyno,
    physics: BirdPhysicsBundle,
    multi: MultiAnimationManager,
}
impl BirdBundle {
    pub fn new(pos: Vec2, vel: Vec2, launches_left: u32, bullets_left: u32) -> Self {
        Self {
            name: Name::new("bird"),
            bird: Bird {
                launches_left,
                bullets_left,
            },
            face_dyno: FaceDyno,
            physics: BirdPhysicsBundle::new(pos, vel),
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        path: "lenny/fly.png",
                        size: (24, 24),
                        length: 3,
                        fps: 16.0,
                    })
                    .with_offset(Vec3::new(-1.0, 0.0, 0.0))
                ),
                (
                    "light",
                    anim_man!({
                        path: "lenny/spotlight.png",
                        size: (48, 48),
                        length: 1,
                    })
                    .with_render_layers(LightCamera::render_layers())
                    .with_scale(Vec2::new(2.5, 2.5))
                ),
            ]),
        }
    }
}

pub(super) struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(flight::BirdFlightConsts::default());
        app.add_plugins(
            ResourceInspectorPlugin::<flight::BirdFlightConsts>::new()
                .run_if(input_toggle_active(false, KeyCode::Tab)),
        );
        app.add_plugins(dragging::DraggingPlugin);
        app.add_plugins(skill_tree::SkillTreePlugin);

        app.add_systems(
            Update,
            (flight::flying,)
                .run_if(in_state(PhysicsState::Active))
                .after(PhysicsSet),
        );

        resource_markers::register_resource_markers(app);
    }
}
