#[allow(unused)]
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::prelude::*;
pub mod damage;
pub mod dragging;
pub mod egg;
pub mod flight;
pub mod ghost;
mod health;
mod resource_markers;
pub mod skill_tree;

pub use damage::*;
pub use egg::*;
pub use ghost::*;
pub use skill_tree::*;

/// The component that marks the bird entity (protagonist)
/// There should only ever be one of these
#[derive(Component, Debug, Clone, Reflect)]
pub struct Bird {
    launches_left: u32,
    bullets_left: u32,
    health: u32,
    taking_damage: Option<Timer>,
}
impl Bird {
    pub fn get_launches_left(&self) -> u32 {
        self.launches_left
    }

    pub fn get_bullets_left(&self) -> u32 {
        self.bullets_left
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn set_health(&mut self, val: u32) {
        self.health = val;
    }

    pub fn get_taking_damage(&self) -> Option<Timer> {
        self.taking_damage.clone()
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
    pub fn new(pos: Vec2, vel: Vec2, launches_left: u32, bullets_left: u32, health: u32) -> Self {
        Self {
            name: Name::new("bird"),
            bird: Bird {
                launches_left,
                bullets_left,
                health,
                taking_damage: None,
            },
            face_dyno: FaceDyno,
            physics: BirdPhysicsBundle::new(pos, vel),
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        normal: {
                            path: "lenny/fly.png",
                            size: (24, 24),
                            length: 3,
                            fps: 16.0,
                        },
                        taking_damage: {
                            path: "lenny/fly_damage.png",
                            size: (24, 24),
                            length: 3,
                            fps: 12.0,
                        }
                        dead: {
                            path: "lenny/fly_damage.png",
                            size: (24, 24),
                            length: 3,
                            // I'm so lazy
                            fps: 0.0,
                        }
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
pub enum BirdAlive {
    Yes,
    No,
}
impl From<bool> for BirdAlive {
    fn from(value: bool) -> Self {
        if value {
            Self::Yes
        } else {
            Self::No
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
pub enum BirdExists {
    Yes,
    No,
}
impl From<bool> for BirdExists {
    fn from(value: bool) -> Self {
        if value {
            Self::Yes
        } else {
            Self::No
        }
    }
}

fn update_bird_alive_and_exists(
    mut next_bird_alive: ResMut<NextState<BirdAlive>>,
    mut next_bird_exists: ResMut<NextState<BirdExists>>,
    bird_dying: Query<&Bird, (Without<Dying>, Without<Dead>)>,
    bird: Query<&Bird>,
) {
    next_bird_alive.set((!bird_dying.is_empty() && !bird.is_empty()).into());
    next_bird_exists.set((!bird.is_empty()).into());
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
        app.insert_state(BirdAlive::No);
        app.insert_state(BirdExists::No);

        app.add_systems(PreUpdate, update_bird_alive_and_exists);
        app.add_systems(
            Update,
            flight::flying
                .run_if(in_state(PhysicsState::Active))
                .run_if(in_state(BirdAlive::Yes))
                .after(PhysicsSet),
        );

        damage::register_damage(app);
        health::register_health_bar(app);
        resource_markers::register_resource_markers(app);

        app.register_type::<Bird>();
    }
}
