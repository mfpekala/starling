use crate::prelude::*;

pub mod background;
pub mod go_next;
pub mod heart;
pub mod platforms;

pub use background::*;
pub use go_next::*;
pub use heart::*;
pub use platforms::*;

pub(super) struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        heart::register_hearts(app);
        go_next::register_go_next(app);
    }
}
