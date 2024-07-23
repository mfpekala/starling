use crate::prelude::*;

/// The component that marks the bird entity (protagonist)
/// There should only ever be one of these
#[derive(Component, Debug, Clone, Reflect)]
pub struct Bird;

#[derive(Bundle)]
pub struct BirdBundle {
    name: Name,
    bird: Bird,
    physics: BirdPhysicsBundle,
}
impl BirdBundle {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            name: Name::new("bird"),
            bird: Bird,
            physics: BirdPhysicsBundle::new(pos, vel),
        }
    }
}

pub(super) struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, _app: &mut App) {}
}
