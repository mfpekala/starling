use crate::prelude::*;

pub mod background;
pub mod platforms;

pub use background::*;
pub use platforms::*;

pub(super) struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, _app: &mut App) {}
}
