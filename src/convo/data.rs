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
    (Ghost) => {
        BoxSpeaker::Ghost
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
                content: "Are you alright?",
            },
            {
                speaker: Lennox,
                content: "*chirp*"
            },
            {
                speaker: None,
                content: "Come on up here."
            },
            {
                speaker: None,
                content: "I want to check your wings."
            },
        ]),
        ConvoState::TutorialLaunchChallengeStart => cvbb!([
            {
                speaker: Ghost,
                content: "You took quite a fall there, little one.",
            },
            {
                speaker: Lennox,
                content: "..."
            },
            {
                speaker: Ghost,
                content: "Not a fall you planned on taking?",
            },
            {
                speaker: Lennox,
                content: "*chirp*"
            },
            {
                speaker: Ghost,
                content: "Well, the good news is your wings look fine.",
            },
            {
                speaker: Ghost,
                content: "More than fine, actually.",
            },
            {
                speaker: Ghost,
                content: "Hey could you do me a favor?",
            },
            {
                speaker: Ghost,
                content: "I want to see how strong you are.",
            },
            {
                speaker: Ghost,
                content: "Launch youself through these goals.",
            },
        ]),
        ConvoState::TutorialLaunchSlowMotionRemark => cvbb!([
            {
                speaker: Ghost,
                content: "Did you notice that, little one?"
            },
            {
                speaker: Lennox,
                content: "*chirp?*"
            },
            {
                speaker: Ghost,
                content: "You have insincts."
            },
            {
                speaker: Ghost,
                content: "Bird instincts."
            },
            {
                speaker: Lennox,
                content: "..."
            },
            {
                speaker: Ghost,
                content: "When you're charging up a launch..."
            },
            {
                speaker: Ghost,
                content: "everything moves slower."
            },
            {
                speaker: Lennox,
                content: "*chirp!*"
            },
            {
                speaker: Ghost,
                content: "Instincts don't last forever, though."
            },
            {
                speaker: Ghost,
                content: "Your body will give out after a couple seconds."
            },
            {
                speaker: Ghost,
                content: "Then you'll launch wherever you happen..."
            },
            {
                speaker: Ghost,
                content: "to be aiming. Yikes."
            },
            {
                speaker: Lennox,
                content: "*chirp.*"
            },
            {
                speaker: Ghost,
                content: "Your instincts can take you far."
            },
            {
                speaker: Ghost,
                content: "Use them wisely."
            },
        ]),
        ConvoState::TutorialLaunchExhaustedWarning => cvbb!([
            {
                speaker: Ghost,
                content: "Woah, slow down there."
            },
            {
                speaker: Lennox,
                content: "*puff*"
            },
            {
                speaker: Ghost,
                content: "Launching yourself is exhausing stuff."
            },
            {
                speaker: Ghost,
                content: "Everytime you touch the ground you get..."
            },
            {
                speaker: Ghost,
                content: "enough energy for two launches."
            },
            {
                speaker: Ghost,
                content: "After that, you're back to flying until..."
            },
            {
                speaker: Ghost,
                content: "you can find a place to rest."
            },
        ]),
        ConvoState::TutorialLaunchFlightWarning => cvbb!([
            {
                speaker: Ghost,
                content: "Not quite."
            },
            {
                speaker: Ghost,
                content: "I said LAUNCH through the goals."
            },
            {
                speaker: Ghost,
                content: "Not fly through them."
            },
            {
                speaker: Ghost,
                content: "Try holding and releasing left mouse."
            },
            {
                speaker: Ghost,
                content: "After you launch, don't touch the keyboard..."
            },
            {
                speaker: Ghost,
                content: "until you hit your target."
            },
        ]),
        ConvoState::TutorialLaunchChallengeCompleted => cvbb!([
            {
                speaker: Ghost,
                content: "Well done!"
            },
            {
                speaker: Ghost,
                content: "Come back up here."
            },
            {
                speaker: Ghost,
                content: "I have something else to teach you."
            },
        ]),
    }
}
