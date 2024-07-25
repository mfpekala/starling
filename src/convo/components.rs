use crate::prelude::*;

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
}

#[derive(Clone, Copy, Debug, Default, Reflect)]
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
    fn new(pos: BoxPos) -> Self {
        let bg_offset = pos.get_bg_offset();
        Self {
            name: Name::new(format!("background_{:?}", pos)),
            spatial: spat_tran(bg_offset.x, bg_offset.y, bg_offset.z),
            multi: multi!(anim_man!({
                path: "convo/background.png",
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
struct FullContent {
    content: String,
}

#[derive(Bundle)]
struct Text {
    name: Name,
    full_content: FullContent,
    text: Text2dBundle,
}
impl Text {
    fn new(pos: BoxPos, content: String) -> Self {
        todo!();
    }
}

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
}
impl ProgressBundle {
    fn new(content: &BoxContent) -> Self {
        Self {
            name: Name::new("progress"),
            progress: Progress::new(content.content.len()),
        }
    }
}

impl ConvoBox {
    #[must_use]
    pub fn do_spawn(self, commands: &mut Commands, parent: Entity) -> Entity {
        commands.spawn(Background::new(self.pos)).set_parent(parent);
        commands
            .spawn(Portrait::new(self.pos, self.speaker))
            .set_parent(parent);
        commands
            .spawn(ProgressBundle::new(&self.content))
            .set_parent(parent)
            .id()
    }
}
