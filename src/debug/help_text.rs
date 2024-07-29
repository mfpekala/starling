use bevy::text::Text2dBounds;

use crate::prelude::*;

const BG_SIZE: UVec2 = UVec2::new(240, 30);

#[derive(Resource, Debug, Clone)]
pub struct HelpText {
    content: Option<String>,
}

#[derive(Bundle)]
pub struct HelpTextBundle {
    name: Name,
    spatial: SpatialBundle,
    multi: MultiAnimationManager,
}

fn startup_help_text(mut commands: Commands, debug_root: Res<DebugRoot>) {
    commands
        .spawn(HelpTextBundle {
            name: Name::new("help_text"),
            spatial: spat_tran(0.0, 60.0, 100.0),
            multi: multi!(anim_man!({
                path: "debug/help_text_background.png",
                size: (240, 30),
            })
            .with_render_layers(MenuCamera::render_layers())),
        })
        .set_parent(debug_root.eid())
        .with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        "The quick brown fox jumped over the lazy dog",
                        TextStyle {
                            font_size: 11.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )
                    .with_justify(JustifyText::Center),
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(230.0, 30.0),
                    },
                    transform: Transform::from_translation(Vec3::new(-2.0, 0.0, 1.0)),
                    ..default()
                },
                MenuCamera::render_layers(),
            ));
        });
}

pub(super) fn register_help_text(app: &mut App) {
    app.insert_resource(HelpText { content: None });
    app.add_systems(Startup, startup_help_text.after(RootInit));
}
