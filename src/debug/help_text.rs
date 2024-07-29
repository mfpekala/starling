use bevy::text::Text2dBounds;

use crate::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct HelpText {
    pub content: Option<String>,
}
impl HelpText {
    pub fn set(&mut self, content: &str) {
        self.content = Some(content.into());
    }

    pub fn clear(&mut self) {
        self.content = None;
    }
}

#[derive(Component)]
struct HelpTextMarker;

#[derive(Bundle)]
pub struct HelpTextBundle {
    name: Name,
    marker: HelpTextMarker,
    spatial: SpatialBundle,
    multi: MultiAnimationManager,
}

fn startup_help_text(mut commands: Commands, debug_root: Res<DebugRoot>) {
    commands
        .spawn(HelpTextBundle {
            name: Name::new("help_text"),
            marker: HelpTextMarker,
            spatial: spat_tran(0.0, 60.0, 100.0),
            multi: multi!(anim_man!({
                path: "debug/help_text_background.png",
                size: (240, 30),
            })
            .with_hidden(true)
            .with_render_layers(MenuCamera::render_layers())),
        })
        .set_parent(debug_root.eid())
        .with_children(|parent| {
            parent.spawn((
                HelpTextMarker,
                Text2dBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font_size: 11.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )
                    .with_justify(JustifyText::Center),
                    // text_2d_bounds: Text2dBounds {
                    //     size: Vec2::new(230.0, 30.0),
                    // },
                    transform: Transform::from_translation(Vec3::Z),
                    ..default()
                },
                MenuCamera::render_layers(),
            ));
        });
}

fn update_help_text(
    mut multi_q: Query<&mut MultiAnimationManager, With<HelpTextMarker>>,
    mut text_q: Query<&mut Text, With<HelpTextMarker>>,
    help_res: Res<HelpText>,
    mut commands: Commands,
) {
    let (Ok(mut multi), Ok(mut text)) = (multi_q.get_single_mut(), text_q.get_single_mut()) else {
        return;
    };
    match &help_res.content {
        Some(content) => {
            multi.single_mut().set_hidden(false, &mut commands);
            text.sections[0].value = content.clone();
        }
        None => {
            multi.single_mut().set_hidden(true, &mut commands);
            text.sections[0].value = "".into();
        }
    }
}

pub(super) fn register_help_text(app: &mut App) {
    app.insert_resource(HelpText { content: None });
    app.add_systems(Startup, startup_help_text.after(RootInit));
    app.add_systems(Update, update_help_text);
}
