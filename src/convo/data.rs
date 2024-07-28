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
    (GhostGreen) => {
        BoxSpeaker::GhostGreen
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
                speaker: GhostGreen,
                content: "You took quite a fall there, little one.",
            },
            {
                speaker: Lennox,
                content: "..."
            },
            {
                speaker: GhostGreen,
                content: "Not a fall you planned on taking?",
            },
            {
                speaker: Lennox,
                content: "*chirp*"
            },
            {
                speaker: GhostGreen,
                content: "Well, the good news is your wings look fine.",
            },
            {
                speaker: GhostGreen,
                content: "More than fine, actually.",
            },
            {
                speaker: GhostGreen,
                content: "Hey could you do me a favor?",
            },
            {
                speaker: GhostGreen,
                content: "I want to see how strong you are.",
            },
            {
                speaker: GhostGreen,
                content: "Launch youself through these goals.",
            },
        ]),
        ConvoState::TutorialLaunchSlowMotionRemark => cvbb!([
            {
                speaker: GhostGreen,
                content: "Did you notice that, little one?"
            },
            {
                speaker: Lennox,
                content: "*chirp?*"
            },
            {
                speaker: GhostGreen,
                content: "You have insincts."
            },
            {
                speaker: Lennox,
                content: "..."
            },
            {
                speaker: GhostGreen,
                content: "When you're charging up a launch..."
            },
            {
                speaker: GhostGreen,
                content: "everything moves slower."
            },
            {
                speaker: Lennox,
                content: "*chirp!*"
            },
            {
                speaker: GhostGreen,
                content: "Instincts don't last forever, though."
            },
            {
                speaker: GhostGreen,
                content: "Your body will give out after a couple seconds."
            },
            {
                speaker: GhostGreen,
                content: "Then you'll be forced to launch."
            },
            {
                speaker: Lennox,
                content: "*chirp.*"
            },
            {
                speaker: GhostGreen,
                content: "Another tip: If you need to stop on a dime..."
            },
            {
                speaker: GhostGreen,
                content: "hold down Space bar. Combining Space with..."
            },
            {
                speaker: GhostGreen,
                content: "launching gives maximum maneuverability."
            },
        ]),
        ConvoState::TutorialLaunchExhaustedWarning => cvbb!([
            {
                speaker: GhostGreen,
                content: "Woah, slow down there."
            },
            {
                speaker: Lennox,
                content: "*puff*"
            },
            {
                speaker: GhostGreen,
                content: "Launching yourself is exhausing stuff."
            },
            {
                speaker: GhostGreen,
                content: "Everytime you touch sticky ground you get..."
            },
            {
                speaker: GhostGreen,
                content: "enough energy for two launches."
            },
            {
                speaker: GhostGreen,
                content: "After that, you're back to flying until..."
            },
            {
                speaker: GhostGreen,
                content: "you can find a place to rest."
            },
        ]),
        ConvoState::TutorialLaunchFlightWarning => cvbb!([
            {
                speaker: GhostGreen,
                content: "Not quite."
            },
            {
                speaker: GhostGreen,
                content: "I said LAUNCH through the goals."
            },
            {
                speaker: GhostGreen,
                content: "Not fly through them."
            },
            {
                speaker: GhostGreen,
                content: "Try holding and releasing left mouse."
            },
            {
                speaker: GhostGreen,
                content: "After you launch, don't touch the keyboard..."
            },
            {
                speaker: GhostGreen,
                content: "until you hit your target."
            },
        ]),
        ConvoState::TutorialLaunchChallengeCompleted => cvbb!([
            {
                speaker: GhostGreen,
                content: "Well done!"
            },
            {
                speaker: GhostGreen,
                content: "Come back up here."
            },
            {
                speaker: GhostGreen,
                content: "I have something else to teach you."
            },
        ]),
        ConvoState::TutorialBulletIntroStart => cvbb!([
            {
                speaker: GhostGreen,
                content: "Flying away from your problems is nice..."
            },
            {
                speaker: GhostGreen,
                content: "but what if you could SHOOT them?"
            },
            {
                speaker: GhostGreen,
                content: "WOAH that sounded dark."
            },
            {
                speaker: GhostGreen,
                content: "I'm sorry."
            },
            {
                speaker: Lennox,
                content: "..."
            },
            {
                speaker: GhostGreen,
                content: "The forest is a dangerous place."
            },
            {
                speaker: GhostGreen,
                content: "I swear I'm not crazy or anything."
            },
            {
                speaker: Lennox,
                content: "*humph*"
            },
            {
                speaker: GhostGreen,
                content: "Anyway..."
            },
            {
                speaker: GhostGreen,
                content: "let's see if you have the nerve to fight back."
            },
        ]),
        ConvoState::TutorialBulletSpeedStart => cvbb!([
            {
                speaker: GhostGreen,
                content: "You're a natural."
            },
            {
                speaker: GhostGreen,
                content: "Let's make things more interesting."
            },
        ]),
        ConvoState::TutorialBulletSpeedTwoBirdsHelp => cvbb!([
            {
                speaker: GhostGreen,
                content: "Struggling?"
            },
            {
                speaker: GhostGreen,
                content: "Try to kill two targets with one bullet thing."
            },
            {
                speaker: GhostGreen,
                content: "P.S: Launching is much faster than flying."
            },
        ]),
        ConvoState::TutorialBulletSpeedTwoBirdsJoke => cvbb!([
            {
                speaker: GhostGreen,
                content: "Two targets with one bullet..."
            },
            {
                speaker: GhostGreen,
                content: "hehe."
            },
        ]),
        ConvoState::TutorialBulletSpeedComplete => cvbb!([
            {
                speaker: GhostGreen,
                content: "Pretty fun, eh?"
            },
            {
                speaker: GhostGreen,
                content: "I mean, only in self-defense obviously."
            },
            {
                speaker: GhostGreen,
                content: "Speaking of, I have one more test for you."
            },
            {
                speaker: GhostGreen,
                content: "And you'll have to forgive me..."
            },
            {
                speaker: GhostGreen,
                content: "because you're probably going to die."
            },
        ]),
        ConvoState::TutorialIntroduceSimp => cvbb!([
            {
                speaker: GhostGreen,
                content: "This here's a Steelbeak Cuckoo."
            },
            {
                speaker: GhostGreen,
                content: "Looks kinda like us..."
            },
            {
                speaker: GhostGreen,
                content: "squaks kinda like us..."
            },
            {
                speaker: GhostGreen,
                content: "but it's probably best you think of him..."
            },
            {
                speaker: GhostGreen,
                content: r#"as a moving, "thinking" target."#
            },
        ]),
        ConvoState::TutorialTakeDamage => cvbb!([
            {
                speaker: GhostGreen,
                content: "Ouch! That looked like it hurt."
            },
            {
                speaker: GhostGreen,
                content: "You can see your health in the bottom left."
            },
            {
                speaker: GhostGreen,
                content: "Try not to get hit again, okay?"
            },
        ]),
        ConvoState::TutorialUnleashSimp => cvbb!([
            {
                speaker: GhostGreen,
                content: "One down, infinity to go..."
            },
        ]),
        ConvoState::TutorialHelloDeath => cvbb!([
            {
                speaker: Ghost,
                content: "AHHHHH"
            },
            {
                speaker: Ghost,
                content: "What the? Where am I?"
            },
            {
                speaker: GhostGreen,
                content: "It's alright. Believe me, I've been there."
            },
            {
                speaker: Ghost,
                content: "Am I dead? Did I just die?"
            },
            {
                speaker: GhostGreen,
                content: "Yes, yes you did."
            },
            {
                speaker: GhostGreen,
                content: "But it's alright. It's a necessary sacrifice..."
            },
            {
                speaker: GhostGreen,
                content: "if we want to get back to the nest."
            },
            {
                speaker: Ghost,
                content: "We? I think you just killed me."
            },
            {
                speaker: GhostGreen,
                content: "You see, when I died, I was only capable..."
            },
            {
                speaker: GhostGreen,
                content: "of charging one launch and one bullet every..."
            },
            {
                speaker: GhostGreen,
                content: "time I landed. But you?"
            },
            {
                speaker: GhostGreen,
                content: "With my help you were so much stronger."
            },
            {
                speaker: Ghost,
                content: "And also dead."
            },
        ]),
        ConvoState::TutorialRightOnQueue => cvbb!([
            {
                speaker: GhostGreen,
                content: "Right on queue."
            },
            {
                speaker: GhostGreen,
                content: "Well?"
            },
            {
                speaker: Ghost,
                content: "Well what?"
            },
            {
                speaker: GhostGreen,
                content: "It's your turn."
            },
            {
                speaker: GhostGreen,
                content: "I'm afraid I must leave you little one."
            },
            {
                speaker: GhostGreen,
                content: "But the forest is in grave danger."
            },
            {
                speaker: GhostGreen,
                content: "It is now your job to save it."
            },
            {
                speaker: GhostGreen,
                content: "Let the cycle continue..."
            },
            {
                speaker: Ghost,
                content: "Wait! Don't leave me!"
            },
            {
                speaker: Ghost,
                content: "I never even got your name."
            },
            {
                speaker: None,
                content: "Deep down, you know."
            },
            {
                speaker: None,
                content: "It's the same as yours..."
            },
            {
                speaker: None,
                content: "Lenny."
            },
        ]),
    }
}
