use bevy::text::Text2dBounds;

use crate::prelude::*;

const DEFAULT_BG_WIDTH: f32 = 160.0;
const DEFAULT_BG_SPACING: f32 = 4.0;
const DEFAULT_BG_BORDER: f32 = 2.0;
const SPEAKER_SIZE: f32 = 24.0;

#[derive(Clone, Copy, Debug, Default, Reflect)]
pub(super) enum BoxPos {
    #[default]
    Default,
}
impl BoxPos {
    fn get_bg_offset(&self) -> Vec3 {
        match self {
            Self::Default => Vec3::new(0.0, -64.0, 0.0),
        }
    }

    fn get_portrait_offset(&self) -> Vec3 {
        match self {
            Self::Default => self.get_bg_offset() + Vec3::new(-62.0, 0.0, 1.0),
        }
    }

    fn get_text_offset_n_bounds(&self, speaker: BoxSpeaker) -> (Vec3, Vec2) {
        match self {
            Self::Default => match speaker {
                BoxSpeaker::None => (
                    self.get_bg_offset() + Vec3::new(0.0, 0.0, 1.0),
                    Vec2::new(
                        DEFAULT_BG_WIDTH - DEFAULT_BG_SPACING * 2.0 - DEFAULT_BG_BORDER * 2.0,
                        SPEAKER_SIZE,
                    ),
                ),
                _ => (
                    self.get_bg_offset()
                        + Vec3::new(SPEAKER_SIZE / 2.0 + DEFAULT_BG_SPACING / 2.0, 0.0, 1.0),
                    Vec2::new(
                        DEFAULT_BG_WIDTH
                            - DEFAULT_BG_SPACING * 3.0
                            - DEFAULT_BG_BORDER * 2.0
                            - SPEAKER_SIZE,
                        SPEAKER_SIZE,
                    ),
                ),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Reflect, PartialEq, Eq)]
pub(super) enum BoxSpeaker {
    #[default]
    None,
    Lennox,
}
impl BoxSpeaker {
    fn get_path(&self) -> String {
        match self {
            Self::None => "sprites/default.png".into(),
            Self::Lennox => "convo/speakers/default.png".into(),
        }
    }
}

#[derive(Clone, Debug, Reflect)]
pub(super) struct BoxContent {
    pub(super) content: String,
}
impl BoxContent {
    pub(super) fn from_content(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

#[derive(Clone, Debug, Reflect)]
pub(super) struct ConvoBox {
    pos: BoxPos,
    speaker: BoxSpeaker,
    content: BoxContent,
}
impl ConvoBox {
    pub(super) fn new(speaker: BoxSpeaker, content: &str) -> Self {
        Self {
            speaker,
            content: BoxContent::from_content(content),
            pos: default(),
        }
    }

    pub(super) fn _fully_custom(pos: BoxPos, speaker: BoxSpeaker, content: &str) -> Self {
        let mut result = Self::new(speaker, content);
        result.pos = pos;
        result
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub(super) struct Convo {
    pub(super) state: ConvoState,
    pub(super) active_box_eid: Option<Entity>,
    /// The data of this conversation. NOTE: Behaves like a stack. First node should be last.
    pub(super) bundles: Vec<ConvoBox>,
}

#[derive(Bundle)]
pub(super) struct ConvoBundle {
    pub(super) name: Name,
    pub(super) convo: Convo,
    pub(super) spatial: SpatialBundle,
}

#[derive(Bundle)]
struct Background {
    name: Name,
    spatial: SpatialBundle,
    multi: MultiAnimationManager,
}
impl Background {
    fn new(pos: BoxPos, speaker: BoxSpeaker) -> Self {
        let bg_offset = pos.get_bg_offset();
        let path = match speaker {
            BoxSpeaker::None => "convo/background_none.png",
            _ => "convo/background_speaker.png",
        };
        Self {
            name: Name::new(format!("background_{:?}", pos)),
            spatial: spat_tran(bg_offset.x, bg_offset.y, bg_offset.z),
            multi: multi!(anim_man!({
                path: path,
                size: (160, 36),
            })
            .with_render_layers(MenuCamera::render_layers())),
        }
    }
}

#[derive(Bundle)]
struct Portrait {
    name: Name,
    spatial: SpatialBundle,
    multi: MultiAnimationManager,
}
impl Portrait {
    fn new(pos: BoxPos, speaker: BoxSpeaker) -> Self {
        let offset = pos.get_portrait_offset();
        Self {
            name: Name::new(format!("portrait_{:?}", speaker)),
            spatial: spat_tran(offset.x, offset.y, offset.z),
            multi: multi!(anim_man!({
                path: speaker.get_path().as_str(),
                size: (22, 22),
            })
            .with_render_layers(MenuCamera::render_layers())),
        }
    }
}

#[derive(Component)]
pub(super) struct FullContent {
    pub(super) content: String,
}

#[derive(Bundle)]
struct Content {
    name: Name,
    full_content: FullContent,
    text: Text2dBundle,
    render_layers: RenderLayers,
}
impl Content {
    fn new(pos: BoxPos, speaker: BoxSpeaker, content: BoxContent) -> Self {
        let (mut offset, bounds) = pos.get_text_offset_n_bounds(speaker);
        offset.x -= bounds.x / 2.0;
        Self {
            name: Name::new(format!("content_{:?}", speaker)),
            full_content: FullContent {
                content: content.content,
            },
            text: Text2dBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        font_size: 10.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_justify(JustifyText::Left),
                text_2d_bounds: Text2dBounds { size: bounds },
                text_anchor: bevy::sprite::Anchor::CenterLeft,
                transform: spat_tran(offset.x, offset.y, offset.z).transform,
                ..default()
            },
            render_layers: MenuCamera::render_layers(),
        }
    }
}

#[derive(Component)]
pub(super) struct ProgressStale;

#[derive(Component, Clone, Debug, Reflect)]
pub(super) struct Progress {
    pub(super) timer: Timer,
    pub(super) absolutely_finished: bool,
}
impl Progress {
    const SECONDS_PER_CHAR: f32 = 0.1;

    fn new(content_length: usize) -> Self {
        Self {
            timer: Timer::from_seconds(
                Self::SECONDS_PER_CHAR * content_length as f32,
                TimerMode::Once,
            ),
            absolutely_finished: false,
        }
    }
}
#[derive(Bundle)]
struct ProgressBundle {
    name: Name,
    progress: Progress,
    spatial: SpatialBundle,
}
impl ProgressBundle {
    fn new(z_adjust: f32, content: &BoxContent) -> Self {
        Self {
            name: Name::new(format!("progress_{}", z_adjust as i32)),
            progress: Progress::new(content.content.len()),
            spatial: spat_tran(0.0, 0.0, z_adjust),
        }
    }
}

impl ConvoBox {
    #[must_use]
    pub fn do_spawn(self, z_adjust: f32, commands: &mut Commands, parent: Entity) -> Entity {
        let new_dad = commands
            .spawn(ProgressBundle::new(z_adjust, &self.content))
            .set_parent(parent)
            .id();
        commands
            .spawn(Background::new(self.pos, self.speaker))
            .set_parent(new_dad);
        if self.speaker != BoxSpeaker::None {
            commands
                .spawn(Portrait::new(self.pos, self.speaker))
                .set_parent(new_dad);
        }
        commands
            .spawn(Content::new(self.pos, self.speaker, self.content.clone()))
            .set_parent(new_dad);
        new_dad
    }
}
