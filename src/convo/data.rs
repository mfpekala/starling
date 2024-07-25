use super::components::*;
use crate::prelude::*;

// MACROS TO MAKE DECLARING THIS SHIT LESS VERBOSE
macro_rules! get_speaker {
    (None) => {
        BoxSpeaker::None
    };
    (Lennox) => {
        BoxSpeaker::Lennox
    };
}
macro_rules! cvbb {
    ([$({
        speaker: $speaker:ident,
        content: $content:expr$(,)?
    },)*]) => {{
        vec![
            $(ConvoBox::new(get_speaker!($speaker), $content),)*
        ]
    }};
}

/// The big boi containing all conversations in the game. Basically just a data dump on the data segment.
/// Nice.
/// NOTE: This returns the conversation in readable order. BUT: the bundle expects it as a stack a.k.a reverse
/// readable order. It is the job of who ever is using this to reverse it
pub(super) fn get_full_convo(state: ConvoState) -> Vec<ConvoBox> {
    match state {
        ConvoState::None => vec![],
        ConvoState::TutorialEggUnwrap => cvbb!([
            {
                speaker: None,
                content: "Yoooo",
            },
            {
                speaker: Lennox,
                content: "Who said that???"
            },
        ]),
    }
}
