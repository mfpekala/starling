use crate::prelude::*;

pub mod background;
pub mod platforms;

pub(super) struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, _app: &mut App) {}
}
