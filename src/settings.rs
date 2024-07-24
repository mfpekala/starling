use crate::prelude::*;

#[derive(Resource)]
pub struct Settings {}

pub(super) struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Settings {});
    }
}
