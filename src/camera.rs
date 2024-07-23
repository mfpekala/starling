use crate::prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("post_processing_camera"),
        Camera2dBundle {
            camera: Camera {
                order: 6,
                hdr: true,
                ..default()
            },
            ..default()
        },
        InheritedVisibility::VISIBLE,
    ));
}

pub(super) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}
