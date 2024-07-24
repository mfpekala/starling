use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug, Default)]
pub enum ConvoBoxPos {
    #[default]
    Default,
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub enum ConvoBoxSpeaker {
    #[default]
    None,
}

#[derive(Component, Clone, Debug)]
pub struct ConvoBoxContent {
    pub content: String,
}
impl ConvoBoxContent {
    pub fn from_content(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct ConvoBoxProgress {
    pub timer: Timer,
    pub absolutely_finished: bool,
}

#[derive(Bundle, Clone, Debug)]
pub struct ConvoBoxBundle {
    pos: ConvoBoxPos,
    speaker: ConvoBoxSpeaker,
    content: ConvoBoxContent,
    progress: ConvoBoxProgress,
}
impl ConvoBoxBundle {
    const SECONDS_PER_CHAR: f32 = 0.1;

    pub fn new(speaker: ConvoBoxSpeaker, content: &str) -> Self {
        Self {
            speaker,
            progress: {
                ConvoBoxProgress {
                    timer: Timer::from_seconds(
                        Self::SECONDS_PER_CHAR * content.len() as f32,
                        TimerMode::Once,
                    ),
                    absolutely_finished: false,
                }
            },
            content: ConvoBoxContent::from_content(content),
            pos: default(),
        }
    }

    pub fn force_duration(mut self, duration: f32) -> Self {
        self.progress.timer = Timer::from_seconds(duration, TimerMode::Once);
        self
    }
}

#[derive(Component, Clone, Debug)]
pub struct Convo {
    pub state: ConvoState,
    pub active_box_eid: Option<Entity>,
    pub bundles: VecDeque<ConvoBoxBundle>,
}
