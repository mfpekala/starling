use crate::prelude::*;

pub mod layering;
pub mod manager;
pub mod mat;
pub mod mesh;

use bevy::sprite::Material2dPlugin;
pub use layering::*;
pub use manager::*;
pub use mat::*;
pub use mesh::*;

pub(super) struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        // More plugins
        app.add_plugins(Material2dPlugin::<AnimationMaterial>::default());
        app.add_plugins(Material2dPlugin::<BlendTexturesMaterial>::default());
        app.add_plugins(layering::LayeringPlugin);

        // Manager logic
        manager::register_manager(app);
    }
}
